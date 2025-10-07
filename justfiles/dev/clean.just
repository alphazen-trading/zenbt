clean:
  #!/usr/bin/env bash
  echo "Cleaning build artifacts..."
  rm -rf .venv
  rm -rf rs/target
  echo "✓ Cleaned .venv and rs/target"
