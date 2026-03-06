<div align="center">

```
██╗  ██╗ █████╗ ███████╗██╗  ██╗ ██████╗██████╗ ██╗   ██╗
██║  ██║██╔══██╗██╔════╝██║  ██║██╔════╝██╔══██╗╚██╗ ██╔╝
███████║███████║███████╗███████║██║     ██████╔╝ ╚████╔╝ 
██╔══██║██╔══██║╚════██║██╔══██║██║     ██╔══██╗  ╚██╔╝  
██║  ██║██║  ██║███████║██║  ██║╚██████╗██║  ██║   ██║   
╚═╝  ╚═╝╚═╝  ╚═╝╚══════╝╚═╝  ╚═╝ ╚═════╝╚═╝  ╚═╝   ╚═╝  
```

### *Stack. Layer. Encrypt.*

<br/>

![Under Development](https://img.shields.io/badge/status-under%20development-orange?style=for-the-badge)
![Rust](https://img.shields.io/badge/rust-000000?style=for-the-badge&logo=rust&logoColor=white)
![License](https://img.shields.io/badge/license-MIT-blue?style=for-the-badge)
![SHA](https://img.shields.io/badge/SHA-224%20%7C%20256%20%7C%20384%20%7C%20512-darkgreen?style=for-the-badge)

<br/>

</div>

---

## 🧠 What is hashcry?

**hashcry** is a Rust CLI tool that lets you encrypt data by chaining multiple SHA hashing algorithms in sequence. Each layer's output becomes the next layer's input — you control the depth and the order.

<br/>

## ⚡ How it works

```
  INPUT DATA
      │
      ▼
┌─────────────┐
│   SHA-256   │  ◄── Layer 1
└──────┬──────┘
       │
       ▼
┌─────────────┐
│   SHA-512   │  ◄── Layer 2
└──────┬──────┘
       │
       ▼
┌─────────────┐
│   SHA-384   │  ◄── Layer 3
└──────┬──────┘
       │
       ▼
   OUTPUT HASH
```

> The more layers, the harder any reversal becomes — computationally and theoretically.

<br/>

## 🔑 Supported Algorithms

| Algorithm | Digest Size | Speed    |
|-----------|-------------|----------|
| SHA-224   | 224 bits    | Fast     |
| SHA-256   | 256 bits    | Fast     |
| SHA-384   | 384 bits    | Moderate |
| SHA-512   | 512 bits    | Moderate |

<br/>

## 📦 Installation

```bash
git clone https://github.com/yourusername/hashcry.git
cd hashcry
cargo build --release
```

<br/>

## 🖥️ Usage

```bash
hashcry encrypt <algo1> <algo2> ... <algoN> <data>
```

**Examples:**

```bash
# Single layer
hashcry encrypt sha256 "hello world"

# Double layer
hashcry encrypt sha256 sha512 "hello world"

# Full chain
hashcry encrypt sha224 sha256 sha384 sha512 "hello world"
```

<br/>

## 🗺️ Roadmap

- [x] SHA-224 / 256 / 384 / 512 support
- [ ] N-layer chaining pipeline
- [ ] File input/output support
- [ ] Verbose layer trace (`--verbose`)
- [ ] JSON output mode (`--json`)
- [ ] Config file support (`.hashcry.toml`)

<br/>

---

<div align="center">

Built with 🦀 Rust &nbsp;·&nbsp; ⚠️ Under Active Development &nbsp;·&nbsp; MIT License

</div>