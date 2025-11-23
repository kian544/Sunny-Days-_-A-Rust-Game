use crate::engine::world::World;
use crate::map::tile::Tile;


use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph, Wrap},
    Frame,
};

const VIEW_W: i32 = 35;  // how wide the camera is (tiles)
const VIEW_H: i32 = 20;  // how tall the camera is (tiles)

fn compute_viewport(px: i32, py: i32, map_w: i32, map_h: i32, view_w: i32, view_h: i32) -> (i32, i32, i32, i32) {
    // top-left corner if perfectly centered
    let mut x0 = px - view_w / 2;
    let mut y0 = py - view_h / 2;

    // clamp to left/top
    if x0 < 0 { x0 = 0; }
    if y0 < 0 { y0 = 0; }

    // clamp to right/bottom
    if x0 + view_w > map_w {
        x0 = (map_w - view_w).max(0);
    }
    if y0 + view_h > map_h {
        y0 = (map_h - view_h).max(0);
    }

    let x1 = (x0 + view_w).min(map_w);
    let y1 = (y0 + view_h).min(map_h);

    (x0, y0, x1, y1)
}


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
    let map = world.current_map();
    let px = world.player.x;
    let py = world.player.y;

    let map_w = map.width as i32;
    let map_h = map.height as i32;

    // Ratatui area size (in characters)
    let area_w = area.width as i32;
    let area_h = area.height as i32;

    // Camera size = min of our desired zoom and screen space
    let view_w = VIEW_W.min(area_w);
    let view_h = VIEW_H.min(area_h);

    let (x0, y0, x1, y1) = compute_viewport(px, py, map_w, map_h, view_w, view_h);

    let mut lines: Vec<Line> = Vec::with_capacity((y1 - y0) as usize);

    for wy in y0..y1 {
        let mut spans: Vec<Span> = Vec::with_capacity((x1 - x0) as usize);
        for wx in x0..x1 {
            if wx == px && wy == py {
                spans.push(Span::styled("@", Style::default().fg(Color::Yellow)));
            } else {
            let tile = map.get(wx as usize, wy as usize);
            let (ch, style) = match tile {
                Tile::Wall => ("#", Style::default().fg(Color::DarkGray)),
                Tile::Floor => (" ", Style::default()),
                Tile::DoorForward => ("+", Style::default().fg(Color::White)), // white door
                Tile::DoorBack => ("+", Style::default().fg(Color::White)),    // return door
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
