# minigrep — A Minimal CLI Text Search Tool in Rust

`minigrep` is a simple command-line tool written in Rust that searches for lines containing a query string in a specified file.  
It supports optional case-insensitive search and is inspired by the Unix `grep` utility.

---

## Usage

```bash
minigrep [OPTIONS] <QUERY> <FILE_PATH>
```

### Arguments

* `<QUERY>` — The string to search for.
* `<FILE_PATH>` — The path to the file where the search will be performed.

### Options

| Option                | Description                       |
| --------------------- | --------------------------------- |
| `-i`, `--ignore-case` | Perform a case-insensitive search |
| `-h`, `--help`        | Show usage information            |

---

## Installation

Clone and build the project using Cargo:

```bash
git clone https://github.com/your-username/minigrep.git
cd minigrep
cargo build --release
```

Run the compiled binary:

```bash
./target/release/minigrep [OPTIONS] <QUERY> <FILE_PATH>
```

---

## Examples

```bash
# Case-sensitive search
minigrep to poem.txt

# Case-insensitive search using a flag
minigrep -i to poem.txt

# Case-insensitive search using an environment variable
IGNORE_CASE=1 minigrep to poem.txt
```