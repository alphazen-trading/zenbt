build-all-platforms:
  #!/usr/bin/env bash
  echo "Building wheels for all platforms..."

  # Make sure we have the right tooling
  pip install -U maturin hatch

  # macOS targets (Intel and Apple Silicon)
  maturin build --release --target x86_64-apple-darwin
  maturin build --release --target aarch64-apple-darwin

  # Windows targets
  maturin build --release --target x86_64-pc-windows-msvc
  maturin build --release --target i686-pc-windows-msvc

  # Linux targets
  maturin build --release --target x86_64-unknown-linux-gnu
  maturin build --release --target i686-unknown-linux-gnu
  maturin build --release --target aarch64-unknown-linux-gnu

  # Build pure Python package with hatch
  hatch build
