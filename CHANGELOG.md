# Release Notes

## [Unreleased]

## [0.1.0] - 2026-02-02

### 🎉 Initial Release
Theme Creator v0.1.0 introduces a comprehensive TUI application for creating custom theme packages from your current desktop environment.

### ✨ Features
- **15+ Theme Components** with real-time style detection
  - GTK Themes (GTK2/GTK3 support)
  - Icons & Cursors (separate components)
  - Qt/KDE Styles (Qt5/Qt6)
  - Application Style (Oxygen, Edge Runner, etc.)
  - Colors Schemes (KDE Plasma)
  - Window Decorations (KWin, AwesomeWM, Openbox)
  - Splash Screen (Plymouth, GRUB)
  - SDDM Theme (login manager)
  - Terminal Themes (Kitty, Alacritty)
  - Window Manager Themes (i3, BSPWM)
  - Shell Themes (Bash, Zsh, Fish)
  - Fonts (system font detection)

### 🔍 Style Detection
- **Real-time Detection**: Shows currently active styles for each component
- **Multiple Methods**: GSettings, config files, directory scanning
- **Desktop Environment Support**:
  - KDE Plasma (full support)
  - GNOME (GTK/GSettings integration)
  - Window Managers (i3, BSPWM, Openbox, AwesomeWM)
  - Hybrid setups (KDE+GTK environments)

### 🛡️ Permission Management
- **Pre-creation Checks**: Validates access to all required paths
- **Smart Resolution Options**:
  - Option 1: Re-run with sudo privileges
  - Option 2: Generate ready-to-use chmod commands
- **Clipboard Integration**: Automatic copy support (xclip, wl-copy, xsel)
- **Fallback Support**: Terminal output when clipboard unavailable

### 🖥 User Interface
- **Modern TUI**: Built with Ratatui framework
- **Full Scrolling**: Navigate through all components seamlessly
- **Keyboard Navigation**:
  - Arrow keys: Navigate components
  - Space: Toggle selection
  - Enter: Continue to next step
  - q/Esc: Exit application
- **Visual Feedback**: Current styles displayed with color indicators

### 📦 Theme Package Creation
- **Organized Structure**: Component-based folder organization
- **Recursive Copying**: Preserves directory structure
- **Metadata Generation**: Creates `theme_info.txt` with details
- **Safe Operations**: Error handling throughout the process

### 🐛 Robustness
- **Error Handling**: Comprehensive error messages with context
- **Graceful Failures**: Safe terminal cleanup on errors
- **Path Validation**: Checks for file/directory existence
- **Permission Detection**: Proactive access verification

### 🔧 Technical
- **Rust 1.93.0**: Modern memory-safe language
- **Cross-Platform**: Linux-focused with extensible architecture
- **No External Dependencies**: Self-contained theme detection
- **Performance**: Efficient file operations and TUI rendering

### 📋 Output Format
Created themes organized as:
```
~/CustomThemes/ThemeName/
├── GTK_Themes/
├── Icons/
├── Cursors/
├── Application_Style/
├── Window_Decorations/
├── Splash_Screen/
├── SDDM_Theme/
├── Terminal_Themes/
├── Window_Manager_Themes/
├── Shell_Themes/
├── Fonts/
└── theme_info.txt
```

### 🎯 Supported Environments
- **KDE Plasma**: Complete theme and configuration support
- **GNOME**: Full GTK3/GTK4 integration
- **Window Managers**: i3, BSPWM, Openbox, AwesomeWM support
- **Hybrid Setups**: Mixed KDE/GTK environments
- **Multiple Toolkits**: Qt + GTK application detection

### 📚 Documentation
- **Comprehensive README**: Installation, usage, and troubleshooting
- **Examples**: Theme creation and restoration guides
- **Contributing Guide**: Development setup and contribution guidelines
- **MIT License**: Permissive open-source license

---

## Installation

### Cargo Install (Recommended)
```bash
cargo install theme-creator
```

### From Source
```bash
git clone <repository>
cd theme-creator
cargo build --release
sudo cp target/release/theme-creator /usr/local/bin/
```

## Quick Start

1. Run `theme-creator` in a terminal
2. Select theme components with arrow keys and spacebar
3. Name your theme when prompted
4. Choose permission options if needed
5. Your theme package is created in `~/CustomThemes/`

## Known Limitations

- **Terminal Required**: Must run in real terminal (not SSH without `-t`)
- **Linux Focused**: Currently supports Linux desktop environments
- **Admin Privileges**: Some paths may require sudo access
- **Theme Dependencies**: Restored themes require installed frameworks

## Acknowledgments

Built with:
- [Ratatui](https://github.com/ratatui-org/ratatui) - TUI framework
- [Crossterm](https://github.com/crossterm-rs/crossterm) - Terminal handling
- Desktop environment communities for configuration standards

---

**Thank you for using Theme Creator v0.1.0! 🎨**

For bug reports and feature requests, visit our GitHub repository.