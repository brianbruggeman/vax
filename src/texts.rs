//! Methods for SMS texting
//!

// /////////////////////////////////////////////////////////////////////
use lettre::transport::smtp::authentication::Credentials;
use lettre::{Message, SmtpTransport, Transport};

#[derive(Clone, Debug, PartialEq)]
pub enum Carrier {
    Att(String),
    Sprint(String),
    Tmobile(String),
    Verizon(String),
    None,
}

pub fn send_text(message: &str, source: &str, phone: &str, carrier: &str) {
    let carrier: Carrier = match carrier {
        "att" => Carrier::Att(phone.to_string()),
        "sprint" => Carrier::Sprint(phone.to_string()),
        "tmobile" => Carrier::Tmobile(phone.to_string()),
        "verizon" => Carrier::Verizon(phone.to_string()),
        "" => Carrier::None,
        _ => panic!(
            "Only the following carriers are supported: `att`, `sprint`, `tmobile`, `verizon`"
        ),
    };
    let destination = match carrier {
        Carrier::Att(phone) => format!("{}@mms.att.net", phone),
        Carrier::Sprint(phone) => format!("{}@pm.sprint.com", phone),
        Carrier::Tmobile(phone) => format!("{}@tmomail.net", phone),
        Carrier::Verizon(phone) => format!("{}@vtext.com", phone),
        _ => return,
    };
    let email = Message::builder()
        .from(format!("Me <{}>", source).parse().unwrap())
        .to(format!("Phone <{}>", destination).parse().unwrap())
        .subject("COVID-19 Vaccination Sign-up")
        .body(message.to_string())
        .unwrap();

    // let creds = Credentials::new("smtp_username".to_string(), "smtp_password".to_string());

    // Open a remote connection to gmail
    let mailer = SmtpTransport::relay("smtp.gmail.com")
        .unwrap()
        // .credentials(creds)
        .build();

    // Send the email
    match mailer.send(&email) {
        Ok(_) => println!("Email sent successfully!"),
        Err(e) => panic!("Could not send email: {:?}", e),
    }
}
