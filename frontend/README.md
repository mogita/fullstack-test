# Frontend

This is the frontend part of the Simple Fullstack Project.

## Tech Stack

- React
- TypeScript
- Vite
- Vitest for testing
- Prettier for code formatting

## Development

```bash
# Install dependencies
npm install

# Start development server
npm run dev

# Run tests
npm run test

# Run linting
npm run lint

# Build for production
npm run build
```

## Deployment

The frontend is automatically deployed to Cloudflare Pages when a new version tag (e.g., v1.0.0) is pushed to the repository. The deployment is handled by GitHub Actions.

## CI/CD

GitHub Actions workflows are set up to:
- Run linting and tests on pull requests and merges to main
- Deploy to Cloudflare Pages when a semantic version tag is pushed
