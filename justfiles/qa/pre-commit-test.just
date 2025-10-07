pre-commit-test:
  ruff format
  ruff check --fix
  uv run pyright
  uv run pre-commit run
