use git2::{Repository, BranchType};
use crossterm::event::KeyCode;
use log::{trace, debug};

use tui::{
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::Spans,
    terminal::Frame,
    widgets::{
        Block, BorderType, Borders, List, ListItem, ListState, Paragraph, Wrap, Clear,
    },
    backend::Backend,
};

use crate::explorer::GitExplorer;
use crate::ui::Component;

use self::action_key::ActionKey;

use super::centered_rect_absolute;

pub mod action_key;

pub struct GraphComponent<'a> {
    node_list_state: ListState,
    percentage_left: u16,
    percentage_right: u16,
    diff_offset: usize,
    help_toggled: bool,
    action_key: ActionKey<'a>,
    edit_mode: bool,
    filter_string: String,
}

impl Component for GraphComponent<'_> {
	// fn event(&mut self, ev: &Event, git_explorer: &GitExplorer) -> Result<String, String> {
    fn command_mode_event(&mut self, key_code: KeyCode, git_explorer: &mut GitExplorer) -> Result<String, String> {
        match key_code {
            KeyCode::Char('i') => {
                // (self.action_key.git_explorer_action)(String::from("ups I did it again"));
                self.edit_mode = true;
            }
            KeyCode::Char('u') => {
                // (self.action_key.git_explorer_action)(String::from("ups I did it again"));
                (self.action_key.git_explorer_action)(git_explorer);
            }
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
            KeyCode::Char(' ') => {
                let selected = self.node_list_state.selected();
                git_explorer.stop_branch(selected)
                // let sub_tree_oid = git_explorer.get_node_id(selected).unwrap();
                // let current_commit = repo.find_commit(sub_tree_oid).unwrap();
                // explorer_wrapper(terminal, repo, current_commit, None)?; // TODO: Add stop condition on recursion
            }
            KeyCode::Char('?') => {
                self.help_toggled = !self.help_toggled;
                trace!("HELP! {}", self.help_toggled);

            }
            KeyCode::Down => {
                if let Some(selected) = self.node_list_state.selected() {
                    let amount_nodes = git_explorer.get_nodes_len();
                    let node = git_explorer.get_node_id(selected);
                    if selected >= amount_nodes - 1 {
                        trace!("DOWN");
                        // debug!("{:?} - parents: ", node, node.unwrap().parents());
                        trace!("DOWN");
                        self.node_list_state.select(Some(0));
                    } else {
                        trace!("DOWN");
                        debug!("{:?}", node);
                        trace!("DOWN");
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
	fn event(&mut self, key_code: KeyCode, git_explorer: &mut GitExplorer) -> Result<String, String> {
        if self.edit_mode {
            match key_code {
                KeyCode::Esc|KeyCode::F(2) => { self.edit_mode = false } // Gets traped in vim
                KeyCode::Char(c) => {
                    self.filter_string.push(c);
                },
                KeyCode::Backspace => {
                    self.filter_string.pop();
                }
                _ => {}
            }

        } else {
            self.command_mode_event(key_code, git_explorer)?;
        }
        Ok(String::from("ok"))
    }
}

impl GraphComponent<'_> {
    pub fn new() -> Self {
        let mut node_list_state = ListState::default();
        node_list_state.select(Some(0));
        let (percentage_left, percentage_right) = (50, 50);
        let action_key = ActionKey::new(KeyCode::Char('k'));
        Self {
            node_list_state,
            percentage_left, percentage_right,
            diff_offset: 0,
            help_toggled: false,
            action_key,
            edit_mode: false,
            filter_string: String::new(),
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

        let items: Vec<ListItem> = git_explorer.nodes()
            .iter()
            .map(|node| node.into())
            .collect();

        let list = List::new(items).block(nodes_block).highlight_style(
            Style::default()
                .bg(Color::Yellow)
                .fg(Color::Black)
                .add_modifier(Modifier::BOLD),
        );

        let i = self.node_list_state.selected().expect("there is always a selected node");

        // let sub_tree_oid = data.get(i).unwrap().id();
        // let sub_tree_oid = git_explorer.get_node_id(i).unwrap();
        // let sub_tree_oid = git_explorer.get_node_id(i);

        // let a = repo.find_commit(sub_tree_oid)
        // let current_commit = repo.find_commit(sub_tree_oid).unwrap();
        // let current_commit = repo.find_commit(sub_tree_oid);
        // match repo.find_commit(sub_tree_oid) {
        match git_explorer.get_node_id(i) {
            Some(sub_tree_oid) => {
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
            },
            None => {
                let node_detail = Paragraph::new("bla bla bla")
                    .block(Block::default().title(format!("Commit COMPLETE ")).borders(Borders::ALL))
                    .style(Style::default().fg(Color::White).bg(Color::Black))
                    .alignment(Alignment::Left)
                    .wrap(Wrap { trim: true });
                (list, node_detail)
            }
        }
    }

    pub fn render<B: Backend>(
        &mut self,
        f: &mut Frame<B>,
        chunks: &mut Vec<Rect>,
        git_explorer: &GitExplorer,
        repo: &Repository,
        ) {

        if self.help_toggled {
            const SIZE: (u16, u16) = (65, 24);
            // let scroll_threshold = SIZE.1 / 3;
            // let scroll =
            //     self.selection.saturating_sub(scroll_threshold);


            let width = SIZE.0;
            let height = SIZE.1;
            let rect = centered_rect_absolute(width, height, f.size());
            f.render_widget(Clear, rect);
            f.render_widget(
                Block::default()
                    // .title(strings::help_title(&self.key_config))
                    .title("HELP")
                    .borders(Borders::ALL)
                    .border_type(BorderType::Thick),
                rect,
            );
			let chunks = Layout::default()
				.vertical_margin(1)
				.horizontal_margin(1)
				.direction(Direction::Vertical)
				.constraints(
					[Constraint::Min(1), Constraint::Length(1)]
						.as_ref(),
				)
				.split(rect);

			f.render_widget(
				Paragraph::new("qweewq")
				// Paragraph::new(self.get_text())
					// .scroll((scroll, 0))
					.alignment(Alignment::Left),
				chunks[0],
			);

			f.render_widget(
				Paragraph::new("asddsa")
				// Paragraph::new(self.get_text())
					// .scroll((scroll, 0))
					.alignment(Alignment::Left)
                    /*
				Paragraph::new(Spans::from(vec![Span::styled(
					Cow::from(format!("gitui {}", Version::new(),)),
					Style::default(),
				)]))
                */
				.alignment(Alignment::Right),
				chunks[1],
			);



        } else {
            let vertical_chunks = Layout::default()
                .direction(Direction::Vertical)
                .constraints(
                    [Constraint::Length(3), Constraint::Min(5)].as_ref()
                )
                .split(chunks[1]);

            // let text = Spans::from(git_explorer.branches_strings()); // FIXME // TODO This has the branches matching current pattern

            let branches: Vec<Spans> = git_explorer.branches(None, Some(&self.filter_string))
            // let branches: Vec<Spans> = git_explorer.branches(Some(BranchType::Local))
            // let branches: Vec<Spans> = git_explorer.branches(Some(BranchType::Remote))
                .iter()
                .map(|b| Spans::from(format!("{}", b)))
                .collect();
            // let text = [vec![Spans::from("count: ")], branches].concat();
            let text = [vec![Spans::from(format!("count: {}", branches.len()))], branches].concat();
            // let branches = Spans::from(git_explorer.branches(None));

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
}


