# 🚀 ImgFlux: High-Performance Image Processing Microservice

**ImgFlux** is a lightning-fast, CPU-bound image processing API built with **Rust**. It is designed to handle high-throughput image transformations directly in memory, bypassing disk I/O for maximum performance.

![Rust](https://img.shields.io/badge/built_with-Rust-dca282.svg)
![Docker](https://img.shields.io/badge/container-Docker-2496ed.svg)
![License](https://img.shields.io/badge/license-MIT-blue.svg)
[![Render](https://img.shields.io/badge/Render-Live_Demo-46e3b7.svg)](https://imgflux-api.onrender.com/)

---

## ⚡ Features

- **In-Memory Processing**: Zero disk I/O. Images are processed entirely in RAM.
- **Async & Non-Blocking**: Built on `Tokio` and `Axum` to handle thousands of concurrent connections.
- **CPU-Bound Optimization**: Heavy image operations are offloaded to blocking threads to keep the event loop responsive.
- **Secure**: API Key authentication middleware via `x-api-key` header.
- **Cloud Ready**: Dockerized with Multi-Stage builds (~80MB image size).
- **DDoS Protection**: Built-in request body limits (Max 10MB).

---

## 🛠 Tech Stack

- **Language**: Rust 🦀
- **Web Framework**: Axum
- **Async Runtime**: Tokio
- **Image Processing**: Image Crate
- **Serialization**: Serde
- **Containerization**: Docker (Debian Slim)

---

## 🚀 Getting Started

### Prerequisites

- **Rust**: `curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh`
- **Docker**: For containerized deployment.

### 1. Local Development

Clone the repository and run the server:

```bash
git clone https://github.com/yourusername/imgflux.git
cd imgflux
# Set API Keys (Optional, defaults provided for dev)
export API_KEYS="my_secret_key,another_key"
cargo run --release
```

The server will start at `http://0.0.0.0:3000`.

### 2. Docker Deployment

Build and run the lightweight Docker container:

```bash
# Build the image (Multi-stage)
docker build -t imgflux .

# Run the container with custom API keys
docker run -p 3000:3000 -e API_KEYS="production_secret_key" imgflux
```

---

## 📡 API Documentation

### `POST /process`

Resizes and formats an uploaded image.

**Base URL:** `https://imgflux-api.onrender.com` (Live Demo)

**Headers:**
- `Content-Type`: `multipart/form-data`
- `x-api-key`: `rust_is_fast_123` (Required)

**Query Parameters:**
- `w` (Optional): Target width (e.g., `?w=500`)
- `h` (Optional): Target height (e.g., `?h=500`)

**Body:**
- `image`: The image file to process.

#### Example Usage (cURL)

```bash
curl -v -H "x-api-key: rust_is_fast_123" \
     -X POST -F "image=@input.jpg" \
     "https://imgflux-api.onrender.com/process?w=800&h=600" \
     --output output.png
```

---

## 🔐 Security

The API is protected by a custom middleware that verifies the `x-api-key` header.

- **Configuration**: Set the `API_KEYS` environment variable with a comma-separated list of valid keys.
- **Default (Dev)**: `rust_is_fast_123`, `demo_user_007`

*Note: In production, always set `API_KEYS` to a strong, secret value.*

---

## ☁️ Deployment

This project is ready for deployment on any container orchestration platform (Kubernetes, AWS ECS, Google Cloud Run, Render, DigitalOcean App Platform).

**Render/Heroku/Railway:**
1. Connect your repository.
2. Set the `API_KEYS` environment variable in your dashboard.
3. Deploy!

---

## 📜 License

This project is licensed under the MIT License.
