use std::{env, time::Duration};

use anyhow::Result;
use crossterm::event::{self, Event, KeyEventKind};
use ratatui::{
    Frame,
    layout::{Alignment, Constraint, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Clear, Gauge, Paragraph},
};

use crate::file_ops::fetch_item_names;

const FRAME_TIME: Duration = Duration::from_millis(50);
const ORANGE: Color = Color::Rgb(255, 126, 0);
const HOT_YELLOW: Color = Color::Rgb(255, 224, 64);
const MOLTEN_RED: Color = Color::Rgb(255, 48, 32);
const STEEL_BLUE: Color = Color::Rgb(40, 130, 210);
const DEEP_PURPLE: Color = Color::Rgb(92, 36, 140);

pub fn plot_easteregg() -> Result<()> {
    let item_names = fetch_item_names(&env::current_dir()?)?;
    let mut terminal = ratatui::init();
    let result = (|| {
        let mut tick = 0usize;
        loop {
            terminal.draw(|frame| render_furnace(frame, &item_names, tick))?;

            if event::poll(FRAME_TIME)?
                && matches!(event::read()?, Event::Key(key) if key.kind == KeyEventKind::Press)
            {
                break;
            }
            tick = tick.wrapping_add(1);
        }
        Ok(())
    })();
    ratatui::restore();
    result
}

fn render_furnace(frame: &mut Frame<'_>, item_names: &[String], tick: usize) {
    let area = frame.area();
    frame.render_widget(Clear, area);
    frame.render_widget(
        Block::default().style(Style::default().bg(Color::Rgb(9, 5, 18))),
        area,
    );

    if area.width < 48 || area.height < 14 {
        let warning = Paragraph::new(vec![
            Line::styled(
                "🔥 CLIPBOARD FOUNDRY 🔥",
                Style::default().fg(HOT_YELLOW).add_modifier(Modifier::BOLD),
            ),
            Line::styled("The furnace needs more room.", Style::default().fg(ORANGE)),
            Line::styled("Press any key to return.", Style::default().fg(Color::Cyan)),
        ])
        .alignment(Alignment::Center);
        frame.render_widget(warning, area);
        return;
    }

    let [header, body, footer] = Layout::vertical([
        Constraint::Length(3),
        Constraint::Min(8),
        Constraint::Length(3),
    ])
    .areas(area);
    render_header(frame, header, tick);
    render_body(frame, body, item_names, tick);
    render_footer(frame, footer, tick);
    render_sparks(frame, body, tick);
}

fn render_header(frame: &mut Frame<'_>, area: Rect, tick: usize) {
    let pulse = if tick % 12 < 6 { HOT_YELLOW } else { ORANGE };
    let title = Paragraph::new(Line::from(vec![
        Span::styled("  CLIPLS  ", Style::default().fg(Color::Black).bg(pulse)),
        Span::styled(
            " // CLIPBOARD FOUNDRY // ",
            Style::default()
                .fg(Color::Magenta)
                .add_modifier(Modifier::BOLD),
        ),
        Span::styled(
            "  2800°C  ",
            Style::default().fg(Color::Black).bg(MOLTEN_RED),
        ),
    ]))
    .block(
        Block::default()
            .borders(Borders::BOTTOM)
            .border_style(Style::default().fg(DEEP_PURPLE)),
    )
    .alignment(Alignment::Center);
    frame.render_widget(title, area);
}

fn render_body(frame: &mut Frame<'_>, area: Rect, item_names: &[String], tick: usize) {
    let [conveyor, _] =
        Layout::horizontal([Constraint::Percentage(48), Constraint::Percentage(52)]).areas(area);

    frame.render_widget(
        Block::default()
            .title(" INTAKE BELT ")
            .borders(Borders::ALL)
            .border_style(Style::default().fg(STEEL_BLUE)),
        conveyor,
    );
    let belt_inner = conveyor.inner(ratatui::layout::Margin {
        horizontal: 1,
        vertical: 1,
    });
    let travel = usize::from(belt_inner.width.saturating_sub(20)).max(1);
    for (index, name) in item_names.iter().take(3).enumerate() {
        let y = belt_inner
            .y
            .saturating_add((index as u16).saturating_mul(2));
        if y >= belt_inner.bottom() {
            break;
        }
        let offset = (tick / 2 + index * (travel / 3).max(1)) % travel;
        let color = [Color::Cyan, Color::Magenta, Color::LightBlue][index];
        let item = Paragraph::new(Line::from(vec![
            Span::styled("▰ ", Style::default().fg(ORANGE)),
            Span::styled(
                name,
                Style::default().fg(color).add_modifier(Modifier::BOLD),
            ),
            Span::styled("  ≫", Style::default().fg(HOT_YELLOW)),
        ]));
        frame.render_widget(
            item,
            Rect::new(
                belt_inner.x.saturating_add(offset as u16),
                y,
                belt_inner.width.saturating_sub(offset as u16),
                1,
            ),
        );
    }
    if belt_inner.height > 1 {
        let belt_y = belt_inner.bottom().saturating_sub(1);
        let teeth = "▱▰".repeat(usize::from(belt_inner.width) / 2 + 1);
        frame.render_widget(
            Paragraph::new(teeth).style(Style::default().fg(STEEL_BLUE)),
            Rect::new(belt_inner.x, belt_y, belt_inner.width, 1),
        );
    }
}

fn render_footer(frame: &mut Frame<'_>, area: Rect, tick: usize) {
    let ratio = (tick % 101) as f64 / 100.0;
    let gauge = Gauge::default()
        .block(
            Block::default()
                .title(" MOLTEN CLIPBOARD PRESSURE ")
                .borders(Borders::TOP),
        )
        .gauge_style(
            Style::default()
                .fg(HOT_YELLOW)
                .bg(DEEP_PURPLE)
                .add_modifier(Modifier::BOLD),
        )
        .ratio(ratio)
        .label(format!("{:>3}%  •  PRESS ANY KEY TO RETURN", tick % 101));
    frame.render_widget(gauge, area);
}

fn render_sparks(frame: &mut Frame<'_>, area: Rect, tick: usize) {
    if area.width == 0 || area.height == 0 {
        return;
    }
    let colors = [HOT_YELLOW, ORANGE, MOLTEN_RED, Color::Magenta];
    let glyphs = ['·', '*', '✦', '•'];
    let buffer = frame.buffer_mut();
    for index in 0..18usize {
        let x = area.x + ((tick * (index + 3) + index * 17) % usize::from(area.width)) as u16;
        let y = area.y + ((tick + index * 7) % usize::from(area.height)) as u16;
        buffer[(x, y)]
            .set_char(glyphs[index % glyphs.len()])
            .set_fg(colors[index % colors.len()]);
    }
}
