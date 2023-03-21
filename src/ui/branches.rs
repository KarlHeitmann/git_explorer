use git2::Repository;
use tui::{
    layout::{Alignment, Constraint, Direction, Rect, Layout},
    text::{Span, Spans},
    style::{Color, Style},
    terminal::Frame,
    widgets::{
        Block, Borders, Paragraph, Wrap
    },
    backend::Backend,
};

use crate::ui::Component;
// use crossterm::event::Event;
use crate::explorer::GitExplorer;
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
        git_explorer: &GitExplorer,
        repo: &Repository,
        ) {
        let vertical_chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints(
                [Constraint::Length(5), Constraint::Min(5)].as_ref()
            )
            .split(rect);

        // let text = Spans::from(git_explorer.branches_strings());
        let mut text = vec![Spans::from(git_explorer.branches_strings())];
        
        let head = repo.head().unwrap();
        let current_commit = head.peel_to_commit().unwrap();

        text.push(
            Spans::from(vec![
                Span::styled(format!("HEAD: {}", head.shorthand().unwrap()), Style::default().fg(Color::White))
            ])
        );
        text.push(
            Spans::from(vec![
                Span::styled(format!("oid: {}", current_commit.id()), Style::default().fg(Color::White))
            ])
        );

        /*
        let text_2 = format!(
            "\nHEAD: {}\noid: {}",
            head.shorthand().unwrap(),
            current_commit.id(),
        );
        */

        let p1 = Paragraph::new(text)
            .block(Block::default().title(format!("Commit COMPLETE")).borders(Borders::ALL))
            .style(Style::default().fg(Color::White).bg(Color::Black))
            .alignment(Alignment::Left)
            .wrap(Wrap { trim: true });

        // .shorthand().unwrap();
        // let text_2 = repo.head().unwrap().peel_to_commit().unwrap();

        let compared_commit_oid = git_explorer.get_selected_branch_oid();

        let parsed_diff = git_explorer.diff_commit_by_id(current_commit.clone(), compared_commit_oid);


        // let p2 = Paragraph::new(String::from(text_2))
        let p2 = Paragraph::new(parsed_diff.test_lines)
            .block(Block::default().title(format!("Commit COMPLETE")).borders(Borders::ALL))
            .style(Style::default().fg(Color::White).bg(Color::Black))
            .alignment(Alignment::Left)
            .wrap(Wrap { trim: true });

        // let detail = git_explorer.diff_commit(current_commit, i+1);

        f.render_widget(p1, vertical_chunks[0]);
        f.render_widget(p2, vertical_chunks[1]);

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

