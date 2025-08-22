# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Overview

Claude Scope is a monitoring tool for Claude Code that collects OpenTelemetry data via gRPC and provides a Next.js-based web dashboard. The project uses a Rust backend with an integrated frontend build system.

## Development Commands

### Building
```bash
# Full build (backend + frontend)
cargo build

# Skip frontend build (faster for backend-only changes)
SKIP_WEB_BUILD=1 cargo build

# Production build
cargo build --release

# Force frontend rebuild
rm web/.build_cache && cargo build
```

### Running
```bash
# Start with defaults (port 3000, otel 4317)
cargo run

# Custom ports
cargo run -- --port 8080 --otel-port 4318 --db-path ./custom.db

# Development mode (backend only)
SKIP_WEB_BUILD=1 cargo run

# Frontend development server (separate terminal)
cd web && pnpm run dev
```

### Frontend Development
```bash
cd web

# Install dependencies (prefers pnpm, falls back to npm)
pnpm install

# Development server
pnpm run dev

# Build static export
pnpm run build

# Linting
pnpm run lint
```

### Database Operations
The SQLite database is automatically initialized on first run with migrations from `migrations/001_initial.sql`. No manual setup required.

## Architecture Overview

### High-Level Architecture
The system operates as a dual-server setup:

1. **HTTP Server** (port 3000): Serves the web UI and REST API
2. **OpenTelemetry gRPC Server** (port 4317): Receives telemetry data from Claude Code

Both servers run concurrently using `tokio::select!` and share the same SQLite database through an Arc-wrapped database trait.

### Backend Architecture (Rust)

**Core Modules:**
- `main.rs` - Application entry point, server orchestration
- `config.rs` - Configuration management with environment variable support
- `server.rs` - HTTP server setup with static file serving and API routing

**API Layer (`src/api/`):**
- `mod.rs` - Common types (`ApiResponse<T>`, error handling, `MetricPoint`)
- `metrics.rs` - Metrics endpoints (`/api/metrics/overview`, `/api/metrics/timeline`)  
- `sessions.rs` - Session management (`/api/sessions`, `/api/sessions/:id`)

**OpenTelemetry Layer (`src/otel/`):**
- `receiver.rs` - gRPC server for OpenTelemetry protocol
- `mod.rs` - Metric classification and processing logic

**Storage Layer (`src/storage/`):**
- `mod.rs` - Database trait abstraction
- `sqlite.rs` - SQLite implementation with async operations

### Frontend Architecture (Next.js)

**App Router Structure:**
- `app/layout.tsx` - Root layout with global styles
- `app/page.tsx` - Main dashboard page

**Component Architecture:**
- `components/ui/` - Base components (Card, Button, Badge) following shadcn/ui patterns
- `components/dashboard/` - Dashboard-specific components:
  - `dashboard.tsx` - Main dashboard container
  - `metric-card.tsx` - Individual metric display cards
  - `metrics-chart.tsx` - Recharts-based visualization
  - `refresh-controls.tsx` - Manual/auto refresh controls

**Data Layer:**
- `lib/api.ts` - Type-safe API client with all backend endpoints
- `hooks/use-metrics.ts` - React hook for data fetching and state management
- `hooks/use-polling.ts` - Configurable polling mechanism

### Build System Integration

The project uses a custom `build.rs` script that:
- Detects Node.js (>=18) and package managers (pnpm preferred, npm fallback)
- Implements intelligent caching based on file timestamps
- Triggers on changes to `web/app/`, `web/components/`, `web/lib/`, `web/hooks/`
- Validates build output in `web/dist/`
- Integrates with Cargo's rebuild detection system

### Database Schema

**Core Tables:**
- `sessions` - Claude Code session tracking (id, user_id, start_time, end_time, command_count)
- `metrics` - Time-series metrics with labels (timestamp, name, value, labels as JSON)

### API Patterns

All REST endpoints follow a consistent pattern:
- Wrapped in `ApiResponse<T>` with success/error handling
- Structured error types using `thiserror`
- Async handlers with database state injection
- Query parameter validation and parsing

### Configuration System

Configuration hierarchy (highest priority first):
1. CLI arguments (`--port`, `--otel-port`, `--db-path`)
2. Environment variables (`CLAUDE_SCOPE_HTTP_PORT`, `CLAUDE_SCOPE_OTEL_PORT`, etc.)
3. Default values

### Static Asset Serving

The Rust backend serves the Next.js static export from `web/dist/`:
- Primary route `/` serves `index.html`
- Static assets served via `ServeDir` fallback
- API routes take precedence over static files

## Key Implementation Details

### Concurrent Server Architecture
Both HTTP and OpenTelemetry servers run concurrently using `tokio::select!`, allowing graceful shutdown on Ctrl+C while handling both web requests and telemetry ingestion.

### Database Connection Management  
Uses SQLx with a connection pool, shared across both servers through `Arc<dyn Database>` trait abstraction.

### Frontend State Management
Uses React hooks with polling for real-time updates. The `useMetricsPolling` hook combines data fetching with configurable auto-refresh (30s default).

### Build System Cache Strategy
The build cache (`web/.build_cache`) prevents unnecessary rebuilds by comparing file modification times across source directories and key config files.