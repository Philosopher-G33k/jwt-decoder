# jwt-decoder
A lightweight and efficient tool written in Rust for decoding JSON Web Tokens (JWT)

## Usage

```bash
certinfo [OPTIONS] <DOMAIN>
```

### Options

- `-d, --decode` : provide JWT string which needs decoding.
- `-h, --help`  : Display help information.

### Example

```bash
jwtd -d example.com
```

## Building from Source

jwt-decoder is written in Rust. Make sure you have Rust and Cargo installed on your system.

```bash
git clone https://github.com/Philosopher-G33k/jwt-decoder
cd jwt-decoder
cargo build --release
```

## Running

After building the tool, you can run it using the following command:

```bash
./target/release/jwtd [OPTIONS] <JWT>
```

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.


## Acknowledgments

- The Rust Community

## Disclaimer

This tool is provided "as is" without warranty of any kind. Use at your own risk.

## Author

[Ishan Malviya](https://github.com/username)
