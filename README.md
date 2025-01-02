# rt - Regex-based Text Extraction Utility

`rt` is a command-line tool written in Rust designed to make text extraction a breeze using regular expressions and capture groups. Unlike traditional utilities like `rg`, `grep`, `awk`, or `sed`, rt focuses specifically on simplifying text extraction tasks, providing a straightforward and intuitive interface for regex-based text processing.

## Features

- **Regex-based Extraction**: Use regular expressions and capture groups to extract specific text patterns effortlessly.
- **Simplicity**: Designed for ease of use, providing a straightforward and clear syntax for text extraction.
- **Rust-Powered**: Built with Rust, rt offers performance, safety, and reliability.

## Installation

### From Source

1. Ensure you have Rust installed. If not, install it from [Rust's official website](https://www.rust-lang.org/tools/install).
2. Clone the `rt` repository:

```bash
git clone https://github.com/ThatXliner/rt.git
```

3. Navigate to the rt directory and build the tool:

```bash
cd rt
cargo build --release
```

4. Once built, the executable will be located at ./target/release/rt.

### Via mise

If you have [mise](https://mise.jdx.dev/) installed:

```bash
mise use cargo:https://github.com/ThatXliner/rt@branch:main
```

## Usage

**Examples:**

Extract email addresses from a file:

```bash
rt '([a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\.[a-zA-Z]{2,})' file.txt --doc
```

Extract phone numbers from a string:

```bash
echo "Contact: 123-456-7890, 987-654-3210" | rt '(\d{3}-\d{3}-\d{4})' 
```

Getting the folder names of tarballs:

```bash
ls | rt '(.+?)\.tar\.gz' --group 1 
```

Get the all IDs of current Docker containers:

```bash
docker container ls | tail -n +2 | rt '(\w+?)\s+.+' --group 1
```

Note that you don't need a `$` at the end of the regex since `rt` is multiline by default.

## FAQ

### Why rt?

`rt` stands out for its simplicity and focus on text extraction via regex capture groups. It provides a more streamlined and intuitive experience compared to other command-line tools, making text extraction tasks more accessible and manageable. It aims to offer a clear and easy-to-understand syntax without the complexities often associated with Perl on the command line.

### What's with the name?

It is a Rust rewrite of [Ret](https://github.com/ThatXliner/ret), with a better interface.

---

Feel free to contribute, report issues, or suggest improvements by visiting the rt GitHub repository. Your feedback is valuable!
