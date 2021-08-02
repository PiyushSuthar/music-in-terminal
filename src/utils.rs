use std::io::Stdout;
use tui::{
    backend::CrosstermBackend,
    style::{Color, Style},
    symbols::line::CROSS,
    text::Spans,
    widgets::{Block, Borders, Tabs},
    Frame,
};

use crate::app::App;

pub fn draw_ui(f: &mut Frame<CrosstermBackend<Stdout>>, app: &mut App) {
    let titles = ["Home [0]", "About [1]", "Contact [2]", "Projects [4]"]
        .iter()
        .cloned()
        .map(Spans::from)
        .collect();

    let tabs = Tabs::new(titles)
        .block(Block::default().title("Hello World").borders(Borders::ALL))
        .style(Style::default().fg(Color::White))
        .highlight_style(Style::default().fg(Color::Yellow))
        .divider(CROSS)
        .select(app.current);
    f.render_widget(tabs, f.size());
}
