[linux]
build-linux:
  cargo build --release

[linux]
package-linux: build-linux
  mkdir -p dist/linux
  cp target/release/yume-bowling dist/linux
  cp -r assets dist/linux
  pushd dist/linux
  7z a release-linux.7z
  mv release-linux.7z ..
  popd

[linux]
build-windows:
  cargo build --release --target x86_64-pc-windows-gnu

[linux]
package-windows: build-windows
  mkdir -p dist/windows
  cp target/x86_64-pc-windows-gnu/release/yume-bowling.exe dist/windows
  cp -r assets dist/windows
  pushd dist/windows
  7z a release-windows.7z
  mv release-windows.7z ..
  popd