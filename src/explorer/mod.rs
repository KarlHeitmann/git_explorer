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

use self::branch_data::BranchData;

pub mod graph_node;
pub mod parsed_diff;
pub mod branch_data;

#[derive(PartialEq)]
enum Status {
    Same,
    Increase,
    Decrease,
}

pub struct GitExplorer {
    path: Option<String>,
    repo: Repository,
    root_oid: Option<Oid>,
    pub nodes: Vec<GraphNode>,
    is_updated: bool,
    stop_condition_i: usize,
    stop_conditions: Vec<Option<BranchData>>,
    nodes_len: usize,
}

impl<'a> GitExplorer {
    pub fn new(path: Option<String>, root_oid: Option<Oid>, stop_condition: Option<BranchData>) -> Self {

        let repo = match path {
            _ => {
                match Repository::open(".") {
                    Ok(repo) => repo,
                    Err(e) => { error!("failed to open: {}", e); process::exit(0x0100); } ,
                }
            }
        };

        let mut stop_conditions: Vec<Option<BranchData>> = vec![stop_condition];

        match repo.head() {
            Ok(head) => {
                for branch in repo.branches(Some(BranchType::Local)).unwrap() {
                    let branch_data = BranchData::new(branch);
                    /*
                    let b = branch.unwrap();
                    let b_string = b.0.get().shorthand().unwrap().to_string();
                    */
                    let b_string = branch_data.shorthand();
                    let head = head.shorthand().unwrap().to_string();
                    if head.contains(b_string) || b_string.contains(&head) {
                        stop_conditions.push(Some(branch_data));
                    }
                }
            },
            Err(_) => {
                for branch in repo.branches(Some(BranchType::Local)).unwrap() {
                    let branch_data = BranchData::new(branch);
                    stop_conditions.push(Some(branch_data));
                }
            }
        };

        Self {
            stop_condition_i: 0,
            repo,
            root_oid,
            path,
            stop_conditions,
            nodes: vec![],
            is_updated: false,
            nodes_len: 0,
        }
    }

    // TODO: fix wrong name, this is branches_vec
    pub fn branches_strings(&self) -> Vec<Span> {
        self
            .stop_conditions
            .clone()
            .into_iter()
            .enumerate()
            .map(|(i, sc)| {
                // let s = sc.unwrap_or_else(|| { (Oid::zero(), String::from(format!("{}/{} None, ", self.stop_condition_i, self.stop_conditions.len())) )}).1;
                let s = match sc {
                    Some(sc) => sc.shorthand().clone(),
                    None => String::from(format!("{}/{} None", self.stop_condition_i + 1, self.stop_conditions.len())),
                };
                if i == self.stop_condition_i {
                    Span::styled(format!("{} ", s), Style::default().fg(Color::Yellow))
                } else {
                    Span::styled(format!("{} ", s), Style::default().fg(Color::White))
                }
            }).collect::<Vec<Span>>()
    }

    pub fn get_nodes_len(&self) -> usize {
        self.nodes_len
    }

    pub fn get_node_id(&self, i: usize) -> Option<Oid> {
        // self.nodes.get(i).unwrap().id()
        match self.nodes.get(i) {
            Some(graph_node) => Some(graph_node.id()),
            None => None,
        }
    }

    pub fn get_selected_branch_oid(&self) -> Option<Oid> {
        let a = self.stop_conditions.get(self.stop_condition_i).unwrap().to_owned();
        match a {
            Some(branch_data) => Some(branch_data.oid()),
            None => None
        }
    }

    pub fn update_graph(&mut self, i: isize) {
        if i > 0 {
            if self.stop_condition_i < (self.stop_conditions.len() - 1) {
                self.stop_condition_i = self.stop_condition_i + 1
            } else {
                self.stop_condition_i = 0
            }
        } else {
            if self.stop_condition_i > 0 {
                self.stop_condition_i = self.stop_condition_i - 1
            } else {
                self.stop_condition_i = self.stop_conditions.len() - 1;
            }
        }
        self.run()
    }

    // TODO: merge fn diff_commit and diff_commit_by_id using Generic types.
    // pub fn diff_commit(&self, commit_1: Commit, commit_2: &Option<&GraphNode>) -> String {
    pub fn diff_commit(&self, commit_1: Commit, i_2: usize) -> ParsedDiff {
        // let parsed_diff = 
        let commit_2 = self.get_node_id(i_2);
        let parsed_diff = ParsedDiff::new(commit_1, commit_2, &self.repo);
        // detail
        parsed_diff
    }

    pub fn diff_commit_by_id(&self, commit_1: Commit, commit_2: Option<Oid>) -> ParsedDiff {
        // let parsed_diff = 
        // let commit_2 = self.get_node_id(i_2);
        ParsedDiff::new(commit_1, commit_2, &self.repo)
    }

    pub fn run(&mut self) {
        let nodes = match self.root_oid {
            _ => {
                let branches = self.repo.branches(Some(BranchType::Local)).unwrap();
                self.paint_commit_track(self.repo.head().unwrap().peel_to_commit().unwrap(), branches)
            }
        };
        self.nodes_len = nodes.len();
        self.nodes = nodes;
    }

    fn find_max_index(&self, times: Vec<Time>) -> usize {
        let mut max = times[0];
        let mut max_index = 0;

        for (index, &x) in times.iter().enumerate() {
            if x > max {
                max = x;
                max_index = index;
            }
        }

        max_index
    }

    fn paint(&self, l: usize, max_index: usize, new_branch: bool) -> String {
        let branches_string = if new_branch {
            format!("{}├●{}",
               String::from("│ ").repeat(max_index),
               String::from("│ ").repeat(l - (max_index + 1)),
            )
        } else {
            format!("{}├●",
               String::from("│ ").repeat(l - 1),
            )
        };
        format!("{}", branches_string)
    }

    fn paint_branch(
        &self,
        mut commits: Vec<Commit>,
        mut output: Vec<GraphNode>,
        limit_stack: Option<usize>,
        branches: Vec<(Branch, BranchType, String)>,
        abort: bool) -> Vec<GraphNode> {
    // fn paint_branch(mut commits: Vec<Commit>, mut output: Vec<(String, Oid)>, limit_stack: Option<usize>) -> Vec<(String, Oid)> {
        // let debug_data: Vec<String> = commits.clone().into_iter().map(|c| short_id(c.id())).collect();
        // println!("{:?}", debug_data);
        let l = commits.len();
        let mut status = Status::Same;

        let (abort, limit_stack) = match limit_stack {
            Some(limit_stack) => { (abort || l == 0 || limit_stack == 0, Some(limit_stack - 1))},
            None => {(abort || l == 0, None)}
        };

        if abort { return vec![] }

        let max_index = self.find_max_index(commits.clone().into_iter().map(|c| c.time()).collect());

        let commit_max = commits[max_index].clone();

        if short_id(commit_max.id()) == String::from("cdd9917") || short_id(commit_max.id()) == String::from("e5a7eb5") {
            let _aux = 1 + 1;
        }

        // PAINT
        //
        let mut i_branch = 0;
        let mut shorthand: Option<String> = None;
        for branch in branches.iter() {
            let b = branch.0.get();
            let commit_branch = b.peel_to_commit().ok().unwrap();
            if commit_branch.id() == commit_max.id() {
                shorthand = Some(b.shorthand().unwrap().to_string());
                break; 
            }
            i_branch = i_branch + 1;
        }

        let parents_max: Vec<Commit> = commit_max.parents().collect();

        let mut paint_string = self.paint(l, max_index, parents_max.len() > 1);
        let mut paint_string_split = String::new();
        let mut paint_string_join = String::new();

        // SUBSTITUTE commit_max by all its parents inside the "commits" vector.
        commits.remove(max_index);
        match parents_max.len() {
            0 => {
                paint_string_split.push_str(&format!("\n╽"));
                status = Status::Decrease;
            },
            1 => {
                commits.insert(max_index, parents_max[0].clone());
            },
            2 => {
                status = Status::Increase;
                paint_string_split.push_str(&format!(
                    "\n{}├{}─┐",
                    String::from("│ ").repeat(max_index),
                    String::from("──").repeat(l - (max_index + 1)),
                ));
                commits.insert(max_index, parents_max[0].clone());
                commits.insert(max_index + 1, parents_max[1].clone());
            },
            _ => { panic!("AAHHH! There is a commit with more than 2 parents!!! I'm so scared... HINT: Use the case above and apply it to general") }
        }

        let mut binding = commits.clone();
        let (dedup, duplicates) = binding.partition_dedup_by(|a, b| a.id() == b.id());

        let dupl_len = duplicates.len();
        if dupl_len > 0 {
            paint_string_join.push_str(&format!(
                "\n{}├─{}┘",
                String::from("│ ").repeat(l-(dupl_len + 1)),
                String::from("──").repeat(dupl_len - 1)
            ));
        }

        if !paint_string_split.is_empty() && !paint_string_join.is_empty() {
            // Occured a join and split: deal with it
            // paint_string.push_str("\n│─┤")
            paint_string.push_str(&format!("\n{}├─{}┤", String::new().repeat(0), String::new().repeat(0))) // TODO: // XXX: This will fail at any time, recreate a git history branch that will stress this condition
        } else {
            paint_string.push_str(&paint_string_split);
            paint_string.push_str(&paint_string_join);
        }

        match status {
            Status::Same => {
            },
            Status::Increase => {
            },
            Status::Decrease => {
            }
        }

        let stop_condition = self.stop_conditions.get(self.stop_condition_i).unwrap();
        let abort_next = match stop_condition {
            Some(stop_condition) => {
                stop_condition.oid() == commit_max.id()
            }
            _ => false
        };

        let vec_str = self.paint_branch(dedup.to_vec(), vec![], limit_stack, branches, abort_next);

        output.push(GraphNode { grapheme: paint_string, oid: commit_max.id(), branch_shorthand: shorthand, summary: commit_max.summary().unwrap().to_string() });

        [output, vec_str].concat()
    }

    pub fn paint_commit_track(&self, commit: Commit, branches: Branches) -> Vec<GraphNode> {
        // let limit_stack = 1000; // Works fine
        let limit_stack = 500; // Works fine
        // let limit_stack = 10000; // Works, but it is unhandeable :/
        let branches: Vec<(Branch, BranchType, String)> = branches
            .map(|b| {
                let b = b.ok();
                let b = b.unwrap();
                let b_aux = &b.0;
                let name = b_aux.get().shorthand().unwrap().to_string();
                (b.0, b.1, name)
            }).collect();

        // paint_branch(vec![commit], vec![], Some(limit_stack), branches)
        self.paint_branch(vec![commit], vec![], Some(limit_stack), branches, false)
    }

}


