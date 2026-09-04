use std::{
    collections::BTreeSet,
    env,
    io::stdout,
    time::{Duration, Instant, SystemTime, UNIX_EPOCH},
};

use anyhow::Result;
use crossterm::{
    ExecutableCommand,
    event::{
        self, DisableMouseCapture, EnableMouseCapture, Event, KeyEventKind, MouseButton,
        MouseEventKind,
    },
};
use ratatui::{
    layout::{Alignment, Constraint, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, List, ListItem, Paragraph, canvas::Canvas},
};

use crate::file_ops::fetch_item_names;

const PUMPKIN: [&str; 26] = [
    "                                                ",
    "                           @@@@%                ",
    "                         #@@@@@@                ",
    "                        @@@@@@                  ",
    "                       @@@@@@                   ",
    "                      @@@@@@                    ",
    "                     @@@@@@@                    ",
    "                    =@@@@@@@                    ",
    "                    @@%  #@@                    ",
    "         @@@@  #@@@@  @@@@  @@@@@ #@@@.         ",
    "       @@@@@  @@@@@ @@@@@@@@ @@@@@  @@@@:       ",
    "      @@@@@@ @@@@@ @@@@@@@@@@ @@@@@ *@@@@@      ",
    "     @@@@@@ @@@@@  @@@@@@@@@@  @@@@@ @@@@@@     ",
    "    @@@@@@% @@@@@ @@@@@@@@@@@@ @@@@@ =@@@@@     ",
    "    @@@@@@ @@+      @@@@@@@@      @@@ @@@@@@    ",
    "   @@@@@@@ @%        @@@@@@        @@ @@@@@@    ",
    "   @@@@@@@ @@       +@@@@@@        @@ @@@@@@    ",
    "   @@@@@@@ @@@*    @@@@@@@@@@    @@@@ @@@@@@    ",
    "    @@@@@@ :@@@@@ @@@@@@@@@@@@ @@@@@# @@@@@@    ",
    "    @@@@@@@ @@@@@ @@@@@@@@@@@@ @@@@@ @@@@@@     ",
    "     @@@@@@ @@@@@# @@@@@@@@@@ .@@@@@ @@@@@      ",
    "      @@@@@@ @@@@@ #@@@@@@@@@ @@@@@ @@@@@       ",
    "       @@@@@@ @@@@@ -@@@@@@# @@@@@ @@@@@        ",
    "          @@%   @@@    @@.   @@@   @@=          ",
    "                                                ",
    "                                                ",
];

const CLIPLS_ICON: [&str; 26] = [
    "                                                ",
    "                                                ",
    "                                                ",
    "                                                ",
    "                                                ",
    "                            @@@ @@@@            ",
    "                    @@    @@@      @@           ",
    "                  @@@   .@@         @@          ",
    "                @@@    @@          @@@          ",
    "              @@@    @@           @@            ",
    "            @@@    @@     @@:@@@@@@             ",
    "           @@    @@@    @@    @@@               ",
    "         @@    @@@    @@     @@@                ",
    "       @@    @@@    @@     @@@@@   @@@@@@@@@    ",
    "     @@     @@    @@     @@@  @@  @@      @@    ",
    "    @@    @@    @@@    @@@    @@   @@@@@        ",
    "    @@  @@    @@@     @@      @@        @@@@    ",
    "    @@   @@ @@@     @@        @@  @@       @    ",
    "    @@     @      @@          @@  @@@@@@@@@     ",
    "     @@         @@@       @@@@@@@@@             ",
    "       @@@@  @@@@                               ",
    "                                                ",
    "                                                ",
    "                                                ",
    "                                                ",
    "                                                ",
];

const H2DEPOT: [&str; 6] = [
    "  ██╗  ██╗██████╗ ██████╗ ███████╗██████╗  ██████╗ ████████╗",
    "  ██║  ██║╚════██╗██╔══██╗██╔════╝██╔══██╗██╔═══██╗╚══██╔══╝",
    "███████║ █████╔╝██║  ██║█████╗  ██████╔╝██║   ██║  ██║",
    "██╔══██║██╔═══╝ ██║  ██║██╔══╝  ██╔═══╝ ██║   ██║  ██║",
    "██║  ██║███████╗██████╔╝███████╗██║     ╚██████╔╝  ██║",
    "╚═╝  ╚═╝╚══════╝╚═════╝ ╚══════╝╚═╝      ╚═════╝   ╚═╝",
];

const FRAME_TIME: Duration = Duration::from_millis(90);
const ICON_FILL_TIME: Duration = Duration::from_millis(180);
const ICON_REVEAL_TIME: Duration = Duration::from_millis(900);
const ICON_REVERSE_TIME: Duration = Duration::from_millis(400);

enum IconPhase {
    Clipls,
    Transition {
        to_pumpkin: bool,
        started_at: Instant,
        reveal_time: Duration,
        seed: u64,
    },
    Pumpkin,
}

impl IconPhase {
    fn settle_if_finished(&mut self, now: Instant) {
        let IconPhase::Transition {
            to_pumpkin,
            started_at,
            reveal_time,
            ..
        } = self
        else {
            return;
        };

        if now.duration_since(*started_at) >= ICON_FILL_TIME + *reveal_time {
            *self = if *to_pumpkin {
                IconPhase::Pumpkin
            } else {
                IconPhase::Clipls
            };
        }
    }
}

fn shade(character: u8) -> usize {
    b" .:-=+*#%@"
        .iter()
        .position(|candidate| *candidate == character)
        .unwrap_or(0)
}

fn fit_row(row: &str, width: usize) -> String {
    if width >= row.len() {
        return row.to_owned();
    }

    (0..width)
        .map(|column| {
            let start = column * row.len() / width;
            let end = ((column + 1) * row.len() / width).max(start + 1);
            let character = row.as_bytes()[start..end]
                .iter()
                .copied()
                .max_by_key(|character| shade(*character))
                .unwrap_or(b' ');
            char::from(character)
        })
        .collect()
}

fn transition_order(row: usize, column: usize, seed: u64) -> f64 {
    let mut value = seed
        ^ (row as u64).wrapping_mul(0x9e37_79b9_7f4a_7c15)
        ^ (column as u64).wrapping_mul(0xbf58_476d_1ce4_e5b9);
    value ^= value >> 30;
    value = value.wrapping_mul(0xbf58_476d_1ce4_e5b9);
    value ^= value >> 27;
    value = value.wrapping_mul(0x94d0_49bb_1331_11eb);
    value ^= value >> 31;
    value as f64 / u64::MAX as f64
}

fn transitioning_icon_line(
    target_row: &str,
    width: usize,
    row: usize,
    elapsed: Duration,
    reveal_time: Duration,
    seed: u64,
    target_style: Style,
) -> Line<'static> {
    let target = fit_row(target_row, width);
    let progress = elapsed.saturating_sub(ICON_FILL_TIME).as_secs_f64() / reveal_time.as_secs_f64();
    let progress = progress.clamp(0.0, 1.0);
    let dissolve_edge = (progress + 0.08).min(1.0);
    let white_style = Style::default()
        .fg(Color::White)
        .add_modifier(Modifier::BOLD);

    Line::from(
        target
            .chars()
            .enumerate()
            .map(|(column, target_character)| {
                let order = transition_order(row, column, seed);
                if order <= progress {
                    Span::styled(target_character.to_string(), target_style)
                } else if order <= dissolve_edge && elapsed >= ICON_FILL_TIME {
                    Span::styled(" ", white_style)
                } else {
                    Span::styled("@", white_style)
                }
            })
            .collect::<Vec<_>>(),
    )
}

fn fit_unicode_row(row: &str, width: usize) -> String {
    let characters = row.chars().collect::<Vec<_>>();
    if width >= characters.len() {
        return row.to_owned();
    }

    (0..width)
        .map(|column| {
            let start = column * characters.len() / width;
            let end = ((column + 1) * characters.len() / width).max(start + 1);
            characters[start..end]
                .iter()
                .copied()
                .max_by_key(|character| usize::from(*character != ' '))
                .unwrap_or(' ')
        })
        .collect()
}

fn moving_pattern(pattern: &str, width: usize, offset: usize) -> String {
    let pattern = pattern.chars().collect::<Vec<_>>();
    (0..width)
        .map(|column| pattern[(column + offset) % pattern.len()])
        .collect()
}

struct CargoSlot {
    gap: usize,
    index: Option<usize>,
}

fn generate_cargo_slots(item_count: usize, mut random: u64) -> Vec<CargoSlot> {
    if item_count == 0 {
        return Vec::new();
    }

    let mut slots = Vec::new();
    let mut current_len = 0;
    while current_len < 512 {
        random = random
            .wrapping_mul(6_364_136_223_846_793_005)
            .wrapping_add(1);
        let gap = 6 + (random as usize % 19);

        random = random
            .wrapping_mul(6_364_136_223_846_793_005)
            .wrapping_add(1);
        let index = if random % 100 < 70 {
            Some(random as usize % item_count)
        } else {
            None
        };
        slots.push(CargoSlot { gap, index });
        current_len += gap + 15;
    }
    slots
}

fn build_cargo_pattern(
    slots: &[CargoSlot],
    item_names: &[String],
    selected: &BTreeSet<usize>,
) -> String {
    if item_names.is_empty() {
        return " ".repeat(512);
    }

    let mut pattern = String::new();
    for slot in slots {
        pattern.extend(std::iter::repeat_n(' ', slot.gap));
        if let Some(index) = slot.index {
            pattern.push_str("▰ ");
            if selected.contains(&index) {
                pattern.push_str("███");
            } else {
                pattern.push_str(&item_names[index]);
            }
        }
    }
    pattern.extend(std::iter::repeat_n(' ', 12));
    pattern
}

fn conveyor(width: usize, tick: usize, reverse: bool, cargo_pattern: &str) -> Vec<Line<'static>> {
    let style = Style::default()
        .fg(Color::Cyan)
        .add_modifier(Modifier::BOLD);
    let item_style = Style::default()
        .fg(Color::Yellow)
        .add_modifier(Modifier::BOLD);
    let offset = |speed: usize| {
        let position = (tick / speed) % 8;
        if reverse {
            (8 - position) % 8
        } else {
            position
        }
    };
    let belt_offset = offset(1);
    let roller_offset = offset(2);
    let pattern_len = cargo_pattern.chars().count().max(1);
    let cargo_offset = if reverse {
        (pattern_len - (tick % pattern_len)) % pattern_len
    } else {
        tick % pattern_len
    };

    let mut lines = vec![Line::from(Span::styled(
        moving_pattern(cargo_pattern, width, cargo_offset),
        item_style,
    ))];
    lines.extend(
        [
            moving_pattern("╔═══╦═══", width, belt_offset),
            moving_pattern("║███║   ", width, belt_offset),
            moving_pattern("╚═══╩═══", width, belt_offset),
            moving_pattern("●───●───", width, roller_offset),
        ]
        .into_iter()
        .map(|row| Line::from(Span::styled(row, style))),
    );
    lines
}

pub fn plot_easteregg() -> Result<()> {
    let item_names = fetch_item_names(&env::current_dir()?)?;
    let random_seed = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_nanos() as u64;
    let upper_slots = generate_cargo_slots(item_names.len(), random_seed);
    let lower_slots = generate_cargo_slots(item_names.len(), random_seed.rotate_left(29));
    let mut selected = BTreeSet::new();
    let mut icon_phase = IconPhase::Clipls;

    stdout().execute(EnableMouseCapture)?;
    let mut terminal = ratatui::init();

    let result = (|| {
        let mut last_tick = Instant::now();
        let mut tick = 0usize;
        loop {
            icon_phase.settle_if_finished(Instant::now());
            let mut list_area = Rect::default();
            let upper_cargo = build_cargo_pattern(&upper_slots, &item_names, &selected);
            let lower_cargo = build_cargo_pattern(&lower_slots, &item_names, &selected);

            terminal.draw(|frame| {
                let [left_area, right_area] =
                    Layout::horizontal([Constraint::Percentage(50), Constraint::Percentage(50)])
                        .areas(frame.area());

                let showing_pumpkin = matches!(icon_phase, IconPhase::Pumpkin);
                let (icon_data, icon_title, icon_style) = if showing_pumpkin {
                    (
                        &PUMPKIN,
                        " PUMPKIN STATE ",
                        Style::default()
                            .fg(Color::Rgb(242, 139, 0))
                            .add_modifier(Modifier::BOLD),
                    )
                } else {
                    (
                        &CLIPLS_ICON,
                        " CLIPLS STATE ",
                        Style::default()
                            .fg(Color::Cyan)
                            .add_modifier(Modifier::BOLD),
                    )
                };

                let left_block = Block::default().title(icon_title).borders(Borders::ALL);
                let left_inner = left_block.inner(left_area);
                let left_canvas = Canvas::default()
                    .block(left_block)
                    .x_bounds([0.0, 1.0])
                    .y_bounds([0.0, 1.0])
                    .paint(|_| {});
                let right_canvas = Canvas::default()
                    .block(Block::default().title(" FACTORY ").borders(Borders::ALL))
                    .x_bounds([0.0, 1.0])
                    .y_bounds([0.0, 1.0])
                    .paint(|_| {});
                let right_inner = Block::default().borders(Borders::ALL).inner(right_area);

                frame.render_widget(left_canvas, left_area);
                frame.render_widget(right_canvas, right_area);

                let image_height = icon_data.len().min(usize::from(left_inner.height));
                let image_area = Rect::new(
                    left_inner.x,
                    left_inner.y + (left_inner.height - image_height as u16) / 2,
                    left_inner.width,
                    image_height as u16,
                );
                let icon_image = (0..image_height)
                    .map(|row| {
                        let source_row = icon_data[row * icon_data.len() / image_height];
                        if let IconPhase::Transition {
                            to_pumpkin,
                            started_at,
                            reveal_time,
                            seed,
                        } = &icon_phase
                        {
                            let target_data = if *to_pumpkin { &PUMPKIN } else { &CLIPLS_ICON };
                            let target_style = if *to_pumpkin {
                                Style::default()
                                    .fg(Color::Rgb(242, 139, 0))
                                    .add_modifier(Modifier::BOLD)
                            } else {
                                Style::default()
                                    .fg(Color::Cyan)
                                    .add_modifier(Modifier::BOLD)
                            };
                            transitioning_icon_line(
                                target_data[row * target_data.len() / image_height],
                                usize::from(left_inner.width),
                                row,
                                Instant::now().duration_since(*started_at),
                                *reveal_time,
                                *seed,
                                target_style,
                            )
                        } else {
                            Line::from(Span::styled(
                                fit_row(source_row, usize::from(left_inner.width)),
                                icon_style,
                            ))
                        }
                    })
                    .collect::<Vec<_>>();
                frame.render_widget(
                    Paragraph::new(icon_image).alignment(Alignment::Center),
                    image_area,
                );

                let logo_height = (H2DEPOT.len() as u16).min(right_inner.height);
                let logo = H2DEPOT
                    .iter()
                    .take(usize::from(logo_height))
                    .map(|row| {
                        Line::from(Span::styled(
                            fit_unicode_row(row, usize::from(right_inner.width)),
                            Style::default()
                                .fg(Color::Cyan)
                                .add_modifier(Modifier::BOLD),
                        ))
                    })
                    .collect::<Vec<_>>();
                frame.render_widget(
                    Paragraph::new(logo).alignment(Alignment::Center),
                    Rect::new(right_inner.x, right_inner.y, right_inner.width, logo_height),
                );

                let conveyor_y = right_inner.y + logo_height + 1;
                let available_height = right_inner.bottom().saturating_sub(conveyor_y);
                let upper_height = available_height.min(5);
                if upper_height > 0 {
                    frame.render_widget(
                        Paragraph::new(conveyor(
                            usize::from(right_inner.width),
                            tick,
                            false,
                            &upper_cargo,
                        )),
                        Rect::new(right_inner.x, conveyor_y, right_inner.width, upper_height),
                    );
                }

                let arrow_y = conveyor_y + upper_height;
                if right_inner.bottom() > arrow_y {
                    frame.render_widget(
                        Paragraph::new(Span::styled(
                            "←  →",
                            Style::default()
                                .fg(Color::Cyan)
                                .add_modifier(Modifier::BOLD),
                        ))
                        .alignment(Alignment::Center),
                        Rect::new(right_inner.x, arrow_y, right_inner.width, 1),
                    );
                }

                let lower_y = arrow_y + 1;
                let lower_height = right_inner.bottom().saturating_sub(lower_y).min(5);
                if lower_height > 0 {
                    frame.render_widget(
                        Paragraph::new(conveyor(
                            usize::from(right_inner.width),
                            tick,
                            true,
                            &lower_cargo,
                        )),
                        Rect::new(right_inner.x, lower_y, right_inner.width, lower_height),
                    );
                }

                let list_y = lower_y + lower_height + 1;
                let list_available_height = right_inner.bottom().saturating_sub(list_y);
                if list_available_height >= 3 {
                    let list_render_height = list_available_height.min(5);
                    let list_block = Block::default()
                        .title(" Files (Click to select) ")
                        .borders(Borders::ALL);
                    let outer_list_rect =
                        Rect::new(right_inner.x, list_y, right_inner.width, list_render_height);
                    list_area = list_block.inner(outer_list_rect);

                    let items = item_names.iter().enumerate().map(|(index, name)| {
                        let is_selected = selected.contains(&index);
                        let marker = if is_selected { "[x] " } else { "[ ] " };
                        let accent_style = Style::default()
                            .fg(Color::Rgb(255, 165, 0))
                            .add_modifier(Modifier::BOLD);
                        let name_style = if is_selected {
                            accent_style
                        } else {
                            Style::default().fg(Color::White)
                        };
                        ListItem::new(Line::from(vec![
                            Span::styled(
                                marker,
                                if is_selected {
                                    accent_style
                                } else {
                                    Style::default().fg(Color::White)
                                },
                            ),
                            Span::styled(name, name_style),
                        ]))
                    });
                    let list = List::new(items).block(list_block);
                    frame.render_widget(list, outer_list_rect);

                    let text_y = list_y + list_render_height;
                    let text_available = right_inner.bottom().saturating_sub(text_y);
                    if text_available > 0 {
                        let status_text =
                            if !item_names.is_empty() && selected.len() == item_names.len() {
                                "clipls packed whole files. clipls developed by H2DEPOT.\n\
                                Thank you for using clipls!\n\
                                Wishing You the Best in Your Computer Life!"
                                    .to_string()
                            } else if !selected.is_empty() {
                                let packed_names: Vec<&str> = selected
                                    .iter()
                                    .filter_map(|&index| item_names.get(index).map(|s| s.as_str()))
                                    .collect();
                                format!("clipls packed {}.", packed_names.join(", "))
                            } else {
                                String::new()
                            };

                        if !status_text.is_empty() {
                            let status_paragraph = Paragraph::new(status_text).style(
                                Style::default()
                                    .fg(Color::Yellow)
                                    .add_modifier(Modifier::BOLD),
                            );
                            frame.render_widget(
                                status_paragraph,
                                Rect::new(right_inner.x, text_y, right_inner.width, text_available),
                            );
                        }
                    }
                }
            })?;

            let timeout = FRAME_TIME.saturating_sub(last_tick.elapsed());
            if event::poll(timeout)? {
                match event::read()? {
                    Event::Key(key) if key.kind == KeyEventKind::Press => {
                        break;
                    }
                    Event::Mouse(mouse) => {
                        if let MouseEventKind::Down(MouseButton::Left) = mouse.kind {
                            if mouse.column >= list_area.x
                                && mouse.column < list_area.right()
                                && mouse.row >= list_area.y
                                && mouse.row < list_area.bottom()
                            {
                                let index = usize::from(mouse.row - list_area.y);
                                if index < item_names.len() {
                                    let was_all_packed = !item_names.is_empty()
                                        && selected.len() == item_names.len();
                                    if !selected.remove(&index) {
                                        selected.insert(index);
                                    }
                                    let is_all_packed = !item_names.is_empty()
                                        && selected.len() == item_names.len();
                                    if is_all_packed != was_all_packed {
                                        icon_phase = IconPhase::Transition {
                                            to_pumpkin: is_all_packed,
                                            started_at: Instant::now(),
                                            reveal_time: if is_all_packed {
                                                ICON_REVEAL_TIME
                                            } else {
                                                ICON_REVERSE_TIME
                                            },
                                            seed: random_seed.rotate_left(index as u32 % u64::BITS),
                                        };
                                    }
                                }
                            }
                        }
                    }
                    _ => {}
                }
            }

            if last_tick.elapsed() >= FRAME_TIME {
                let passed_frames =
                    (last_tick.elapsed().as_millis() / FRAME_TIME.as_millis()) as usize;
                tick = tick.wrapping_add(passed_frames);
                last_tick += FRAME_TIME * passed_frames as u32;
            }
        }

        Ok(())
    })();

    let _ = stdout().execute(DisableMouseCapture);
    ratatui::restore();
    result
}
