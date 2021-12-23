//! kraken-websockets-token - Convenience crate to get Kraken WebSockets API tokens
//!
//! # Example
//!
//! ```ignore
//! use kraken_websockets_token::get_websockets_token;
//!
//! fn main() {
//!
//!   const API_SECRET: &str = "<my API secret>";
//!   const API_KEY: &str = "<my API key>";
//!
//!   match get_websockets_token(API_SECRET, API_KEY) {
//!     Ok(token) => println!("My token is: {}", token),
//!     Err(err) => println!("There was an error: {}", err),
//!   }
//! }
//! ```

use hmac::Mac;
use sha2::Digest;
use std::time::UNIX_EPOCH;

const HOST: &str = "https://api.kraken.com";
const PATH: &str = "/0/private/GetWebSocketsToken";

/// Gets your Kraken WebSockets API token
///
/// # Parameters
///
/// `api_secret` is your Kraken API secret
///
/// `api_key` is your Kraken API key
///
/// # Example
///
/// ```
/// get_websockets_token(api_secret, api_key) {
/// ```
pub fn get_websockets_token(api_secret: &str, api_key: &str) -> Result<String, String> {
    let nonce = std::time::SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map_err(|err| format!("Error calculating nonce ({})", err))?
        .as_millis();

    let content = format!("nonce={}", nonce);

    let hmac = {
        let secret = base64::decode(api_secret)
            .map_err(|err| format!("Error decoding the API secret ({})", err))?;

        let mut hmacer = hmac::Hmac::<sha2::Sha512>::new_varkey(&secret)
            .map_err(|err| format!("Error creating the HMAC ({})", err))?;

        hmacer.input(&PATH.as_bytes().to_vec());
        hmacer.input(
            sha2::Sha256::digest(format!("{}{}", nonce, content).as_bytes())
                .to_vec()
                .as_ref(),
        );

        base64::encode(&hmacer.result().code())
    };

    let body = reqwest::Client::new()
        .post(format!("{}{}", HOST, PATH).as_str())
        .header("User-Agent", "kraken-websockets-token.rs v0.2.1")
        .header("API-Key", api_key)
        .header("API-Sign", hmac)
        .body(content)
        .send()
        .map_err(|err| format!("Error sending the token request ({})", err))?
        .text()
        .map_err(|err| format!("Error getting the token response ({})", err))?;

    let token = serde_json::from_str::<serde_json::Value>(&body)
        .map_err(|err| format!("Token response was an error ({})", err))?
        .get("result")
        .ok_or("Missing 'result' key from token response")?
        .get("token")
        .ok_or("Missing 'token' key from token response")?
        .as_str()
        .ok_or("Error reading token response")?
        .to_string();

    Ok(token)
}
