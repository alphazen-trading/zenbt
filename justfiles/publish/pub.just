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
