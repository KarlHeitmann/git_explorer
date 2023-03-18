use git2::Repository;

use tui::{
    layout::Alignment,
    style::{Color, Modifier, Style},
    text::Text,
    widgets::{
        Block, BorderType, Borders, List, ListItem, ListState, Paragraph, Wrap
    }
};

use crate::graph::GitExplorer;

pub fn render_home<'a>(node_list_state: &ListState, repo: &Repository, git_explorer: &GitExplorer) -> (List<'a>, Paragraph<'a>) {
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

    let i = node_list_state.selected().expect("there is always a selected node");

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
