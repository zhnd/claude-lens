# Claude Scope

A monitoring tool for Claude Code that receives OpenTelemetry data and provides a web interface for analysis.

## Features

- **OpenTelemetry Data Collection**: Receives metrics, traces, and logs via gRPC
- **SQLite Storage**: Lightweight database for storing telemetry data
- **Web Interface**: Built-in web UI for analyzing Claude Code usage
- **Single Binary Deployment**: All assets embedded in the binary

## Usage

```bash
# Start with default settings
claude-scope

# Custom configuration
claude-scope --port 8080 --otel-port 4317 --db-path ./data.db
```

## Command Line Options

- `--port <PORT>`: HTTP server port (default: 3000)
- `--otel-port <PORT>`: OpenTelemetry gRPC server port (default: 4317) 
- `--db-path <PATH>`: SQLite database path (default: ./claude-scope.db)

## Building

```bash
cargo build --release
```

## Development

The project structure:

```
claude-scope/
├── src/
│   ├── main.rs          # Entry point and CLI
│   ├── server.rs        # HTTP server setup
│   ├── config.rs        # Configuration management
│   ├── api/             # REST API endpoints
│   ├── otel/            # OpenTelemetry receiver
│   ├── storage/         # Database operations
│   └── ...
├── migrations/          # Database schema
├── web/                 # Frontend assets (future)
└── Cargo.toml
```

## License

MIT