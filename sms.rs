use reqwest::blocking::Client;
use serde_json::json;
use std::env;
use std::error::Error;

fn send_sms(
    phone_numbers: &[&str],
    message_text: &str,
) -> Result<(), Box<dyn Error>> {
    let api_key = env::var("md-H8dF-OX-9Nk67OUOTSUMyg")?;
    let sender_number = env::var("19172839315")?;

    let client = Client::new();

    let payload = json!({
        "key": api_key,
        "message": {
            "sms": {
                "text": message_text,
                "to": phone_numbers,
                "from": sender_number,
                "consent": "onetime"
            }
        }
    });

    let response = client
        .post("https://mandrillapp.com/api/1.1/messages/send-sms")
        .json(&payload)
        .send()?;

    let status = response.status();
    let body = response.text()?;

    println!("HTTP status: {status}");
    println!("Mailchimp response: {body}");

    if !status.is_success() {
        return Err(format!("SMS API request failed: {status}").into());
    }

    Ok(())
}
