# Backend API

This is the Rust backend API for the Simple Fullstack Project text editor with AI capabilities. It provides authentication and text processing endpoints using OpenAI's API.

## Tech Stack

- Rust
- Tokio (async runtime)
- Axum (web framework)
- JWT for authentication
- OpenAI API for text processing
- Docker for containerization

## Features

- Authentication with JWT
- Text processing endpoints:
  - Paraphrase
  - Expand
  - Summarize
  - Translate (English/Spanish)
- Server-Sent Events (SSE) for streaming responses
- Comprehensive error handling
- Unit and integration tests

## Development

### Prerequisites

- Rust (latest stable version)
- Docker (for containerization)

### Environment Variables

Create a `.env` file in the backend directory with the following variables:

```
SERVER_PORT=3001
SERVER_HOST=127.0.0.1
OPENAI_API_KEY=your_openai_api_key
OPENAI_BASE_URL=https://api.openai.com/v1
OPENAI_MODEL=gpt-3.5-turbo
JWT_SECRET=your_jwt_secret_key
JWT_EXPIRATION=86400
```

### Running Locally

```bash
# Install dependencies and build
cargo build

# Run the server
cargo run

# Run tests
cargo test
```

### Docker

```bash
# Build the Docker image
docker build -t simple-fullstack-backend .

# Run the Docker container
docker run -p 3001:3001 --env-file .env simple-fullstack-backend
```

## API Endpoints

### Public Endpoints

- `GET /health` - Health check
- `POST /api/auth/login` - Login with username and password

### Protected Endpoints (require JWT authentication)

- `POST /api/text/paraphrase` - Paraphrase text
- `POST /api/text/expand` - Expand text with more details
- `POST /api/text/summarize` - Summarize text
- `POST /api/text/translate` - Translate text between English and Spanish

## Authentication

The API uses JWT for authentication. To access protected endpoints, include the JWT token in the Authorization header:

```
Authorization: Bearer your_jwt_token
```

## Contributing

1. Fork the repository
2. Create a feature branch
3. Make your changes
4. Run tests: `cargo test`
5. Submit a pull request

## License

This project is licensed under the BSD 3-Clause License - see the [LICENSE](../LICENSE) file for details.