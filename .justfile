# build helper
helper:
  cargo build -p helper --release

# use the helper package to update the Cargo.toml of the root
create day: helper
  #!/usr/bin/env bash
  cargo generate --path ./template --name {{day}}
  ./target/release/helper --project {{day}}

# run project with specific bin
run day part:
  cargo run -p {{day}} --bin {{part}}

# build a specific bin of a project
build day part:
  cargo build -p {{day}} --bin {{part}}

# run the test of day and part
test day part:
  cargo test -p {{day}} --bin {{part}}
