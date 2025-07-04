set dotenv-load

default:
	just --list --unsorted

# ============================================= #
# Dev Section
# ============================================= #
install:
  #!/usr/bin/env bash
  uv sync
  uv run pre-commit install --hook-type pre-push --hook-type commit-msg --allow-missing-config
  if [ ! -d "/opt/logs" ]; then
    sudo mkdir /opt/logs
  fi
  sudo chown -R $USER:$USER /opt/logs

cz:
  uv run cz commit --write-message-to-file /tmp/msg

czr:
  uv run cz commit --write-message-to-file /tmp/msg --retry

cleanup:
  rm -f /tmp/msg

local_docker:
  #!/usr/bin/env bash
  docker stop $(docker ps -aq)
  sudo chown -R 472:472 ./docker/grafana_data
  sudo chown -R 472:472 ./docker/provisioning
  docker compose --env-file .env -f docker/docker-compose.yml up -d


# ============================================= #
# Code Section
# ============================================= #
clippy:
  #!/usr/bin/env bash
  cd ./rs
  cargo clippy -- -W clippy::pedantic

# Build wheels for all platforms in one command
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
  

# Build only for Linux platforms
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

# Build Linux wheels using Docker (more reliable)
build-linux-docker:
  #!/usr/bin/env bash
  echo "Building Linux wheels using Docker..."
  maturin build --release --compatibility manylinux2014 --target x86_64-unknown-linux-gnu --docker

# Build wheels for all platforms using Docker (more reliable)
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

# Simplified one-liner that builds for all platforms
build-all: 
  maturin build --release --all-features --compatibility manylinux2014 --sdist --strip

# Set default build-all alias
alias build := build-all

pre-commit-test:
  ruff format
  ruff check --fix
  uv run pyright
  uv run pre-commit run

alias dev := py_dev
py_dev:
  nodemon -e py --exec uv run dev

rs_dev:
  nodemon -e rs --exec just _rs_dev

_rs_dev:
  maturin develop --skip-install -r
  just _rs_dev_pyi

_rs_dev_pyi:
  uv run python scripts/scanner.py zenbt.zbt ./src/zenbt
  cp ./src/zenbt/zbt.pyi ./src/zenbt/backtester.pyi

zellij:
  #!/usr/bin/env bash
  # Check if a Zellij session named "multi" exists
  if zellij ls | grep -q "trading_bot"; then
      zellij kill-session trading_bot
      zellij delete-session trading_bot
  fi

  zellij -s trading_bot --layout layout.kdl


build_pyi:
  #!/usr/bin/env bash
  cd rs
  mkdir _rs
  cargo build --features pyi
  cd ../
  mv ./rs/_rs/_rs.pyi ./src/zenbt/rs.pyi
  rm -r rs/_rs

gdocker:
  sudo chown -R $USER:$USER ./docker


pub:
  #!/usr/bin/env bash
  rye version -b minor
  rm -rf ./dist
  uv build --all --wheel 
  # uv publish 

  # just pre-commit-test
  # git add .
  # touch /tmp/msg
  # git commit -m "build: automatic uv bump of project version"
  # git push
  # just pub-docs


pub-docs:
  uv run mike deploy --update-aliases $(uv version) latest
  uv run mike set-default --push latest
  git checkout gh-pages

  git push
  git checkout develop


import_pickl:
  uv run import_pickl

test_pickl:
  nodemon -e py --exec uv run test_pickl


# ============================================= #
# Docs
# ============================================= #
docs:
  nodemon -e *.py* --exec uv run mkdocs serve

docs-build:
  uv run mkdocs build


# ============================================= #
# Custom Section
# ============================================= #
custom:
  echo "Do whatever you want here"
