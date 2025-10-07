pub-docs:
  uv run mike deploy --update-aliases $(uv version) latest
  uv run mike set-default --push latest
  git checkout gh-pages

  git push
  git checkout develop
