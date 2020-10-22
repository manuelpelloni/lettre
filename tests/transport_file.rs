#[cfg(test)]
#[cfg(all(feature = "file-transport", feature = "builder"))]
mod test {
    use lettre::{transport::file::FileTransport, Message};
    use std::{
        env::temp_dir,
        fs::{remove_file, File},
        io::Read,
    };

    #[cfg(feature = "tokio02")]
    use tokio02_crate as tokio;

    #[test]
    fn file_transport() {
        use lettre::Transport;
        let sender = FileTransport::new(temp_dir());
        let email = Message::builder()
            .from("NoBody <nobody@domain.tld>".parse().unwrap())
            .reply_to("Yuin <yuin@domain.tld>".parse().unwrap())
            .to("Hei <hei@domain.tld>".parse().unwrap())
            .subject("Happy new year")
            .date("Tue, 15 Nov 1994 08:12:31 GMT".parse().unwrap())
            .body("Be happy!")
            .unwrap();

        let result = sender.send(&email);
        let id = result.unwrap();

        let file = temp_dir().join(format!("{}.json", id));
        let mut f = File::open(file.clone()).unwrap();
        let mut buffer = String::new();
        let _ = f.read_to_string(&mut buffer);

        assert_eq!(
            buffer,
            "{\"envelope\":{\"forward_path\":[\"hei@domain.tld\"],\"reverse_path\":\"nobody@domain.tld\"},\"raw_message\":null,\"message\":\"From: NoBody <nobody@domain.tld>\\r\\nReply-To: Yuin <yuin@domain.tld>\\r\\nTo: Hei <hei@domain.tld>\\r\\nSubject: Happy new year\\r\\nDate: Tue, 15 Nov 1994 08:12:31 GMT\\r\\n\\r\\nBe happy!\"}");
        remove_file(file).unwrap();
    }

    #[cfg(feature = "async-std1")]
    #[async_attributes::test]
    async fn file_transport_asyncstd1() {
        use lettre::AsyncStd1Transport;

        let sender = FileTransport::new(temp_dir());
        let email = Message::builder()
            .from("NoBody <nobody@domain.tld>".parse().unwrap())
            .reply_to("Yuin <yuin@domain.tld>".parse().unwrap())
            .to("Hei <hei@domain.tld>".parse().unwrap())
            .subject("Happy new year")
            .date("Tue, 15 Nov 1994 08:12:31 GMT".parse().unwrap())
            .body("Be happy!")
            .unwrap();

        let result = sender.send(email).await;
        let id = result.unwrap();

        let file = temp_dir().join(format!("{}.json", id));
        let mut f = File::open(file.clone()).unwrap();
        let mut buffer = String::new();
        let _ = f.read_to_string(&mut buffer);

        assert_eq!(
            buffer,
            "{\"envelope\":{\"forward_path\":[\"hei@domain.tld\"],\"reverse_path\":\"nobody@domain.tld\"},\"raw_message\":null,\"message\":\"From: NoBody <nobody@domain.tld>\\r\\nReply-To: Yuin <yuin@domain.tld>\\r\\nTo: Hei <hei@domain.tld>\\r\\nSubject: Happy new year\\r\\nDate: Tue, 15 Nov 1994 08:12:31 GMT\\r\\n\\r\\nBe happy!\"}");
        remove_file(file).unwrap();
    }

    #[cfg(feature = "tokio02")]
    #[tokio::test]
    async fn file_transport_tokio02() {
        use lettre::Tokio02Transport;

        let sender = FileTransport::new(temp_dir());
        let email = Message::builder()
            .from("NoBody <nobody@domain.tld>".parse().unwrap())
            .reply_to("Yuin <yuin@domain.tld>".parse().unwrap())
            .to("Hei <hei@domain.tld>".parse().unwrap())
            .subject("Happy new year")
            .date("Tue, 15 Nov 1994 08:12:31 GMT".parse().unwrap())
            .body("Be happy!")
            .unwrap();

        let result = sender.send(email).await;
        let id = result.unwrap();

        let file = temp_dir().join(format!("{}.json", id));
        let mut f = File::open(file.clone()).unwrap();
        let mut buffer = String::new();
        let _ = f.read_to_string(&mut buffer);

        assert_eq!(
            buffer,
            "{\"envelope\":{\"forward_path\":[\"hei@domain.tld\"],\"reverse_path\":\"nobody@domain.tld\"},\"raw_message\":null,\"message\":\"From: NoBody <nobody@domain.tld>\\r\\nReply-To: Yuin <yuin@domain.tld>\\r\\nTo: Hei <hei@domain.tld>\\r\\nSubject: Happy new year\\r\\nDate: Tue, 15 Nov 1994 08:12:31 GMT\\r\\n\\r\\nBe happy!\"}");
        remove_file(file).unwrap();
    }
}
