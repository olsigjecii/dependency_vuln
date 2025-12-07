# Known Vulnerabilities in Dependencies (Rust Edition)

This project demonstrates how using an outdated or vulnerable crate (dependency) can expose a Rust application to Remote Code Execution (RCE), and how updating to a secure version mitigates this risk.

## Concepts Covered

- **Source:** Known vulnerabilities in open-source libraries (CVEs).
- **Risk:** Remote Code Execution (RCE) via command injection.
- **Fix:** Updating dependencies and avoiding unsafe system calls.

## Setup Instructions

1.  **Initialize the Project:**
    Ensure you have Rust installed. Create a new project:

    ```bash
    cargo new dependency_vuln
    cd dependency_vuln
    ```

2.  **Add Dependencies:**
    Replace the contents of your `Cargo.toml` with the version provided in the lesson (requires `actix-web`, `serde`, `serde_json`).

3.  **Add Code:**
    Replace `src/main.rs` with the code provided in the lesson.

4.  **Run the Server:**
    ```bash
    cargo run
    ```
    You should see: `Server running on port 3000`

## Demonstration

### 1. The Vulnerability (RCE)

The `vulnerable-input-lib` blindly passes input to a shell command. We can inject a command to list files in the directory.

**Request:**

```bash
curl -X POST http://127.0.0.1:3000/vulnerable/process \
   -H "Content-Type: application/json" \
   -d '{"input": "normal; ls -la"}'
```
