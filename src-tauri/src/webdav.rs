use log::debug;
use reqwest_dav::list_cmd::ListEntity;
use reqwest_dav::{Auth, Client, ClientBuilder, Depth};
use std::path::PathBuf;

#[derive(Debug)]
struct WebdavClient {
    client: Client,
    notes_path_from_root: String,
}

//TODO: Refactor to use &str
pub fn normalize_note_path(prefix: String, note_path: String) -> Result<String, String> {
    let mut mutated_note_path = PathBuf::from(note_path);
    if mutated_note_path.is_absolute() {
        mutated_note_path = mutated_note_path.strip_prefix("/").unwrap().to_path_buf();
    }
    let mut val = PathBuf::from(prefix)
        .join(mutated_note_path)
        .with_extension("md");
    if !val.is_absolute() {
        val = PathBuf::from("/").join(val);
    }
    match val.to_str() {
        None => Err(format!("Could not parse created path: {:?}", val)),
        Some(value) => {
            return Ok(value.to_string());
        }
    }
}

#[cfg(test)]
mod test_normalize_note_path {
    use crate::webdav::normalize_note_path;

    #[test]
    fn it_handles_both_being_root() {
        let val = normalize_note_path(
            "/remote.php/dav/files/username/notes_folder/".to_string(),
            "/inner1/file1.md".to_string(),
        );
        assert_eq!(
            val,
            Ok("/remote.php/dav/files/username/notes_folder/inner1/file1.md".to_string())
        )
    }

    #[test]
    fn it_handles_both_being_non_root() {
        let val = normalize_note_path(
            "remote.php/dav/files/username/notes_folder/".to_string(),
            "inner1/file1.md".to_string(),
        );
        assert_eq!(
            val,
            Ok("/remote.php/dav/files/username/notes_folder/inner1/file1.md".to_string())
        )
    }

    #[test]
    fn it_handles_no_trailing_slash_on_prefix() {
        let val = normalize_note_path(
            "/remote.php/dav/files/username/notes_folder".to_string(),
            "/inner1/file1.md".to_string(),
        );
        assert_eq!(
            val,
            Ok("/remote.php/dav/files/username/notes_folder/inner1/file1.md".to_string())
        )
    }

    #[test]
    fn it_handles_both_being_non_root_no_trailing() {
        let val = normalize_note_path(
            "remote.php/dav/files/username/notes_folder".to_string(),
            "inner1/file1.md".to_string(),
        );
        assert_eq!(
            val,
            Ok("/remote.php/dav/files/username/notes_folder/inner1/file1.md".to_string())
        )
    }
}

impl WebdavClient {
    pub async fn create_note(&self, note_name: String) -> Result<(), String> {
        let note_path =
            normalize_note_path(self.notes_path_from_root.to_string(), note_name.to_string());

        //TODO: Refactor this error handling
        if let Err(err) = note_path {
            return Err(format!(
                "Failure to parse note path when creating note: {}",
                err
            ));
        }
        let create_note_result = self.client.put(&note_path?, "").await;
        match create_note_result {
            Ok(_) => Ok(()),
            Err(e) => Err(format!(
                "Error creating note with name {}, network error: {}",
                note_name, e
            )),
        }
    }
    pub async fn list_notes(&self) -> Result<Vec<String>, String> {
        let mut notes = Vec::<String>::new();
        let val = &self
            .client
            .list(&self.notes_path_from_root, Depth::Number(1))
            .await;
        match val {
            Ok(list) => list.iter().for_each(|note| match note {
                ListEntity::File(file) => notes.push(file.href.to_string()),
                ListEntity::Folder(_) => {}
            }),
            Err(e) => {
                let err = format!("Error while reading notes list: {}", e);
                debug!("{}", err);
                return Err(err);
            }
        }
        Ok(notes)
    }
    pub async fn build_client(
        user: String,
        pass: String,
        domain: String,
        path_to_notes: String,
    ) -> Result<Self, String> {
        let cl = ClientBuilder::new()
            .set_host(domain)
            .set_auth(Auth::Basic(user.clone(), pass))
            .build();
        //TODO: Make folder if it doesn't already exist
        match cl {
            Ok(c) => {
                Ok(WebdavClient {
                    //TODO: Make path joining safer
                    notes_path_from_root: format!("/remote.php/dav/files/{}", user).to_string()
                        + &path_to_notes,
                    client: c,
                })
            }
            Err(e) => {
                let err = format!("Error building client: {}", e);
                debug!("{}", err);
                Err(err)
            }
        }
    }
    pub async fn check_conn(&self) -> bool {
        let val = self
            .client
            .list(&self.notes_path_from_root, Depth::Number(0))
            .await;
        match val {
            Ok(_) => true,
            Err(e) => {
                debug!("Can't connect: {}", e);
                false
            }
        }
    }
}

#[cfg(test)]
mod webdav_tests {
    use lazy_static::lazy_static;
    lazy_static! {
        pub static ref USERNAME: String = std::env::var("USERNAME").unwrap();
        pub static ref PASS: String = std::env::var("PASS").unwrap();
        pub static ref DOMAIN: String = std::env::var("DOMAIN").unwrap();
    }
    const NOTES_PATH: &str = "/notes";
    use crate::webdav::WebdavClient;
    use reqwest_dav::{
        list_cmd::ListEntity, re_exports::serde_json, Auth, ClientBuilder, Depth, Error,
    };

    #[tokio::test]
    async fn verify_library_functionality() -> Result<(), Error> {
        // build a client
        let client = ClientBuilder::new()
            .set_host(DOMAIN.to_string())
            .set_auth(Auth::Basic(USERNAME.to_string(), PASS.to_owned()))
            .build()?;

        // list files
        println!(
            "{}",
            serde_json::to_string(
                &client
                    .list(
                        &format!("/remote.php/dav/files/{}/", *USERNAME),
                        Depth::Number(1)
                    )
                    .await?
            )
            .unwrap()
        );
        Ok(())
    }

    #[tokio::test]
    async fn test_check_conn() {
        let client = WebdavClient::build_client(
            USERNAME.to_string(),
            PASS.to_string(),
            DOMAIN.to_string(),
            NOTES_PATH.to_string(),
        )
        .await
        .unwrap();
        assert_eq!(true, client.check_conn().await);
    }

    #[tokio::test]
    async fn creates_and_verifies_file_in_user_folder() -> Result<(), Error> {
        // build a client
        let client = ClientBuilder::new()
            .set_host(DOMAIN.to_string())
            .set_auth(Auth::Basic(USERNAME.to_owned(), PASS.to_owned()))
            .build()?;

        // create content for the test file
        let test_content = "Hello from WebDAV! This is a test file.".to_string();

        // put/create a new file in user folder
        let filepath = format!(
            "/remote.php/dav/files/{}/testfile_verification.txt",
            *USERNAME
        )
        .to_string();
        client.put(&filepath, test_content).await?;

        // list files to confirm the new file exists
        let response = client
            .list(
                &format!("/remote.php/dav/files/{}/", *USERNAME),
                Depth::Number(1),
            )
            .await?;
        println!(
            "Files in user folder:\n{}",
            serde_json::to_string_pretty(&response).unwrap()
        );
        let mut flag = false;
        let mut file_href = "".to_string();
        for resp in response {
            match resp {
                ListEntity::File(a) => {
                    if a.href.contains("testfile_verification.txt") {
                        file_href = a.href.to_string();
                        println!("{}", file_href);
                        flag = true;
                    }
                }
                ListEntity::Folder(_) => {}
            }
        }
        assert!(flag);
        println!("{:?}", client.delete(&file_href).await?);
        Ok(())
    }

    #[tokio::test]
    async fn test3() {
        let client = WebdavClient::build_client(
            USERNAME.to_string(),
            PASS.to_string(),
            DOMAIN.to_string(),
            NOTES_PATH.to_string(),
        )
        .await
        .unwrap();
        assert!(client.list_notes().await.unwrap().len() > 0);
    }

    #[tokio::test]
    async fn it_creates_notes() {
        let client = WebdavClient::build_client(
            USERNAME.to_string(),
            PASS.to_string(),
            DOMAIN.to_string(),
            NOTES_PATH.to_string(),
        )
        .await
        .unwrap();
        assert_eq!(client.create_note("wow1.md".to_string()).await, Ok(()));
    }
}
