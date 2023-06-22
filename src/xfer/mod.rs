mod cache;

pub use cache::*;

pub struct HttpClient {
    pub(crate) client: reqwest::Client,
    pub(crate) cache: Cache,
}
