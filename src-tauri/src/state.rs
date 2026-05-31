use crate::webdav_frontend_bindings::WebdavFrontend;

#[derive(Default, Debug)]
pub struct AppData {
    pub webdav_frontend: WebdavFrontend,
}

pub type AppState = tokio::sync::Mutex<AppData>;
