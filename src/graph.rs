use std::cmp::Ordering;

use git2::{Commit, Oid, Time};

fn paint_branch(mut commits: Vec<Commit>) {
    let l = commits.len();
    match l {
        1 => {
            let commit = commits[0].to_owned();
            unsafe {
                let id = commit.id().to_string();
                println!("● ({}) {} ", id.get_unchecked(0..7), commit.summary().unwrap());
            }
            let parents: Vec<Commit> = commit.parents().collect();
            match parents.len() {
                0 => { println!("---No parent---") },
                1 => {
                    // println!("│");
                    paint_branch(parents)
                },
                2 => {
                    // let parent_1 = &parents[0];
                    // let parent_2 = &parents[1];

                    println!("├─┐");
                    paint_branch(parents);
                    // paint_branch(parent_2.clone());
                    // paint_branch(parent_1.clone());
                },
                _ => {},
            }
        },
        l => {
            let times: Vec<Time> = commits.clone().into_iter().map(|c| c.time()).collect();
            let mut max = times[0];
            let mut max_index = 0;

            for (index, &x) in times.iter().enumerate() {
                if x > max {
                    max = x;
                    max_index = index;
                }
            }

            // PAINT
            let mut branches_string = String::new();
            for i in 0..l {
                if i != max_index {
                    branches_string.push_str("│ ")
                } else {
                    branches_string.push_str("● ");
                }
            }
            let commit = commits[max_index].clone();
            unsafe {
                let id = commit.id().to_string();
                println!("{} ({}) {} ", branches_string, id.get_unchecked(0..7), commit.summary().unwrap());
            }

            // REDUCE
            let cs = commits.clone();
            'outer: for (i, c) in cs.iter().enumerate() {
                if c.id() != commit.id() {
                    for p in commit.parents() {
                        if c.id() == p.id() {
                            commits.remove(max_index);
                            if max_index >= i { max_index = max_index - 1; }
                            break 'outer;
                        }
                    }
                }
            }
            // CALL
            match commit.parent_count() {
                1 => {
                    commits[max_index] = commit.parent(0).unwrap();
                    paint_branch(commits);
                },
                l => {
                    commit.parents().collect_into(&mut commits);
                    paint_branch(commits);
                }
            }

            // let &mut parents: Vec<Commit> = commit.parents().collect();
            // commits.append(parents);
        },
    }
}

pub fn paint_commit_track(commit: Commit) {
    paint_branch(vec![commit]);
}

