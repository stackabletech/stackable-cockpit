use std::{
    borrow::Borrow,
    collections::HashMap,
    path::Path,
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
use tower_http::validate_request::{ValidateRequest, ValidateRequestHeaderLayer};
use uuid::Uuid;

pub use self::htpasswd::Error as HtpasswdError;

mod htpasswd;

#[derive(PartialEq, Eq, Hash, Clone, Debug)]
pub struct Username(String);
#[cfg_attr(test, derive(PartialEq, Debug))]
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

impl Authenticator {
    pub fn load_htpasswd(path: &Path) -> Result<Self, HtpasswdError> {
        Ok(Self {
            state: Arc::new(State {
                accounts: htpasswd::load(path)?,
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
