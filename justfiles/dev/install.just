install:
  #!/usr/bin/env bash
  uv sync
  uv run pre-commit install --hook-type pre-push --hook-type commit-msg --allow-missing-config
  if [ ! -d "/opt/logs" ]; then
    sudo mkdir /opt/logs
  fi
  sudo chown -R $USER:$USER /opt/logs
