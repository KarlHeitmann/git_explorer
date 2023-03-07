use std::cmp::Ordering;

use git2::{Commit, Oid, Time};

fn short_id(id: Oid) -> String {
    let id = id.to_string();
    unsafe {
        format!("{}", id.get_unchecked(0..7))
    }
}

fn find_max_index(times: Vec<Time>) -> usize {
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

fn paint(l: usize, max_index: usize, commit: &Commit) {
    // PAINT
    let mut branches_string = String::new();
    for i in 0..l {
        if i != max_index {
            branches_string.push_str("│ ")
        } else {
            // branches_string.push_str("● ");
            branches_string.push_str("├●");
            // branches_string.push_str("┝ ");
        }
    }
    let id = short_id(commit.id());
    println!("{} ({}) {} ", branches_string, id, commit.summary().unwrap());
}

enum Status {
    Same,
    Increase,
    Decrease,
}

fn paint_branch(mut commits: Vec<Commit>) {
    // let debug_data: Vec<String> = commits.clone().into_iter().map(|c| short_id(c.id())).collect();
    // println!("{:?}", debug_data);
    let l = commits.len();
    let mut status = Status::Same;

    if l == 0 { return }

    // let mut max_index = find_max_index(commits.clone().into_iter().map(|c| c.time()).collect());
    let max_index = find_max_index(commits.clone().into_iter().map(|c| c.time()).collect());

    let commit_max = commits[max_index].clone();

    // PAINT
    paint(l, max_index, &commit_max);
    
    let parents_max: Vec<Commit> = commit_max.parents().collect();

    match parents_max.len() {
        0 => {
            commits.remove(max_index);
            // println!("├─┘");
            status = Status::Decrease;
        },
        1 => {
            commits.remove(max_index);
            commits.insert(max_index, parents_max[0].clone());
        },
        _ => {
            commits.remove(max_index);
            status = Status::Increase;
            // println!("├─┐");
            commits.insert(max_index, parents_max[0].clone());
            commits.insert(max_index + 1, parents_max[1].clone());
        }
    }

    // commits.du
    commits.dedup_by(|a,b| a.id() == b.id());
    /*
    for p in commits {

    }
    */

    paint_branch(commits);
}

pub fn paint_commit_track(commit: Commit) {
    paint_branch(vec![commit]);
}

