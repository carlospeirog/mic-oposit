# Mic-Oposit

A REST API service built with Rust and Actix Web.

## Features

- User management with specialties
- MongoDB integration
- RESTful API endpoints
- Environment-based configuration
- Error handling
- Logging

## API Endpoints

### Users

- `GET /teachers` - List all teachers with optional filtering
  - Query parameters:
    - `name`: Filter by first name (case-insensitive)
    - `surname`: Filter by last name (case-insensitive, partial match)
    - `specialty`: Filter by specialty name
- `GET /teachers/{id}` - Get a specific teacher by ID

## Specialties

The system supports the following specialties:
- Informatics (inf)
- Primary Education (pri)
- English (ing)
- French (fra)
- Physical Education (ef)
- Therapeutic (pt)
- Audition and Language (al)
- Music (mus)

## Setup

1. Clone the repository
2. Create a `.env` file with the following variables:
   ```
   DATABASE_URL=mongodb://localhost:27017
   DATABASE_NAME=primary
   COLLECTION_NAME=teachers
   HOST=127.0.0.1
   PORT=8080
   LOG_LEVEL=info
   ```
3. Install dependencies:
   ```bash
   cargo build
   ```
4. Run the server:
   ```bash
   cargo run
   ```

## Development

### Prerequisites

- Rust (latest stable version)
- MongoDB
- Cargo

### Building

```bash
cargo build
```

### Running Tests

```bash
cargo test
```

### Documentation

Generate documentation:
```bash
cargo doc --open
```

## Project Structure

```
src/
├── api/         # API endpoints
├── config.rs    # Configuration management
├── db/          # Database layer
├── error.rs     # Error handling
├── models.rs    # Data models
└── main.rs      # Application entry point
```
