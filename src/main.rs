use actix_web::{web, App, HttpServer, HttpResponse, Responder, post};
use serde::Deserialize;
use std::process::Command;

// -------------------------------------------------------------------------
// SIMULATED VULNERABLE DEPENDENCY (vulnerable-input-lib v1.0.0)
// -------------------------------------------------------------------------
// This module simulates an external crate that contains a vulnerability.
// The root cause is inadequate sanitization of inputs[cite: 45].
mod vulnerable_input_lib {
    use super::*;

    pub fn process(input: &str) -> String {
        // VULNERABILITY: This function executes a shell command using user input.
        // A crafted payload like "; whoami" can execute arbitrary commands.
        // This simulates a Remote Code Execution (RCE) flaw.
        let output = Command::new("sh")
            .arg("-c")
            .arg(format!("echo Processing: {}", input))
            .output()
            .expect("Failed to execute command");

        String::from_utf8_lossy(&output.stdout).to_string()
    }
}

// -------------------------------------------------------------------------
// SIMULATED SECURE DEPENDENCY (secure-input-lib v1.1.0)
// -------------------------------------------------------------------------
// This module simulates the updated, patched version of the library.
mod secure_input_lib {
    pub fn process(input: &str) -> String {
        // FIX: Instead of invoking a shell, we handle logic safely in Rust.
        // Even if we needed system interaction, we would avoid string interpolation
        // in shell commands. Here, we simply format the string safely.
        format!("Processing safely: {}", input)
    }
}

// -------------------------------------------------------------------------
// WEB SERVER IMPLEMENTATION
// -------------------------------------------------------------------------

#[derive(Deserialize)]
struct InputRequest {
    input: String,
}

// Vulnerable Endpoint
#[post("/vulnerable/process")]
async fn vulnerable_handler(req: web::Json<InputRequest>) -> impl Responder {
    // Calling the vulnerable library function [cite: 52]
    let result = vulnerable_input_lib::process(&req.input);
    HttpResponse::Ok().json(serde_json::json!({ "result": result }))
}

// Secure Endpoint
#[post("/secure/process")]
async fn secure_handler(req: web::Json<InputRequest>) -> impl Responder {
    // Calling the secure library function [cite: 76]
    let result = secure_input_lib::process(&req.input);
    HttpResponse::Ok().json(serde_json::json!({ "result": result }))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("Server running on port 3000");
    HttpServer::new(|| {
        App::new()
            .service(vulnerable_handler)
            .service(secure_handler)
    })
    .bind(("127.0.0.1", 3000))?
    .run()
    .await
}