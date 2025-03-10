# Simple Fullstack Project

This is a fullstack project with separate backend and frontend components.

## Project Structure

- `backend/`: Contains all backend-related files
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

The backend structure is prepared for future implementation.

## Deployment

- Frontend: Deployed to Cloudflare Pages via GitHub Actions
- Backend: Prepared for Docker image building (not implemented yet)

## CI/CD

GitHub Actions workflows are set up to:
- Run linting and tests on pull requests and merges
- Deploy to production when a semantic version tag is pushed