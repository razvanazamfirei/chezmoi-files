# Contributing to chezmoi-files

Thank you for your interest in contributing to chezmoi-files!
This document provides guidelines and instructions for contributing.

## Code of Conduct

Be respectful and constructive in all interactions. We're all here to make this project better.

## Getting Started

### Prerequisites

- Rust 1.92.0 or later
- Git
- Cargo (comes with Rust)

### Setting Up Your Development Environment

1. Fork the repository on GitHub
2. Clone your fork:
   ```bash
   git clone https://github.com/YOUR_USERNAME/chezmoi-files.git
   cd chezmoi-files
   ```
3. Add the upstream repository:
   ```bash
   git remote add upstream https://github.com/razvanazamfirei/chezmoi-files.git
   ```
4. Create a new branch for your feature:
   ```bash
   git checkout -b feature/your-feature-name
   ```

## Development Workflow

### Building

```bash
cargo build
```

### Running

```bash
echo "test/path" | cargo run
```

### Testing

Run all tests:

```bash
cargo test
```

Run specific tests:

```bash
cargo test test_name
```

Run tests with coverage:

```bash
cargo llvm-cov --all-features --workspace --html
```

### Linting

The project uses clippy with strict lints. Run:

```bash
cargo clippy --all-targets --all-features
```

Fix issues automatically where possible:

```bash
cargo clippy --fix --all-targets --all-features
```

### Formatting

Format your code:

```bash
cargo fmt
```

Check formatting without modifying files:

```bash
cargo fmt -- --check
```

## Making Changes

### Code Style

- Follow Rust conventions and idioms
- Write clear, self-documenting code
- Add comments only when the logic isn't obvious
- Keep functions small and focused
- Use descriptive variable names

### Commit Messages

Write clear, concise commit messages:

```
Add feature: brief description

Longer explanation if needed, wrapping at 72 characters.
Explain what and why, not how.

Fixes #123
```

### Testing Requirements

- Add tests for all new functionality
- Ensure all tests pass before submitting PR
- Aim for high test coverage (>80%)
- Include both unit tests and integration tests where appropriate

### Documentation

- Update README.md if adding user-facing features
- Add rustdoc comments for public APIs
- Update CHANGELOG.md following the existing format
- Update configuration examples if changing config format

## Submitting Changes

### Pull Request Process

1. Update your branch with the latest upstream changes:
   ```bash
   git fetch upstream
   git rebase upstream/main
   ```

2. Push your changes to your fork:
   ```bash
   git push origin feature/your-feature-name
   ```

3. Create a Pull Request on GitHub with:
    - Clear title describing the change
    - Description of what changed and why
    - Reference to any related issues
    - Screenshots if applicable

4. Wait for review and address feedback

### PR Checklist

Before submitting, ensure:

- [ ] All tests pass (`cargo test`)
- [ ] Code is formatted (`cargo fmt`)
- [ ] No clippy warnings (`cargo clippy`)
- [ ] Documentation is updated
- [ ] CHANGELOG.md is updated
- [ ] Commit messages are clear
- [ ] Branch is rebased on latest main

## Types of Contributions

### Bug Reports

When filing a bug report, include:

- Clear description of the issue
- Steps to reproduce
- Expected vs actual behavior
- Your environment (OS, Rust version)
- Relevant error messages

### Feature Requests

When suggesting a feature:

- Explain the use case
- Describe the desired behavior
- Consider backwards compatibility
- Be open to discussion about implementation

### Code Contributions

We welcome:

- Bug fixes
- New features
- Performance improvements
- Documentation improvements
- Test coverage improvements

## Project Structure

```
chezmoi-files/
├── src/
│   ├── main.rs       # CLI entry point and main logic
│   ├── config.rs     # Configuration handling
│   ├── color.rs      # Color scheme management
│   └── tree.rs       # Tree structure and rendering
├── tests/            # Integration tests
├── CLAUDE.md         # Project documentation for Claude
├── CHANGELOG.md      # Version history
├── CONTRIBUTING.md   # This file
├── LICENSE           # Apache 2.0 license
└── README.md         # User documentation
```

## Code Review

All contributions go through code review. Reviewers will check:

- Code quality and style
- Test coverage
- Documentation completeness
- Performance implications
- Backwards compatibility

Be patient and responsive to feedback. Code review is collaborative!

## Release Process

Releases are managed by maintainers:

1. Update version in Cargo.toml
2. Update CHANGELOG.md
3. Create git tag
4. Publish to crates.io
5. Create GitHub release

## Getting Help

- Open an issue for questions
- Check existing issues and PRs
- Read the documentation

## License

By contributing, you agree that your contributions will be licensed under the Apache 2.0 License.
