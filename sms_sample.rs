
use reqwest::blocking::Client;
use serde_json::json;
use std::env;
use std::error::Error;

fn send_sms(
    phone_numbers: &[&str],
    message_text: &str,
) -> Result<(), Box<dyn Error>> {
    let api_key = env::var("MAILCHIMP_TRANSACTIONAL_API_KEY")?;
    let sender_number = env::var("MAILCHIMP_SMS_SENDER")?;

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

/*
Below explains how to use the code and target a number.  my recommendation is to use variables instead of the actual numbers and find a secure way of storing the API key and the SMS sender number in lines 11 & 12

fn main() {
    let recipients = [
        "+18456495711",
    ];

    match send_sms(&recipients, "Heck Yeah!!! This is Chinnonye from secure systems engineering! I think we got it! Also, Hello from RUST and mailchimp.") {
        Ok(()) => println!("SMS request completed."),
        Err(error) => eprintln!("Error sending SMS: {error}"),
    }
}

*/
