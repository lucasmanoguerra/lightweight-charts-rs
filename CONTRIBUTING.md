# Contributing to Lightweight Charts RS

Thank you for your interest in contributing to Lightweight Charts RS! This document provides guidelines and information for contributors.

## 🚀 Getting Started

### Prerequisites

- **Rust**: Latest stable version (1.70+ recommended)
- **GTK4**: Development libraries for your platform
- **Cairo**: Development libraries for rendering

#### Linux (Ubuntu/Debian)
```bash
sudo apt update
sudo apt install rustc cargo libgtk-4-dev libcairo2-dev
```

#### Fedora
```bash
sudo dnf install rust cargo gtk4-devel cairo-devel
```

#### macOS
```bash
brew install rust gtk4 cairo
```

#### Windows
Follow the [GTK4 installation guide](https://www.gtk.org/docs/installations/windows) and install Rust via [rustup](https://rustup.rs/).

### Development Setup

1. **Fork the repository**
   ```bash
   git clone https://github.com/your-username/lightweight-charts-rs.git
   cd lightweight-charts-rs
   ```

2. **Add upstream remote**
   ```bash
   git remote add upstream https://github.com/lucasmanoguerra/lightweight-charts-rs.git
   ```

3. **Create a development branch**
   ```bash
   git checkout -b feature/your-feature-name
   ```

4. **Install dependencies and build**
   ```bash
   cargo build
   ```

## 📋 Development Workflow

### Before You Start

- Check existing [issues](https://github.com/lucasmanoguerra/lightweight-charts-rs/issues) and [pull requests](https://github.com/lucasmanoguerra/lightweight-charts-rs/pulls)
- Look for issues labeled `good first issue` for beginner-friendly tasks
- Comment on the issue you plan to work on to avoid duplication

### Making Changes

1. **Write code**: Follow the existing code style and conventions
2. **Test thoroughly**: Ensure your changes work as expected
3. **Update documentation**: If your change affects the API or user experience
4. **Run tests**: Make sure all tests pass

### Testing

```bash
# Run all tests
cargo test

# Run tests with output
cargo test -- --nocapture

# Run specific test
cargo test test_name

# Run clippy for linting
cargo clippy -- -D warnings

# Format code
cargo fmt

# Check documentation
cargo doc --no-deps --open
```

### Committing Changes

Follow conventional commit format:
- `feat:` for new features
- `fix:` for bug fixes
- `docs:` for documentation changes
- `style:` for code style changes
- `refactor:` for code refactoring
- `test:` for adding or modifying tests
- `chore:` for maintenance tasks

Examples:
```
feat: add Bollinger Bands indicator
fix: resolve memory leak in chart rendering
docs: update installation instructions
```

### Pull Request Process

1. **Update your fork**
   ```bash
   git fetch upstream
   git rebase upstream/main
   ```

2. **Push your branch**
   ```bash
   git push origin feature/your-feature-name
   ```

3. **Create a Pull Request**
   - Use the PR template
   - Provide a clear description
   - Link to related issues
   - Include screenshots if applicable

4. **Address feedback**
   - Respond to reviewer comments promptly
   - Make requested changes
   - Keep your PR up to date

## 🏗️ Code Style and Conventions

### Rust Guidelines

- Use `cargo fmt` for consistent formatting
- Use `cargo clippy` for linting (fix all warnings)
- Prefer explicit types for public APIs
- Write comprehensive documentation comments (`///`)
- Use `#[derive(Debug)]` for public types
- Error handling with `Result<T, E>` where appropriate

### Code Organization

```
src/
├── app/              # Application logic and state
├── chart/            # Core charting functionality
│   ├── core/        # Core chart rendering
│   └── api.rs        # Public API
├── indicators/       # Technical indicators
├── ui/               # UI components
├── settings_ui/      # Settings panel
└── lib.rs           # Library exports
```

### Naming Conventions

- **Types**: `PascalCase`
- **Functions/Methods**: `snake_case`
- **Constants**: `SCREAMING_SNAKE_CASE`
- **Modules**: `snake_case`

## 📖 Documentation

### Code Documentation

All public items must have documentation comments:

```rust
/// Creates a new candlestick chart with the specified symbol and style.
///
/// # Arguments
///
/// * `symbol` - The trading symbol (e.g., "BTC/USD")
/// * `style` - Chart styling options
///
/// # Examples
///
/// ```
/// use lightweight_charts_rs::{create_chart, ChartStyle};
/// let chart = create_chart("BTC/USD", ChartStyle::default());
/// ```
pub fn create_chart(symbol: &str, style: ChartStyle) -> Chart {
    // implementation
}
```

### README Updates

Update the README when:
- Adding new features
- Changing installation requirements
- Updating API usage examples
- Modifying supported platforms

## 🐛 Bug Reports

When reporting bugs:
1. Use the bug report template
2. Provide minimal reproduction code
3. Include system information (OS, Rust version, GTK version)
4. Add screenshots if applicable
5. Include full error messages and stack traces

## 💡 Feature Requests

When suggesting features:
1. Check if it already exists or is planned
2. Explain the use case clearly
3. Consider the impact on existing users
4. Suggest API design if possible
5. Break down large features into smaller tasks

## 🎯 Development Areas

We welcome contributions in these areas:

### High Priority
- Bug fixes and stability improvements
- Performance optimizations
- Additional technical indicators
- Enhanced documentation

### Medium Priority
- Chart drawing tools
- Export functionality
- WebSocket connection improvements
- UI/UX enhancements

### Low Priority
- Plugin system architecture
- Mobile app support
- WebAssembly compilation
- Advanced chart types

## 🏆 Recognition

Contributors are recognized in:
- README contributors section
- Release notes
- Documentation acknowledgments

## 📞 Getting Help

- **Discussions**: [GitHub Discussions](https://github.com/lucasmanoguerra/lightweight-charts-rs/discussions)
- **Issues**: [GitHub Issues](https://github.com/lucasmanoguerra/lightweight-charts-rs/issues)
- **Documentation**: [API Docs](https://docs.rs/lightweight-charts-rs)

## 📜 Code of Conduct

Please read and follow our [Code of Conduct](CODE_OF_CONDUCT.md). We expect all contributors to be respectful and inclusive.

## 🔄 Release Process

1. Update version in `Cargo.toml`
2. Update CHANGELOG.md
3. Create git tag
4. Publish to crates.io
5. Create GitHub release

---

Thank you for contributing to Lightweight Charts RS! 🎉