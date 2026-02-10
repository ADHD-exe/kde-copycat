use anyhow::{Context, Result};
use crossterm::{
    event::{self, Event, KeyCode, KeyEventKind},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use dirs::home_dir;
use ratatui::{
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout, Rect},
    prelude::Stylize,
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, List, ListItem, ListState, Paragraph, Wrap},
    Frame, Terminal,
};

use std::path::Path;
use std::{env, fs, io, process::Command};

#[derive(Debug, Clone)]
pub struct ThemeComponent {
    pub name: String,
    pub source_paths: Vec<String>,
    pub description: String,
    pub checked: bool,
    pub current_style: Option<String>,
}

impl ThemeComponent {
    pub fn new(name: &str, source_paths: Vec<&str>, description: &str) -> Self {
        let mut component = Self {
            name: name.to_string(),
            source_paths: source_paths.into_iter().map(|s| s.to_string()).collect(),
            description: description.to_string(),
            checked: false,
            current_style: None,
        };

        component.current_style = component.detect_current_style();
        component
    }

    fn detect_current_style(&self) -> Option<String> {
        match self.name.as_str() {
            "GTK Themes" => detect_gtk_theme(),
            "Icons" => detect_icon_theme(),
            "Cursors" => detect_cursor_theme(),
            "Qt/KDE Styles" => detect_qt_style(),
            "Application Style" => detect_application_style(),
            "Colors Schemes" => detect_color_scheme(),
            "Window Decorations" => detect_window_decorations(),
            "Splash Screen" => detect_splash_screen(),
            "SDDM Theme" => detect_sddm_theme(),
            "Terminal Themes" => detect_terminal_theme(),
            "Window Manager Themes" => detect_wm_theme(),
            "Shell Themes" => detect_shell_theme(),
            "Fonts" => detect_font_theme(),
            _ => None,
        }
    }
}

#[derive(Debug)]
pub struct App {
    pub components: Vec<ThemeComponent>,
    pub selected: usize,
    pub theme_name: String,
    pub mode: Mode,
    pub message: String,
    pub permission_issues: Vec<PermissionIssue>,
    pub theme_directory: String,
    pub directory_entries: Vec<String>,
    pub directory_selected: usize,
}

#[derive(Debug, PartialEq)]
pub enum Mode {
    Selecting,
    Naming,
    DirectorySelection,
    Summary,
    PermissionCheck,
}

#[derive(Debug)]
pub struct PermissionIssue {
    pub component: String,
    pub path: String,
    pub issue_type: PermissionIssueType,
}

#[derive(Debug)]
pub enum PermissionIssueType {
    NoReadAccess,
    NoWriteAccess,
    SudoRequired,
}

impl App {
    pub fn new() -> Self {
        let components = vec![
            ThemeComponent::new(
                "GTK Themes",
                vec!["~/.themes/", "~/.local/share/themes/", "/usr/share/themes/"],
                "GTK2/GTK3 theme files",
            ),
            ThemeComponent::new(
                "Icons",
                vec!["~/.icons/", "~/.local/share/icons/", "/usr/share/icons/"],
                "Icon themes",
            ),
            ThemeComponent::new(
                "Cursors",
                vec!["~/.icons/", "~/.local/share/icons/", "/usr/share/icons/"],
                "Mouse cursor themes",
            ),
            ThemeComponent::new("Qt/KDE Styles", vec!["~/.config/"], "Qt5/Qt6 styles"),
            ThemeComponent::new(
                "Application Style",
                vec!["~/.config/", "/etc/xdg/"],
                "Current desktop application style (Oxygen, Edge Runner, etc.)",
            ),
            ThemeComponent::new(
                "Colors Schemes",
                vec!["~/.local/share/color-schemes/"],
                "KDE color schemes",
            ),
            ThemeComponent::new(
                "Window Decorations",
                vec![
                    "~/.config/kwinrc",
                    "~/.config/awesome/",
                    "~/.config/i3/",
                    "~/.config/openbox/",
                    "~/.config/bspwm/",
                    "/usr/share/kde4/config/",
                ],
                "Window manager decorations and borders",
            ),
            ThemeComponent::new(
                "Splash Screen",
                vec![
                    "/usr/share/plymouth/themes/",
                    "/boot/grub/themes/",
                    "/etc/alternatives/",
                    "~/.config/plymouth/",
                ],
                "Boot splash screen and login animations",
            ),
            ThemeComponent::new(
                "SDDM Theme",
                vec!["/usr/share/sddm/themes/"],
                "SDDM login manager theme",
            ),
            ThemeComponent::new(
                "Terminal Themes",
                vec!["~/.config/alacritty/", "~/.config/kitty/"],
                "Terminal themes",
            ),
        ];

        let default_theme_dir = if let Some(home) = home_dir() {
            home.join("CustomThemes").to_string_lossy().to_string()
        } else {
            "./CustomThemes".to_string()
        };

        Self {
            components,
            selected: 0,
            theme_name: String::new(),
            mode: Mode::Selecting,
            message: "Space to toggle, Enter to continue".to_string(),
            permission_issues: Vec::new(),
            theme_directory: default_theme_dir,
            directory_entries: Vec::new(),
            directory_selected: 0,
        }
    }

    pub fn toggle(&mut self) {
        if let Some(comp) = self.components.get_mut(self.selected) {
            comp.checked = !comp.checked;
        }
    }

    pub fn next(&mut self) {
        self.selected = (self.selected + 1) % self.components.len();
    }

    pub fn prev(&mut self) {
        self.selected = if self.selected == 0 {
            self.components.len() - 1
        } else {
            self.selected - 1
        };
    }

    pub fn checked_components(&self) -> Vec<&ThemeComponent> {
        self.components.iter().filter(|c| c.checked).collect()
    }
}

fn draw_ui(f: &mut Frame, app: &App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),
            Constraint::Min(0),
            Constraint::Length(3),
        ])
        .split(f.area());

    // Title
    let title = Paragraph::new("Theme Creator")
        .style(Style::default().add_modifier(Modifier::BOLD))
        .block(Block::default().borders(Borders::ALL));
    f.render_widget(title, chunks[0]);

    // Main content
    match app.mode {
        Mode::Selecting => draw_selection(f, app, chunks[1]),
        Mode::Naming => draw_naming(f, app, chunks[1]),
        Mode::DirectorySelection => draw_directory_selection(f, app, chunks[1]),
        Mode::Summary => draw_summary(f, app, chunks[1]),
        Mode::PermissionCheck => draw_permission_check(f, app, chunks[1]),
    }

    // Status
    let status_text = match app.mode {
        Mode::Selecting => app.message.clone(),
        Mode::Naming => format!("Name: {}_", app.theme_name),
        Mode::DirectorySelection => format!(
            "Path: {} | Enter: accept, Esc: cancel, Tab: create new",
            app.theme_directory
        ),
        Mode::Summary => "Enter to create, Esc to cancel".to_string(),
        Mode::PermissionCheck => {
            "1: Re-run with sudo, 2: Copy chmod commands, Esc: Cancel".to_string()
        }
    };

    let status = Paragraph::new(status_text)
        .style(Style::default().add_modifier(Modifier::REVERSED))
        .block(Block::default().borders(Borders::ALL));
    f.render_widget(status, chunks[2]);
}

fn draw_selection(f: &mut Frame, app: &App, area: Rect) {
    let items: Vec<ListItem> = app
        .components
        .iter()
        .enumerate()
        .map(|(i, comp)| {
            let checkbox = if comp.checked { "[x]" } else { "[ ]" };
            let style = if i == app.selected {
                Style::default().add_modifier(Modifier::REVERSED | Modifier::BOLD)
            } else {
                Style::default()
            };

            let mut content = vec![
                Line::from(vec![
                    Span::styled(format!(" {} ", checkbox), Style::default()),
                    Span::styled(&comp.name, style),
                ]),
                Line::from(vec![
                    Span::styled("     ", Style::default()),
                    Span::styled(&comp.description, Style::default().fg(Color::DarkGray)),
                ]),
            ];

            // Add current style info if available
            if let Some(ref current_style) = comp.current_style {
                content.push(Line::from(vec![
                    Span::styled("     ", Style::default()),
                    Span::styled("→ ", Style::default().fg(Color::Green)),
                    Span::styled(current_style, Style::default().fg(Color::Cyan)),
                ]));
            } else {
                content.push(Line::from(vec![
                    Span::styled("     ", Style::default()),
                    Span::styled("→ (none detected)", Style::default().fg(Color::DarkGray)),
                ]));
            }

            ListItem::new(content)
        })
        .collect();

    let mut state = ListState::default();
    state.select(Some(app.selected));

    let list = List::new(items)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title("Select Components"),
        )
        .highlight_style(Style::default().add_modifier(Modifier::REVERSED));

    f.render_stateful_widget(list, area, &mut state);
}

fn draw_naming(f: &mut Frame, app: &App, area: Rect) {
    let text = vec![
        Line::from("Enter theme name:"),
        Line::from(""),
        Line::from(vec![
            Span::styled("> ", Style::default().fg(Color::Green)),
            Span::styled(&app.theme_name, Style::default()),
            Span::styled("_", Style::default().fg(Color::Green)),
        ]),
    ];

    let paragraph =
        Paragraph::new(text).block(Block::default().borders(Borders::ALL).title("Name Theme"));
    f.render_widget(paragraph, area);
}

fn draw_directory_selection(f: &mut Frame, app: &App, area: Rect) {
    let mut lines = vec![
        Line::from("Choose where to save your theme:"),
        Line::from(""),
        Line::from(vec![
            Span::styled("Current: ", Style::default().fg(Color::Yellow)),
            Span::styled(&app.theme_directory, Style::default().fg(Color::Cyan)),
        ]),
        Line::from(""),
    ];

    if app.directory_entries.is_empty() {
        lines.push(Line::from("Loading directory contents..."));
    } else {
        lines.push(Line::from("Directories:"));

        for (i, entry) in app.directory_entries.iter().enumerate() {
            let style = if i == app.directory_selected {
                Style::default().add_modifier(Modifier::REVERSED | Modifier::BOLD)
            } else {
                Style::default()
            };

            let prefix = if entry.ends_with('/') {
                "📁 "
            } else {
                "📄 "
            };

            lines.push(Line::from(vec![
                Span::styled("  ", Style::default()),
                Span::styled(prefix, Style::default()),
                Span::styled(entry, style),
            ]));
        }

        lines.push(Line::from(""));
        lines.push(Line::from(
            "↑↓: Navigate | Enter: Select | Tab: Create new directory",
        ));
    }

    let paragraph = Paragraph::new(lines)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title("Select Directory"),
        )
        .wrap(Wrap { trim: true });
    f.render_widget(paragraph, area);
}

fn draw_summary(f: &mut Frame, app: &App, area: Rect) {
    let checked = app.checked_components();

    let mut lines = vec![
        Line::from(vec![
            Span::styled("Theme: ", Style::default().bold()),
            Span::styled(&app.theme_name, Style::default().fg(Color::Cyan)),
        ]),
        Line::from(""),
    ];

    if checked.is_empty() {
        lines.push(Line::from("No components selected!"));
    } else {
        lines.push(Line::from("Components to include:"));
        for comp in checked {
            lines.push(Line::from(vec![
                Span::styled("✓ ", Style::default().fg(Color::Green)),
                Span::styled(&comp.name, Style::default().bold()),
            ]));
            lines.push(Line::from(vec![
                Span::styled("  ", Style::default()),
                Span::styled(&comp.description, Style::default().fg(Color::DarkGray)),
            ]));
        }
    }

    let paragraph = Paragraph::new(lines)
        .block(Block::default().borders(Borders::ALL).title("Summary"))
        .wrap(Wrap { trim: true });
    f.render_widget(paragraph, area);
}

fn draw_permission_check(f: &mut Frame, app: &App, area: Rect) {
    let mut lines = vec![
        Line::from(vec![Span::styled(
            "Permission Issues Found",
            Style::default().fg(Color::Red).bold(),
        )]),
        Line::from(""),
    ];

    if app.permission_issues.is_empty() {
        lines.push(Line::from("No permission issues detected!"));
    } else {
        lines.push(Line::from(
            "The following components have permission issues:",
        ));
        lines.push(Line::from(""));

        for (i, issue) in app.permission_issues.iter().enumerate() {
            let issue_text = match issue.issue_type {
                PermissionIssueType::NoReadAccess => "No read access",
                PermissionIssueType::NoWriteAccess => "No write access",
                PermissionIssueType::SudoRequired => "Sudo required",
            };

            lines.push(Line::from(vec![
                Span::styled(format!("{}.", i + 1), Style::default().fg(Color::Yellow)),
                Span::styled(" ", Style::default()),
                Span::styled(&issue.component, Style::default().bold()),
                Span::styled(
                    format!(" ({})", issue_text),
                    Style::default().fg(Color::Red),
                ),
            ]));
            lines.push(Line::from(vec![
                Span::styled("   Path: ", Style::default()),
                Span::styled(&issue.path, Style::default().fg(Color::Blue)),
            ]));
            lines.push(Line::from(""));
        }

        lines.push(Line::from(vec![Span::styled(
            "Options:",
            Style::default().bold(),
        )]));
        lines.push(Line::from("1. Re-run with sudo privileges"));
        lines.push(Line::from("2. Copy chmod commands to clipboard"));
        lines.push(Line::from("Esc. Cancel and go back"));
    }

    let paragraph = Paragraph::new(lines)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title("Permission Check"),
        )
        .wrap(Wrap { trim: true });
    f.render_widget(paragraph, area);
}

fn main() -> Result<()> {
    let mut app = App::new();

    // Initialize terminal with error handling
    let result = (|| -> Result<()> {
        enable_raw_mode()?;
        let mut stdout = io::stdout();
        execute!(stdout, EnterAlternateScreen)?;
        let backend = CrosstermBackend::new(stdout);
        let mut terminal = Terminal::new(backend)?;

        // Run the main app loop
        run_app_loop(&mut terminal, &mut app)?;

        // Cleanup
        disable_raw_mode()?;
        execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
        terminal.show_cursor()?;
        Ok(())
    })();

    if let Err(e) = result {
        eprintln!(
            "Terminal error: {}. Make sure you're running this in a proper terminal.",
            e
        );
        return Err(e);
    }

    Ok(())
}

fn run_app_loop(
    terminal: &mut Terminal<CrosstermBackend<io::Stdout>>,
    app: &mut App,
) -> Result<()> {
    loop {
        terminal.draw(|f| draw_ui(f, &app))?;

        if event::poll(std::time::Duration::from_millis(100))
            .context("Failed to poll for events")?
        {
            if let Event::Key(key) = event::read()? {
                if key.kind == KeyEventKind::Press {
                    match app.mode {
                        Mode::Selecting => match key.code {
                            KeyCode::Char('q') | KeyCode::Esc => break,
                            KeyCode::Up | KeyCode::Left => app.prev(),
                            KeyCode::Down | KeyCode::Right => app.next(),
                            KeyCode::Char(' ') => app.toggle(),
                            KeyCode::Enter => {
                                if app.checked_components().is_empty() {
                                    app.message = "Select at least one component".to_string();
                                } else {
                                    app.mode = Mode::Naming;
                                }
                            }
                            _ => {}
                        },
                        Mode::Naming => {
                            match key.code {
                                KeyCode::Esc => app.mode = Mode::Selecting,
                                KeyCode::Enter => {
                                    if app.theme_name.trim().is_empty() {
                                        // Stay in naming mode
                                    } else {
                                        update_directory_entries(app);
                                        app.mode = Mode::DirectorySelection;
                                    }
                                }
                                KeyCode::Backspace => {
                                    app.theme_name.pop();
                                }
                                KeyCode::Char(c) => app.theme_name.push(c),
                                _ => {}
                            }
                        }
                        Mode::DirectorySelection => {
                            match key.code {
                                KeyCode::Esc => app.mode = Mode::Naming,
                                KeyCode::Enter => {
                                    let selected_entry = if !app.directory_entries.is_empty()
                                        && app.directory_selected < app.directory_entries.len()
                                    {
                                        app.directory_entries.get(app.directory_selected).cloned()
                                    } else {
                                        None
                                    };

                                    if let Some(entry) = selected_entry {
                                        if entry.ends_with('/') {
                                            // Navigate into subdirectory
                                            let new_path =
                                                std::path::Path::new(&app.theme_directory)
                                                    .join(entry.trim_end_matches('/'));
                                            app.theme_directory =
                                                new_path.to_string_lossy().to_string();
                                            app.directory_selected = 0;
                                            update_directory_entries(app);
                                        }
                                    } else {
                                        // Accept current directory
                                        app.mode = Mode::Summary;
                                    }
                                }
                                KeyCode::Up => {
                                    if !app.directory_entries.is_empty() {
                                        app.directory_selected = if app.directory_selected == 0 {
                                            app.directory_entries.len() - 1
                                        } else {
                                            app.directory_selected - 1
                                        };
                                    }
                                }
                                KeyCode::Down => {
                                    if !app.directory_entries.is_empty() {
                                        app.directory_selected = (app.directory_selected + 1)
                                            % app.directory_entries.len();
                                    }
                                }
                                KeyCode::Tab => {
                                    // Create new directory functionality would go here
                                    // For now, just accept current directory
                                    app.mode = Mode::Summary;
                                }
                                _ => {}
                            }
                        }
                        Mode::Summary => match key.code {
                            KeyCode::Esc => app.mode = Mode::Selecting,
                            KeyCode::Enter => {
                                app.permission_issues = check_permissions(&app);
                                if app.permission_issues.is_empty() {
                                    create_theme(&app)?;
                                    break;
                                } else {
                                    app.mode = Mode::PermissionCheck;
                                }
                            }
                            _ => {}
                        },
                        Mode::PermissionCheck => {
                            match key.code {
                                KeyCode::Esc => app.mode = Mode::Summary,
                                KeyCode::Char('1') => {
                                    // Re-run with sudo
                                    let current_exe =
                                        env::current_exe().context("Failed to get current exe")?;
                                    let args: Vec<String> = env::args().collect();
                                    let status = Command::new("sudo")
                                        .arg(current_exe)
                                        .args(&args[1..])
                                        .status()?;

                                    if status.success() {
                                        break;
                                    } else {
                                        app.message = "Sudo execution failed".to_string();
                                        app.mode = Mode::Selecting;
                                    }
                                }
                                KeyCode::Char('2') => {
                                    // Generate chmod commands
                                    let chmod_commands =
                                        generate_chmod_commands(&app.permission_issues);

                                    // Try to copy to clipboard
                                    if let Ok(()) = copy_to_clipboard(&chmod_commands) {
                                        app.message =
                                            "Chmod commands copied to clipboard!".to_string();
                                    } else {
                                        println!(
                                            "\n=== Chmod Commands ===\n{}\n=== End Commands ===\n",
                                            chmod_commands
                                        );
                                        app.message =
                                            "Chmod commands printed to terminal".to_string();
                                    }
                                    app.mode = Mode::Selecting;
                                }
                                _ => {}
                            }
                        }
                    }
                }
            }
        }
    }

    Ok(())
}

fn update_directory_entries(app: &mut App) {
    app.directory_entries.clear();
    app.directory_selected = 0;

    let path = std::path::Path::new(&app.theme_directory);
    if let Ok(entries) = fs::read_dir(path) {
        for entry in entries.flatten() {
            if let Ok(file_type) = entry.file_type() {
                let name = entry.file_name().to_string_lossy().to_string();
                if file_type.is_dir() && !name.starts_with('.') {
                    app.directory_entries.push(name + "/");
                }
            }
        }
        app.directory_entries.sort();
    }
}

fn create_theme(app: &App) -> Result<()> {
    let theme_dir = std::path::Path::new(&app.theme_directory).join(&app.theme_name);

    // Ensure we have absolute path for display
    let display_theme_dir = if theme_dir.is_absolute() {
        theme_dir.clone()
    } else {
        std::env::current_dir()
            .context("Failed to get current directory")?
            .join(&theme_dir)
    };

    fs::create_dir_all(&display_theme_dir)?;

    let mut copied_files = Vec::new();
    let mut skipped_files = Vec::new();

    // Show user what we're doing
    println!("\n🔍 Scanning for theme files...\n");

    for comp in app.checked_components() {
        let component_dir = display_theme_dir.join(comp.name.replace(&[' ', '/'][..], "_"));
        fs::create_dir_all(&component_dir)?;

        println!("📁 Processing: {}", comp.name);

        for path_str in &comp.source_paths {
            let path = expand_tilde(path_str);
            println!("   Checking: {} -> {}", path_str, path.display());

            if path.exists() {
                if let Err(e) = copy_recursive(&path, &component_dir) {
                    println!("   ❌ Failed to copy: {}", e);
                    skipped_files.push(format!("{}: {} ({})", comp.name, path.display(), e));
                } else {
                    copied_files.push(format!("{}: {}", comp.name, path.display()));
                    println!("   ✓ Successfully copied");
                }
            } else {
                println!("   ⚠ Path not found");
                skipped_files.push(format!("{}: {} (not found)", comp.name, path.display()));
            }
        }
        println!();
    }

    // Create theme metadata
    let metadata_file = display_theme_dir.join("theme_info.txt");
    let metadata_content = format!(
        "Theme Name: {}\nCreated: {}\nSaved at: {}\nComponents:\n{}\n\nSuccessfully copied files:\n{}\n\nSkipped files:\n{}\n\nRuntime info:\n- USER: {}\n- HOME: {}\n- SUDO_USER: {}\n",
        app.theme_name,
        chrono::Utc::now().format("%Y-%m-%d %H:%M:%S UTC"),
        display_theme_dir.display(),
        app.checked_components()
            .iter()
            .map(|c| format!("- {}: {}", c.name, c.description))
            .collect::<Vec<_>>()
            .join("\n"),
        if copied_files.is_empty() {
            "No files were copied".to_string()
        } else {
            copied_files.iter().map(|f| format!("- {}", f)).collect::<Vec<_>>().join("\n")
        },
        if skipped_files.is_empty() {
            "No files were skipped".to_string()
        } else {
            skipped_files.iter().map(|f| format!("- {}", f)).collect::<Vec<_>>().join("\n")
        },
        std::env::var("USER").unwrap_or_else(|_| "unknown".to_string()),
        std::env::var("HOME").unwrap_or_else(|_| "unknown".to_string()),
        std::env::var("SUDO_USER").unwrap_or_else(|_| "not set".to_string()),
    );
    fs::write(metadata_file, metadata_content)?;

    // Clear screen and show success message
    println!("\n{}\n", "=".repeat(60));
    println!("🎉 THEME CREATION COMPLETE! 🎉");
    println!("{}", "=".repeat(60));
    println!("Theme Name: {}", app.theme_name);
    println!("Saved at: {}", display_theme_dir.display());
    println!("Components included: {}", app.checked_components().len());
    println!("Files successfully copied: {}", copied_files.len());
    if !skipped_files.is_empty() {
        println!("Files skipped/not found: {}", skipped_files.len());
    }
    println!("{}", "=".repeat(60));
    println!(
        "You can find your theme at: {}",
        display_theme_dir.display()
    );
    println!("A theme_info.txt file has been created with complete details.");
    if copied_files.is_empty() {
        println!("\n⚠️  Warning: No files were copied. Check the paths and permissions.");
        println!("The app might be looking for files in the wrong home directory.");
    }
    println!("{}", "=".repeat(60));

    Ok(())
}

fn check_permissions(app: &App) -> Vec<PermissionIssue> {
    let mut issues = Vec::new();

    for component in app.checked_components() {
        for path_str in &component.source_paths {
            let path = expand_tilde(path_str);

            if path.exists() {
                // Check read permissions
                if !path.readable() {
                    issues.push(PermissionIssue {
                        component: component.name.clone(),
                        path: path.display().to_string(),
                        issue_type: PermissionIssueType::NoReadAccess,
                    });
                }

                // Check if we need sudo for system directories
                if path.starts_with("/usr") || path.starts_with("/etc") {
                    // Try to create a temp file to test write access
                    let test_file = path.join(".theme_creator_test");
                    if fs::write(&test_file, "test").is_err() {
                        let _ = fs::remove_file(&test_file); // Clean up if it was created
                        issues.push(PermissionIssue {
                            component: component.name.clone(),
                            path: path.display().to_string(),
                            issue_type: PermissionIssueType::SudoRequired,
                        });
                    } else {
                        let _ = fs::remove_file(&test_file); // Clean up test file
                    }
                }
            }
        }
    }

    issues
}

fn generate_chmod_commands(issues: &[PermissionIssue]) -> String {
    let mut commands = Vec::new();
    let mut processed_paths = std::collections::HashSet::new();

    for issue in issues {
        let path = &issue.path;
        if !processed_paths.contains(path) {
            if path.starts_with("/usr") || path.starts_with("/etc") {
                commands.push(format!("sudo chmod -R 755 \"{}\"", path));
            } else {
                commands.push(format!("chmod -R 755 \"{}\"", path));
            }
            processed_paths.insert(path.clone());
        }
    }

    if commands.is_empty() {
        "No chmod commands needed".to_string()
    } else {
        commands.join("\n")
    }
}

fn copy_to_clipboard(text: &str) -> Result<()> {
    // Try xclip first (most common)
    if Command::new("xclip")
        .arg("-selection")
        .arg("clipboard")
        .stdin(std::process::Stdio::piped())
        .spawn()
        .and_then(|mut child| {
            if let Some(stdin) = child.stdin.as_mut() {
                use std::io::Write;
                stdin.write_all(text.as_bytes())?;
            }
            child.wait()
        })
        .is_ok()
    {
        return Ok(());
    }

    // Try wl-copy (Wayland)
    if Command::new("wl-copy")
        .stdin(std::process::Stdio::piped())
        .spawn()
        .and_then(|mut child| {
            if let Some(stdin) = child.stdin.as_mut() {
                use std::io::Write;
                stdin.write_all(text.as_bytes())?;
            }
            child.wait()
        })
        .is_ok()
    {
        return Ok(());
    }

    // Try xsel (alternative)
    if Command::new("xsel")
        .arg("--clipboard")
        .arg("--input")
        .stdin(std::process::Stdio::piped())
        .spawn()
        .and_then(|mut child| {
            if let Some(stdin) = child.stdin.as_mut() {
                use std::io::Write;
                stdin.write_all(text.as_bytes())?;
            }
            child.wait()
        })
        .is_ok()
    {
        return Ok(());
    }

    Err(anyhow::anyhow!("No clipboard utility found"))
}

fn copy_recursive(source: &std::path::Path, destination: &std::path::Path) -> Result<()> {
    if source.is_file() {
        let file_name = source.file_name().context("Invalid filename")?;
        let dest_path = destination.join(file_name);
        fs::copy(source, dest_path)?;
    } else if source.is_dir() {
        let dir_name = source.file_name().context("Invalid directory name")?;
        let dest_path = destination.join(dir_name);
        fs_extra::dir::copy(
            source,
            &dest_path,
            &fs_extra::dir::CopyOptions::new()
                .copy_inside(true)
                .content_only(true)
                .overwrite(true),
        )?;
    }
    Ok(())
}

// Style detection functions
fn detect_gtk_theme() -> Option<String> {
    // Check GTK3 settings
    if let Ok(content) = fs::read_to_string(home_dir()?.join(".config/gtk-3.0/settings.ini")) {
        for line in content.lines() {
            if line.trim().starts_with("gtk-theme-name=") {
                let theme = line.split('=').nth(1)?.trim().trim_matches('"');
                return Some(format!("GTK3: {}", theme));
            }
        }
    }

    // Check dconf settings (requires dconf command)
    if let Ok(output) = Command::new("gsettings")
        .args(&["get", "org.gnome.desktop.interface", "gtk-theme"])
        .output()
    {
        if output.status.success() {
            let theme = String::from_utf8_lossy(&output.stdout);
            let theme = theme.trim().trim_matches('\'');
            return Some(format!("GTK: {}", theme));
        }
    }

    None
}

fn detect_icon_theme() -> Option<String> {
    // Check GTK3 settings for icons
    if let Ok(content) = fs::read_to_string(home_dir()?.join(".config/gtk-3.0/settings.ini")) {
        for line in content.lines() {
            if line.trim().starts_with("gtk-icon-theme-name=") {
                let theme = line.split('=').nth(1)?.trim().trim_matches('"');
                return Some(format!("Icons: {}", theme));
            }
        }
    }

    // Check gsettings
    if let Ok(output) = Command::new("gsettings")
        .args(["get", "org.gnome.desktop.interface", "icon-theme"])
        .output()
    {
        if output.status.success() {
            let theme = String::from_utf8_lossy(&output.stdout);
            let theme = theme.trim().trim_matches('\'');
            return Some(format!("Icons: {}", theme));
        }
    }

    None
}

fn detect_cursor_theme() -> Option<String> {
    // Check GTK3 settings for cursor theme
    if let Ok(content) = fs::read_to_string(home_dir()?.join(".config/gtk-3.0/settings.ini")) {
        for line in content.lines() {
            if line.trim().starts_with("gtk-cursor-theme-name=") {
                let theme = line.split('=').nth(1)?.trim().trim_matches('"');
                return Some(format!("Cursor: {}", theme));
            }
        }
    }

    // Check gsettings
    if let Ok(output) = Command::new("gsettings")
        .args(["get", "org.gnome.desktop.interface", "cursor-theme"])
        .output()
    {
        if output.status.success() {
            let theme = String::from_utf8_lossy(&output.stdout);
            let theme = theme.trim().trim_matches('\'');
            return Some(format!("Cursor: {}", theme));
        }
    }

    // Check icon theme directories for cursor themes
    let icon_paths = [
        home_dir()?.join(".icons"),
        home_dir()?.join(".local/share/icons"),
        std::path::PathBuf::from("/usr/share/icons"),
    ];

    for path in &icon_paths {
        if path.exists() {
            if let Ok(entries) = fs::read_dir(path) {
                for entry in entries.flatten() {
                    if let Ok(file_type) = entry.file_type() {
                        if file_type.is_dir() {
                            let dir_name_owned = entry.file_name().to_string_lossy().to_string();
                            let dir_name = dir_name_owned.as_str();
                            if dir_name.to_lowercase().contains("cursor") {
                                return Some(format!("Cursor: {}", dir_name));
                            }
                        }
                    }
                }
            }
        }
    }

    None
}

fn detect_qt_style() -> Option<String> {
    // Check qt5ct
    if let Ok(content) = fs::read_to_string(home_dir()?.join(".config/qt5ct/qt5ct.conf")) {
        for line in content.lines() {
            if line.trim().starts_with("style=") {
                let style = line.split('=').nth(1)?.trim();
                return Some(format!("Qt5: {}", style));
            }
        }
    }

    // Check qt6ct
    if let Ok(content) = fs::read_to_string(home_dir()?.join(".config/qt6ct/qt6ct.conf")) {
        for line in content.lines() {
            if line.trim().starts_with("style=") {
                let style = line.split('=').nth(1)?.trim();
                return Some(format!("Qt6: {}", style));
            }
        }
    }

    None
}

fn detect_color_scheme() -> Option<String> {
    // Check KDE color schemes
    if let Ok(content) = fs::read_to_string(home_dir()?.join(".config/kdeglobals")) {
        for line in content.lines() {
            if line.trim().starts_with("ColorScheme=") {
                let scheme = line.split('=').nth(1)?.trim();
                return Some(format!("KDE: {}", scheme));
            }
        }
    }

    // Check Plasma colors
    if let Ok(output) = Command::new("kreadconfig5")
        .args(&["--group", "Colors:Window", "--key", "BackgroundNormal"])
        .output()
    {
        if output.status.success() {
            let color_str = String::from_utf8_lossy(&output.stdout);
            let color = color_str.trim();
            return Some(format!("Plasma: {}", color));
        }
    }

    None
}

fn detect_window_decorations() -> Option<String> {
    // Check KDE KWin window decorations
    if let Ok(output) = Command::new("kreadconfig5")
        .args(&["--group", "org.kde.kdecoration2", "--key", "library"])
        .output()
    {
        if output.status.success() {
            let deco_str = String::from_utf8_lossy(&output.stdout);
            let decoration = deco_str.trim();
            if !decoration.is_empty() && decoration != "org.kde.kwin.aurorae" {
                return Some(format!("KWin: {}", decoration));
            }
        }
    }

    // Check KWin config directly
    if let Ok(content) = fs::read_to_string(home_dir()?.join(".config/kwinrc")) {
        for line in content.lines() {
            if line.trim().starts_with("plugin=") {
                let plugin = line.split('=').nth(1)?.trim();
                return Some(format!("KWin Plugin: {}", plugin));
            }
        }
    }

    // Check for AwesomeWM decorations
    if let Ok(content) = fs::read_to_string(home_dir()?.join(".config/awesome/rc.lua")) {
        for line in content.lines() {
            if line.trim().contains("beautiful.init") {
                return Some("AwesomeWM: Beautiful".into());
            }
        }
    }

    // Check for Openbox theme
    if let Ok(content) = fs::read_to_string(home_dir()?.join(".config/openbox/rc.xml")) {
        for line in content.lines() {
            if line.trim().contains("<theme>") {
                if let Some(start) = line.find("<name>") {
                    if let Some(end) = line.find("</name>") {
                        let theme = &line[start + 6..end];
                        return Some(format!("Openbox: {}", theme.trim()));
                    }
                }
            }
        }
    }

    None
}

fn detect_splash_screen() -> Option<String> {
    // Check Plymouth (boot splash)
    if let Ok(output) = Command::new("plymouth-set-default-theme")
        .arg("--show-current")
        .output()
    {
        if output.status.success() {
            let theme_str = String::from_utf8_lossy(&output.stdout);
            let theme = theme_str.trim();
            if !theme.is_empty() {
                return Some(format!("Plymouth: {}", theme));
            }
        }
    }

    // Check Plymouth config
    if let Ok(content) = fs::read_to_string("/etc/plymouth/plymouthd.conf") {
        for line in content.lines() {
            if line.trim().starts_with("Theme=") {
                let theme = line.split('=').nth(1)?.trim();
                return Some(format!("Plymouth: {}", theme));
            }
        }
    }

    // Check GRUB themes
    if let Ok(content) = fs::read_to_string("/etc/default/grub") {
        for line in content.lines() {
            if line.trim().starts_with("GRUB_THEME=") {
                let theme = line.split('=').nth(1)?.trim().trim_matches('"');
                return Some(format!("GRUB: {}", theme));
            }
        }
    }

    // Check for available splash themes
    if std::path::Path::new("/usr/share/plymouth/themes").exists() {
        if let Ok(entries) = fs::read_dir("/usr/share/plymouth/themes") {
            for entry in entries.flatten() {
                if let Ok(file_type) = entry.file_type() {
                    if file_type.is_dir() {
                        return Some("Plymouth: Available".into());
                    }
                }
            }
        }
    }

    None
}

fn detect_sddm_theme() -> Option<String> {
    // Check current SDDM theme
    if let Ok(content) = fs::read_to_string("/etc/sddm.conf") {
        for line in content.lines() {
            if line.trim().starts_with("Current=") {
                let theme = line.split('=').nth(1)?.trim();
                return Some(format!("SDDM: {}", theme));
            }
        }
    }

    // Check in sddm.conf.d
    if let Ok(entries) = fs::read_dir("/etc/sddm.conf.d") {
        for entry in entries.flatten() {
            if let Ok(content) = fs::read_to_string(entry.path()) {
                for line in content.lines() {
                    if line.trim().starts_with("Current=") {
                        let theme = line.split('=').nth(1)?.trim();
                        return Some(format!("SDDM: {}", theme));
                    }
                }
            }
        }
    }

    None
}

fn detect_terminal_theme() -> Option<String> {
    // Check alacritty
    if let Ok(content) = fs::read_to_string(home_dir()?.join(".config/alacritty/alacritty.yml")) {
        for line in content.lines() {
            if line.trim().starts_with("colors:") || line.trim().contains("primary:") {
                return Some("Alacritty: Custom theme".into());
            }
        }
    }

    // Check kitty
    if let Ok(content) = fs::read_to_string(home_dir()?.join(".config/kitty/kitty.conf")) {
        for line in content.lines() {
            if line.trim().starts_with("include") && line.contains("theme") {
                let theme = line.split_whitespace().nth(1)?;
                return Some(format!("Kitty: {}", theme));
            }
        }
    }

    // Check gnome-terminal
    if let Ok(output) = Command::new("gsettings")
        .args(&[
            "get",
            "org.gnome.Terminal.Profiles:/org/gnome/terminal/legacy/profiles:/",
            "default-profile",
        ])
        .output()
    {
        if output.status.success() {
            return Some("GNOME Terminal: Configured".into());
        }
    }

    None
}

fn detect_wm_theme() -> Option<String> {
    // Check current window manager
    if let Ok(desktop) = std::env::var("XDG_CURRENT_DESKTOP") {
        if !desktop.is_empty() {
            return Some(format!("WM: {}", desktop));
        }
    }

    // Check for specific window managers
    if std::env::var("I3SOCK").is_ok() {
        return Some("WM: i3".into());
    }

    if std::env::var("BSPWM_SOCKET").is_ok() {
        return Some("WM: bspwm".into());
    }

    // Check processes
    if let Ok(output) = Command::new("ps")
        .args(&["-u", std::env::var("USER").unwrap_or_default().as_str()])
        .output()
    {
        let output_str = String::from_utf8_lossy(&output.stdout);
        if output_str.contains("openbox") {
            return Some("WM: Openbox".into());
        }
        if output_str.contains("xfwm4") {
            return Some("WM: Xfwm4".into());
        }
        if output_str.contains("kwin") {
            return Some("WM: KWin".into());
        }
    }

    None
}

fn detect_shell_theme() -> Option<String> {
    // Check current shell
    if let Ok(shell) = std::env::var("SHELL") {
        if shell.contains("zsh") {
            if let Ok(content) = fs::read_to_string(home_dir()?.join(".zshrc")) {
                if content.contains("ZSH_THEME=") {
                    return Some("Shell: Zsh (Oh My Zsh)".into());
                }
                return Some("Shell: Zsh".into());
            }
        } else if shell.contains("bash") {
            return Some("Shell: Bash".into());
        } else if shell.contains("fish") {
            return Some("Shell: Fish".into());
        }
    }

    None
}

fn detect_application_style() -> Option<String> {
    // First check if KDE style is set (Oxygen, Breeze, etc.)
    if let Ok(output) = Command::new("kreadconfig5")
        .args(&["--group", "KDE", "--key", "style"])
        .output()
    {
        if output.status.success() {
            let style_str = String::from_utf8_lossy(&output.stdout);
            let style = style_str.trim();
            if !style.is_empty() && style != "default" {
                return Some(format!("KDE Style: {}", style));
            }
        }
    }

    // Check for KDE global theme (which includes application style)
    if let Ok(output) = Command::new("kreadconfig5")
        .args(&["--group", "General", "--key", "ColorSchemeKey"])
        .output()
    {
        if output.status.success() {
            let color_scheme_str = String::from_utf8_lossy(&output.stdout);
            let color_scheme = color_scheme_str.trim();
            if !color_scheme.is_empty() {
                return Some(format!("KDE Theme: {}", color_scheme));
            }
        }
    }

    // Check GTK theme as fallback (since it controls application styling)
    if let Ok(output) = Command::new("gsettings")
        .args(&["get", "org.gnome.desktop.interface", "gtk-theme"])
        .output()
    {
        if output.status.success() {
            let theme = String::from_utf8_lossy(&output.stdout);
            let theme = theme.trim().trim_matches('\'');
            if !theme.is_empty() && theme != "Adwaita" {
                return Some(format!("GTK Style: {}", theme));
            }
        }
    }

    // Fallback: detect what toolkits are available
    let mut toolkits = Vec::new();

    if home_dir()?.join(".config/gtk-3.0/settings.ini").exists() {
        toolkits.push("GTK3");
    }

    if home_dir()?.join(".config/qt5ct/qt5ct.conf").exists() {
        toolkits.push("Qt5");
    }

    if home_dir()?.join(".config/qt6ct/qt6ct.conf").exists() {
        toolkits.push("Qt6");
    }

    if !toolkits.is_empty() {
        return Some(format!("Available: {}", toolkits.join(", ")));
    }

    Some("Default".to_string())
}

fn detect_font_theme() -> Option<String> {
    // Check font configuration
    if let Ok(output) = Command::new("gsettings")
        .args(&["get", "org.gnome.desktop.interface", "font-name"])
        .output()
    {
        if output.status.success() {
            let font = String::from_utf8_lossy(&output.stdout);
            let font = font.trim().trim_matches('\'');
            return Some(format!("Font: {}", font));
        }
    }

    // Check .fonts.conf
    if let Ok(content) = fs::read_to_string(home_dir()?.join(".config/fontconfig/fonts.conf")) {
        for line in content.lines() {
            if line.trim().contains("<family>") {
                if let Some(start) = line.find("<family>") {
                    if let Some(end) = line.find("</family>") {
                        let font = &line[start + 8..end];
                        return Some(format!("Font: {}", font.trim()));
                    }
                }
            }
        }
    }

    None
}

trait PathExt {
    fn readable(&self) -> bool;
}

impl PathExt for Path {
    fn readable(&self) -> bool {
        match std::fs::metadata(self) {
            Ok(metadata) => {
                // Simple check - if we can read metadata, we likely have read access
                // For a more thorough check, we'd need to check file permissions
                !metadata.permissions().readonly()
            }
            Err(_) => false,
        }
    }
}

fn expand_tilde(path: &str) -> std::path::PathBuf {
    if path.starts_with("~/") {
        // Get the real user's home directory
        let home = get_user_home_dir();
        return home.join(&path[2..]);
    } else if path == "~" {
        let home = get_user_home_dir();
        return home;
    }

    // Handle relative paths by making them absolute to current directory
    let path_buf = std::path::PathBuf::from(path);
    if path_buf.is_relative() {
        if let Ok(current_dir) = std::env::current_dir() {
            return current_dir.join(path_buf);
        }
    }

    path_buf
}

fn get_user_home_dir() -> std::path::PathBuf {
    // CRITICAL: Always prioritize SUDO_USER to get original user when running with sudo
    if let Ok(sudo_user) = std::env::var("SUDO_USER") {
        let home = std::path::PathBuf::from("/home").join(&sudo_user);
        if home.exists() {
            return home;
        }
    }

    // If not sudo, try normal environment
    if let Ok(home) = std::env::var("HOME") {
        let home_path = std::path::PathBuf::from(&home);
        // Don't use root's home directory
        if !home_path.ends_with("/root") && home_path.exists() {
            return home_path;
        }
    }

    // Try to get the current user and construct their home directory
    if let Ok(username) = std::env::var("USER") {
        if username != "root" {
            let home = std::path::PathBuf::from("/home").join(&username);
            if home.exists() {
                return home;
            }
        }
    }

    // Last resort: find first non-root user directory in /home
    if let Ok(entries) = std::fs::read_dir("/home") {
        for entry in entries.flatten() {
            if let Ok(file_type) = entry.file_type() {
                if file_type.is_dir() {
                    let path = entry.path();
                    if let Some(name) = path.file_name() {
                        if name != "root" {
                            return path;
                        }
                    }
                }
            }
        }
    }

    // Ultimate fallback: current directory
    std::env::current_dir().unwrap_or_else(|_| std::path::PathBuf::from("."))
}
