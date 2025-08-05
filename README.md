# Installation instruction

1. Install Rust on your machine
2. Clone this repo
3. Build with: `cargo build --release`
4. On Mac/Linus move compiled program to directory included in $PATH

#  Installation using just (steps 3. and 4.)

1. just release

# Usage

* szambo add --from=input.json --key="foo.bar.baz" --where=lang/ 
* szambo remove --key="foo.bar.baz" --where=lang/
* szambo replace --key="foo.bar.baz" --from=input.json --where=lang/
* szambo rename --from="foo.bar.baz" --to="aaa.bbb.ccc" --where=lang/
* szambo sort  --where=lang/
* szambo compare --reference=en.json --target=sv.json

# Caveats

* on save all files are being sorted in alphabetical order (by key)
