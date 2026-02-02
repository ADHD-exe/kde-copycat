# KDE-Copycat - Plasma KDE 6 Global Theme & Configuration Builder

A powerful terminal-based application for compiling Global KDE themes, configurations, and desktop settings by bundling you're current Global Theme Configuration.

<img width="950" height="1020" alt="Screenshot_20260202_061821" src="https://github.com/user-attachments/assets/75bfb632-2c3f-406f-b386-4f16fc4a38f6" />

![KDE Logo](https://img.shields.io/badge/KDE-💙-blue?style=flat&logo=kde)
![Rust Version](https://img.shields.io/badge/rust-1.93.0+-orange?style=flat&logo=rust)
![License](https://img.shields.io/badge/license-MIT-blue?style=flat)

## Features

### 🎨 KDE-Focused Theme Backup
- **KDE Plasma Themes** - Complete Plasma theme backup with wallpapers, widgets, and schemes
- **KWin Configurations** - Window manager settings, effects, and decorations
- **Plasma Settings** - Desktop configuration files and preferences
- **Application Settings** - KDE-specific application configurations
- **Color Schemes** - KDE Plasma color schemes and palettes
- **Icon Themes** - KDE and system icon collections
- **Widget Configurations** - Plasma widgets and their settings

### 🔍 Real-Time Configuration Detection
Each component displays your currently active KDE settings:
```
[x] KDE Plasma Theme
    Complete desktop theme with widgets
    → KDE Theme: Sweet-Dark

[x] KWin Configuration  
    Window manager settings and effects
    → KWin Effects: Blur, Shake
```

### 🔐 Smart Permission Management
- Automatic permission checking before backup
- Option to re-run with sudo privileges for system-wide access
- Generate ready-to-use chmod commands
- Clipboard integration for easy command copying

### 📦 Organized Backup Creation
- KDE component-based folder structure
- Recursive file copying with permissions preservation
- Backup metadata generation with timestamp
- Created in `~/KDE-Backups/[backup-name]/`

### 🖥 KDE-Specific Features
- **Plasma Integration**: Direct Plasma 5/6 support
- **KWin Support**: Window manager settings and effects backup
- **Konsole Profiles**: Terminal configurations and color schemes
- **Dolphin Settings**: File manager configurations and service menus
- **System Settings**: KDE system configuration files

## Installation

### Prerequisites
- **Rust 1.93.0+** - Install from [rustup.rs](https://rustup.rs/)
- **KDE Plasma** - Primary desktop environment (partial KDE support available)
- **Terminal** - Must run in a real terminal

### Install with Cargo (Recommended)
```bash
cargo install kde-copycat
```

### Build from Source
```bash
git clone <repository-url>
cd kde-copycat
cargo build --release
sudo cp target/release/kde-copycat /usr/local/bin/
```

## Usage

### Basic Usage
```bash
kde-copycat
# or
cargo run
```

### Navigation Controls
- **↑/↓ or ←/→** - Navigate components
- **Space** - Toggle component selection
- **Enter** - Continue to backup naming
- **q/Esc** - Exit application

### KDE Component Selection
1. **Plasma Themes** - Complete desktop themes with widgets and wallpapers
2. **KWin Settings** - Window manager configurations, effects, and decorations
3. **Plasma Config** - Desktop settings, panels, and workspace configuration
4. **KDE Applications** - KDE-specific app configurations and profiles
5. **Color Schemes** - Plasma color schemes and palettes
6. **Icon Themes** - KDE and system icon collections
7. **Konsole** - Terminal profiles, color schemes, and settings

### Permission Options
When permission issues are detected, choose:

**Option 1: Re-run with Sudo**
- Automatically restarts with elevated privileges
- Maintains your selections
- Preserves backup name input

**Option 2: Generate Chmod Commands**
- Creates ready-to-paste commands like:
  ```bash
  sudo chmod -R 755 "/usr/share/kde4/config"
  sudo chmod -R 755 "~/.config/kdeglobalrc"
  ```
- Attempts clipboard integration (xclip/wl-copy/xsel)
- Falls back to terminal output

## KDE Backup Structure

Created backups are organized as:
```
~/KDE-Backups/YourBackupName/
├── KDE_Plasma_Themes/
│   ├── plasma/
│   ├── look-and-feel/
│   └── wallpapers/
├── KWin_Settings/
│   ├── kwinrc
│   └── kwinrules/
├── Plasma_Config/
│   ├── plasmarc
│   └── kdeglobals
├── KDE_Applications/
│   ├── konsole/
│   └── dolphin/
├── KDE_Color_Schemes/
│   ├── Sweet-Dark.colors
│   └── Oxygen.colors
├── KDE_Icon_Themes/
│   ├── breeze/
│   └── Papirus/
└── backup_info.txt
```

### backup_info.txt
Contains metadata about your KDE backup:
```
Backup Name: MyKdeBackup
Created: 2026-02-02 15:30:45 UTC
KDE Components:
- KDE Plasma Themes: Complete desktop theme with widgets
- KWin Settings: Window manager configurations
- Color Schemes: KDE Plasma color schemes and palettes
```

## Supported KDE Environments

### KDE Plasma 5/6
- ✅ Full theme backup support
- ✅ Plasma widget configurations
- ✅ KWin effects and settings
- ✅ System settings integration
- ✅ Application-specific configs

### Partial KDE Support
- ✅ KDE applications on other desktops
- ✅ QT theme integration
- ✅ KDE configuration file detection

## Troubleshooting

### "No such device or address" Error
**Cause**: Running in unsupported terminal or non-interactive session  
**Solution**: Run in a real terminal directly (not through SSH without `-t`, multiplexers, or IDE terminals)

### KDE Components Not Found
**Cause**: KDE not properly installed or configured  
**Solution**: Install required tools:
```bash
# KDE Plasma
sudo apt install kde-config kde-config-widgets plasma-desktop-data

# KDE Frameworks
sudo apt install kde-frameworks5-extra kconfig

# KDE System Settings
sudo apt install systemsettings ksysguard
```

### Permission Errors
**Cause**: No access to KDE configuration directories  
**Solutions**:
1. Use sudo re-run option (built-in)
2. Generate chmod commands (built-in)
3. Manually fix permissions:
   ```bash
   sudo chmod -R 755 ~/.config/kdeglobalrc
   sudo chmod -R 755 /usr/share/kde4/config
   sudo chmod -R 755 /usr/share/knotifications5
   ```

### Empty Backup Package
**Cause**: Selected KDE paths don't exist or aren't accessible  
**Solution**: Verify KDE installation:
```bash
ls ~/.config/kde*     # Check KDE config
ls /usr/share/kde4/   # Check system KDE
kde5-config --version     # Check KDE version
```

## Development

### Building for Development
```bash
cd kde-copycat
cargo build
```

### Running Tests
```bash
cargo test
```

### Code Quality
```bash
cargo clippy  # Linting
cargo fmt      # Formatting
```

## Contributing

Contributions are welcome! Please:

1. **Fork** the repository
2. **Create** a feature branch: `git checkout -b feature/amazing-kde-feature`
3. **Commit** your changes: `git commit -m 'Add amazing KDE feature'`
4. **Push** to the branch: `git push origin feature/amazing-kde-feature`
5. **Open** a Pull Request

### Areas for Contribution
- **More KDE Components** - Support additional KDE applications
- **Plasma 6 Features** - Latest Plasma desktop enhancements
- **Widget Support** - Plasma widget configuration backup
- **Effect Settings** - KWin desktop effects integration
- **Export Formats** - Support for KDE theme pack formats
- **Cross-Platform** - Windows and macOS KDE support

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Changelog

### v0.1.0 (2026-02-02)
- ✅ Initial release focused on KDE environments
- ✅ 7+ KDE-specific components with detection
- ✅ Real-time KDE configuration detection
- ✅ Permission management with sudo/chmod options
- ✅ Full TUI interface with scrolling
- ✅ KDE Plasma 5/6 support
- ✅ KWin configuration backup
- ✅ Konsole profile preservation
- ✅ System settings integration

## Acknowledgments

- **[Ratatui](https://github.com/ratatui-org/ratatui)** - TUI framework
- **[Crossterm](https://github.com/crossterm-rs/crossterm)** - Terminal handling
- **[FS Extra](https://github.com/webdesus/fs_extra)** - File operations
- **KDE Community** - For configuration standards documentation

---

**Made with 💙 Rust for KDE desktop users**

[GitHub Repository](https://github.com/yourusername/kde-copycat) | [Report Issues](https://github.com/yourusername/kde-copycat/issues) | [Request Features](https://github.com/yourusername/kde-copycat/discussions)# kde-copycat
