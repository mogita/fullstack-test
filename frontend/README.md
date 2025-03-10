# AI Text Editor Frontend

A modern React TypeScript frontend for an AI-powered text editor that allows users to paraphrase, expand, summarize, and translate text.

## Features

- Clean, modern UI with responsive design
- Light and dark mode support (automatically detects system preference)
- JWT-based authentication
- Text operations: paraphrase, expand, summarize, and translate
- Server-Sent Events (SSE) for real-time streaming of AI responses
- Comprehensive test coverage

## Tech Stack

- React 19 with TypeScript
- Vite for fast development and building
- Tailwind CSS for styling
- React Router for navigation
- Vitest and Testing Library for testing
- Axios for API requests

## Getting Started

### Prerequisites

- Node.js 18+ and npm/yarn
- Rust backend running (see backend README)

### Installation

1. Clone the repository
2. Navigate to the frontend directory:
   ```bash
   cd frontend
   ```
3. Install dependencies:
   ```bash
   npm install
   ```

### Running the Development Server

```bash
npm run dev
```

The application will be available at http://localhost:5173

### Building for Production

```bash
npm run build
```

The built files will be in the `dist` directory.

### Running Tests

```bash
npm test
```

## Environment Variables

Create a `.env` file in the frontend directory with the following variables:

```
VITE_API_URL=http://localhost:3001
```

## Authentication

The application uses JWT authentication. The demo credentials are:

- Username: `neo`
- Password: `script-chairman-fondly-yippee`

## Architecture Decisions

- **Context API**: Used for global state management (auth and theme) to avoid prop drilling
- **Custom Hooks**: Encapsulated API logic in custom hooks for better reusability
- **Component Structure**: Organized components into reusable UI components and page components
- **SSE for Streaming**: Used Server-Sent Events for real-time streaming of AI responses
- **Responsive Design**: Implemented mobile-first design with Tailwind CSS

## Future Improvements

- Add WebSocket support for real-time collaboration
- Implement rate limiting and offline support
- Add more AI operations and language options
- Improve error handling and retry mechanisms
- Add user preferences and settings

## License

This project is licensed under the MIT License - see the LICENSE file for details.
