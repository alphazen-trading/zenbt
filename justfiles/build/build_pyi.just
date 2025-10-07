build_pyi:
  #!/usr/bin/env bash
  cd rs
  mkdir _rs
  cargo build --features pyi
  cd ../
  mv ./rs/_rs/_rs.pyi ./src/zenbt/rs.pyi
  rm -r rs/_rs
