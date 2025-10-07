rs_dev:
  nodemon -e rs --exec just _rs_dev

_rs_dev:
  maturin develop --skip-install -r
  just _rs_dev_pyi

_rs_dev_pyi:
  uv run python scripts/scanner.py zenbt.zbt ./src/zenbt
  cp ./src/zenbt/zbt.pyi ./src/zenbt/backtester.pyi
