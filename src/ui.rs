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
                    let (ch, color) = if app.collectors.iter().any(|&c| c == (x, y)) {
                        ('R', Color::Red)
                    } else if app.scouts.iter().any(|&s| s == (x, y)) {
                        ('S', Color::Magenta)
                    } else {
                        match &app.map.tiles[y][x] {
                            MapTile::Empty => ('.', Color::DarkGray),
                            MapTile::Obstacle => ('#', Color::White),
                            MapTile::Energy(_) => ('E', Color::Yellow),
                            MapTile::Crystal(_) => ('C', Color::Cyan),
                            MapTile::Base => ('B', Color::Green),
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
            Span::styled("  S", Style::default().fg(Color::Magenta)),
            Span::raw(format!(" Scouts    : {}", app.scouts.len())),
        ]),
        Line::from(vec![
            Span::styled("  R", Style::default().fg(Color::Red)),
            Span::raw(format!(" Collecteurs: {}", app.collectors.len())),
        ]),
        Line::from(""),
        Line::from(Span::styled("= Collecte =", Style::default().fg(Color::Yellow))),
        Line::from(vec![
            Span::raw("  Energie:  "),
            Span::styled(
                app.collected_energy.to_string(),
                Style::default().fg(Color::Yellow),
            ),
        ]),
        Line::from(vec![
            Span::raw("  Cristaux: "),
            Span::styled(
                app.collected_crystals.to_string(),
                Style::default().fg(Color::Cyan),
            ),
        ]),
        Line::from(""),
        Line::from(Span::styled("= Exploration =", Style::default().fg(Color::DarkGray))),
        Line::from(vec![
            Span::raw("  Energie:  "),
            Span::styled(format!("{} sites", energy_known), Style::default().fg(Color::Yellow)),
        ]),
        Line::from(vec![
            Span::raw("  Cristaux: "),
            Span::styled(format!("{} sites", crystal_known), Style::default().fg(Color::Cyan)),
        ]),
        Line::from(format!("  En route: {}", app.claimed_resources.len())),
        Line::from(""),
        Line::from(Span::styled("= Legende =", Style::default().fg(Color::DarkGray))),
        Line::from(Span::styled("  . vide", Style::default().fg(Color::DarkGray))),
        Line::from(Span::styled("  # obstacle", Style::default().fg(Color::White))),
        Line::from(Span::styled("  E energie", Style::default().fg(Color::Yellow))),
        Line::from(Span::styled("  C cristal", Style::default().fg(Color::Cyan))),
        Line::from(Span::styled("  B base", Style::default().fg(Color::Green))),
        Line::from(Span::styled("  S scout", Style::default().fg(Color::Magenta))),
        Line::from(Span::styled("  R collecteur", Style::default().fg(Color::Red))),
        Line::from(""),
        Line::from(Span::styled("[q] Quitter", Style::default().fg(Color::DarkGray))),
    ];

    let stats_widget = Paragraph::new(Text::from(lines))
        .block(Block::default().borders(Borders::ALL).title(" Stats "));

    frame.render_widget(stats_widget, area);
}
