use git2::{Repository, Commit, Oid, Time, Branches, Branch, BranchType};
use std::fmt::{Display, Formatter, Result as FmtResult};

use crate::utils::short_id;

#[derive(PartialEq)]
enum Status {
    Same,
    Increase,
    Decrease,
}

#[derive(Clone)]
pub struct GraphNode {
    pub grapheme: String,
    pub oid: Oid,
    pub branch_shorthand: Option<String>,
    pub summary: String,
    // TODO: add commit summary
}

impl GraphNode {
    pub fn id(&self) -> Oid {
        self.oid
    }
}
 
impl Display for GraphNode {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        let (grapheme, oid, branch_shorthand, summary) = (&self.grapheme, &self.oid, &self.branch_shorthand, &self.summary);
        let branch_shorthand = match branch_shorthand {
            Some(branch_shorthand) => { format!("[{}]", branch_shorthand) },
            None => { String::new() },
        };
        write!(f, "{} ({}) {} {}", grapheme, short_id(*oid), branch_shorthand, summary)
    }
}
 
pub struct GitExplorer {
    path: Option<String>,
    repo: Repository,
    root_oid: Option<Oid>,
    stop_condition: Option<(Oid, String)>
}

impl<'a> GitExplorer {
    pub fn new(path: Option<String>, root_oid: Option<Oid>, stop_condition: Option<(Oid, String)>) -> Self {

        let repo = match path {
            _ => {
                match Repository::open(".") {
                    Ok(repo) => repo,
                    Err(e) => panic!("failed to open: {}", e),
                }
            }
        };

        Self {
            stop_condition,
            repo,
            root_oid,
            path,
        }
    }

    pub fn update_graph(mut self, stop_condition: Option<(Oid, String)>) -> Vec<GraphNode> {
        self.stop_condition = stop_condition;
        self.run()
    }

    pub fn diff_commit(&self, commit_1: Commit, commit_2: &Option<&GraphNode>) -> String {
        let current_commit = commit_1;

        let mut detail = String::new();
        let parents = current_commit.parents().map(|c| short_id(c.id())).collect::<Vec<String>>().join(" - ");

        detail.push_str(
            &format!("\n{}\nCommiter: {}\nAuthor: {}\n{}\nPARENTS:\n{}\n\n",
                current_commit.message().unwrap_or("NO COMMIT MESSAGE"),
                current_commit.committer().to_string(),
                current_commit.author(),
                short_id(current_commit.id()),
                parents,
            )
        );

        let mut string_0 = String::from("FD\n");
        let mut string_a = String::new();
        let mut string_b = String::new();

        match commit_2 {
            Some(graph_node) => {
                let sub_tree_oid_previous = graph_node.id();
                let previous_commit = self.repo.find_commit(sub_tree_oid_previous).unwrap();

                let my_first_diff = self.repo.diff_tree_to_tree(
                    previous_commit.tree().ok().as_ref(),
                    current_commit.tree().ok().as_ref(),
                    None
                ).unwrap();

                let _foreach_result = my_first_diff.foreach(
                    &mut |delta, _| {
                        let old_file = delta.old_file();
                        let old_file = old_file.path().unwrap();
                        let new_file = delta.new_file();
                        let new_file = new_file.path().unwrap();
                        string_0.push_str(&format!("{:?} - {:?}\n", old_file, new_file));
                        true
                    },
                    None,
                    Some(&mut |_, _hunk| {
                        /*
                        let s = format!("{}\n",
                            String::from_utf8(hunk.header().to_vec()).unwrap()
                        );
                        string_a.push_str(&s);
                        */
                        // string_a = String::from_utf8(hunk.header().to_vec()).unwrap();
                        true
                    }),
                    Some(&mut |_, hunk, line| {
                        match hunk {
                            Some(hunk) => {
                                let hunk = String::from_utf8(hunk.header().to_vec()).unwrap();
                                if string_a == hunk {
                                    let s = format!("{}:{}{}",
                                        line.new_lineno().unwrap_or_else(|| line.old_lineno().unwrap()),
                                        line.origin().to_string(),
                                        String::from_utf8(line.content().to_vec()).unwrap()
                                    );
                                    string_b.push_str(&s);
                                } else {
                                    let s = format!("{}{}:{}{}",
                                        hunk,
                                        line.new_lineno().unwrap_or_else(|| line.old_lineno().unwrap()),
                                        line.origin().to_string(),
                                        String::from_utf8(line.content().to_vec()).unwrap()
                                    );
                                    string_b.push_str(&s);
                                }
                                string_a = hunk;
                            }
                            None => {
                                let s = format!("{}:{}{}",
                                    line.new_lineno().unwrap_or_else(|| line.old_lineno().unwrap()),
                                    line.origin().to_string(),
                                    String::from_utf8(line.content().to_vec()).unwrap()
                                );
                                string_b.push_str(&s);
                            }
                        }
                        true
                    }),
                );
            },
            None => {}
        }
        detail.push_str(&string_0);
        detail.push_str(&string_a);
        detail.push_str(&string_b);
        detail
    }

    pub fn run(&self) -> Vec<GraphNode> {
        match self.root_oid {
            _ => {
                let branches = self.repo.branches(Some(BranchType::Local)).unwrap();
                self.paint_commit_track(self.repo.head().unwrap().peel_to_commit().unwrap(), branches)
            }
        }
        
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

        let abort_next = match &self.stop_condition {
            Some(stop_condition) => {
                // let reference = stop_condition.get();
                // reference.peel_to_commit().unwrap().id() == commit_max.id()
                stop_condition.0 == commit_max.id()
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

