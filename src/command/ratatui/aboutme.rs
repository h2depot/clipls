use anyhow::Result;
use crossterm::{
    ExecutableCommand,
    cursor::{MoveTo, Show},
    event::{self, Event, KeyCode, KeyEventKind, KeyModifiers},
    style::Print,
    terminal::disable_raw_mode,
};
use ratatui::{
    TerminalOptions, Viewport,
    layout::{Alignment, Constraint, Layout},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph},
};

const GITHUB_URL: &str = "https://github.com/h2depot";
const CLIP_REPO: &str = "https://github.com/h2depot/clipls";

pub fn plot_aboutme() -> Result<()> {
    let options = TerminalOptions {
        viewport: Viewport::Inline(8),
    };
    let mut terminal = ratatui::init_with_options(options);
    let result = (|| {
        loop {
            terminal.draw(|frame| {
                let container = Block::default()
                    .title(" About H2DEPOT | Enter/q/Esc: close ")
                    .borders(Borders::ALL);
                let inner = container.inner(frame.area());
                frame.render_widget(container, frame.area());

                let [name_area, profile_area, repository_area] = Layout::vertical([
                    Constraint::Length(2),
                    Constraint::Length(1),
                    Constraint::Length(1),
                ])
                .areas(inner);
                let name = Paragraph::new(Line::from(Span::styled(
                    "H2DEPOT",
                    Style::default()
                        .fg(Color::Rgb(255, 165, 0))
                        .add_modifier(Modifier::BOLD),
                )))
                .alignment(Alignment::Center);
                let github_link = Paragraph::new(Line::from(vec![
                    Span::raw("GitHub Profile: "),
                    Span::styled(
                        GITHUB_URL,
                        Style::default()
                            .fg(Color::Blue)
                            .add_modifier(Modifier::UNDERLINED),
                    ),
                ]))
                .alignment(Alignment::Center);
                let repo_link = Paragraph::new(Line::from(vec![
                    Span::raw("clipls Repo: "),
                    Span::styled(
                        CLIP_REPO,
                        Style::default()
                            .fg(Color::Blue)
                            .add_modifier(Modifier::UNDERLINED),
                    ),
                ]))
                .alignment(Alignment::Center);
                frame.render_widget(name, name_area);
                frame.render_widget(github_link, profile_area);
                frame.render_widget(repo_link, repository_area);
            })?;

            if let Event::Key(key) = event::read()? {
                let is_close_key =
                    matches!(key.code, KeyCode::Enter | KeyCode::Esc | KeyCode::Char('q'));
                let is_control_close = matches!(key.code, KeyCode::Char('c') | KeyCode::Char('d'))
                    && key.modifiers.contains(KeyModifiers::CONTROL);
                if key.kind == KeyEventKind::Press && (is_close_key || is_control_close) {
                    break;
                }
            }
        }

        let bottom_row = terminal.get_frame().area().bottom().saturating_sub(1);
        let mut stdout = std::io::stdout();
        stdout.execute(MoveTo(0, bottom_row))?;
        stdout.execute(Print("\r\n"))?;
        stdout.execute(Show)?;
        Ok(())
    })();
    disable_raw_mode()?;
    result
}
