add package:
  #!/usr/bin/env bash
  md {{package}}
  cd {{invocation_directory()}}
  cargo new {{package}}

helper:
  cargo build -p helper --release

create day: helper
  #!/usr/bin/env bash
  cargo generate --path ./template --name {{day}}
  ./target/release/helper --project {{day}}

run day part:
  cargo run -p {{day}} --bin {{part}}

build day part:
  cargo build -p {{day}} --bin {{part}}

# run the test of day and part
test day part:
  cargo test -p {{day}} --bin {{part}}