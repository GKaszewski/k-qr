# K-QR - Rust QR Code Generator

A lightweight, high-performance QR code generator built with Rust, Axum, Maud, and HTMX.

## Features

- **Blazing Fast**: Built with Rust for maximum performance.
- **Embedded UI**: The web interface is completely embedded in the binary.
- **Interactive**: Uses HTMX for seamless, single-page-like experience without complex JS frameworks.
- **Configurable**: Easily customize host, port, and default styles via environment variables.

## Tech Stack

- **Backend**: Rust / Axum
- **Templating**: Maud (Type-safe HTML)
- **Frontend Interactivity**: HTMX
- **QR Generation**: `qrcode` crate

## Configuration

The application can be configured using environment variables:

| Variable | Description | Default |
|----------|-------------|---------|
| `SERVER_HOST` | Network interface to bind to | `0.0.0.0` |
| `SERVER_PORT` | Port to listen on | `3000` |
| `QR_DEFAULT_COLOR` | Default color for generated QR codes (HEX) | `#000000` |
| `RUST_LOG` | Logging level (`debug`, `info`, `warn`, `error`) | `qr_generator=debug` |

## Getting Started

### Prerequisites

- [Rust](https://rustup.rs/) (latest stable)
- [Docker](https://www.docker.com/) (optional, for containerized deployment)

### Local Development

1. Clone the repository:
   ```bash
   git clone <repository-url>
   cd qr-generator
   ```

2. Run the backend:
   ```bash
   cd backend
   cargo run
   ```

3. Open your browser at `http://localhost:3000`.

### Docker Deployment

Use Docker Compose to spin up the application:

```bash
docker-compose up --build -d
```

Or pull the pre-built image:

```bash
docker run -d -p 3000:3000 --name k-qr registry.gabrielkaszewski.dev/k-qr:latest
```

## API Usage

You can also generate QR codes directly via the API:

```bash
GET /api/qr?data=https://github.com
```

**Parameters:**
- `data` (required): The URL or text to encode in the QR code.

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.
