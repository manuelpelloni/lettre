#[cfg(test)]
#[cfg(all(feature = "smtp-transport", feature = "builder"))]
mod test {
    use lettre::{Message, SmtpTransport, Transport};

    #[test]
    fn smtp_transport_simple() {
        let email = Message::builder()
            .from("NoBody <nobody@domain.tld>".parse().unwrap())
            .reply_to("Yuin <yuin@domain.tld>".parse().unwrap())
            .to("Hei <hei@domain.tld>".parse().unwrap())
            .subject("Happy new year")
            .body("Be happy!")
            .unwrap();
        SmtpTransport::builder_dangerous("127.0.0.1")
            .port(2525)
            .build()
            .send(&email)
            .unwrap();
    }
}
