import os
import sys
import subprocess
import urllib.request


def ensure_content_in_file(content, file_path=".gitignore"):
    # Read the file if it exists, otherwise initialize as empty
    try:
        with open(file_path, "r") as file:
            existing_content = file.read()
    except FileNotFoundError:
        existing_content = ""

    # Check if the content already exists
    if content.strip() not in existing_content:
        with open(file_path, "a") as file:
            file.write(content)
        print("Added content")
    else:
        print("Content for already exists")


def download_file(url, save_path):
    try:
        # Ensure the local directory exists
        os.makedirs(os.path.dirname(save_path), exist_ok=True)

        # Download the file
        urllib.request.urlretrieve(url, save_path)

        print(f"File downloaded successfully to: {save_path}")

    except Exception as e:
        print(f"An error occurred: {e}")


def main():
    module_name = input("Enter your module name: ")
    # Create necessary directories
    os.makedirs("rs/src", exist_ok=True)

    # Write the Rust library file
    with open("./rs/src/lib.rs", "w") as lib_file:
        lib_file.write("""use pyo3::prelude::*;

/// Formats the sum of two numbers as string.
#[pyfunction]
fn sum_as_string(a: usize, b: usize) -> PyResult<String> {
    Ok((a + b).to_string())
}

/// A Python module implemented in Rust. The name of this function must match
/// the `lib.name` setting in the `Cargo.toml`, else Python will not be able to
/// import the module.
#[pymodule]
fn rs(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(sum_as_string, m)?)
}
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
    pyproject_content = f"""
[tool.maturin]
profile = "release"
python-source = "src"
module-name = "{module_name}.rs"
features = ["pyo3/extension-module"]
manifest-path = "rs/Cargo.toml"
"""

    # Write the Justfile
    just_file_content = f"""
rs_dev:
  nodemon -e rs --exec just _rs_dev

_rs_dev:
  maturin develop --skip-install -r
  just _rs_dev_pyi

_rs_dev_pyi:
  rye run python scripts/scanner.py {module_name}.rs ./src/{module_name}
"""

    ensure_content_in_file(pyproject_content, "pyproject.toml")
    ensure_content_in_file(just_file_content, "justfile")
    ensure_content_in_file("""rs/target""", ".gitignore")

    # Add pip as a dev dependency
    subprocess.run(["uv", "add", "--dev", "pip"], check=True)

    print("Script execution complete.")

    # Download the scanner used to created the pyi
    url = "https://raw.githubusercontent.com/alphazen-trading/zenbt/refs/heads/master/scripts/scanner.py"
    save_path = "./scripts/scanner.py"
    download_file(url, save_path)


if __name__ == "__main__":
    main()
