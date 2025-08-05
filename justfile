release:
  cargo build --release
  sudo mkdir -p /usr/local/bin
  sudo cp ./target/release/szambo /usr/local/bin 
