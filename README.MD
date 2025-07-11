# GitHub User Data Fetcher

A simple asynchronous Rust application that fetches user data from the GitHub API for multiple users concurrently using Tokio and reqwest.

## Overview

This application demonstrates concurrent HTTP requests in Rust by fetching GitHub user information for multiple users simultaneously. It uses Tokio for asynchronous runtime and reqwest for HTTP client functionality.

## Features

- **Concurrent API Calls**: Fetches data for multiple GitHub users simultaneously using Tokio tasks
- **Error Handling**: Robust error handling for both network errors and task execution errors
- **GitHub API Integration**: Retrieves user data from GitHub's REST API
- **Async/Await**: Modern Rust asynchronous programming patterns

## Prerequisites

- Rust 1.70 or higher
- Cargo package manager
- Internet connection for API calls

## Dependencies

Add the following dependencies to your `Cargo.toml`:

```toml
[dependencies]
reqwest = "0.11"
tokio = { version = "1", features = ["full"] }
```

## Installation

1. Clone or download this repository
2. Navigate to the project directory
3. Build the project:
   ```bash
   cargo build
   ```

## Usage

Run the application with:

```bash
cargo run
```

The application will fetch GitHub user data for three predefined users:

- `rust-lang`
- `tokio-rs`
- `async-rust`

### Sample Output

```
{
  "login": "rust-lang",
  "id": 5430905,
  "node_id": "MDEyOk9yZ2FuaXphdGlvbjU0MzA5MDU=",
  "avatar_url": "https://avatars.githubusercontent.com/u/5430905?v=4",
  "gravatar_id": "",
  "url": "https://api.github.com/users/rust-lang",
  ...
}

Fetching user data completed.
```

## Code Structure

### Main Function (`main`)

- Creates a vector of GitHub usernames to fetch
- Spawns concurrent Tokio tasks for each user
- Collects and processes results with comprehensive error handling
- Prints fetched user data or error messages

### Fetch Function (`fetch_username_data`)

- Takes a username as input parameter
- Constructs GitHub API URL (`https://api.github.com/users/{username}`)
- Creates HTTP client with proper User-Agent header
- Sends GET request and returns response body as string
- Returns `Result<String, reqwest::Error>` for error handling

## Error Handling

The application handles two types of errors:

1. **HTTP/Network Errors**: Issues with API requests (network connectivity, API rate limits, invalid responses)
2. **Task Execution Errors**: Problems with Tokio task execution

Both error types are caught and logged with descriptive messages.

## API Information

This application uses the GitHub REST API v3:

- **Endpoint**: `https://api.github.com/users/{username}`
- **Method**: GET
- **Authentication**: Not required for public user data
- **Rate Limit**: 60 requests per hour for unauthenticated requests

## Customization

To fetch data for different users, modify the `users` vector in the `main` function:

```rust
let users = vec!["your-username", "another-user", "third-user"];
```

## Rate Limiting

GitHub API has rate limits:

- **Unauthenticated**: 60 requests per hour
- **Authenticated**: 5,000 requests per hour

For production use with many users, consider implementing:

- Authentication tokens
- Rate limiting logic
- Request batching

## Contributing

1. Fork the repository
2. Create a feature branch
3. Make your changes
4. Add tests if applicable
5. Submit a pull request

## License

This project is open source and available under the MIT License.

## Troubleshooting

### Common Issues

1. **Network Connection**: Ensure you have a stable internet connection
2. **API Rate Limits**: If you exceed GitHub's rate limits, wait an hour or implement authentication
3. **Invalid Usernames**: The application will return an error for non-existent GitHub users

### Debug Mode

Run with debug information:

```bash
RUST_LOG=debug cargo run
```

## Future Enhancements

- Add command-line argument support for custom usernames
- Implement GitHub API authentication
- Add JSON parsing for structured data output
- Include retry logic for failed requests
- Add configuration file support
- Implement caching for user data
