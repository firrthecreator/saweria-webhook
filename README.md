# Saweria Webhook Handler 🦀

<div align="center">

[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](LICENSE)
[![Rust](https://img.shields.io/badge/Language-Rust-orange.svg)](https://www.rust-lang.org/)
[![Code Quality](https://img.shields.io/badge/Code%20Quality-Production%20Ready-brightgreen.svg)]()

A high-performance, type-safe webhook handler for [Saweria](https://saweria.co) donations. Built with **Rust** and **Axum**, designed for streamers and developers who want to integrate custom donation actions with OBS, Discord, IoT devices, databases, and more.

</div>

---

## ✨ Features

- **⚡ Blazingly Fast**: Built on Axum and Tokio async runtime for maximum performance
- **🔒 Type-Safe**: Strict JSON schema validation for all incoming Saweria payloads
- **⚙️ Configurable**: Simple `.env` file configuration for quick setup
- **📊 Production Ready**: Structured logging, comprehensive error handling, and robustness
- **🔌 Extensible**: Easy to add custom handlers and integrations

---

## 📋 Prerequisites

- [Rust & Cargo](https://rustup.rs/) (Latest Stable)
- Git
- (Optional) [ngrok](https://ngrok.com/) for local testing with public URL

---

## 🚀 Quick Start

### 1. Clone the Repository

```bash
git clone https://github.com/firrthecreator/saweria-webhook.git
cd saweria-webhook
```

### 2. Configure Environment Variables

```bash
cp .env.example .env
```

Edit `.env` to customize your settings (see [Configuration](#-configuration) section below).

### 3. Run the Server

```bash
cargo run
```

The server will start on the configured port (default: `8080`).

---

## ⚙️ Configuration

Configure your webhook handler using environment variables in the `.env` file:

| Variable | Description | Default |
|----------|-------------|---------|
| `SERVER_PORT` | Port the server listens on | `8080` |
| `WEBHOOK_PATH` | URL path for Saweria webhooks | `/api/saweria` |
| `RUST_LOG` | Logging level (trace, debug, info, warn, error) | `info` |

### Example `.env` File

```env
SERVER_PORT=8080
WEBHOOK_PATH=/api/saweria
RUST_LOG=info
```

---

## 🔗 Connecting to Saweria

### Step 1: Expose Your Server

For local development, use ngrok to expose your server:

```bash
ngrok http 8080
```

Copy the generated HTTPS URL (e.g., `https://abcd-1234.ngrok.app`).

### Step 2: Configure Saweria Webhook

1. Go to [Saweria Dashboard](https://saweria.co)
2. Navigate to **Overlays** → **Alert Settings**
3. Set your Webhook URL to: `https://your-ngrok-url.app/api/saweria`
4. Save and click **"Test Alert"** to verify the connection

---

## 📁 Project Structure

```
saweria-webhook/
├── src/
│   ├── main.rs         # Application entry point & server setup
│   ├── handlers.rs     # Business logic for webhook requests
│   ├── models.rs       # Data schemas & struct definitions
│   ├── config.rs       # Environment variable configuration loader
│   └── ...
├── Cargo.toml          # Project dependencies & metadata
├── .env.example        # Example environment variables
├── LICENSE             # MIT License
└── README.md           # This file
```

### File Descriptions

- **`main.rs`**: Sets up the Axum web server and registers routes
- **`handlers.rs`**: Contains webhook endpoint handlers and donation logic
- **`models.rs`**: Defines Saweria donation payload structures using serde
- **`config.rs`**: Loads and validates environment variables

---

## 🛠 Building & Development

### Build Release Binary

```bash
cargo build --release
```

The optimized binary will be in `target/release/saweria-webhook`.

### Run Tests

```bash
cargo test
```

### Check Code

```bash
cargo clippy
```

---

## 📦 Dependencies

- **[Axum](https://github.com/tokio-rs/axum)** - Web framework
- **[Tokio](https://tokio.rs/)** - Async runtime
- **[Serde](https://serde.rs/)** - Serialization/deserialization
- **[Dotenvy](https://github.com/allan2/dotenvy)** - `.env` file loader
- **[Tracing](https://docs.rs/tracing/)** - Structured logging

See `Cargo.toml` for complete dependencies and versions.

---

## 🤝 Contributing

Contributions are welcome! Here's how you can help:

1. **Fork** the repository
2. **Create** a feature branch (`git checkout -b feature/amazing-feature`)
3. **Commit** your changes (`git commit -m 'feat: add amazing feature'`)
4. **Push** to the branch (`git push origin feature/amazing-feature`)
5. **Open** a Pull Request

Please ensure your code follows Rust conventions and passes `cargo clippy` checks.

---

## 📄 License

This project is licensed under the **MIT License**. See the [LICENSE](LICENSE) file for details.

---

## 📞 Support

- 📖 Check the [Saweria API Documentation](https://saweria.co/docs)
- 🐛 Open an [issue](https://github.com/firrthecreator/saweria-webhook/issues) for bugs
- 💡 Start a [discussion](https://github.com/firrthecreator/saweria-webhook/discussions) for questions

---

<div align="center">

Made with ❤️ by Firr, The Creator.

</div>
