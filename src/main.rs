use std:: {
    collections::{BTreeMap, VecDeque},
    sync::mpsc,
    thread,
    time::{Duration, Instant},
};

use color_eyre::Result;
use rand::distributions::{Distribution, Uniform};
use ratatui::{
    backend::Backend,
    crossterm::event,
    layout::{Alignment, Constraint, Layout, Rect},
    style::{Color, Modifier, Style},
    symbols,
    text::{Line, Span},
    widgets::{block, Block, Gauge, LineGauge, List, ListItem, Paragraph, Widget},
    Frame, Terminal, TerminalOptions, Viewport,
};


fn main() -> Result<()> {
    color_eyre::install()?;
    let mut terminal = ratatui::init_with_options(TerminalOptions {
       viewport: Viewport::Inline(8)
    });

    let (tx, rx) = mpsc::channel();

}

const NUM_DOWNLOADS: usize = 10;

type DownloadId = usize;
type WorkerId = usize;
enum Event {
    Input(event::KeyEvent),
    Tick,
    Resize,
    DownloadUpdate(WorkerId, DownloadId, f64),
    DownloadDone(WorkerId, DownloadId),
}
struct Downloads {
    pending: VecDeque<Download>,
    in_progress: BTreeMap<WorkerId, DownloadInProgress>,
}


impl Downloads {
    fn next(&mut self, worker_id: WorkerId) -> Option<Download> {
        match self.pending.pop_front() {
            Some(d) => {
                self.in_progress.insert(
                    worker_id,
                    DownloadInProgress {
                        id: d.id,
                        started_at: Instant::now(),
                        progress: 0.0,
                    },
                );
                Some(d)
            }
            None => None,
        }
    }
}

struct DownloadInProgress {
    id: DownloadId,
    started_at: Instant,
    progress: f64,
}

struct Download {
    id: DownloadId,
    size: usize,
}

struct Worker {
    id: WorkerId,
    tx: mpsc::Sender<Download>,
}

fn input_handling(tx: mpsc::Sender<Event>) {
    let tick_rate = Duration::from_millis(200);
    thread::spawn(move || {
        let mut last_tick = Instant::now();
        loop {
            let timeout = tick_rate.saturating_sub(last_tick.elapsed());
            if event::poll(timeout).unwrap() {
                match event::read().unwrap() {
                    event::Event::Key(key) => tx.send(Event::Input(key)).unwrap(),
                    event::Event::Resize(_, _) => tx.send(Event::Resize).unwrap(),
                    _ => {}
                };
            }
            if last_tick.elapsed() >= tick_rate {
                tx.send(Event::Tick).unwrap();
                last_tick = Instant::now();
            }
        }
    });
}

#[allow(clippy::cast_precision_loss, clippy::needless_pass_by_value)]
fn workers(tx: mpsc::Sender<Event>) -> Vec<Worker> {
    (0..4)
        .map(|id| {
            let (worker_tx, worker_rx) = mpsc::channel::<Download>();
            let tx = tx.clone();
            thread::spawn(move || {
                while let Ok(download) = worker_rx.recv() {
                    let mut remaining = download.size;
                    while remaining > 0 {
                        let wait = (remaining as u64).min(10);
                        thread::sleep(Duration::from_millis(wait * 10));
                        remaining = remaining.saturating_sub(10);
                        let progress = (download.size - remaining) * 100 / download.size;
                        tx.send(Event::DownloadUpdate(id, download.id, progress as f64))
                            .unwrap();
                    }
                    tx.send(Event::DownloadDone(id, download.id)).unwrap();
                }
            });
            Worker { id, tx: worker_tx }
        })
        .collect()
}

fn downloads() -> Downloads {
    let distribution = Uniform::new(0, 1000);
    let mut rng = rand::thread_rng();
    let pending = (0..NUM_DOWNLOADS)
        .map(|id| {
            let size = distribution.sample(&mut rng);
            Download { id, size }
        })
        .collect();
    Downloads {
        pending,
        in_progress: BTreeMap::new(),
    }
}
