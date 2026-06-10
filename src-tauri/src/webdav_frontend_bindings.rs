use crate::file_cache::{InMemoryFileCache, NoteTakerFile};
use crate::state::AppState;
use crate::webdav::{get_webdav_tree, WebdavClient};
use reqwest_dav::re_exports::serde_json;
use tauri::State;

#[derive(Debug)]
pub struct WebdavFrontend {
    client: Option<WebdavClient>,
}
impl Default for WebdavFrontend {
    fn default() -> Self {
        WebdavFrontend { client: None }
    }
}

impl WebdavFrontend {
    pub async fn init(
        &mut self,
        username: String,
        pass: String,
        host: String,
        notes_path: String,
    ) -> Result<(), String> {
        match WebdavClient::build_client(username, pass, host, notes_path).await {
            Ok(val) => {
                self.client = Some(val);
                Ok(())
            }
            Err(e) => Err(format!("Could not init webdav client: {}", e)),
        }
    }
}

#[tauri::command]
pub async fn init_webdav_with_creds(
    state: State<'_, AppState>,
    username: String,
    password: String,
    host: String,
    notes_path: String,
) -> Result<(), String> {
    let mut stater = state.lock().await;
    let res = stater
        .webdav_frontend
        .init(username, password, host, notes_path)
        .await;
    stater.file_cache = Some(InMemoryFileCache::new(
        stater.webdav_frontend.client.clone().unwrap(),
    ));
    println!(
        "{:?}",
        serde_json::to_string(
            &stater
                .webdav_frontend
                .client
                .as_ref()
                .unwrap()
                .list_notes()
                .await?
        )
    );

    return res;
}

#[tauri::command]
pub async fn get_webdav_notes_tree(state: State<'_, AppState>) -> Result<String, String> {
    let mut stater = state.lock().await;
    if let Some(val) = &stater.webdav_frontend.client {
        let t = get_webdav_tree(&val).await;
        if let Ok(tree) = t {
            let disp_tree = tree.build_disp_tree();
            if let Ok(disp_result) = disp_tree {
                return Ok(disp_result
                    .pointer(val.get_notes_path())
                    .unwrap()
                    .to_string());
            }
        }
    }
    Err("Could not get notes".to_string())
}

//TODO: Sync filesystem and local
//TODO: Better file tree system
//TODO: Local files module
