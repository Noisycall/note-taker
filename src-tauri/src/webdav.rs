#[cfg(test)]
mod webdav_tests {
    use lazy_static::lazy_static;
    lazy_static!{
        pub static ref USERNAME: String = std::env::var("USERNAME").unwrap();
        pub static ref PASS: String = std::env::var("PASS").unwrap();
        pub static ref DOMAIN: String = std::env::var("DOMAIN").unwrap();
    }
    use reqwest_dav::{
        Auth,
        ClientBuilder,
        Depth,
        Error,
        list_cmd::ListEntity,
        re_exports::serde_json
    };

    #[tokio::test]
    async fn it_works() -> Result<(), Error> {
        // build a client
        let client = ClientBuilder::new()
            .set_host(DOMAIN.to_string())
            .set_auth(Auth::Basic(USERNAME.to_string(), PASS.to_owned()))
            .build()?;

        // list files
        println!(
            "{}",
            serde_json::to_string(&client.list(&format!("/files/{}/",*USERNAME), Depth::Number(1)).await?).unwrap()
        );
        Ok(())
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
        let filepath = format!("/remote.php/dav/files/{}/testfile_verification.txt",*USERNAME).to_string();
        client.put(&filepath, test_content).await?;

        // list files to confirm the new file exists
        let response = client.list(&format!("/remote.php/dav/files/{}/",*USERNAME), Depth::Number(1)).await?;
        println!("Files in user folder:\n{}", serde_json::to_string_pretty(&response).unwrap());
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
        println!("{:?}",client.delete(&file_href).await?);
        Ok(())
    }
}