import os
import sys
import subprocess
import requests


def download_file(url, save_path):
    try:
        # Send a GET request to the URL
        response = requests.get(url, stream=True)
        response.raise_for_status()  # Check for HTTP errors

        # Ensure the local directory exists
        os.makedirs(os.path.dirname(save_path), exist_ok=True)

        # Write the content to the specified file
        with open(save_path, "wb") as file:
            for chunk in response.iter_content(chunk_size=8192):  # Download in chunks
                file.write(chunk)

        print(f"File downloaded successfully to: {save_path}")

    except requests.exceptions.RequestException as e:
        print(f"An error occurred: {e}")


def main():
    module_name = input("Enter your module name: ")
    # Create necessary directories
    os.makedirs("rs/src", exist_ok=True)

    # Write the Rust library file
    with open("./rs/src/lib.rs", "w") as lib_file:
        lib_file.write(f"""use pyo3::prelude::*;

#[pymodule]
fn {module_name}(m: &PyModule) -> PyResult<()> {{
    Ok(())
}}
""")

    # Write the Cargo.toml file
    with open("./rs/Cargo.toml", "w") as cargo_file:
        cargo_file.write(f"""[package]
name = "{module_name}"
version = "0.1.0"
edition = "2021"

# See more keys and their definitions at https://doc.rust-lang.org/cargo/reference/manifest.html
[lib]
name = "{module_name}"
crate-type = ["cdylib"]
""")

    # Navigate to the rs directory and add pyo3 dependency
    os.chdir("rs")
    subprocess.run(["cargo", "add", "pyo3"], check=True)
    os.chdir("..")

    # Replace _lowlevel with the input value in pyproject.toml
    with open("pyproject.toml", "w") as pyproject_file:
        pyproject_file.write(f"""
[tool.maturin]
python-source = "python"
module-name = "rs.{module_name}"
features = ["pyo3/extension-module"]
manifest-path = "rs/Cargo.toml"
""")

    # Write the Justfile
    with open("justfile", "w") as justfile:
        justfile.write(f"""
rs_dev:
  nodemon -e rs --exec just _rs_dev

_rs_dev:
  maturin develop --skip-install -r
  just _rs_dev_pyi

_rs_dev_pyi:
  rye run python scripts/scanner.py rs.{module_name} ./src/{module_name}
  cp ./src/rs/{module_name}.pyi ./src/rs/{module_name}.pyi
""")

    # Add pip as a dev dependency
    subprocess.run(["uv", "add", "--dev", "pip"], check=True)

    print("Script execution complete.")


if __name__ == "__main__":
    main()
