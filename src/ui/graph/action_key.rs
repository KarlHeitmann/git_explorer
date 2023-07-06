use crossterm::event::KeyCode;

use crate::explorer::GitExplorer;

enum ActionTypes {
    GitExplorer,
    Graph,
}

pub struct ActionKey<'a> {
    key_code: KeyCode,
    pub git_explorer_action: &'a dyn Fn(&mut GitExplorer)
}

impl<'a> ActionKey<'a> {
    pub fn new(key_code: KeyCode) -> Self {
        Self {
            key_code,
            git_explorer_action: &|git_explorer: &mut GitExplorer| {git_explorer.update_graph(1)}
        }
    }
}


