//! Methods for SMS texting
//!

// /////////////////////////////////////////////////////////////////////
use lettre::{EmailAddress, Envelope, SendableEmail, SmtpClient, Transport};

#[derive(Clone, Debug, PartialEq)]
pub enum Carrier {
    Att(String),
    Sprint(String),
    Tmobile(String),
    Verizon(String),
    None,
}

pub fn send_text(message: &str, source: &str, carrier: Carrier) {
    let destination = match carrier {
        Carrier::Att(phone) => format!("{}@mms.att.net", phone),
        Carrier::Sprint(phone) => format!("{}@pm.sprint.com", phone),
        Carrier::Tmobile(phone) => format!("{}@tmomail.net", phone),
        Carrier::Verizon(phone) => format!("{}@vtext.com", phone),
        _ => return,
    };
    let email = SendableEmail::new(
        Envelope::new(
            Some(EmailAddress::new(source.to_string()).unwrap()),
            vec![EmailAddress::new(destination).unwrap()],
        )
        .unwrap(),
        "id".to_string(),
        message.to_string().into_bytes(),
    );

    // Open a local connection on port 25
    let mut mailer = SmtpClient::new_unencrypted_localhost().unwrap().transport();

    // Send the email
    let result = mailer.send(email);
    if result.is_ok() {
        println!("Text sent");
    } else {
        println!("Could not send text: {:?}", result);
    }
}
