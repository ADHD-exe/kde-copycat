# KDE-Copycat (Plasma 6)

Terminal app that bundles your current KDE Plasma 5/6 setup into a reusable backup/theme-style package. It detects what you’re using now (theme, effects, icons, etc.) and exports the relevant config files into a clean folder structure.

<img width="475" height="510" alt="CopyCat" src="https://github.com/user-attachments/assets/101c5a27-e3f1-4ed9-adbc-e7bfd239599c" />

## What it backs up
- Plasma global theme bits (look-and-feel, wallpapers, widgets/panels where applicable)
- KWin settings (effects, decorations, rules)
- Plasma desktop config files and preferences
- KDE app configs (ex: Konsole profiles, Dolphin settings)
- Color schemes + icon themes

## How it works
- Shows your active KDE settings while you pick what to include
- Copies files into `~/KDE-Backups/<name>/` and writes a `backup_info.txt`
- If permissions block a path, it can re-run with sudo or generate chmod commands

## Install

### Cargo (recommended)
```bash
cargo install kde-copycat
````

### Build from source

```bash
git clone <repository-url>
cd kde-copycat
cargo build --release
sudo cp target/release/kde-copycat /usr/local/bin/
```
### Keys

* Arrow keys: move
* Space: toggle selection
* Enter: continue
* q / Esc: quit
