cargo run --release -- -o 2349872 -m frobose batch --min-m 7 --max-m 8 -n 20 | tee result.txt
cargo run -- --offset 123 --model frobose single -p 0.01 -w
