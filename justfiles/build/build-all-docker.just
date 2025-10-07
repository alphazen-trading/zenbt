build-all-docker:
  #!/usr/bin/env bash
  echo "Building wheels for all platforms using Docker..."

  # This uses maturins built-in Docker support for cross-compilation
  # It requires Docker to be installed and running

  # Build for all supported platforms using manylinux
  maturin build --release --compatibility manylinux2014 --target x86_64-unknown-linux-gnu
  maturin build --release --compatibility manylinux2014 --target i686-unknown-linux-gnu
  maturin build --release --compatibility manylinux2014 --target aarch64-unknown-linux-gnu

  # MacOS/Windows builds (requires additional Docker setup)
  if command -v docker &> /dev/null; then
    # Use the cross platform builder if available
    maturin build --release --target x86_64-apple-darwin --docker
    maturin build --release --target aarch64-apple-darwin --docker
    maturin build --release --target x86_64-pc-windows-msvc --docker
  else
    echo "Docker not found, skipping MacOS/Windows cross-builds"
  fi

  # Build pure Python package with hatch
  hatch build

  echo "All platform builds complete! Check the 'target/wheels' and 'dist' directories for the output."
