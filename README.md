have fun with the chatgpt ahh readme

# Screenshot Auto Upload

A command-line tool that watches a directory for new screenshots and automatically uploads them to various image hosting services.

## Currently Supported Services
- s-ul.eu
- imgur (coming soon)

## Installation

1. Make sure you have Rust installed on your system
2. Clone this repository
3. Build the project:
```bash
cargo build --release
```

## Usage

Basic usage:
```bash
screenshot-auto-upload -p /path/to/screenshots/folder -t s-ul -k your_api_key
```

### Command Line Arguments

- `-p, --path`: Path to the directory to watch for screenshots
- `-t, --provider`: Image hosting service to use (`s-ul` or `imgur`)
- `-k, --key`: API key for the selected service
- `-s, --save-config`: Save the current configuration for future use
- `-c, --config`: Path to config file (default: system config directory)

### Configuration

You can save your configuration to avoid typing the same arguments every time:

```bash
screenshot-auto-upload -p /path/to/screenshots -t s-ul -k your_api_key -s
```

Note: If you pass in arguments, they will be favored over the values in the config file.

The config will be saved to:
- Linux: `~/.config/screenshot-auto-upload/config.json`
- macOS: `~/Library/Application Support/screenshot-auto-upload/config.json`
- Windows: `%APPDATA%\screenshot-auto-upload\config.json`

## License

[MIT License](LICENSE)
