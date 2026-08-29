# Bewerbungstracker

A small web application for keeping track of job applications. The project is
written in Rust and uses Axum, Askama, SQLite, and SeaORM.

## Requirements

- Rust and Cargo
- `just` (optional, for the project shortcuts)
- SeaORM CLI (only required when generating migrations or entities)

The repository includes a Nix development shell with all of these tools:

```sh
nix develop
```

## Configuration

The application reads `DATABASE_URL` from the environment. If it is not set,
the application uses `sqlite://db.sqlite?mode=rwc`. For local development, you
can export the value explicitly:

```dotenv
DATABASE_URL=sqlite://db.sqlite?mode=rwc
```

```sh
export DATABASE_URL=sqlite://db.sqlite?mode=rwc
```

This creates or opens `db.sqlite` in the repository root. You can override the
database location by exporting a different `DATABASE_URL` before starting the
application. The migration CLI loads values from `.env` through its dotenv
support; the application itself uses the process environment and its built-in
default.

## Run the application

From the repository root:

```sh
cargo run
```

Or, using `just`:

```sh
just run
```

The server listens on <http://localhost:3000>. Pending migrations are applied
automatically when the application starts.

## Build

```sh
cargo build --release
```

The release binary is written to `target/release/bewerbungs_tool`.

To build a ZIP bundle for a 64-bit ARM Raspberry Pi:

```sh
nix build .#aarch64
```

Copy the resulting `result` file to the Raspberry Pi, extract it, and start
the application:

```sh
unzip result
./run.sh
```

The Raspberry Pi must be running a 64-bit ARM Linux system. If it uses Nix,
you can also build the bundle directly there with the same command.

## Database migrations

Migration commands and the workflow for creating a migration are documented in
[`migration/README.md`](migration/README.md).

The most common commands, run from the repository root, are:

```sh
# Apply all pending migrations
cargo run -p migration -- up

# Show migration status
cargo run -p migration -- status

# Create a new migration
cargo run -p migration -- generate add_some_field
```

## Generate SeaORM entities

After changing the database schema, regenerate the entity files with:

```sh
just generate_entity
```

This writes the generated entities to `entity/src`.

## Project structure

```text
.
├── src/main.rs                 # Application entry point
├── api/
│   ├── src/lib.rs              # Axum server, routes, and database setup
│   ├── src/http/               # Request handlers and application state
│   ├── templates/              # Askama HTML templates
│   └── assets/                 # CSS and JavaScript served by the app
├── entity/
│   └── src/                    # SeaORM entity models used by the API
├── migration/
│   ├── src/main.rs             # Migration CLI entry point
│   ├── src/lib.rs              # Registered migration list
│   └── src/m*.rs               # Individual, ordered migrations
├── db.sqlite                   # Local SQLite database (created at runtime)
├── .env                        # Local environment configuration
├── justfile                    # Shortcuts for common development commands
├── Cargo.toml                  # Rust workspace and root package
└── flake.nix                   # Reproducible Nix development shell and builds
```

The workspace is split into three main crates: `api` contains the web
application, `entity` contains database models, and `migration` owns schema
changes. The root crate starts the application in `src/main.rs`.
