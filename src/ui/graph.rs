use git2::Repository;

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

use crate::graph::GitExplorer;
use crate::ui::Component;

pub struct GraphComponent {
    node_list_state: ListState
}

impl Component for GraphComponent {}

impl GraphComponent {
    pub fn new() -> Self {
        let mut node_list_state = ListState::default();
        node_list_state.select(Some(0));
        Self {
            node_list_state,
        }
    }

    pub fn render_home<'a>(&self, repo: &Repository, git_explorer: &GitExplorer) -> (List<'a>, Paragraph<'a>) {
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

        let node_detail = Paragraph::new(detail)
            .block(Block::default().title(format!("Commit COMPLETE {} ", sub_tree_oid)).borders(Borders::ALL))
            .style(Style::default().fg(Color::White).bg(Color::Black))
            .alignment(Alignment::Left)
            .wrap(Wrap { trim: true });

        (list, node_detail)
    }

    pub fn render<B: Backend>(
        &mut self,
        f: &mut Frame<B>,
        percentage_left: u16, percentage_right: u16,
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
                [Constraint::Percentage(percentage_left), Constraint::Percentage(percentage_right)].as_ref(),
            )
            .split(vertical_chunks[1]);
        let (left, right) = self.render_home(repo, git_explorer);
        f.render_stateful_widget(left, nodes_chunks[0], &mut self.node_list_state);
        f.render_widget(right, nodes_chunks[1]);
    }
}


