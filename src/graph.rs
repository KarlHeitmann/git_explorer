use std::cmp::Ordering;

use git2::{Commit, Oid, Time};

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
    unsafe {
        let id = commit.id().to_string();
        println!("{} ({}) {} ", branches_string, id.get_unchecked(0..7), commit.summary().unwrap());
    }
}

fn paint_branch(mut commits: Vec<Commit>) {
    // let debug_data: Vec<String> = commits.clone().into_iter().map(|c| c.id().to_string()).collect();
    // println!("{:?}", debug_data);
    let l = commits.len();
    if l == 0 { return }
    let mut reduced = false;

    let mut max_index = find_max_index(commits.clone().into_iter().map(|c| c.time()).collect());

    let commit_max = commits[max_index].clone();

    // PAINT
    paint(l, max_index, &commit_max);

    // REDUCE
    let cs = commits.clone();
    'outer: for (i, c) in cs.iter().enumerate() {
        if c.id() != commit_max.id() {
            for p in commit_max.parents() {
                if c.id() == p.id() {
                    commits.remove(max_index);
                    if max_index >= i { max_index = max_index - 1; }
                    reduced = true;
                    break 'outer;
                }
            }
        }
    }

    // CALL
    match commit_max.parent_count() {
        0 => {
            println!("THE END");
            commits = vec![];
        }
        1 => {
            commits[max_index] = commit_max.parent(0).unwrap();
            if reduced { println!("├─┘"); }
        },
        l => {
            println!("├─┐");
            let parents: Vec<Commit> = commit_max.parents().collect();
            commits.remove(max_index);
            commits.insert(max_index, parents[0].clone());
            commits.insert(max_index + 1, parents[1].clone());

            // parents.collect_into(&mut commits)
        }
    };

    paint_branch(commits);

    // let &mut parents: Vec<Commit> = commit_max.parents().collect();
    // commits.append(parents);
}

pub fn paint_commit_track(commit: Commit) {
    paint_branch(vec![commit]);
}

