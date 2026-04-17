
run:
    cargo run

generate_entity:
    sea-orm-cli generate entity --output-dir ./entity/src --lib --compact-format --with-serde both

migrate:
    sea-orm-cli migrate up
