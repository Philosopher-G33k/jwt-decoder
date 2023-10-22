mod config;

use clap::Parser;
use openssl::base64;
use std::error::Error;
use serde::{Deserialize, Serialize};


pub fn run() {
    get_args();
}

fn get_args() {
    let config = config::config::Config::parse();
    decode(&config.decoce);
}

#[derive(Debug, Serialize, Deserialize)]
struct Claims {
   sub: String,
   company: String
}

fn decode(jwt: &str) {
    let corrected_jwt = crate::process_token(jwt);
    let decoded_jwt = process_jwt(&corrected_jwt);
    printout_decoded_jwt(&decoded_jwt.ok().expect("Failed to parse"));
}

fn process_token(jwt: &str) -> String {
    let splitted_jwt_strings: Vec<_> = jwt.split('.').collect();
    let jwt_header = splitted_jwt_strings
        .get(0)
        .expect("split always returns at least one element");
    let jwt_body = splitted_jwt_strings.get(1).expect("split always returns at least one element");
    let jwt_signature = splitted_jwt_strings.get(2).expect("split always returns at least one element");
    let len_correction_needed = jwt_body.len().div_ceil(4) * 4 - (jwt_body.len());
    format!("{}{}{}{}{}{}",jwt_header,".", jwt_body, "=".repeat(len_correction_needed), ".", jwt_signature)
}

fn process_jwt(jwt: &str) -> Result<[serde_json::Value; 2], Box<dyn Error>> {
    let splitted_jwt_strings: Vec<_> = jwt.split('.').collect();
    let jwt_header = splitted_jwt_strings
        .get(0)
        .expect("split always returns at least one element");
    let jwt_body = splitted_jwt_strings.get(1).ok_or(Box::<dyn Error>::from(
        "Could not find separator in jwt string.",
    ))?;
    let decoded_jwt_header = base64::decode_block(jwt_header);
    let decoded_jwt_body = base64::decode_block(jwt_body);

    let converted_jwt_header = String::from_utf8(decoded_jwt_header?)?;
    let converted_jwt_body = String::from_utf8(decoded_jwt_body?)?;

    let parsed_jwt_header = serde_json::from_str::<serde_json::Value>(&converted_jwt_header)?;
    let parsed_jwt_body = serde_json::from_str::<serde_json::Value>(&converted_jwt_body)?;
    Ok([parsed_jwt_header, parsed_jwt_body])
}

fn printout_decoded_jwt(jwt: &[serde_json::Value; 2]) {
    println!(
        "{}",
        serde_json::to_string_pretty(&jwt[0])
            .expect("to_string_pretty always works on serde_json::Value")
    );
    println!(
        "{}",
        serde_json::to_string_pretty(&jwt[1])
            .expect("to_string_pretty always works on serde_json::Value")
    );
}
