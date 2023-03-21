use git2::{Repository, Oid};

// use crossterm::event::Event;
use crossterm::event::{self, Event, KeyCode};
use crate::ui::app::App;
use crate::graph::GraphNode;
use crate::{utils::short_id, graph::GitExplorer};

mod graph;
mod app;
mod branches;

use tui::{
    text::{Spans, Text},
    layout::Rect,
    backend::Backend,
    terminal::Frame,
    widgets::ListState,
    Terminal
};

pub trait DrawableComponent {
	///
	fn draw<B: Backend>(
		&self,
		f: &mut Frame<B>,
		rect: Rect,
	);
	// ) -> Result<()>; // TODO implement some Result
}



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
    terminal.clear()?;
    let mut app = app::App::new();
    app.run(terminal, &mut git_explorer, repo)?;
    // app::app(terminal, &mut node_list_state, &mut git_explorer, repo);


    Ok(())
}

pub trait Component {
	fn event(&mut self, ev: KeyCode, git_explorer: &mut GitExplorer) -> Result<String, String>;
}
/*
pub trait Component {
	///
	fn commands(
		&self,
		out: &mut Vec<CommandInfo>,
		force_all: bool,
	) -> CommandBlocking;

	///
	fn event(&mut self, ev: &Event) -> Result<EventState>;

	///
	fn focused(&self) -> bool {
		false
	}
	/// focus/unfocus this component depending on param
	fn focus(&mut self, _focus: bool) {}
	///
	fn is_visible(&self) -> bool {
		true
	}
	///
	fn hide(&mut self) {}
	///
	fn show(&mut self) -> Result<()> {
		Ok(())
	}

	///
	fn toggle_visible(&mut self) -> Result<()> {
		if self.is_visible() {
			self.hide();
			Ok(())
		} else {
			self.show()
		}
	}
}
*/


