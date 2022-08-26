use std::process::Command;
use actix_web::{HttpResponse, get};
use serde::{Deserialize, Serialize};
use crate::constants::{APPLICATION_JSON, PING_SCRIPT_PATH, TARGET_IP_ADDRESS};

#[derive(Debug, Deserialize, Serialize)]
pub struct Status {
    pub status: String,
}

impl Status {
    pub fn new(status: String) -> Self {
        Self {
            status
        }
    }
}

#[get("/check")]
pub async fn check() -> HttpResponse {

    let output = Command::new("sh")
        .arg(PING_SCRIPT_PATH)
        .arg(TARGET_IP_ADDRESS)
        .output()
        .expect("Failed to execute command");

    let output = std::str::from_utf8(output.stdout.as_slice()).unwrap().trim();

    let status: &str = match output {
        "OK" => "ON",
        "ERROR" => "OFF",
        _ => "UNKNOWN_ERROR"
    };

    HttpResponse::Ok()
        .content_type(APPLICATION_JSON)
        .json(Status::new(status.to_string()))
}
