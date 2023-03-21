use git2::Repository;
use crossterm::event::{self, Event, KeyCode};

use tui::{
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Spans, Text},
    terminal::Frame,
    widgets::{
        Block, BorderType, Borders, List, ListItem, ListState, Paragraph, Wrap
    },
    backend::Backend,
};

use crate::explorer::GitExplorer;
use crate::ui::Component;

pub struct GraphComponent {
    node_list_state: ListState,
    percentage_left: u16,
    percentage_right: u16,
    diff_offset: usize,
}

impl Component for GraphComponent {
	// fn event(&mut self, ev: &Event, git_explorer: &GitExplorer) -> Result<String, String> {
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
            /*
            KeyCode::Char('q') => {
                break;
            }
            */
            KeyCode::Left => {
                if self.percentage_left > 0 {
                    self.percentage_left -= 1;
                    self.percentage_right += 1;
                }
            }
            KeyCode::Right => {
                if self.percentage_right > 0 {
                    self.percentage_left += 1;
                    self.percentage_right -= 1;
                }
            }
            KeyCode::Char('j') => {
                // TODO: protect increment of diff_offset to not overflow diff lines in git_explorer.diff_commit() -> ParsedDiff
                self.diff_offset += 1;
            }
            KeyCode::Char('k') => {
                if self.diff_offset > 0 {
                    self.diff_offset -= 1;
                }
            }
            KeyCode::Down => {
                if let Some(selected) = self.node_list_state.selected() {
                    let amount_nodes = git_explorer.get_nodes_len();
                    if selected >= amount_nodes - 1 {
                        self.node_list_state.select(Some(0));
                    } else {
                        self.node_list_state.select(Some(selected + 1));
                    }
                    self.diff_offset = 0;
                }
            }
            KeyCode::Enter => {
                // TODO: restore this feature: when hitting enter on a commit, spawns a nes
                // instance recursively with root commit the commit under cursor
                // let selected = node_list_state.selected().unwrap();
                // let sub_tree_oid = data.get(selected).unwrap().id();
                // let sub_tree_oid = git_explorer.get_node_id(selected).unwrap();
                // let current_commit = repo.find_commit(sub_tree_oid).unwrap();
                // explorer_wrapper(terminal, repo, current_commit, None)?; // TODO: Add stop condition on recursion
                // explorer_wrapper(terminal, repo, None)?; // TODO: Add stop condition on recursion
            }
            KeyCode::PageDown => {
                if let Some(selected) = self.node_list_state.selected() {
                    let amount_nodes = git_explorer.get_nodes_len();
                    if selected >= amount_nodes - 10 {
                        self.node_list_state.select(Some(0));
                    } else {
                        self.node_list_state.select(Some(selected + 10));
                    }
                    self.diff_offset = 0;
                }
            }
            KeyCode::Up => {
                if let Some(selected) = self.node_list_state.selected() {
                    let amount_nodes = git_explorer.get_nodes_len();
                    if selected > 0 {
                        self.node_list_state.select(Some(selected - 1));
                    } else {
                        self.node_list_state.select(Some(amount_nodes - 1));
                    }
                    self.diff_offset = 0;
                }
            }
            KeyCode::PageUp => {
                if let Some(selected) = self.node_list_state.selected() {
                    let amount_nodes = git_explorer.get_nodes_len();
                    if selected > 10 {
                        self.node_list_state.select(Some(selected - 10));
                    } else {
                        self.node_list_state.select(Some(amount_nodes - 1));
                    }
                    self.diff_offset = 0;
                }
            }
            _ => {}
        }
        Ok(String::from("ok"))
    }
}

impl GraphComponent {
    pub fn new() -> Self {
        let mut node_list_state = ListState::default();
        node_list_state.select(Some(0));
        let (percentage_left, percentage_right) = (50, 50);
        Self {
            node_list_state,
            percentage_left, percentage_right,
            diff_offset: 0,
        }
    }

    pub fn render_home<'a>(&self, repo: &Repository, git_explorer: &'a GitExplorer) -> (List<'a>, Paragraph<'a>) {
    // pub fn render_home<'a>(&self, repo: &Repository, git_explorer: &'a GitExplorer) -> (List<'a>, Text<'a>) {
        let style_list = Style::default().fg(Color::White);
        let nodes_block:Block = Block::default()
            .borders(Borders::ALL)
            .style(style_list)
            .title(format!("Graph"))
            .border_type(BorderType::Plain);

        let items: Vec<ListItem> = git_explorer.nodes
            .iter()
            .map(|node| {
                // let text = Text::from(node.clone());
                let text = Text::from(node);
                // let text = Spans::from(node);
                let l = ListItem::new(text);
                l
            })
            .collect();

        let list = List::new(items).block(nodes_block).highlight_style(
            Style::default()
                .bg(Color::Yellow)
                .fg(Color::Black)
                .add_modifier(Modifier::BOLD),
        );

        let i = self.node_list_state.selected().expect("there is always a selected node");

        // let sub_tree_oid = data.get(i).unwrap().id();
        let sub_tree_oid = git_explorer.get_node_id(i).unwrap();

        let current_commit = repo.find_commit(sub_tree_oid).unwrap();

        // let detail = git_explorer.diff_commit(current_commit, &data.get(i+1));
        let detail = git_explorer.diff_commit(current_commit, i+1);

        let spans_to_build = &detail.test_lines[self.diff_offset..].to_owned();

        let node_detail = Paragraph::new(spans_to_build.clone())
            .block(Block::default().title(format!("Commit COMPLETE {} ", sub_tree_oid)).borders(Borders::ALL))
            .style(Style::default().fg(Color::White).bg(Color::Black))
            .alignment(Alignment::Left)
            .wrap(Wrap { trim: true });

        (list, node_detail)
    }

    pub fn render<B: Backend>(
        &mut self,
        f: &mut Frame<B>,
        chunks: &mut Vec<Rect>,
        git_explorer: &GitExplorer,
        repo: &Repository,
        ) {
        let vertical_chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints(
                [Constraint::Length(3), Constraint::Min(5)].as_ref()
            )
            .split(chunks[1]);

        let text = Spans::from(git_explorer.branches_strings());

        let paragraph = Paragraph::new(text);
        f.render_widget(paragraph, vertical_chunks[0]);

        let nodes_chunks = Layout::default()
            .direction(Direction::Horizontal)
            .constraints(
                [Constraint::Percentage(self.percentage_left), Constraint::Percentage(self.percentage_right)].as_ref(),
            )
            .split(vertical_chunks[1]);
        let (left, right) = self.render_home(repo, git_explorer);
        f.render_stateful_widget(left, nodes_chunks[0], &mut self.node_list_state);
        f.render_widget(right, nodes_chunks[1]);
    }
}


