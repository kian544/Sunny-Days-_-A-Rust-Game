use crate::engine::world::World;
use crate::map::tile::Tile;

use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph, Wrap},
    Frame,
};

pub fn render(f: &mut Frame, world: &World) {
    let size = f.size();

    let vertical = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Min(5),
            Constraint::Length(7),
        ])
        .split(size);

    let top = vertical[0];
    let bottom = vertical[1];

    let horizontal = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Min(10),
            Constraint::Length(26),
        ])
        .split(top);

    let map_area = horizontal[0];
    let sidebar_area = horizontal[1];

    draw_map(f, map_area, world);
    draw_sidebar(f, sidebar_area, world);
    draw_logs(f, bottom, world);
}

fn draw_map(f: &mut Frame, area: Rect, world: &World) {
    let map = &world.map;
    let px = world.player.x;
    let py = world.player.y;

    let mut lines: Vec<Line> = Vec::with_capacity(map.height);

    for y in 0..map.height {
        let mut spans: Vec<Span> = Vec::with_capacity(map.width);
        for x in 0..map.width {
            if x as i32 == px && y as i32 == py {
                spans.push(Span::styled("@", Style::default().fg(Color::Yellow)));
            } else {
                let tile = map.get(x, y);
                let (ch, style) = match tile {
                    Tile::Wall => ("#", Style::default().fg(Color::DarkGray)),
                    Tile::Floor => (".", Style::default().fg(Color::Gray)),
                };
                spans.push(Span::styled(ch, style));
            }
        }
        lines.push(Line::from(spans));
    }

    let map_widget = Paragraph::new(lines)
        .block(Block::default().borders(Borders::ALL).title("Map"))
        .wrap(Wrap { trim: false });

    f.render_widget(map_widget, area);
}

fn draw_sidebar(f: &mut Frame, area: Rect, world: &World) {
    let p = &world.player;

    let text = vec![
        Line::from(vec![
            Span::styled("HP: ", Style::default().fg(Color::White)),
            Span::styled(format!("{}/{}", p.hp, p.hp), Style::default().fg(Color::Green)),
        ]),
        Line::from(format!("Pos: ({}, {})", p.x, p.y)),
        Line::from(format!("Seed: {}", world.seed)),
        Line::from(""),
        Line::from(Span::styled("Controls", Style::default().fg(Color::Cyan))),
        Line::from("WASD / Arrows: Move"),
        Line::from("Q: Quit"),
    ];

    let sidebar = Paragraph::new(text)
        .block(Block::default().borders(Borders::ALL).title("Player"))
        .wrap(Wrap { trim: true });

    f.render_widget(sidebar, area);
}

fn draw_logs(f: &mut Frame, area: Rect, world: &World) {
    let mut lines: Vec<Line> = Vec::new();
    for msg in world.logs.iter() {
        lines.push(Line::from(msg.clone()));
    }

    let logs = Paragraph::new(lines)
        .block(Block::default().borders(Borders::ALL).title("Log"))
        .wrap(Wrap { trim: true });

    f.render_widget(logs, area);
}
