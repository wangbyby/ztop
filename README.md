
# Dev Loop
`cargo watch -x check -x test -x run`
# Code Coverage 
linux-x86 only
`cargo tarpaulin --ignore-tests`
# What CI should run
1. cargo test
2. cargo clippy -- -D warnings # lint
3. cargo fmt -- --check
4. cargo audit # secure
