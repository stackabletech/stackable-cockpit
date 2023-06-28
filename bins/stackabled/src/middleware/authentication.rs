use std::{
    borrow::Borrow,
    collections::HashMap,
    path::{Path, PathBuf},
    sync::{Arc, RwLock},
};

use axum::{
    body::BoxBody,
    extract::Extension,
    headers::{
        authorization::{Basic, Bearer},
        HeaderMapExt,
    },
    http::{HeaderMap, HeaderValue, StatusCode},
    response::IntoResponse,
    Json,
};
use snafu::{ResultExt, Snafu};
use tower_http::validate_request::{ValidateRequest, ValidateRequestHeaderLayer};
use uuid::Uuid;

#[derive(PartialEq, Eq, Hash, Clone)]
pub struct Username(String);
enum PasswordHash {
    Bcrypt(String),
}
#[derive(PartialEq, Eq, Hash, Clone)]
struct SessionToken(String);

impl Borrow<str> for Username {
    fn borrow(&self) -> &str {
        &self.0
    }
}
impl Borrow<str> for SessionToken {
    fn borrow(&self) -> &str {
        &self.0
    }
}

#[derive(Default)]
struct State {
    accounts: HashMap<Username, PasswordHash>,
    sessions: RwLock<HashMap<SessionToken, Username>>,
}

#[derive(Default, Clone)]
pub struct Authenticator {
    state: Arc<State>,
}

#[derive(Debug, Snafu)]
#[snafu(module)]
pub enum HtpasswdError {
    #[snafu(display("failed to read htpasswd file at {path:?}"))]
    Read {
        source: std::io::Error,
        path: PathBuf,
    },
    #[snafu(display("malformed htpasswd entry on line {line}"))]
    Entry {
        source: HtpasswdEntryError,
        line: usize,
    },
}
#[derive(Debug, Snafu)]
#[snafu(module)]
pub enum HtpasswdEntryError {
    #[snafu(display("invalid hash type (only bcrypt is currently supported)"))]
    InvalidHashType,
    #[snafu(display("no username/password separator"))]
    NoSeparator,
}

impl Authenticator {
    pub fn load_htpasswd(path: &Path) -> Result<Self, HtpasswdError> {
        use htpasswd_entry_error::*;
        use htpasswd_error::*;
        let htaccess = std::fs::read_to_string(path).context(ReadSnafu { path })?;
        let mut accounts = HashMap::new();
        for (line, entry) in htaccess.lines().enumerate() {
            if let Some((username, prefixed_pw_hash)) = entry.split_once(':') {
                if prefixed_pw_hash.starts_with("$2y$") {
                    accounts.insert(
                        Username(username.to_string()),
                        PasswordHash::Bcrypt(prefixed_pw_hash.to_string()),
                    );
                    Ok(())
                } else {
                    InvalidHashTypeSnafu.fail()
                }
            } else {
                NoSeparatorSnafu.fail()
            }
            .context(EntrySnafu { line })?
        }
        Ok(Self {
            state: Arc::new(State {
                accounts,
                sessions: RwLock::new(HashMap::new()),
            }),
        })
    }

    fn validate_login(&self, headers: &HeaderMap<HeaderValue>) -> Result<Username, &'static str> {
        if let Some(session_token) = headers.typed_get::<axum::headers::Authorization<Bearer>>() {
            if let Some(username) = self
                .state
                .sessions
                .read()
                .unwrap()
                .get(session_token.0.token())
            {
                Ok(username.clone())
            } else {
                Err("request authentication failed: invalid session token")
            }
        } else if let Some(basic_creds) = headers.typed_get::<axum::headers::Authorization<Basic>>()
        {
            let username = basic_creds.username();
            let password_hash = self.state.accounts.get(username);
            if password_hash
                .and_then(|PasswordHash::Bcrypt(hash)| {
                    bcrypt::verify(basic_creds.password(), hash).ok()
                })
                .unwrap_or(false)
            {
                Ok(Username(username.to_string()))
            } else {
                Err("request authentication failed: invalid username or password")
            }
        } else {
            Err("request authentication failed: no valid authentication method presented")
        }
    }

    pub fn layer(self) -> ValidateRequestHeaderLayer<Self> {
        ValidateRequestHeaderLayer::custom(self)
    }
}

impl<B> ValidateRequest<B> for Authenticator {
    type ResponseBody = BoxBody;

    fn validate(
        &mut self,
        request: &mut axum::http::Request<B>,
    ) -> Result<(), axum::http::Response<Self::ResponseBody>> {
        let username = self
            .validate_login(request.headers())
            .map_err(|msg| (StatusCode::UNAUTHORIZED, Json(msg)).into_response())?;
        let exts = request.extensions_mut();
        exts.insert(username);
        exts.insert(self.clone());
        Ok(())
    }
}

#[utoipa::path(post, path = "/login", responses((status = 200, body = String), (status = 401, body = String)))]
pub async fn log_in(
    Extension(username): Extension<Username>,
    Extension(authn): Extension<Authenticator>,
) -> (StatusCode, Json<String>) {
    let token = SessionToken(Uuid::new_v4().to_string());
    authn
        .state
        .sessions
        .write()
        .unwrap()
        .insert(token.clone(), username);
    (StatusCode::OK, Json(token.0))
}
