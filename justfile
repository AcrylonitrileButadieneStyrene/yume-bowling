[linux]
build:
  cargo build --release --target x86_64-pc-windows-gnu

[linux]
package: build
  mkdir -p dist
  cp target/x86_64-pc-windows-gnu/release/yume-bowling.exe dist
  cp -r assets dist
  7z a release.7z dist/*