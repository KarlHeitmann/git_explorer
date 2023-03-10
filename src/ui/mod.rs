use crossterm::event::{self, Event, KeyCode};
use std::io::Stdout;
use git2::{Repository, Oid};

use crate::utils::short_id;
use crate::graph::paint_commit_track;

use tui::{
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Span, Spans, Text},
    backend::CrosstermBackend,
    widgets::{
        Block, BorderType, Borders, List, ListItem, ListState, Paragraph, Tabs, Wrap
    },

    Terminal
};

#[derive(Copy, Clone, Debug)]
pub enum MenuItem {
    Home,
    Nodes,
    Edit,
    SubSearch
}

impl From<MenuItem> for usize {
    fn from(input: MenuItem) -> usize {
        match input {
            MenuItem::Home => 0,
            MenuItem::Nodes => 1,
            MenuItem::Edit => 2,
            MenuItem::SubSearch => 3,
        }
    }
}

pub fn get_layout_chunks(size: Rect) -> Vec<Rect> {
    Layout::default()
        .direction(Direction::Vertical)
        .margin(2)
        .constraints(
            [
                Constraint::Length(3),
                Constraint::Min(2),
                Constraint::Length(3),
            ]
            .as_ref(),
        )
        .split(size)
}

pub fn draw_status_bar<'layout>() -> Paragraph<'layout> {
    let (title, color) = ("NORMAL MODE +++FILTER MODE CONTAIN+++", Color::LightCyan);

    Paragraph::new(title)
        .style(Style::default().fg(color))
        .alignment(Alignment::Center)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .style(Style::default().fg(Color::White))
                .title("Status")
                .border_type(BorderType::Plain),
        )
}

pub fn draw_menu_tabs<'a>(menu_titles: &'a Vec<&'a str>, active_menu_item: MenuItem) -> Tabs<'a> {
    let menu = menu_titles
        .iter()
        .map(|t| {
            let (first, rest) = t.split_at(1);
            Spans::from(vec![
                Span::styled(
                    first,
                    Style::default()
                        .fg(Color::Yellow)
                        .add_modifier(Modifier::UNDERLINED),
                ),
                Span::styled(rest, Style::default().fg(Color::White)),
            ])
        })
        .collect();

    Tabs::new(menu)
        .select(active_menu_item.into())
        .block(Block::default().title("Menu").borders(Borders::ALL))
        .style(Style::default().fg(Color::White))
        .highlight_style(Style::default().fg(Color::Yellow))
        .divider(Span::raw("|"))
}

pub fn render_home<'a>(node_list_state: &ListState, data: &'a Vec<(String, Oid)>, repo: &Repository) -> (List<'a>, Paragraph<'a>) {
    let style_list = Style::default().fg(Color::White);
    let nodes_block:Block = Block::default()
        .borders(Borders::ALL)
        .style(style_list)
        .title(format!("Graph"))
        .border_type(BorderType::Plain);

    let items: Vec<ListItem> = data
        .iter()
        .map(|node| {
            let grapheme = &node.0;
            let commit = repo.find_commit(node.1).unwrap();
            let text = format!("{} ({}) {} ", grapheme.clone(), short_id(commit.id()), commit.summary().unwrap());
            let text = Text::from(text);
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

    let sub_tree_oid = data.get(i).unwrap().1;

    let mut detail = String::new();
    let current_commit = repo.find_commit(sub_tree_oid).unwrap();

    let parents = current_commit.parents().map(|c| short_id(c.id())).collect::<Vec<String>>().join(" - ");

    detail.push_str(
        &format!("\n{}\nCommiter: {}\nAuthor: {}\n{}\nPARENTS:\n{}\n\n",
            current_commit.message().unwrap_or("NO COMMIT MESSAGE"),
            current_commit.committer().to_string(),
            current_commit.author(),
            short_id(current_commit.id()),
            parents,
            // current_commit.time(), // TODO add date time to commit detail
        )
    );

    let mut string_a = String::new();
    let mut string_b = String::new();

    match data.get(i+1) {
        Some((_, sub_tree_oid_previous)) => {
            let previous_commit = repo.find_commit(*sub_tree_oid_previous).unwrap();

            let my_first_diff = repo.diff_tree_to_tree(
                current_commit.tree().ok().as_ref(),
                previous_commit.tree().ok().as_ref(),
                None
            ).unwrap();

            let _foreach_result = my_first_diff.foreach(
                &mut |_, _| true,
                None,
                Some(&mut |_, hunk| {
                    let s = format!("{}\n",
                        String::from_utf8(hunk.header().to_vec()).unwrap()
                    );
                    string_a.push_str(&s);
                    true
                }),
                Some(&mut |_, _hunk, line| {
                    let s = format!("{}:{}{}",
                        line.new_lineno().unwrap_or_else(|| line.old_lineno().unwrap()),
                        line.origin().to_string(),
                        String::from_utf8(line.content().to_vec()).unwrap()
                    );
                    string_b.push_str(&s);
                    true
                }),
            );
        },
        None => {}
    }

    detail.push_str(&string_a);
    detail.push_str(&string_b);

    let node_detail = Paragraph::new(detail)
        .block(Block::default().title(format!("Commit COMPLETE {} ", sub_tree_oid)).borders(Borders::ALL))
        .style(Style::default().fg(Color::White).bg(Color::Black))
        .alignment(Alignment::Left)
        .wrap(Wrap { trim: true });

    (list, node_detail)
}

pub fn explorer_wrapper(terminal: &mut Terminal<CrosstermBackend<Stdout>>, repo: &Repository) -> Result<(), Box<dyn std::error::Error>> {
    let menu_titles = vec!["Home", "Quit"];
    let active_menu_item = MenuItem::Home;
    let mut node_list_state = ListState::default();
    let data = paint_commit_track(repo.head().unwrap().peel_to_commit().unwrap());
    node_list_state.select(Some(0));

    // let (mut percentage_left, mut percentage_right) = (60, 40);
    let (mut percentage_left, mut percentage_right) = (50, 50);

    terminal.clear()?;
    loop {
        terminal.draw(|rect| {
            let chunks = get_layout_chunks(rect.size());

            let status_bar = draw_status_bar();

            let tabs = draw_menu_tabs(&menu_titles, active_menu_item);

            rect.render_widget(tabs, chunks[0]);
            let nodes_chunks = Layout::default()
                .direction(Direction::Horizontal)
                .constraints(
                    [Constraint::Percentage(percentage_left), Constraint::Percentage(percentage_right)].as_ref(),
                )
                .split(chunks[1]);
            let (left, right) = render_home(&node_list_state, &data, &repo);
            rect.render_stateful_widget(left, nodes_chunks[0], &mut node_list_state);
            rect.render_widget(right, nodes_chunks[1]);

            rect.render_widget(status_bar, chunks[2]);
        })?;

        if let Event::Key(key) = event::read()? {
            match key.code {
                KeyCode::Char('q') => {
                    break;
                }
                KeyCode::Left => {
                    if percentage_left > 0 {
                        percentage_left -= 1;
                        percentage_right += 1;
                    }
                }
                KeyCode::Enter => {
                }
                KeyCode::Right => {
                    if percentage_right > 0 {
                        percentage_left += 1;
                        percentage_right -= 1;
                    }
                }
                KeyCode::Down => {
                    if let Some(selected) = node_list_state.selected() {
                        let amount_nodes = data.len();
                        if selected >= amount_nodes - 1 {
                            node_list_state.select(Some(0));
                        } else {
                            node_list_state.select(Some(selected + 1));
                        }
                    }
                }
                KeyCode::PageDown => {
                    if let Some(selected) = node_list_state.selected() {
                        let amount_nodes = data.len();
                        if selected >= amount_nodes - 10 {
                            node_list_state.select(Some(0));
                        } else {
                            node_list_state.select(Some(selected + 10));
                        }
                    }
                }
                KeyCode::Up => {
                    if let Some(selected) = node_list_state.selected() {
                        let amount_nodes = data.len();
                        if selected > 0 {
                            node_list_state.select(Some(selected - 1));
                        } else {
                            node_list_state.select(Some(amount_nodes - 1));
                        }
                    }
                }
                KeyCode::PageUp => {
                    if let Some(selected) = node_list_state.selected() {
                        let amount_nodes = data.len();
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

