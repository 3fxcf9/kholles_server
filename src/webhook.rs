use crate::webhook_error::WebhookError;

use hex::encode;
use hmac::{Hmac, Mac};
use rocket::data::{Data, ToByteUnit};
use rocket::http::Status;
use rocket::post;
use rocket::request::{FromRequest, Outcome, Request};
use rocket::serde::json::Json;
use rocket::serde::Deserialize;
use sha2::Sha256;
use std::env;
use std::process::Command;

type HmacSha256 = Hmac<Sha256>;

/// Webhook Payload Structure
#[derive(Debug, Deserialize)]
pub struct WebhookPayload {
    pub repository: RepoInfo,
}

#[derive(Debug, Deserialize)]
pub struct RepoInfo {
    pub name: String,
    pub owner: RepoOwner,
}

#[derive(Debug, Deserialize)]
pub struct RepoOwner {
    pub id: u32,
}

/// Verify GitHub Webhook Signature
pub fn verify_signature(secret: &str, signature_header: &str, payload: &str) -> bool {
    let mut mac =
        HmacSha256::new_from_slice(secret.as_bytes()).expect("HMAC can take key of any size");
    mac.update(payload.as_bytes());

    let expected_mac = mac.finalize();
    let expected_signature = format!("sha256={}", encode(expected_mac.into_bytes()));

    // Use constant-time comparison to prevent timing attacks
    expected_signature
        .as_bytes()
        .eq(signature_header.as_bytes())
}
#[test]
fn test_verify_signature() {
    let secret = "It's a Secret to Everybody";
    let payload = "Hello, World!";
    let expected_signature =
        "sha256=757107ea0eb2509fc211221cce984b8a37570b6d7586c22c46f4379c8b043e17";

    assert!(verify_signature(secret, expected_signature, payload));
}
#[test]
fn test_invalid_signature() {
    let secret = "It's a Secret to Everybody";
    let payload = "Hello, World!";
    let wrong_signature = "sha256=wrong_signature";

    assert!(!verify_signature(secret, wrong_signature, payload));
}

/// Header signature extractor
#[derive(Debug)]
pub struct HeaderSignature<'r>(pub &'r str);

#[rocket::async_trait]
impl<'r> FromRequest<'r> for HeaderSignature<'r> {
    type Error = WebhookError;

    async fn from_request(req: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        match req.headers().get_one("X-Hub-Signature-256") {
            None => Outcome::Error((
                Status::Unauthorized,
                WebhookError::new(
                    Status::Unauthorized,
                    "Missing 'X-Hub-Signature-256' header".to_string(),
                ),
            )),
            Some(sig) => Outcome::Success(HeaderSignature(sig)),
        }
    }
}

fn run_git_pull() -> () {
    let output = Command::new("git")
        .arg("-C")
        .arg(env::var(crate::LISTEN_PATH_ENV_NAME).unwrap_or(".".to_string()))
        .arg("pull")
        .output();

    match output {
        Ok(out) => {
            let stdout = String::from_utf8_lossy(&out.stdout);
            let stderr = String::from_utf8_lossy(&out.stderr);
            println!("Git Pull Output:\n{}", stdout);
            if !stderr.is_empty() {
                eprintln!("Git Pull Error:\n{}", stderr);
            }
        }
        Err(err) => {
            eprintln!("Failed to execute `git pull`: {:?}", err);
        }
    };
}

/// Webhook Handler
#[post("/events/push", format = "json", data = "<data>")]
pub async fn handle_webhook(
    sig: HeaderSignature<'_>,
    data: Data<'_>,
) -> Result<Json<String>, WebhookError> {
    let secret = env::var("GITHUB_WEBHOOK_SECRET").expect("GITHUB_WEBHOOK_SECRET is not set");

    let mut body = String::new();
    if let Err(_) = data
        .open(64.kibibytes())
        .into_string()
        .await
        .map(|s| body = s.into_inner())
    {
        return Err(WebhookError::new(
            Status::InternalServerError,
            "Failed to read request body".to_string(),
        ));
    }

    // Verify signature
    if !verify_signature(&secret, sig.0, &body) {
        return Err(WebhookError::new(
            Status::Forbidden,
            "Invalid signature".to_string(),
        ));
    }

    // Check repository info
    let parsed_payload: Result<WebhookPayload, _> = serde_json::from_str(&body);
    match parsed_payload {
        Ok(payload)
            if payload.repository.name == "kholles_content"
                && payload.repository.owner.id == 123954477 =>
        {
            run_git_pull();
        }
        Ok(_) => {
            return Err(WebhookError::new(
                Status::BadRequest,
                "Unhandled event".to_string(),
            ));
        }
        Err(_) => {
            return Err(WebhookError::new(
                Status::BadRequest,
                "Invalid JSON format".to_string(),
            ));
        }
    }

    Ok(Json("Success !".to_string()))
}
