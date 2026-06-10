use crate::webdav::WebdavClient;
use serde::{Deserialize, Serialize};
use std::cell::RefCell;
use std::collections::HashMap;
use std::sync::RwLock;

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct NoteTakerFile {
    pub id: String,
    pub contents: String,
}

#[derive(Debug)]
pub struct InMemoryFileCache {
    files: HashMap<String, NoteTakerFile>,
    client: WebdavClient,
}
impl InMemoryFileCache {
    pub fn new(client: WebdavClient) -> InMemoryFileCache {
        return InMemoryFileCache {
            files: HashMap::new(),
            client,
        };
    }
}

impl InMemoryFileCache {
    pub async fn get_file(&mut self, id: &str) -> Result<NoteTakerFile, String> {
        if self.files.contains_key(id) {
            return Ok(self.files.get(id).unwrap().clone());
        }
        let note_result = self.client.get_note(id).await;
        match note_result {
            Ok(note) => {
                let note_taker_file = NoteTakerFile {
                    id: id.to_string(),
                    contents: note,
                };
                self.files
                    .insert(note_taker_file.id.clone(), note_taker_file.clone());
                Ok(note_taker_file)
            }
            Err(err) => Err(format!("Could not get file in InMemoryFileCache: {}", err)),
        }
    }
}
