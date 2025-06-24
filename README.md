# 📖🔢 word counter

**wordcounter** is a simple command-line application written in Rust that counts words in plain text files. It's designed to be fast, efficient, and easy to use.

## Features

- Count total words in one or more text files 
- Outputs word count per file and total
- Show top 10, 100, n, all words in files 
- Minimal dependencies and fast execution

## Usage

```bash
wordcounter [topn] <file1> <file2> ...
```

### Example

```bash
$ wordcounter sample1.txt sample2.txt

sample1.txt: 234 words  
sample2.txt: 187 words  
Total: 421 words
```

## Installation

### From Source

Make sure you have [Rust and Cargo](https://www.rust-lang.org/tools/install) installed.

```bash
git clone https://github.com/yourusername/wordcounter.git
cd wordcounter
cargo build --release
```

You’ll find the binary in `target/release/wordcounter`.

### Cargo Install (if published)

```bash
cargo install --git https://github.com/yourusername/wordcounter
```

## License

This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for details.

---

Made with 🦀 by Matheus Soares - 2025