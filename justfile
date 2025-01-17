set dotenv-load

default:
	just --list --unsorted

# ============================================= #
# Dev Section
# ============================================= #
install:
  #!/usr/bin/env bash
  rye sync
  rye run pre-commit install --hook-type pre-push --hook-type commit-msg --allow-missing-config
  if [ ! -d "/opt/logs" ]; then
    sudo mkdir /opt/logs
  fi
  sudo chown -R $USER:$USER /opt/logs

cz:
  rye run cz commit --write-message-to-file /tmp/msg

czr:
  rye run cz commit --write-message-to-file /tmp/msg --retry

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

pre-commit-test:
  ruff format
  ruff check --fix
  rye run pyright
  rye run pre-commit run

alias dev := py_dev
py_dev:
  nodemon -e py --exec uv run dev

rs_dev:
  nodemon -e rs --exec just _rs_dev

_rs_dev:
  maturin develop --skip-install -r
  just _rs_dev_pyi

_rs_dev_pyi:
  rye run python scripts/scanner.py zenbt.zbt ./src/zenbt
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
  rye build --all --wheel --clean
  rye publish --yes

  just pre-commit-test
  git add .
  touch /tmp/msg
  git commit -m "build: automatic rye bump of project version"
  git push
  just pub-docs


pub-docs:
  rye run mike deploy --update-aliases $(rye version) latest
  rye run mike set-default --push latest
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
  nodemon -e *.py* --exec rye run mkdocs serve

docs-build:
  rye run mkdocs build


# ============================================= #
# Custom Section
# ============================================= #
custom:
  echo "Do whatever you want here"
