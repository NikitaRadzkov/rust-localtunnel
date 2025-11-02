# Contributing to rust-localtunnel

Thank you for your interest in contributing to rust-localtunnel! This document provides guidelines and instructions for contributing to the project.

## Code of Conduct

By participating in this project, you agree to abide by our [Code of Conduct](CODE_OF_CONDUCT.md).

## How to Contribute

### Reporting Bugs

Before creating bug reports, please check the issue list to see if the problem has already been reported. When creating a bug report, include:

- A clear, descriptive title
- Steps to reproduce the issue
- Expected behavior
- Actual behavior
- Environment details (OS, Rust version, etc.)
- Any relevant error messages or logs

### Suggesting Enhancements

Enhancement suggestions are tracked as GitHub issues. When creating an enhancement suggestion:

- Provide a clear, descriptive title
- Describe the enhancement in detail
- Explain why this enhancement would be useful
- Include examples of how the enhancement would be used

### Pull Requests

1. **Fork the repository**
   ```bash
   git clone https://github.com/yourusername/rust-localtunnel.git
   cd rust-localtunnel
   ```

2. **Create a branch for your changes**
   ```bash
   git checkout -b feature/your-feature-name
   # or
   git checkout -b fix/your-bug-fix
   ```

3. **Make your changes**
   - Write clean, readable code
   - Follow Rust conventions and style
   - Add tests for new functionality
   - Update documentation as needed

4. **Run tests and linters**
   ```bash
   cargo test
   cargo clippy
   cargo fmt --check
   ```

5. **Commit your changes**
   ```bash
   git add .
   git commit -m "Add: description of your changes"
   ```
   
   Use clear, descriptive commit messages. Common prefixes:
   - `Add:` for new features
   - `Fix:` for bug fixes
   - `Update:` for updates to existing features
   - `Refactor:` for code refactoring
   - `Docs:` for documentation changes

6. **Push to your fork**
   ```bash
   git push origin feature/your-feature-name
   ```

7. **Create a Pull Request**
   - Provide a clear title and description
   - Reference any related issues
   - Wait for code review and address any feedback

## Development Guidelines

### Code Style

- Follow Rust standard formatting: `cargo fmt`
- Run Clippy for linting: `cargo clippy`
- Write self-documenting code with clear variable names
- Add comments for complex logic

### Testing

- Write unit tests for new functionality
- Ensure all existing tests pass
- Aim for good test coverage

### Documentation

- Update README.md if adding new features
- Add inline documentation for public APIs
- Include examples in doc comments

### Dependencies

- Be mindful of dependency additions
- Prefer minimal, well-maintained crates
- Discuss significant dependency additions before implementing

## Project Structure

```
rust-localtunnel/
├── src/
│   ├── main.rs          # Server binary
│   └── client.rs        # Client binary
├── Cargo.toml           # Project dependencies
├── README.md            # Project documentation
├── CONTRIBUTING.md      # This file
├── CODE_OF_CONDUCT.md   # Code of conduct
└── LICENSE              # MIT License
```

## Getting Help

If you need help or have questions:

- Open an issue on GitHub
- Check existing issues and discussions
- Review the code and documentation

## Recognition

Contributors will be recognized in:
- The project's README (if applicable)
- Release notes for significant contributions
- GitHub's contributors page

Thank you for contributing to rust-localtunnel! 🎉

