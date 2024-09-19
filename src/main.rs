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
