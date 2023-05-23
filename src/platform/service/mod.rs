use indexmap::IndexMap;
use serde::Serialize;
use snafu::Snafu;

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct InstalledProduct {
    /// Name of the service.
    pub name: String,

    /// Some CRDs are cluster scoped.
    pub namespace: Option<String>,

    /// List of service endpoints. The key describes the use of the endpoint
    /// like `web-ui`, `grpc` or `http`. The value is a URL at which the
    /// endpoint is accessible.
    pub endpoints: IndexMap<String, String>,

    // List of extra information about the service.
    pub extra_info: Vec<String>,
}

#[derive(Debug, Snafu)]
pub enum ServiceError {}

/// [`ServiceListOptions`] describes available options when listing deployed
/// services.
pub struct ServiceListOptions {
    /// Toggle wether to show credentials / secrets in the output. This defaults
    /// to `false` because of security reasons. Users need to explicitly tell
    /// the ctl or the web UI to show these credentials.
    pub show_credentials: bool,

    /// Toggle wether to show product versions in the output. This defaults to
    /// `true`.
    pub show_versions: bool,
}

impl Default for ServiceListOptions {
    fn default() -> Self {
        Self {
            show_credentials: false,
            show_versions: true,
        }
    }
}

pub type ServiceList = IndexMap<String, Vec<InstalledProduct>>;

/// Lists all installed services. If `namespace` is [`None`], services from ALL
/// namespaces are returned. If `namespace` is [`Some`], only services installed
/// in the specified namespace are returned. The `options` allow further
/// customization of the returned information.
pub fn list_services(
    _namespace: Option<&str>,
    _options: ServiceListOptions,
) -> Result<ServiceList, ServiceError> {
    Ok(IndexMap::new())
}
