# szambo

Szambo is CLI tool created to simplify management of JSON translations files.

## Installation instruction

1. Install Rust on your machine
2. Clone this repo
3. Build with: `cargo build --release`
4. On Mac/Linus move compiled program to directory included in $PATH

##  Installation using just (steps 3. and 4.)

1. just release

## Usage

* szambo add-to-many --from=input.json --key="foo.bar.baz" --where=lang/ 
* szambo add-to-single --from=input.json --where=lang/
* szambo remove --key="foo.bar.baz" --where=lang/
* szambo replace --key="foo.bar.baz" --from=input.json --where=lang/
* szambo rename --from="foo.bar.baz" --to="aaa.bbb.ccc" --where=lang/
* szambo sort  --where=lang/
* szambo compare --reference=en.json --target=sv.json
* szambo compare-all --where=lang/
* szambo list-unused-keys --translations="en.json" --source="src/"
* szambo remove-unused-keys --translations="en.json" --source="src/" --where=lang/

## Update file format

Szambo assumes that update file will match files in lang/ directory. Example:

```
// input.json
{
    "en": "Hi",
    "pl": "Czesc",
    "es": "Ola"
}
```

where:
 
```
// lang/
├── en.json
├── pl.json
└── es.json

```



## Caveats

* on save all files are being sorted in alphabetical order (by key)
