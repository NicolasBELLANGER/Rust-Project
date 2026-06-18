use ratatui::{
    Frame,
    layout::{Constraint, Direction, Layout},
    style::{Color, Style},
    text::{Line, Span, Text},
    widgets::{Block, Borders, Paragraph},
};

use crate::app::App;
use crate::map::MapTile;

pub fn draw(frame: &mut Frame, app: &App) {
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(70), Constraint::Percentage(30)])
        .split(frame.area());

    draw_map(frame, app, chunks[0]);
    draw_stats(frame, app, chunks[1]);
}

fn draw_map(frame: &mut Frame, app: &App, area: ratatui::layout::Rect) {
    let lines: Vec<Line> = (0..app.map.height)
        .map(|y| {
            let spans: Vec<Span> = (0..app.map.width)
                .flat_map(|x| {
                    let (ch, color) = match &app.map.tiles[y][x] {
                        MapTile::Empty => ('.', Color::DarkGray),
                        MapTile::Obstacle => ('#', Color::White),
                        MapTile::Energy(_) => ('E', Color::Yellow),
                        MapTile::Crystal(_) => ('C', Color::Cyan),
                        MapTile::Base => ('B', Color::Green),
                    };
                    // char + espace pour que la carte soit carrée visuellement
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

fn draw_stats(frame: &mut Frame, app: &App, area: ratatui::layout::Rect) {
    let base = app.base.pos;
    let lines = vec![
        Line::from(Span::styled("= Base =", Style::default().fg(Color::Green))),
        Line::from(format!("  Position : ({}, {})", base.0, base.1)),
        Line::from(""),
        Line::from(Span::styled("= Ressources =", Style::default().fg(Color::Yellow))),
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
        Line::from(format!("  Ressources: {}", app.known_resources.len())),
        Line::from(format!("  Obstacles:  {}", app.known_obstacles.len())),
        Line::from(""),
        Line::from(Span::styled("= Legende =", Style::default().fg(Color::DarkGray))),
        Line::from(Span::styled("  . vide", Style::default().fg(Color::DarkGray))),
        Line::from(Span::styled("  # obstacle", Style::default().fg(Color::White))),
        Line::from(Span::styled("  E energie", Style::default().fg(Color::Yellow))),
        Line::from(Span::styled("  C cristal", Style::default().fg(Color::Cyan))),
        Line::from(Span::styled("  B base", Style::default().fg(Color::Green))),
        Line::from(""),
        Line::from(Span::styled("[q] Quitter", Style::default().fg(Color::DarkGray))),
    ];

    let stats_widget = Paragraph::new(Text::from(lines))
        .block(Block::default().borders(Borders::ALL).title(" Stats "));

    frame.render_widget(stats_widget, area);
}
