# HTA Generator

HTA Generator is a Rust-based tool designed to generate HTA (HTML Application) files with embedded drivers. This tool is intended for educational and research purposes only.

## Features

- Embeds driver files into HTA files
- Base64 encodes driver data
- Generates a runnable HTA file for Windows

## Installation

To install HTA Generator, follow these steps:

1. **Clone the Repository:**
   ```
   git clone https://github.com/0romos/driver-hta-gen-hsdk.git
   cd hta-gen
   ```

2. **Build the Project:**
   ```
   cargo build --release
   ```

## Usage

Run the tool with the following command:
   ```
   ./target/release/hta-gen <output_file> <driver_file>
   ```

### Example
   ```
   ./target/release/hta-gen output.hta driver.bin
   ```

### Configuration

The tool can be configured using a TOML configuration file. Create a file named `config.toml` with the following structure:
   ```
   [settings]
   log_level = "info"
   ```

To use this configuration file, set the `CONFIG_FILE` environment variable:
   ```
   CONFIG_FILE=config.toml ./target/release/hta-gen output.hta driver.bin
   ```

### Debugging

Enable debug logging by setting the `LOG_LEVEL` environment variable:
   ```
   LOG_LEVEL=debug ./target/release/hta-gen output.hta driver.bin
   ```

## Contributing to HTA Generator

We welcome contributions to HTA Generator! To contribute, please follow these guidelines:

### Reporting Issues

If you encounter any issues, please open an issue on the [GitHub Issues page](https://github.com/0romos/driver-hta-gen-hsdk/issues).

### Pull Requests

1. Fork the repository.
2. Create a new branch for your feature or bug fix.
3. Write tests for your changes.
4. Submit a pull request with a clear description of your changes.

### Coding Standards

- Follow Rust's coding conventions.
- Write clear and concise commit messages.
- Ensure all tests pass before submitting a pull request.

Thank you for contributing!

## Changelog

### [Unreleased]

#### Added
- Initial release with basic HTA generation functionality.

### [0.1.0] - 2024-08-26

#### Added
- Basic HTA generation with Base64 encoded driver data.
- Command-line interface support.
- Configuration file handling.

#### Changed
- Improved logging with various levels.

## Troubleshooting

### Common Issues

#### Issue: HTA file not generated

**Possible Causes:**
- Incorrect file paths.
- Permissions issues.

**Solutions:**
- Verify that the file paths are correct.
- Ensure you have the necessary permissions to write to the output directory.

#### Issue: Base64 encoding errors

**Possible Causes:**
- Corrupted driver file.

**Solutions:**
- Check that the driver file is not corrupted and is properly formatted.

### Getting Help

If you need further assistance, please open an issue on the [GitHub Issues page](https://github.com/0romos/driver-hta-gen-hsdk/issues).
