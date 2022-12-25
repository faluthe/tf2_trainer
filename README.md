Be sure to:
```
rustup toolchain install nightly
rustup override add nightly-x86_64-pc-windows-msvc
rustup target install i686-pc-windows-msvc
```

And build with: 
```
cargo build --target=i686-pc-windows-msvc
```