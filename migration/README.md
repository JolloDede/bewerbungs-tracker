# Database migrations

This crate contains the SeaORM migration history for the application. Each
migration is a Rust file in `migration/src` with a timestamped name. Migrations
are executed in the order returned by `migration/src/lib.rs`.

All commands below can be run from the repository root. The migration CLI loads
`DATABASE_URL` from `.env`, so the examples use the local `db.sqlite` database
by default. You can also provide it explicitly:

```sh
DATABASE_URL=sqlite://db.sqlite?mode=rwc cargo run -p migration -- up
```

## Create a migration

Generate a new migration file with a descriptive snake-case name:

```sh
cargo run -p migration -- generate add_company_phone
```

This creates a new file such as
`migration/src/m20260828_153000_add_company_phone.rs`. Add the migration module
to `migration/src/lib.rs` and register its `Migration` type in
`MigratorTrait::migrations()` if the CLI does not add those entries
automatically.

Implement both directions:

- `up`: apply the schema change.
- `down`: undo the schema change when rolling back.

Keep existing migrations immutable after they have been applied anywhere
outside your local machine. Create a new migration for follow-up changes
instead of editing an old one.

## Run migrations

Apply every pending migration:

```sh
cargo run -p migration -- up
```

Check which migrations have run:

```sh
cargo run -p migration -- status
```

Apply only the next migration (or the next `N` migrations):

```sh
cargo run -p migration -- up -n 1
cargo run -p migration -- up -n 10
```

Roll back the latest migration (or the latest `N` migrations):

```sh
cargo run -p migration -- down
cargo run -p migration -- down -n 10
```

## Reset and refresh a local database

These commands are destructive. Use them only when it is safe to discard local
data:

```sh
# Drop all tables, then apply every migration
cargo run -p migration -- fresh

# Roll back all migrations, then apply them again
cargo run -p migration -- refresh

# Roll back all applied migrations
cargo run -p migration -- reset
```

The application also runs `Migrator::up` during startup, so starting the app
with `cargo run` applies pending migrations automatically. Running migrations
explicitly is still useful for checking status, creating a controlled
deployment step, or troubleshooting schema changes.

## Regenerate entities

When a schema change affects a model, regenerate the SeaORM entities after
applying the migration:

```sh
just generate_entity
```

Review the generated changes in `entity/src` before committing them.
