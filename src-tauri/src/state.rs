use crate::file_cache::InMemoryFileCache;
use crate::webdav_frontend_bindings::WebdavFrontend;

#[derive(Debug)]
pub struct AppData {
    pub webdav_frontend: WebdavFrontend,
    pub file_cache: Option<InMemoryFileCache>,
}
impl AppData {
    pub fn new() -> AppData {
        return AppData {
            webdav_frontend: WebdavFrontend::default(),
            file_cache: None,
        };
    }
}
pub type AppState = tokio::sync::Mutex<AppData>;
