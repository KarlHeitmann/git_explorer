use git2::Repository;
use tui::{
    layout::{Alignment, Rect},
    text::Spans,
    style::{Color, Style},
    terminal::Frame,
    widgets::{
        Block, Borders, Paragraph, Wrap
    },
    backend::Backend,
};

use crate::ui::Component;
// use crossterm::event::Event;
use crate::graph::GitExplorer;
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
        git_explorer: &GitExplorer
        ) {
        let text = Spans::from(git_explorer.branches_strings());
        let p = Paragraph::new(text)
            .block(Block::default().title(format!("Commit COMPLETE")).borders(Borders::ALL))
            .style(Style::default().fg(Color::White).bg(Color::Black))
            .alignment(Alignment::Left)
            .wrap(Wrap { trim: true });

        f.render_widget(p, rect);

    }
}

impl Component for BranchesComponent {
	fn event(&mut self, key_code: KeyCode, git_explorer: &mut GitExplorer) -> Result<String, String> {
        match key_code {
            KeyCode::Tab => {
                // TODO: Reset selected to zero to prevent bug when attempting to look at a
                // commit that there is not anymore
                git_explorer.update_graph(1);
            }
            KeyCode::BackTab => {
                git_explorer.update_graph(-1);
            }
            _ => {}
        }
        Ok(String::from("ok"))
    }
}

