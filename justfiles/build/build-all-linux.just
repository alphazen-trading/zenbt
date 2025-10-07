build-all-linux:
  #!/usr/bin/env bash
  echo "Building wheels for Linux platforms..."

  # Build for Linux using manylinux2014 for better compatibility
  maturin build --release --compatibility manylinux2014 --target x86_64-unknown-linux-gnu

  # Optionally build for other Linux architectures
  # Uncomment these if you need them:
  # maturin build --release --compatibility manylinux2014 --target i686-unknown-linux-gnu
  # maturin build --release --compatibility manylinux2014 --target aarch64-unknown-linux-gnu

  echo "Linux builds complete! Check the 'target/wheels' directory for the output."
