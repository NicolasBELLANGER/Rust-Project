use ratatui::{
    Frame,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Style},
    text::{Line, Span, Text},
    widgets::{Block, Borders, Paragraph},
};

use crate::app::App;
use crate::map::MapTile;
use crate::messages::ResourceKind;

pub fn draw(frame: &mut Frame, app: &App) {
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(70), Constraint::Percentage(30)])
        .split(frame.area());

    draw_map(frame, app, chunks[0]);
    draw_stats(frame, app, chunks[1]);
}

fn draw_map(frame: &mut Frame, app: &App, area: Rect) {
    let lines: Vec<Line> = (0..app.map.height)
        .map(|y| {
            let spans: Vec<Span> = (0..app.map.width)
                .flat_map(|x| {
                    let tile = &app.map.tiles[y][x];
                    let (ch, color) = if matches!(tile, MapTile::Base) {
                        ('#', Color::LightGreen)
                    } else if app.collectors.iter().any(|&c| c == (x, y)) {
                        ('o', Color::Magenta)
                    } else if app.scouts.iter().any(|&s| s == (x, y)) {
                        ('x', Color::Red)
                    } else {
                        match tile {
                            MapTile::Empty => ('.', Color::DarkGray),
                            MapTile::Obstacle => ('O', Color::LightCyan),
                            MapTile::Energy(_) => ('E', Color::Green),
                            MapTile::Crystal(_) => ('C', Color::LightMagenta),
                            MapTile::Base => ('#', Color::LightGreen),
                        }
                    };
                    [
                        Span::styled(ch.to_string(), Style::default().fg(color)),
                        Span::raw(" "),
                    ]
                })
                .collect();
            Line::from(spans)
        })
        .collect();

    let map_widget = Paragraph::new(Text::from(lines))
        .block(Block::default().borders(Borders::ALL).title(" Carte "));

    frame.render_widget(map_widget, area);
}

fn draw_stats(frame: &mut Frame, app: &App, area: Rect) {
    let base = app.base.pos;

    let energy_known = app
        .known_resources
        .iter()
        .filter(|(_, _, k)| matches!(k, ResourceKind::Energy))
        .count();
    let crystal_known = app
        .known_resources
        .iter()
        .filter(|(_, _, k)| matches!(k, ResourceKind::Crystal))
        .count();

    let lines = vec![
        Line::from(Span::styled("= Base =", Style::default().fg(Color::Green))),
        Line::from(format!("  Pos : ({}, {})", base.0, base.1)),
        Line::from(""),
        Line::from(Span::styled("= Robots =", Style::default().fg(Color::White))),
        Line::from(vec![
            Span::styled("  x", Style::default().fg(Color::Red)),
            Span::raw(format!(" Scouts    : {}", app.scouts.len())),
        ]),
        Line::from(vec![
            Span::styled("  o", Style::default().fg(Color::Magenta)),
            Span::raw(format!(" Collecteurs: {}", app.collectors.len())),
        ]),
        Line::from(""),
        Line::from(Span::styled("= Collecte =", Style::default().fg(Color::Green))),
        Line::from(vec![
            Span::raw("  Energie:  "),
            Span::styled(
                app.collected_energy.to_string(),
                Style::default().fg(Color::Green),
            ),
        ]),
        Line::from(vec![
            Span::raw("  Cristaux: "),
            Span::styled(
                app.collected_crystals.to_string(),
                Style::default().fg(Color::LightMagenta),
            ),
        ]),
        Line::from(""),
        Line::from(Span::styled("= Exploration =", Style::default().fg(Color::DarkGray))),
        Line::from(vec![
            Span::raw("  Energie:  "),
            Span::styled(format!("{} sites", energy_known), Style::default().fg(Color::Green)),
        ]),
        Line::from(vec![
            Span::raw("  Cristaux: "),
            Span::styled(format!("{} sites", crystal_known), Style::default().fg(Color::LightMagenta)),
        ]),
        Line::from(format!("  En route: {}", app.claimed_resources.len())),
        Line::from(""),
        Line::from(Span::styled("= Legende =", Style::default().fg(Color::DarkGray))),
        Line::from(Span::styled("  . vide", Style::default().fg(Color::DarkGray))),
        Line::from(Span::styled("  O obstacle", Style::default().fg(Color::LightCyan))),
        Line::from(Span::styled("  E energie", Style::default().fg(Color::Green))),
        Line::from(Span::styled("  C cristal", Style::default().fg(Color::LightMagenta))),
        Line::from(Span::styled("  # base", Style::default().fg(Color::LightGreen))),
        Line::from(Span::styled("  x scout", Style::default().fg(Color::Red))),
        Line::from(Span::styled("  o collecteur", Style::default().fg(Color::Magenta))),
        Line::from(""),
        Line::from(Span::styled("[touche] Quitter", Style::default().fg(Color::DarkGray))),
    ];

    let stats_widget = Paragraph::new(Text::from(lines))
        .block(Block::default().borders(Borders::ALL).title(" Stats "));

    frame.render_widget(stats_widget, area);
}
