#!/bin/sh

mkdir -p rs/src

echo 'use pyo3::prelude::*;

#[pymodule]
fn rs(m: &PyModule) -> PyResult<()> {
    Ok(())
}' > ./rs/src/lib.rs


echo '[package]
name = "my-project"
version = "0.1.0"
edition = "2021"

# See more keys and their definitions at https://doc.rust-lang.org/cargo/reference/manifest.html
[lib]
name = "my_project"
crate-type = ["cdylib"]

' > ./rs/Cargo.toml


cd rs
cargo add pyo3
cd ..

echo '
[tool.maturin]
python-source = "python"
module-name = "rs._lowlevel"
features = ["pyo3/extension-module"]
manifest-path = "rs/Cargo.toml"
' >> pyproject.toml
# Get the input parameter
MODULE_NAME=$1

# Check if input is provided
if [ -z "$MODULE_NAME" ]; then
    echo "Usage: $0 <module_name>"
    exit 1
fi

# Write to pyproject.toml, replacing _lowlevel with the input value
echo "
[tool.maturin]
python-source = \"python\"
module-name = \"rs.$MODULE_NAME\"
features = [\"pyo3/extension-module\"]
manifest-path = \"rs/Cargo.toml\"
" > pyproject.toml

echo '

rs_dev:
  nodemon -e rs --exec just _rs_dev

_rs_dev:
  maturin develop --skip-install -r
  just _rs_dev_pyi

_rs_dev_pyi:
  rye run python scripts/scanner.py zenbt.zbt ./src/zenbt
  cp ./src/rs/zbt.pyi ./src/rs/backtester.pyi
' > justfile

uv add --dev pip
