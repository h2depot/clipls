use std::collections::BTreeSet;

use anyhow::Result;
use crossterm::{
    ExecutableCommand,
    cursor::{MoveTo, Show},
    event::{
        self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode, KeyEventKind, KeyModifiers,
        MouseButton, MouseEventKind,
    },
    style::Print,
    terminal::disable_raw_mode,
};
use ratatui::{
    DefaultTerminal, TerminalOptions, Viewport,
    layout::{Constraint, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, List, ListItem, ListState, Paragraph},
};

pub struct PickerItem {
    pub label: String,
    pub is_directory: bool,
    pub is_hidden: bool,
}

/// Shows an interactive file picker and returns the selected item indexes.
pub fn plot(file_items: &[PickerItem]) -> Result<Vec<usize>> {
    let height = file_items.len().saturating_add(3).clamp(4, 21) as u16;
    let options = TerminalOptions {
        viewport: Viewport::Inline(height),
    };
    let mut terminal = ratatui::init_with_options(options);
    let result = (|| {
        std::io::stdout().execute(EnableMouseCapture)?;
        let picker_result = FilePicker::new(file_items).run(&mut terminal);
        std::io::stdout().execute(DisableMouseCapture)?;

        // Leave the shell cursor below the inline viewport. Without the explicit
        // CRLF, PowerShell can redraw its next prompt over the preserved TUI.
        let bottom_row = terminal.get_frame().area().bottom().saturating_sub(1);
        let mut stdout = std::io::stdout();
        stdout.execute(MoveTo(0, bottom_row))?;
        stdout.execute(Print("\r\n"))?;
        stdout.execute(Show)?;
        picker_result
    })();
    // `init_with_options` does not enter the alternate screen for an inline
    // viewport. `ratatui::restore()` would still try to leave it, which makes
    // some shells restore the cursor to the command's original row.
    disable_raw_mode()?;
    result
}

struct FilePicker<'a> {
    items: &'a [PickerItem],
    cursor: usize,
    selected: BTreeSet<usize>,
}

impl<'a> FilePicker<'a> {
    fn new(items: &'a [PickerItem]) -> Self {
        Self {
            items,
            cursor: 0,
            selected: BTreeSet::new(),
        }
    }

    fn run(mut self, terminal: &mut DefaultTerminal) -> Result<Vec<usize>> {
        loop {
            let mut list_area = Rect::default();
            let mut state =
                ListState::default().with_selected((!self.items.is_empty()).then_some(self.cursor));
            terminal.draw(|frame| {
                let container = Block::default()
                    .title(" Files | Click/Space: select | Enter: copy | q/Esc: cancel ")
                    .borders(Borders::ALL);
                let inner_area = container.inner(frame.area());
                frame.render_widget(container, frame.area());

                let [files_area, status_area] =
                    Layout::vertical([Constraint::Min(3), Constraint::Length(1)]).areas(inner_area);
                list_area = files_area;
                let items = self.items.iter().enumerate().map(|(index, item)| {
                    let is_focused = self.cursor == index;
                    let is_selected = self.selected.contains(&index);
                    let cursor = if is_focused { "> " } else { "  " };
                    let marker = if is_selected { "[x]" } else { "[ ]" };
                    let accent_style = Style::default()
                        .fg(Color::Rgb(255, 165, 0))
                        .add_modifier(Modifier::BOLD);
                    let name_style = if is_focused {
                        accent_style
                    } else if item.is_hidden {
                        Style::default().fg(Color::Red)
                    } else if item.is_directory {
                        Style::default().fg(Color::Blue)
                    } else {
                        Style::default()
                    };
                    ListItem::new(Line::from(vec![
                        Span::styled(
                            cursor,
                            if is_focused {
                                accent_style
                            } else {
                                Style::default().fg(Color::White)
                            },
                        ),
                        Span::styled(
                            marker,
                            if is_selected {
                                accent_style
                            } else {
                                Style::default().fg(Color::White)
                            },
                        ),
                        Span::raw(" "),
                        Span::styled(&item.label, name_style),
                    ]))
                });
                let list = List::new(items);
                frame.render_stateful_widget(list, files_area, &mut state);

                let status = format!("Status: {} file(s) selected.", self.selected.len());
                frame.render_widget(Paragraph::new(status), status_area);
            })?;

            match event::read()? {
                Event::Key(key) if key.kind == KeyEventKind::Press => match key.code {
                    KeyCode::Enter => return Ok(self.selected.into_iter().collect()),
                    KeyCode::Char('q') | KeyCode::Esc => return Ok(Vec::new()),
                    KeyCode::Char('c') if key.modifiers.contains(KeyModifiers::CONTROL) => {
                        return Ok(Vec::new());
                    }
                    KeyCode::Char('d') if key.modifiers.contains(KeyModifiers::CONTROL) => {
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
        if !self.items.is_empty() && !self.selected.remove(&self.cursor) {
            self.selected.insert(self.cursor);
        }
    }
}
