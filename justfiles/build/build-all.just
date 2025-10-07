build-all:
  maturin build --release --all-features --compatibility manylinux2014 --sdist --strip

alias build := build-all
