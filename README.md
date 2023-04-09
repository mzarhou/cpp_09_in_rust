commands:
```bash
cargo run --release -p btc assets/btc/input.txt
cargo run --release -p rpn -- "8 9 * 9 - 9 - 9 - 4 - 1 +"
cargo run --release -p pmerge_me -- `shuf -i 1-100000 -n 3000 | tr "\n" " "`
```
