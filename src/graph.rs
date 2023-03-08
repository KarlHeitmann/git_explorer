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
    // PAINT // ┼
    let mut branches_string = String::new();
    for i in 0..l {
        if i == max_index {
        // if i == l-1 {
            // branches_string.push_str("● ");
            branches_string.push_str("├●");
            // branches_string.push_str("┝ ");
        } else {
            branches_string.push_str("│ ")
        }
    }
    let id = short_id(commit.id());
    println!("{} ({}) {} ", branches_string, id, commit.summary().unwrap());
}

#[derive(PartialEq)]
enum Status {
    Same,
    Increase,
    Decrease,
}

fn paint_branch(mut commits: Vec<Commit>) {
    let debug_data: Vec<String> = commits.clone().into_iter().map(|c| short_id(c.id())).collect();
    // println!("{:?}", debug_data);
    let l = commits.len();
    let mut status = Status::Same;

    if l == 0 { return }

    // let mut max_index = find_max_index(commits.clone().into_iter().map(|c| c.time()).collect());
    let max_index = find_max_index(commits.clone().into_iter().map(|c| c.time()).collect());

    let commit_max = commits[max_index].clone();

    // let dbg = 
    if short_id(commit_max.id()) == String::from("cdd9917") || short_id(commit_max.id()) == String::from("e5a7eb5") {
        let aux = 1 + 1;
    }
    let commit_max_id = short_id(commit_max.id());
    // PAINT
    paint(l, max_index, &commit_max);
    
    let parents_max: Vec<Commit> = commit_max.parents().collect();

    // SUBSTITUTE commit_max by all its parents inside the "commits" vector.
    match parents_max.len() {
        0 => {
            commits.remove(max_index);
            println!("├─┘");
            status = Status::Decrease;
        },
        1 => {
            commits.remove(max_index);
            commits.insert(max_index, parents_max[0].clone());
        },
        2 => {
            commits.remove(max_index);
            status = Status::Increase;
            println!("├─{}┐", String::from("┼─").repeat(l-1));
            commits.insert(max_index, parents_max[0].clone());
            commits.insert(max_index + 1, parents_max[1].clone());
        },
        _ => { panic!("AAHHH! There is a commit with more than 2 parents!!! I'm so scared... HINT: Use the case above and apply it to general") }
    }

    // commits.du
    // commits.dedup_by(|a,b| a.id() == b.id());
    let mut binding = commits.clone();
    let (dedup, duplicates) = binding.partition_dedup_by(|a, b| a.id() == b.id()); // duplicates: each repeated element appears in the array

    let mut reduces_string = String::new();
    if duplicates.len() > 0 {
        // println!("{:?}", duplicates);
        // let binding = commits.clone();
        // println!("alaracaaaaaaaaaaa");
        for dup in duplicates {
            // binding.iter
            let mut i = 0;
            let mut first_encounter_done = false;
            for c in commits.iter() {
                if first_encounter_done {
                    if c.id() == dup.id() {
                        reduces_string.push_str("┘ ");
                        break;
                    } else {
                        reduces_string.push_str("───");
                    }
                } else {
                    if c.id() == dup.id() {
                        first_encounter_done = true;
                        reduces_string.push_str("├─");
                    } else {
                        reduces_string.push_str("  ");
                    }
                }
                i = i + 1;
            }
        }
    }
    // if !reduces_string.is_empty() { println!("{}", reduces_string); }
    match status {
        Status::Same => {
        },
        Status::Increase => {
        },
        Status::Decrease => {
        }
    }
    paint_branch(dedup.to_vec());



    /*
    for p in commits {

    }
    */

}

pub fn paint_commit_track(commit: Commit) {
    paint_branch(vec![commit]);
}

