//! Dashboard - main TUI layout with panes

use anyhow::Result;
use crossterm::{
    event::{self, Event, KeyCode, KeyEvent},
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    ExecutableCommand,
};
use ratatui::{
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout},
    style::{Color, Style},
    widgets::{Block, Borders, Paragraph},
    Terminal,
};
use std::io::stdout;

use crate::utils::config::Config;
use crate::graphics::GraphicsBackend;
use crate::shell::PowerShellIntegration;
use crate::tui::theme::Theme;
use crate::tui::terminal_guard::TerminalGuard;

pub struct Dashboard {
    config: Config,
    theme: Theme,
    graphics: Box<dyn GraphicsBackend>,
    shell: PowerShellIntegration,
    should_quit: bool,
}

impl Dashboard {
    pub fn new(
        config: Config,
        graphics: Box<dyn GraphicsBackend>,
        shell: PowerShellIntegration,
    ) -> Result<Self> {
        let theme = Theme::from_config(&config.theme);
        
        Ok(Dashboard {
            config,
            theme,
            graphics,
            shell,
            should_quit: false,
        })
    }

    pub async fn run(&mut self) -> Result<()> {
        // Setup terminal with guard - ensures cleanup on early return or panic
        let _guard = TerminalGuard::new()?;
        
        let backend = CrosstermBackend::new(stdout());
        let mut terminal = Terminal::new(backend)?;

        terminal.clear()?;

        // Main event loop
        while !self.should_quit {
            // Draw UI
            terminal.draw(|frame| {
                let size = frame.area();
                
                // Create layout based on config
                let chunks = Layout::default()
                    .direction(Direction::Vertical)
                    .constraints([
                        Constraint::Percentage(60),
                        Constraint::Percentage(40),
                    ])
                    .split(size);

                let top_chunks = Layout::default()
                    .direction(Direction::Horizontal)
                    .constraints([
                        Constraint::Percentage(60),
                        Constraint::Percentage(40),
                    ])
                    .split(chunks[0]);

                let bottom_chunks = Layout::default()
                    .direction(Direction::Horizontal)
                    .constraints([
                        Constraint::Percentage(50),
                        Constraint::Percentage(50),
                    ])
                    .split(chunks[1]);

                // Shell pane
                let shell_block = Block::default()
                    .title("Shell")
                    .borders(Borders::ALL)
                    .style(Style::default().fg(self.theme.foreground));
                let shell_content = Paragraph::new("PowerShell console will appear here...")
                    .block(shell_block);
                frame.render_widget(shell_content, top_chunks[0]);

                // Agent pane
                let agent_block = Block::default()
                    .title("Agent Console")
                    .borders(Borders::ALL)
                    .style(Style::default().fg(self.theme.foreground));
                let agent_content = Paragraph::new("AI agent outputs will stream here...")
                    .block(agent_block);
                frame.render_widget(agent_content, top_chunks[1]);

                // Preview pane
                let preview_block = Block::default()
                    .title("Preview")
                    .borders(Borders::ALL)
                    .style(Style::default().fg(self.theme.foreground));
                let preview_content = Paragraph::new("Media and file previews...")
                    .block(preview_block);
                frame.render_widget(preview_content, bottom_chunks[0]);

                // Log pane
                let log_block = Block::default()
                    .title("Log")
                    .borders(Borders::ALL)
                    .style(Style::default().fg(self.theme.foreground));
                let log_content = Paragraph::new("System logs and errors...")
                    .block(log_block);
                frame.render_widget(log_content, bottom_chunks[1]);
            })?;

            // Handle input
            if event::poll(std::time::Duration::from_millis(100))? {
                if let Event::Key(key) = event::read()? {
                    self.handle_key(key).await?;
                }
            }
        }

        // Terminal cleanup handled automatically by TerminalGuard drop
        Ok(())
    }

    async fn handle_key(&mut self, key: KeyEvent) -> Result<()> {
        match key.code {
            KeyCode::Char('q') | KeyCode::Esc => {
                self.should_quit = true;
            }
            KeyCode::Char('c') if key.modifiers.contains(event::KeyModifiers::CONTROL) => {
                self.should_quit = true;
            }
            _ => {}
        }
        Ok(())
    }
}
