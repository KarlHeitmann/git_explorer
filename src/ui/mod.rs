use crossterm::event::{self, Event, KeyCode};
use git2::{Repository, Oid};

use crate::graph::GraphNode;
use crate::{utils::short_id, graph::GitExplorer};

mod home;
mod app;

use tui::{
    text::{Spans, Text},
    backend::Backend,
    widgets::ListState,
    Terminal
};

impl From<&GraphNode> for Spans<'_> {
    fn from(graph_node: &GraphNode) -> Self {
        let (grapheme, oid, branch_shorthand, summary) = (&graph_node.grapheme, graph_node.oid, &graph_node.branch_shorthand, &graph_node.summary);
        let branch_shorthand = match branch_shorthand {
            Some(b) => format!("[{}] ", b.to_string()),
            None => String::new()
        };
        Spans::from(
            {
                match grapheme.split_once("\n") {
                    Some((g1, g_right)) => format!("{} ({}) {}{}\n{}", g1, short_id(oid), branch_shorthand, summary, g_right),
                    None => format!("{} ({}) {}{}", grapheme, short_id(oid), branch_shorthand, summary),
                }
            }
        )
    }
}
 
impl From<&GraphNode> for Text<'_> {
    fn from(graph_node: &GraphNode) -> Self {
        let (grapheme, oid, branch_shorthand, summary) = (&graph_node.grapheme, graph_node.oid, &graph_node.branch_shorthand, &graph_node.summary);
        let branch_shorthand = match branch_shorthand {
            Some(b) => format!("[{}] ", b.to_string()),
            None => String::new()
        };
        Text::from(
            {
                match grapheme.split_once("\n") {
                    Some((g1, g_right)) => format!("{} ({}) {}{}\n{}", g1, short_id(oid), branch_shorthand, summary, g_right),
                    None => format!("{} ({}) {}{}", grapheme, short_id(oid), branch_shorthand, summary),
                }
            }
        )
    }
}
 



// fn run_app<B: Backend>(terminal: &mut Terminal<B>) -> io::Result<()> {

// pub fn explorer_wrapper<B: Backend>(terminal: &mut Terminal<B>, repo: &Repository, root_commit: Commit, stop_condition: Option<(Oid, String)>) -> Result<(), Box<dyn std::error::Error>> {
pub fn explorer_wrapper<B: Backend>(terminal: &mut Terminal<B>, repo: &Repository, stop_condition: Option<(Oid, String)>) -> Result<(), Box<dyn std::error::Error>> {
    let mut node_list_state = ListState::default();
    let mut git_explorer = GitExplorer::new(None, None, stop_condition.clone()); // TARGET
    git_explorer.run();
    node_list_state.select(Some(0));

    // let (mut percentage_left, mut percentage_right) = (60, 40);
    let (mut percentage_left, mut percentage_right) = (50, 50);

    terminal.clear()?;
    loop {
        terminal.draw(|rect| {
            app::app(rect, &mut node_list_state, &git_explorer, repo, percentage_left, percentage_right);
        })?;

        if let Event::Key(key) = event::read()? {
            match key.code {
                KeyCode::Tab => {
                    // TODO: Reset selected to zero to prevent bug when attempting to look at a
                    // commit that there is not anymore
                    git_explorer.update_graph(1);
                }
                KeyCode::BackTab => {
                    git_explorer.update_graph(-1);
                }
                KeyCode::Char('q') => {
                    break;
                }
                KeyCode::Left => {
                    if percentage_left > 0 {
                        percentage_left -= 1;
                        percentage_right += 1;
                    }
                }
                KeyCode::Right => {
                    if percentage_right > 0 {
                        percentage_left += 1;
                        percentage_right -= 1;
                    }
                }
                KeyCode::Down => {
                    if let Some(selected) = node_list_state.selected() {
                        let amount_nodes = git_explorer.get_nodes_len();
                        if selected >= amount_nodes - 1 {
                            node_list_state.select(Some(0));
                        } else {
                            node_list_state.select(Some(selected + 1));
                        }
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
                    explorer_wrapper(terminal, repo, None)?; // TODO: Add stop condition on recursion
                }
                KeyCode::PageDown => {
                    if let Some(selected) = node_list_state.selected() {
                        let amount_nodes = git_explorer.get_nodes_len();
                        if selected >= amount_nodes - 10 {
                            node_list_state.select(Some(0));
                        } else {
                            node_list_state.select(Some(selected + 10));
                        }
                    }
                }
                KeyCode::Up => {
                    if let Some(selected) = node_list_state.selected() {
                        let amount_nodes = git_explorer.get_nodes_len();
                        if selected > 0 {
                            node_list_state.select(Some(selected - 1));
                        } else {
                            node_list_state.select(Some(amount_nodes - 1));
                        }
                    }
                }
                KeyCode::PageUp => {
                    if let Some(selected) = node_list_state.selected() {
                        let amount_nodes = git_explorer.get_nodes_len();
                        if selected > 10 {
                            node_list_state.select(Some(selected - 10));
                        } else {
                            node_list_state.select(Some(amount_nodes - 1));
                        }
                    }
                }
                _ => {}
            }

        }
    }

    Ok(())
}

