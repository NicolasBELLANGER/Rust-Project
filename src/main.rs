mod app;
mod base;
mod map;
mod messages;
mod robot;
mod ui;

use std::io;
use std::sync::mpsc;
use std::thread;
use std::time::{Duration, Instant};

use crossterm::{
    event::{self, Event},
    execute,
    terminal::{EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode},
};
use ratatui::{Terminal, backend::CrosstermBackend};

use app::new_shared;
use messages::{ResourceKind, RobotMessage};
use robot::{Collector, Scout};

const NUM_SCOUTS: usize = 5;
const NUM_COLLECTORS: usize = 2;

fn main() -> io::Result<()> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;

    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let shared = new_shared(20, 20);

    // Initialiser les positions des scouts à la base, puis les spawner
    let (tx, rx) = mpsc::channel::<RobotMessage>();
    {
        let mut app = shared.lock().unwrap();
        let base_pos = app.base.pos;
        app.scouts = vec![base_pos; NUM_SCOUTS];
    }

    for id in 0..NUM_SCOUTS {
        let shared_clone = shared.clone();
        let tx_clone = tx.clone();
        let base_pos = shared.lock().unwrap().base.pos;
        thread::spawn(move || {
            Scout::new(id, base_pos).run(shared_clone, tx_clone);
        });
    }

    // Initialiser et spawner les collecteurs
    {
        let mut app = shared.lock().unwrap();
        let base_pos = app.base.pos;
        app.collectors = vec![base_pos; NUM_COLLECTORS];
    }

    for id in 0..NUM_COLLECTORS {
        let shared_clone = shared.clone();
        let tx_clone = tx.clone();
        let base_pos = shared.lock().unwrap().base.pos;
        thread::spawn(move || {
            Collector::new(id, base_pos).run(shared_clone, tx_clone);
        });
    }

    let tick_rate = Duration::from_millis(100);
    let mut last_tick = Instant::now();

    loop {
        // Traiter tous les messages en attente des robots
        while let Ok(msg) = rx.try_recv() {
            let mut app = shared.lock().unwrap();
            match msg {
                RobotMessage::ResourceFound { pos, kind } => {
                    let already_known = app
                        .known_resources
                        .iter()
                        .any(|(x, y, _)| *x == pos.0 && *y == pos.1);
                    if !already_known {
                        app.known_resources.push((pos.0, pos.1, kind));
                    }
                }
                RobotMessage::ObstacleFound { pos } => {
                    if !app.known_obstacles.contains(&(pos.0, pos.1)) {
                        app.known_obstacles.push(pos);
                    }
                }
                RobotMessage::ResourceCollected { amount, kind, .. } => match kind {
                    ResourceKind::Energy => app.collected_energy += amount,
                    ResourceKind::Crystal => app.collected_crystals += amount,
                },
                RobotMessage::GoCollect { .. } => {}
            }
        }

        {
            let app = shared.lock().unwrap();
            terminal.draw(|f| ui::draw(f, &app))?;
        }

        let timeout = tick_rate.saturating_sub(last_tick.elapsed());
        if event::poll(timeout)? {
            if let Event::Key(_) = event::read()? {
                shared.lock().unwrap().active = false;
                break;
            }
        }

        if last_tick.elapsed() >= tick_rate {
            last_tick = Instant::now();
        }
    }

    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    terminal.show_cursor()?;

    Ok(())
}
