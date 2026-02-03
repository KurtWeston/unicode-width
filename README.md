# unicode-width

Calculate the true display width of Unicode strings for proper terminal alignment

## Features

- Calculate accurate display width for strings containing any Unicode characters
- Handle emojis (including multi-codepoint sequences with ZWJ)
- Handle CJK (Chinese, Japanese, Korean) wide characters
- Handle combining characters and zero-width characters
- Truncate strings to fit within a specified display width
- Pad strings to reach a desired display width (left, right, center)
- Process input from stdin or command-line arguments
- Support batch processing of multiple strings
- Display character-by-character width breakdown for debugging
- Return exit code 0 for success, non-zero for errors

## How to Use

Use this project when you need to:

- Quickly solve problems related to unicode-width
- Integrate rust functionality into your workflow
- Learn how rust handles common patterns

## Installation

```bash
# Clone the repository
git clone https://github.com/KurtWeston/unicode-width.git
cd unicode-width

# Install dependencies
cargo build
```

## Usage

```bash
cargo run
```

## Built With

- rust

## Dependencies

- `unicode-width`
- `clap`

## Contributing

1. Fork the repository
2. Create your feature branch (`git checkout -b feature/amazing-feature`)
3. Commit your changes (`git commit -m 'Add amazing feature'`)
4. Push to the branch (`git push origin feature/amazing-feature`)
5. Open a Pull Request

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.
