rust_log := env_var_or_default('RUST_LOG', "INFO")

watch EXAMPLE *args:
    RUST_LOG={{rust_log}} cargo watch -x 'run --example=day{{EXAMPLE}} --release {{args}}'

run IDX *args:
    RUST_LOG={{rust_log}} cargo run --example=day{{IDX}} {{args}}

release IDX *args:
    RUST_LOG={{rust_log}} cargo run --example=day{{IDX}} --release {{args}}

profile IDX *args:
    CARGO_PROFILE_RELEASE_DEBUG=true cargo flamegraph --example=day{{IDX}} {{args}}

clippy IDX *args:
    cargo clippy --example=day{{IDX}} {{args}}

test *args:
    cargo test --release {{args}}
    cargo test --examples --release {{args}}
