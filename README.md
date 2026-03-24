# Rotrics Studio App (Rust Migration)

This repository contains the Rust-based migration of the Rotrics Studio App. The project has been transitioned from a JavaScript/Electron architecture to a high-performance Rust backend using **Tauri**.

## 🚀 Getting Started

### Prerequisites
- **Rust Toolchain**: Install via [rustup.rs](https://rustup.rs/).
- **Node.js**: Required for the frontend (Tauri's webview).
- **System Dependencies**:
  - Linux: `libwebkit2gtk-4.0-dev`, `build-essential`, `curl`, `wget`, `libssl-dev`, `libgtk-3-dev`, `libayatana-appindicator3-dev`, `librsvg2-dev`, `pkg-config`.

---

## 🛠 Build Process

### Local Build
1. Install dependencies:
   ```bash
   cargo build --release
   ```
2. Run in development mode:
   ```bash
   cargo tauri dev
   ```

### Docker Build (Recommended for Testing)
You can build and run the entire application using Docker, which handles all system dependencies automatically.

1. Build the image:
   ```bash
   docker compose build
   ```
2. Start the container:
   ```bash
   docker compose up
   ```

---

## 🧪 Testing Process

### Unit Tests
The Rust backend includes unit tests for core logic (Serial Port, G-Code Parsing).
```bash
cargo test
```

### Hardware Integration Test
Since this app interacts with hardware, ensure your robot is connected to `/dev/ttyUSB0` (or update `docker-compose.yml`).
1. Connect the Rotrics robot via USB.
2. Run the application and check the "List Ports" functionality to verify communication.

---

## 📁 Project Structure
- `src/main.rs`: Entry point and Tauri command handlers.
- `src/serial_port.rs`: Manages serial communication with the robot.
- `src/gcode_parser.rs`: Handles G-Code file processing and filtering.
- `Dockerfile`: Multi-stage build for a lean production image.
- `docker-compose.yml`: Local deployment setup with hardware mapping.

## ✅ Recent Review and Corrections
During this maintenance pass, the Rust modules were evaluated and cleaned up:
- Added clear Rustdoc comments to improve maintainability.
- Replaced placeholder serial-port methods with practical implementations for listing/opening/sending/receiving.
- Fixed G-code filtering to correctly ignore whitespace-only lines and inline comments.
- Switched command queue internals to `VecDeque` for efficient FIFO reads.
- Added parser/serial defensive limits and input validation to reduce memory or malformed-input risk.

## 📝 Commit Message Translation Tip (EN + ZH)
When a commit message starts in Chinese, add an English equivalent in the body so both local and international contributors can quickly understand changes.

Example:
```text
feat: 完善串口管理模块

English: Improve serial port manager with open/read/write helpers.
```

---

## Additional Resources
- [Tauri Documentation](https://tauri.app/)
- [Rust Book](https://doc.rust-lang.org/book/)
- [Cargo Guide](https://doc.rust-lang.org/cargo/index.html)
