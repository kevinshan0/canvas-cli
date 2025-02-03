# Canvas CLI

A command-line interface tool for interacting with the Canvas Learning Management System API. Built with Rust, this CLI tool allows you to manage courses, assignments, discussions, and more directly from your terminal.

## Features

- Course management and viewing
- Assignment submission and tracking
- Discussion board interaction
- Module navigation
- Todo list viewing
- Local data synchronization
- Grade checking
- Announcement viewing

## Technologies Used

- **Rust** - Core programming language
- **Clap** - Command line argument parsing
- **Reqwest** - HTTP client for API requests
- **Serde** - Serialization/deserialization of JSON
- **Rusqlite** - SQLite database integration
- **Chrono** - DateTime handling
- **Dotenv** - Environment variable management

## Prerequisites

- Rust (1.56 or later)
- SQLite
- Canvas API access token
- Canvas instance URL

## Installation

1. Clone the repository:
```bash
git clone https://github.com/kevinshan0/canvas-cli.git
cd canvas-cli
```

2. Build the project:
```bash
cargo build --release
```

3. Create a `.env` file in the project root:
```env
BASE_URL=https://your-institution.instructure.com
ACCESS_KEY=your_canvas_api_token
```

## Usage

### Basic Commands

```bash
# List all courses
canvas-cli courses

# View course details
canvas-cli course <course_id>

# List assignments for a course
canvas-cli assignments <course_id>

# Submit an assignment
canvas-cli assignment <course_id> <assignment_id> --file-path path/to/file

# View todo items
canvas-cli todos

# Check grades
canvas-cli grades <course_id>

# View course modules
canvas-cli modules <course_id>

# View discussion topics
canvas-cli discussions <course_id>

# Post to a discussion
canvas-cli post-discussion <course_id> <topic_id> "Your message here"

# Sync data locally
canvas-cli sync
```

### Command Line Options

```bash
canvas-cli [OPTIONS] <COMMAND>

Options:
  -b, --base-url <URL>        Canvas instance URL
  -a, --access-token <TOKEN>  Canvas API access token
  -h, --help                  Print help information
  -V, --version              Print version information
```

## Local Data Synchronization

The CLI includes a local SQLite database for offline access and caching. To sync data:

```bash
canvas-cli sync
```

This will:
- Update course information
- Sync assignments and submissions
- Cache discussion topics
- Update module information
- Track sync status

## Development

### Project Structure

```
canvas-cli/
├── src/
│   ├── main.rs         # Entry point
│   ├── cli.rs          # CLI argument definitions
│   ├── client.rs       # Canvas API client
│   ├── models.rs       # Data structures
│   └── db.rs           # Database operations
├── Cargo.toml          # Dependencies and metadata
└── .env               # Configuration
```

### Adding New Features

1. Define new models in `models.rs`
2. Add API endpoints in `client.rs`
3. Create CLI commands in `cli.rs`
4. Implement handlers in `main.rs`
5. Add database operations in `db.rs` if needed

## Contributing

1. Fork the repository
2. Create a feature branch
3. Commit your changes
4. Push to the branch
5. Create a Pull Request

## License

MIT License - see LICENSE file for details
