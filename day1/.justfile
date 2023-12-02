# test part
test part:
  cargo test -p day1 --bin {{part}}

# run part
run part:
  cargo run -p day1 --bin {{part}}

# run release build
run-release part:
  #!/usr/bin/env bash
  cargo run -p day1 --bin {{part}} --release
