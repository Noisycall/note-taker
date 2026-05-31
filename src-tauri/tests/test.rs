#[cfg(test)]
mod webdav_to_tree {
    use lazy_static::lazy_static;
    use note_taker_lib::webdav::WebdavClient;
    lazy_static! {
        pub static ref USERNAME: String = std::env::var("WEBDAV_USERNAME").unwrap();
        pub static ref PASS: String = std::env::var("WEBDAV_PASS").unwrap();
        pub static ref DOMAIN: String = std::env::var("WEBDAV_DOMAIN").unwrap();
    }
    const NOTES_PATH: &str = "/notes";

    #[tokio::test]
    async fn test1() {
        println!("wow {}", DOMAIN.to_string());
        println!("wow {}", USERNAME.to_string());
        println!("wow {}", PASS.to_string());
        println!("wow {}", NOTES_PATH.to_string());
        let client = WebdavClient::build_client(
            USERNAME.to_string(),
            PASS.to_string(),
            DOMAIN.to_string(),
            NOTES_PATH.to_string(),
        )
        .await
        .unwrap();
        println!("Client got");
        let val = client.list_notes().await;
        println!("helllo {:?}", val)
    }
}
