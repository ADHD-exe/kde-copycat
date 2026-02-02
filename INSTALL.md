# Theme Creator

A powerful terminal-based application for creating custom theme packages by backing up your current desktop environment configurations.

## Installation

### From Cargo (Recommended)
```bash
cargo install theme-creator
```

### From Source
```bash
git clone <repository-url>
cd theme-creator
cargo build --release
sudo cp target/release/theme-creator /usr/local/bin/
```

## Quick Start

```bash
theme-creator
```

1. Select components with arrow keys and spacebar
2. Enter a theme name
3. Choose permission options if prompted
4. Your theme package is created in `~/CustomThemes/ThemeName/`

## Features

- **15+ Theme Components**: GTK, Icons, Cursors, Window Decorations, Splash Screen, SDDM, etc.
- **Real-Time Detection**: Shows currently active styles for each component
- **Smart Permission Handling**: Sudo re-run or chmod command generation
- **Full TUI Interface**: Scrollable list with keyboard navigation
- **Comprehensive Support**: KDE Plasma, GNOME, i3, BSPWM, Openbox, AwesomeWM

## System Requirements

- Rust 1.93.0+
- Linux with desktop environment
- Real terminal access (not SSH without `-t`)

For detailed documentation, see [README.md](README.md).