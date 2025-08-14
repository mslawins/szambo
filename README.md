# szambo

**szambo** is a CLI tool for managing JSON translation files in a simple and consistent way.  
It supports adding, removing, replacing, renaming, sorting, comparing translations, and cleaning up unused keys.

---

## 📦 Installation

### 1) Build from source

1. Install Rust: https://www.rust-lang.org/tools/install  
2. Clone the repository:
   ~~~bash
   git clone https://github.com/yourusername/szambo.git
   cd szambo
   ~~~
3. Build the release binary:
   ~~~bash
   cargo build --release
   ~~~
4. Move the compiled binary into your `$PATH`:
   ~~~bash
   # macOS/Linux
   mv target/release/szambo /usr/local/bin/
   # Windows (PowerShell) – add target\release to PATH or copy szambo.exe to a dir already on PATH
   ~~~

---

### 2) Build & install with `just` (optional)

If you use [`just`](https://github.com/casey/just):
~~~bash
just release
~~~

---

## 🚀 Usage

General form:
~~~bash
szambo <command> [options]
~~~

---

## 🧭 Commands & Examples

### 1) Add entries to **all** files in a directory
Add a single key’s value, pulling per-language values from an input file:
~~~bash
szambo add-to-many --from=input.json --key="foo.bar.baz" --where=lang/
~~~
Update only specific files (comma-separated list):
~~~bash
szambo add-to-many --from=input.json --key="foo.bar.baz" --where=lang/ --files=en.json,sv.json
~~~

### 2) Add entries to a **single** file
`--where` points to a single target file:
~~~bash
szambo add-to-single --from=input.json --where=lang/en.json
~~~

### 3) Remove a key from all files in a directory
~~~bash
szambo remove --key="foo.bar.baz" --where=lang/
~~~

### 4) Replace an existing key’s value from an input file
~~~bash
szambo replace --key="foo.bar.baz" --from=input.json --where=lang/
~~~
Partial replace in selected files:
~~~bash
szambo replace --key="foo.bar.baz" --from=input.json --where=lang/ --files=en.json,sv.json
~~~

### 5) Rename a key in all files
~~~bash
szambo rename --from="foo.bar.baz" --to="aaa.bbb.ccc" --where=lang/
~~~

### 6) Sort all translation files alphabetically by keys
~~~bash
szambo sort --where=lang/
~~~

### 7) Compare one translation file to another
Lists keys that are missing in `--target` when compared to `--reference`:
~~~bash
szambo compare --target=sv.json --reference=en.json
~~~

### 8) Compare **all** files in a directory
Detects missing translations across the directory (exits with a CI-friendly status code):
~~~bash
szambo compare-all --where=lang/
~~~

### 9) List unused keys referenced in source code
May include false positives if keys aren’t hardcoded:
~~~bash
szambo list-unused-keys --translations=en.json --source=src/
~~~

### 10) Remove unused keys from translation files
Use with care; might remove keys detected as unused:
~~~bash
szambo remove-unused-keys --translations=en.json --source=src/ --where=lang/
~~~

---

## 📂 Input file format for updates

`szambo` expects the input JSON to map language codes to values for the given key.

Example `input.json`:
~~~json
{
  "en": "Hi",
  "pl": "Cześć",
  "es": "Hola"
}
~~~

Expected directory structure:
~~~
lang/
├── en.json
├── pl.json
└── es.json
~~~

When you run (example):
~~~bash
szambo add-to-many --from=input.json --key="greeting.hello" --where=lang/
~~~
`szambo` will insert/update `greeting.hello` in each `*.json` using the corresponding value from `input.json`.

---

## 📝 Notes & Tips

- `--files` accepts a comma-separated list (e.g., `en.json,sv.json`) for partial updates in `add-to-many` and `replace`.
- All file writes **sort keys alphabetically**.
- `list-unused-keys` / `remove-unused-keys` can produce **false positives** when keys are generated dynamically or not directly present in code.
- Use `compare-all` in CI to ensure all locales are in sync. A non-zero exit typically indicates missing translations.
- Need help at any time:
  ~~~bash
  szambo --help
  szambo <command> --help
  ~~~

---

## ✅ Examples at a glance

Add a nested key to all locales:
~~~bash
szambo add-to-many --from=input.json --key="auth.errors.invalidPassword" --where=lang/
~~~

Rename a key across the project:
~~~bash
szambo rename --from="auth.login" --to="auth.signIn" --where=lang/
~~~

Sort everything:
~~~bash
szambo sort --where=lang/
~~~

Compare target to reference:
~~~bash
szambo compare --target=fr.json --reference=en.json
~~~

Clean up unused keys (review diffs!):
~~~bash
szambo remove-unused-keys --translations=en.json --source=src/ --where=lang/
~~~

---

