use git2::{Repository, Commit, Oid, Time, Branches, Branch, BranchType};
use crate::explorer::graph_node::GraphNode;
use crate::explorer::branch_data::BranchData;
use crate::explorer::short_id;
use crate::explorer::ParsedDiff;
use log::{debug, error, info, trace, warn, LevelFilter, SetLoggerError};

use tui::{
    style::{Color, Style},
    text::Span,
};

const LIMIT_STACK: usize = 500;


pub struct Kernel {
    root_oid: Option<Oid>,
    pub nodes: Vec<GraphNode>,
    is_updated: bool,
    stop_condition_i: usize,
    stop_conditions: Vec<Option<BranchData>>,
    nodes_len: usize,
    abort: bool,
    limit_stack: Option<usize>,
}

impl Kernel {
    pub fn new(root_oid: Option<Oid>, stop_conditions: Vec<Option<BranchData>>) -> Self {
        Self {
            abort: false,
            limit_stack: Some(LIMIT_STACK),
            stop_condition_i: 0,
            root_oid,
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

                let style = Style::default().fg(
                    if i == self.stop_condition_i {
                        Color::White
                    } else {
                        Color::Yellow
                    }
                );

                Span::styled(format!("{} ", s), style)
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

    pub fn update_graph(&mut self, i: isize, repo: &Repository) {
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
        self.run(repo)
    }

    // TODO: merge fn diff_commit and diff_commit_by_id using Generic types.
    // pub fn diff_commit(&self, commit_1: Commit, commit_2: &Option<&GraphNode>) -> String {
    pub fn diff_commit(&self, commit_1: Commit, i_2: usize, repo: &Repository) -> ParsedDiff {
        // let parsed_diff = 
        let commit_2 = self.get_node_id(i_2);
        let parsed_diff = ParsedDiff::new(commit_1, commit_2, &repo);
        // detail
        parsed_diff
    }

    pub fn diff_commit_by_id(&self, commit_1: Commit, commit_2: Option<Oid>, repo: &Repository) -> ParsedDiff {
        // let parsed_diff = 
        // let commit_2 = self.get_node_id(i_2);
        ParsedDiff::new(commit_1, commit_2, &repo)
    }

    pub fn run(&mut self, repo: &Repository) {
        trace!("fn run");
        let nodes = match self.root_oid {
            _ => {
                let branches_tmp = repo.branches(Some(BranchType::Local)).unwrap(); // TODO use BranchData
                let mut branches: Vec<BranchData> = vec![];
                for b in branches_tmp {
                    branches.push(BranchData::from(b));
                }
                // let branches = branches.map(|b| BranchData::new(b)).collect();
                self.paint_commit_track(repo.head().unwrap().peel_to_commit().unwrap(), branches, repo)
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

    fn abort(&mut self, commits_len: usize) -> bool {
        match self.limit_stack {
            Some(limit_stack) => {
                let result = self.abort || commits_len == 0 || limit_stack == 0;
                self.limit_stack = Some(limit_stack - 1);
                result
            },
            None => self.abort || commits_len == 0
        }
    }

    // fn maybe_set_abort(&mut self, current_drawn_commit: Commit, target_branch_data: &Option<BranchData>) {
    fn maybe_set_abort(&mut self, current_drawn_commit: &Commit) {
        if self.abort {return}
        let stop_condition = self.stop_conditions.get(self.stop_condition_i);
        self.abort = match stop_condition {
            Some(stop_condition) => {
                match stop_condition.to_owned() {
                    Some(branch_data) => branch_data.oid() == current_drawn_commit.id(),
                    None => false
                }
            }
            _ => false
        };
    }

    fn short_hand_current_commit(&self, branches: &Vec<BranchData>, repo: &Repository, commit_max: &Commit) -> Option<String> {
        let mut i_branch = 0;
        let mut shorthand: Option<String> = None;
        for branch in branches.iter() {
            let commit_branch = repo.find_commit(branch.oid()).unwrap();
            if commit_branch.id() == commit_max.id() {
                shorthand = Some(branch.shorthand().to_string());
                break; 
            }
            i_branch = i_branch + 1;
        }
        shorthand
    }

    fn paint_split_commit<'a>(&self, parents_max: &Vec<Commit<'a>>, commits: &mut Vec<Commit<'a>>, max_index: usize, l: usize) -> (String, String) {
        let mut paint_string_split = String::new();
        let mut paint_string_join = String::new();
        match parents_max.len() {
            0 => {
                paint_string_split.push_str(&format!("\n╽"));
            },
            1 => {
                commits.insert(max_index, parents_max[0].clone());
            },
            2 => {
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
        (paint_string_split, paint_string_join)
    }

    fn paint_branch(
        &mut self,
        mut commits: Vec<Commit>,
        mut output: Vec<GraphNode>,
        branches: Vec<BranchData>,
        repo: &Repository) -> Vec<GraphNode> {
        let l = commits.len();

        if self.abort(l) { return vec![] }

        let max_index = self.find_max_index(commits.clone().into_iter().map(|c| c.time()).collect());

        let commit_max = commits[max_index].clone();

        // Figures out if the current commit has a branch name
        let shorthand = self.short_hand_current_commit(&branches, repo, &commit_max);

        let parents_max: Vec<Commit> = commit_max.parents().collect();

        let mut paint_string = self.paint(l, max_index, parents_max.len() > 1);

        // SUBSTITUTE commit_max by all its parents inside the "commits" vector.
        commits.remove(max_index);
        let (paint_string_split, mut paint_string_join) = self.paint_split_commit(&parents_max, &mut commits, max_index, l);

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
            paint_string.push_str(&format!("\n{}├─{}┤", String::new().repeat(0), String::new().repeat(0))) // TODO: // XXX: This will fail at any time, recreate a git history branch that will stress this condition
        } else {
            paint_string.push_str(&paint_string_split);
            paint_string.push_str(&paint_string_join);
        }

        self.maybe_set_abort(&commit_max);

        let vec_str = self.paint_branch(dedup.to_vec(), vec![], branches, repo);

        output.push(GraphNode { grapheme: paint_string, oid: commit_max.id(), branch_shorthand: shorthand, summary: commit_max.summary().unwrap().to_string() });

        [output, vec_str].concat()
    }

    pub fn paint_commit_track(&mut self, commit: Commit, branches: Vec<BranchData>, repo: &Repository) -> Vec<GraphNode> {
    // pub fn paint_commit_track(&self, commit: Commit, branches: Vec<BranchData>, repo: &Repository) -> Vec<GraphNode> {
        // let limit_stack = 1000; // Works fine

        self.abort = false;
        self.limit_stack = Some(LIMIT_STACK); // Works fine
        self.paint_branch(vec![commit], vec![], branches, repo)
    }
}

