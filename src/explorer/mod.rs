#![allow(unused)]  // FIXME

use log::{debug, error, info, trace, warn, LevelFilter, SetLoggerError};
use git2::{Repository, Commit, Oid, Time, Branches, Branch, BranchType};
use tui::{
    style::{Color, Style},
    text::Span,
};

use std::process;

use crate::utils::short_id;
use crate::explorer::graph_node::GraphNode;
use crate::explorer::parsed_diff::ParsedDiff;
use crate::explorer::git_wrapper::GitWrapper;
use crate::explorer::kernel::Kernel;

use self::branch_data::BranchData;

pub mod graph_node;
pub mod parsed_diff;
pub mod branch_data;
pub mod git_wrapper;
pub mod kernel;

pub struct GitExplorer {
    kernel: Kernel,
    git_wrapper: GitWrapper,
}

impl<'a> GitExplorer {
    pub fn new(path: Option<String>, root_oid: Option<Oid>, stop_condition: Option<BranchData>) -> Self {

        let git_wrapper = GitWrapper::new(path);

        let stop_conditions = git_wrapper.branches_data(stop_condition);

        let kernel = Kernel::new(root_oid, stop_conditions);

        Self {
            git_wrapper,
            kernel,
        }
    }


    pub fn run(&mut self) {
        self.kernel.run(&self.git_wrapper.repo)
    }

    pub fn update_graph(&mut self, i: isize) {
        self.kernel.update_graph(i, &self.git_wrapper.repo)
    }

    pub fn diff_commit_by_id(&self, commit_1: Commit, commit_2: Option<Oid>) -> ParsedDiff {
        self.kernel.diff_commit_by_id(commit_1, commit_2, &self.git_wrapper.repo)
    }

    pub fn get_selected_branch_oid(&self) -> Option<Oid> {
        self.kernel.get_selected_branch_oid()
    }
    pub fn branches_strings(&self) -> Vec<Span> {
        self.kernel.branches_strings()
    }
    pub fn diff_commit(&self, commit_1: Commit, i_2: usize) -> ParsedDiff {
        self.kernel.diff_commit(commit_1, i_2, &self.git_wrapper.repo)
    }
    pub fn get_node_id(&self, i: usize) -> Option<Oid> {
        // self.kernel.get_node_id(i).clone()
        self.kernel.get_node_id(i)
    }
    pub fn nodes(&self) -> Vec<GraphNode> {
        self.kernel.nodes.clone()
    }
    pub fn get_nodes_len(&self) -> usize {
        self.kernel.get_nodes_len()
    }
}

