use tui::{
    layout::{Alignment, Rect},
    style::{Color, Style},
    terminal::Frame,
    widgets::{
        Block, Borders, Paragraph, Wrap
    },
    backend::Backend,
};

use crate::ui::Component;
use crate::graph::GitExplorer;
// use crossterm::event::Event;
use crossterm::event::{self, Event, KeyCode};

pub struct BranchesComponent {
    paragraph_title: String,
}

impl BranchesComponent {
    // pub const fn new() -> Self {
    pub fn new() -> Self {
        let paragraph_title = String::from("Branches title");
        Self {
            paragraph_title,
        }
    }

    pub fn render<B: Backend>(
        &self,
        f: &mut Frame<B>,
        // chunks: &mut Vec<Rect>,
        // rect: &mut Rect,
        rect: Rect,
        ) {
        let p = Paragraph::new("Branches")
            .block(Block::default().title(format!("Commit COMPLETE")).borders(Borders::ALL))
            .style(Style::default().fg(Color::White).bg(Color::Black))
            .alignment(Alignment::Left)
            .wrap(Wrap { trim: true });

        f.render_widget(p, rect);

    }
}

impl Component for BranchesComponent {
	fn event(&mut self, key_code: KeyCode, git_explorer: &mut GitExplorer) -> Result<String, String> { Ok(String::from("ok")) }
}

