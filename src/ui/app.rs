use crossterm::event::{self, Event, KeyCode};
use git2::Repository;

use tui::{
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    terminal::Frame,
    Terminal,
    text::{Span, Spans, Text},
    backend::Backend,
    widgets::{
        Block, BorderType, Borders, ListState, Paragraph, Tabs
    },
};

use crate::ui::Component;
use crate::ui::branches::BranchesComponent;
use crate::graph::GitExplorer;
use crate::ui::graph::GraphComponent;

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

fn get_layout_chunks(size: Rect) -> Vec<Rect> {
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

fn draw_status_bar<'layout>() -> Paragraph<'layout> {
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

fn draw_menu_tabs<'a>(menu_titles: &'a Vec<&'a str>, active_menu_item: MenuItem) -> Tabs<'a> {
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

pub struct App {
    // terminal: Terminal<B>,
    node_list_state: ListState,
    branches_component: BranchesComponent,
    graph_component: GraphComponent,
}

impl App {
    pub fn new() -> Self {
        let mut node_list_state = ListState::default();
        node_list_state.select(Some(0));
        let branches_component = BranchesComponent::new();
        let graph_component = GraphComponent::new();
        Self { 
            node_list_state,
            graph_component,
            branches_component,
        }
    }

    pub fn run<B: Backend>(
    //     f: &mut Frame<B>,
    // pub fn app(
        &mut self,
        terminal: &mut Terminal<B>,
        git_explorer: &mut GitExplorer,
        repo: &Repository,) -> Result<(), Box<dyn std::error::Error>> {

        let (mut percentage_left, mut percentage_right) = (50, 50);
        let mut tab_index = 0;

        let menu_titles = vec!["Home", "Quit"];
        let active_menu_item = MenuItem::Home;
        loop {
            terminal.draw(|f| {
                let mut chunks = get_layout_chunks(f.size());

                let status_bar = draw_status_bar();

                let tabs = draw_menu_tabs(&menu_titles, active_menu_item);

                f.render_widget(tabs, chunks[0]);

                match tab_index {
                    // 0 => wrapper(f, percentage_left, percentage_right, &mut self.node_list_state, &mut chunks, &git_explorer, repo),
                    // 0 => wrapper(f, percentage_left, percentage_right, &mut self.node_list_state, &mut chunks, &git_explorer, repo),
                    0 => self.graph_component.render(f, percentage_left, percentage_right, &mut chunks, &git_explorer, repo),
                    // 1 => render_branches(f, &mut chunks),
                    1 => self.branches_component.render(f, chunks[1]),
                    _ => {},
                }
                // wrapper(f, percentage_left, percentage_right, node_list_state, &mut chunks, &git_explorer, repo);
                // render_branches(f, &mut chunks);

                f.render_widget(status_bar, chunks[2]);
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
                    KeyCode::Char('1') => {tab_index = 0}
                    KeyCode::Char('2') => {tab_index = 1}
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
                        if let Some(selected) = self.node_list_state.selected() {
                            let amount_nodes = git_explorer.get_nodes_len();
                            if selected >= amount_nodes - 1 {
                                self.node_list_state.select(Some(0));
                            } else {
                                self.node_list_state.select(Some(selected + 1));
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
                        }
                    }
                    _ => {}
                }
            }
        }
        Ok(())
    }

    fn get_tabs(&mut self) -> Vec<&mut dyn Component> {
        vec![
            &mut self.branches_component,
            &mut self.graph_component,
        ]
        /*
        vec![
            &mut self.status_tab,
            &mut self.revlog,
            &mut self.files_tab,
            &mut self.stashing_tab,
            &mut self.stashlist_tab,
        ]
            */
    }
}

