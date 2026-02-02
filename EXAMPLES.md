# Example theme packages created by Theme Creator

## Theme Structure
Each created theme follows this organized structure:

```
~/CustomThemes/MyThemeName/
├── GTK_Themes/           # GTK2/GTK3 theme files
├── Icons/                 # Icon themes  
├── Cursors/               # Mouse cursor themes
├── Application_Style/      # Application styling configs
├── Window_Decorations/    # Window manager decorations
├── Splash_Screen/         # Boot splash screen themes
├── SDDM_Theme/          # SDDM login manager theme
├── Terminal_Themes/       # Terminal emulator themes
├── Window_Manager_Themes/  # WM configurations
├── Shell_Themes/          # Shell themes (Bash, Zsh, Fish)
├── Application_Styles/     # Application-specific styling
└── theme_info.txt        # Theme metadata
```

## Restoring a Theme

### From the Package
```bash
# Copy GTK themes
cp -r ~/CustomThemes/MyThemeName/GTK_Themes/* ~/.themes/

# Copy icon themes  
cp -r ~/CustomThemes/MyThemeName/Icons/* ~/.icons/

# Copy cursor themes
cp -r ~/CustomThemes/MyThemeName/Cursors/* ~/.icons/

# Copy SDDM theme (requires sudo)
sudo cp -r ~/CustomThemes/MyThemeName/SDDM_Theme/* /usr/share/sddm/themes/

# Update SDDM configuration
sudo sed -i 's/Current=.*/Current=MyThemeName/' /etc/sddm.conf

# Reload desktop environment
# KDE: kquitapp5 plasmashell && kstart5 plasmashell
# GNOME: killall -r gnome-shell && gnome-shell --replace
```

## Sharing Themes

### Package for Distribution
```bash
cd ~/CustomThemes/
tar -czf MyThemeName.tar.gz MyThemeName/
zip -r MyThemeName.zip MyThemeName/

# Optional: Create install script
cat > install-theme.sh << 'EOF'
#!/bin/bash
THEME_NAME="MyThemeName"
DEST_DIR="$HOME/.themes"
BACKUP_DIR="$HOME/.themes-backup"

# Backup existing themes
mkdir -p "$BACKUP_DIR"
cp -r "$DEST_DIR"/* "$BACKUP_DIR/" 2>/dev/null

# Install new theme
cp -r "$HOME/CustomThemes/$THEME_NAME/GTK_Themes"/* "$DEST_DIR/"
echo "Theme installed! Restart your desktop environment to apply."
EOF
chmod +x install-theme.sh
```

## Compatibility Notes

### KDE Plasma
- GTK themes: Work with KDE applications
- KDE themes: Full integration with Plasma
- Window decorations: KWin-specific

### GNOME
- GTK themes: Native integration
- Icons: Automatic theme detection
- Extensions: May need manual configuration

### Window Managers
- i3: GTK themes work for i3-gaps
- BSPWM: inherits system GTK themes  
- Openbox: requires openbox --reconfigure
- AwesomeWM: edit rc.lua for theme paths

## Example: Creating a KDE Theme Package

1. Run `theme-creator`
2. Select: 
   - [x] Qt/KDE Styles
   - [x] Colors Schemes  
   - [x] SDDM Theme
   - [x] Window Decorations
3. Name: "MyKdeTheme"
4. Theme created in `~/CustomThemes/MyKdeTheme/`

This creates a complete KDE theme package ready for sharing or backup!