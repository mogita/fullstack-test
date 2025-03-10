# Simple Fullstack Project

This is a fullstack project with separate backend and frontend components. It implements a simple text editor with AI capabilities for text processing.

## Project Structure

- `backend/`: Contains all backend-related files (Rust API with OpenAI integration)
- `frontend/`: Contains the React TypeScript frontend application

## Development

### Frontend

The frontend is built with:
- React
- TypeScript
- Vite
- Vitest for testing
- Prettier for code formatting

### Backend

The backend is implemented with:
- Rust programming language
- Tokio async runtime
- Axum web framework
- JWT for authentication
- OpenAI API integration for text processing

Features implemented:
- Authentication with JWT
- Text processing endpoints:
  - Paraphrase
  - Expand
  - Summarize
  - Translate (English/Spanish)
- Server-Sent Events (SSE) for streaming responses
- Comprehensive error handling

## Deployment

- Frontend: Deployed to Cloudflare Pages via GitHub Actions
- Backend: Docker containerization for easy deployment

## CI/CD

GitHub Actions workflows are set up to:
- Run linting and tests on pull requests and merges
- Deploy to production when a semantic version tag is pushed
- Build and test the backend Rust application

## Environment Setup

### Backend Requirements

- Rust (latest stable version)
- Docker (for containerization)
- OpenAI API key for text processing features

## License

This project is licensed under the BSD 3-Clause License - see the [LICENSE](LICENSE) file for details.