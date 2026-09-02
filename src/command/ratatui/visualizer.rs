use anyhow::Result;
use crossterm::{
    cursor::{MoveTo, Show},
    event::{
        self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode, KeyEventKind, MouseButton,
        MouseEventKind,
    },
    style::Print,
    terminal::disable_raw_mode,
};
use ratatui::{
    DefaultTerminal, TerminalOptions, Viewport,
    layout::{Constraint, Layout, Rect},
    style::{Color, Modifier, Style},
    widgets::{Block, Borders, List, ListItem, ListState, Paragraph},
};

/// Shows an interactive file picker and returns the selected item indexes.
pub fn plot(file_items: &[String]) -> Result<Vec<usize>> {
    let height = file_items.len().saturating_add(3).clamp(4, 21) as u16;
    let options = TerminalOptions {
        viewport: Viewport::Inline(height),
    };
    let mut terminal = ratatui::init_with_options(options);
    let result = (|| {
        crossterm::execute!(std::io::stdout(), EnableMouseCapture)?;
        let picker_result = FilePicker::new(file_items).run(&mut terminal);
        crossterm::execute!(std::io::stdout(), DisableMouseCapture)?;

        // Leave the shell cursor below the inline viewport. Without the explicit
        // CRLF, PowerShell can redraw its next prompt over the preserved TUI.
        let bottom_row = terminal.get_frame().area().bottom().saturating_sub(1);
        crossterm::execute!(
            std::io::stdout(),
            MoveTo(0, bottom_row),
            Print("\r\n"),
            Show
        )?;
        picker_result
    })();
    // `init_with_options` does not enter the alternate screen for an inline
    // viewport. `ratatui::restore()` would still try to leave it, which makes
    // some shells restore the cursor to the command's original row.
    disable_raw_mode()?;
    result
}

struct FilePicker<'a> {
    items: &'a [String],
    cursor: usize,
    selected: Option<usize>,
}

impl<'a> FilePicker<'a> {
    fn new(items: &'a [String]) -> Self {
        Self {
            items,
            cursor: 0,
            selected: None,
        }
    }

    fn run(mut self, terminal: &mut DefaultTerminal) -> Result<Vec<usize>> {
        loop {
            let mut list_area = Rect::default();
            let mut state =
                ListState::default().with_selected((!self.items.is_empty()).then_some(self.cursor));
            terminal.draw(|frame| {
                let container = Block::default()
                    .title(" Files | Click/Space: select | Enter/Esc: close ")
                    .borders(Borders::ALL);
                let inner_area = container.inner(frame.area());
                frame.render_widget(container, frame.area());

                let [files_area, status_area] =
                    Layout::vertical([Constraint::Min(3), Constraint::Length(1)]).areas(inner_area);
                list_area = files_area;
                let items = self.items.iter().enumerate().map(|(index, item)| {
                    let marker = if self.selected == Some(index) {
                        "[x]"
                    } else {
                        "[ ]"
                    };
                    ListItem::new(format!("{marker} {item}"))
                });
                let list = List::new(items)
                    .highlight_style(
                        Style::default()
                            .fg(Color::Yellow)
                            .add_modifier(Modifier::BOLD),
                    )
                    .highlight_symbol("> ");
                frame.render_stateful_widget(list, files_area, &mut state);

                let status = self
                    .selected
                    .and_then(|index| self.items.get(index))
                    .map_or_else(
                        || "Status: Nothing copied.".to_owned(),
                        |filename| format!("Status: {filename} copied!"),
                    );
                frame.render_widget(Paragraph::new(status), status_area);
            })?;

            match event::read()? {
                Event::Key(key) if key.kind == KeyEventKind::Press => match key.code {
                    KeyCode::Char('q') | KeyCode::Esc | KeyCode::Enter => {
                        return Ok(self.selected.into_iter().collect());
                    }
                    KeyCode::Up | KeyCode::Char('k') => self.move_up(),
                    KeyCode::Down | KeyCode::Char('j') => self.move_down(),
                    KeyCode::Char(' ') => self.toggle_selected(),
                    _ => {}
                },
                Event::Mouse(mouse) => match mouse.kind {
                    MouseEventKind::Down(MouseButton::Left) => {
                        let content_top = list_area.y;
                        let content_bottom = list_area.bottom();
                        if mouse.column >= list_area.x
                            && mouse.column < list_area.right()
                            && mouse.row >= content_top
                            && mouse.row < content_bottom
                        {
                            let index = state.offset() + usize::from(mouse.row - content_top);
                            if index < self.items.len() {
                                self.cursor = index;
                                self.toggle_selected();
                            }
                        }
                    }
                    MouseEventKind::ScrollUp => self.move_up(),
                    MouseEventKind::ScrollDown => self.move_down(),
                    _ => {}
                },
                _ => {}
            }
        }
    }

    fn move_up(&mut self) {
        self.cursor = self.cursor.saturating_sub(1);
    }

    fn move_down(&mut self) {
        if !self.items.is_empty() {
            self.cursor = (self.cursor + 1).min(self.items.len() - 1);
        }
    }

    fn toggle_selected(&mut self) {
        if !self.items.is_empty() {
            self.selected = (self.selected != Some(self.cursor)).then_some(self.cursor);
        }
    }
}
