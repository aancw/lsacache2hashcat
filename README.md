# lsacache2hashcat
Give me lsadump:cache from mimikatz, I will transform it to DCC2 Hashcat compatible. Useful for so many credentials cache

## Feature
- Parsing DCC2 dump cache from mimikatz

## Requirements
- Rust
- Cargo

## Build Instructions
- For binary release, build it with 
```
cargo build --release
```
- For binary debug, build it with
```
cargo build
```

## Usage
Before you can use it, you need to build first. Please read build instructions

- Help information
```
Give me lsadump:cache from mimikatz, I will transform it to DCC2 Hashcat compatible. Useful for so many credentials cache

Usage: lsacache2hashcat --file <FILE>

Options:
  -f, --file <FILE>  Lsadump cache output from mimikatz
  -h, --help         Print help information
  -V, --version      Print version information
```

- Parsing mimikatz lsadump::cache output
```
lsacache2hashcat --file <FILE>
```

## License

MIT License