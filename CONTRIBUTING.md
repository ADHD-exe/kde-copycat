# Contributing to Theme Creator

We welcome contributions! Theme Creator is a community-driven project and we appreciate any help.

## Getting Started

1. **Fork** the repository on GitHub
2. **Clone** your fork locally:
   ```bash
   git clone https://github.com/yourusername/theme-creator.git
   cd theme-creator
   ```
3. **Create** a feature branch:
   ```bash
   git checkout -b feature/your-feature-name
   ```

## Development Setup

### Prerequisites
- Rust 1.93.0+
- A Linux desktop environment (for testing)

### Building and Testing
```bash
# Build in development mode
cargo build

# Run tests
cargo test

# Run with examples
cargo run --example

# Check formatting
cargo fmt --check

# Run linter
cargo clippy

# Build release version
cargo build --release
```

## Areas for Contribution

### 1. New Desktop Environment Support
Add detection for:
- **XFCE** - XFWM4 themes, XFCE settings
- **LXDE** - LXPanel themes, Openbox configurations  
- **Cinnamon** - Cinnamon spices, desktop effects
- **MATE** - MATE themes, marco configurations
- **Budgie** - Budgie desktop settings

### 2. Enhanced Style Detection
Improve detection for existing environments:
- **More KDE** - Plasma mobile, KDE Connect settings
- **GNOME** - Shell extensions, Adwaita variants
- **Hybrid Setups** - Better handling of mixed environments
- **Version Detection** - Distinguish between theme versions

### 3. User Interface Improvements
- **Progress Indicators** - Show theme creation progress
- **Preview Mode** - Visual preview of selected components
- **Keyboard Shortcuts** - Additional navigation options
- **Theme Validation** - Check theme package integrity
- **Batch Operations** - Select multiple components at once

### 4. Export/Import Formats
Add support for:
- **OARS Theme Format** - Open Animation Reboot System
- **DEB/RPM Packages** - Installable theme packages
- **Git Integration** - Clone themes from repositories
- **Cloud Storage** - Upload/download from theme services
- **Cross-Platform** - Windows and macOS support

### 5. Advanced Features
- **Theme Editor** - Modify themes before export
- **Dependency Resolution** - Handle theme requirements
- **Rollback System** - Revert theme changes safely
- **Scheduler** - Automatic theme switching by time/location
- **Profile Support** - Different themes for work/gaming

## Code Style

### Formatting
```bash
cargo fmt
```

### Linting
```bash
cargo clippy -- -D warnings
```

### Testing
Write tests for new features:
```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_feature() {
        // Test implementation
    }
}
```

## Documentation

### Code Comments
- Add `///` documentation for public functions
- Include examples in documentation
- Explain complex detection logic

### README Updates
Update relevant sections when adding:
- New components
- New supported environments
- Breaking changes
- New dependencies

## Submitting Changes

### Commit Messages
Use conventional commits:
```
feat: add XFCE desktop environment support
fix: resolve cursor detection on Wayland
docs: update installation instructions
refactor: improve permission checking
test: add integration tests for KDE detection
```

### Pull Requests

1. **Update** documentation if needed
2. **Ensure** all tests pass: `cargo test`
3. **Run** linter: `cargo clippy`
4. **Format** code: `cargo fmt`
5. **Push** to your fork
6. **Create** Pull Request with:
   - Clear title
   - Detailed description
   - Testing instructions
   - Screenshots if UI changes

## Reporting Issues

When reporting bugs, include:
- **System Information**: Distribution, desktop environment, terminal
- **Steps to Reproduce**: Exact commands used
- **Expected Behavior**: What should have happened
- **Actual Behavior**: What actually happened
- **Logs**: Error messages or backtraces

## Feature Requests

Request features with:
- **Use Case**: Why you need this feature
- **Proposed Solution**: How you imagine it working
- **Alternatives**: Solutions you've tried
- **Priority**: Low/Medium/High importance

## Community

- **Discussions**: Use GitHub Discussions for questions
- **Issues**: Use GitHub Issues for bugs and feature requests
- **Code Reviews**: Participate in reviewing other contributions

## Recognition

Contributors will be acknowledged in:
- README.md contributors section
- Release changelog
- GitHub contributor statistics

Thank you for making Theme Creator better for everyone! 🎨