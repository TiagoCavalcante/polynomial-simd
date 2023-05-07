# polynomial-simd

Performance comparison of polynomial evaluation with/without SIMD

## How faster is it?
SIMD is ~2x faster than a simple loop:
```sh
echo "1 -1 2 -2 3 -3 4 -4 5 -5 6 -6 7 -7 8 -8 9 -9 10 -10
2" | cargo run --release
at: -3378745 (135 ns)
at_simd: -3378745 (65 ns)
```

## How to install?

Just execute the command bellow and you are ready to go:
```sh
wget -qO- https://raw.githubusercontent.com/TiagoCavalcante/polynomial-simd/main/scripts/install.sh | bash
```

## Build yourself
Building it yourself is very easy:
```sh
git clone https://github.com/TiagoCavalcante/polynomial-simd
cargo run --release
```
