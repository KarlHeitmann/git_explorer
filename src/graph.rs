use git2::{Commit, Oid, Time};
use crate::utils::short_id;

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

fn paint(l: usize, max_index: usize, commit: &Commit) -> String {
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
    format!("{} ({}) {} ", branches_string, id, commit.summary().unwrap())
}

#[derive(PartialEq)]
enum Status {
    Same,
    Increase,
    Decrease,
}

fn paint_branch(mut commits: Vec<Commit>, mut output: Vec<(String, Oid)>) -> Vec<(String, Oid)> {
    // let debug_data: Vec<String> = commits.clone().into_iter().map(|c| short_id(c.id())).collect();
    // println!("{:?}", debug_data);
    let l = commits.len();
    let mut status = Status::Same;

    if l == 0 { return vec![] }

    // let mut max_index = find_max_index(commits.clone().into_iter().map(|c| c.time()).collect());
    let max_index = find_max_index(commits.clone().into_iter().map(|c| c.time()).collect());

    let commit_max = commits[max_index].clone();

    // let dbg = 
    if short_id(commit_max.id()) == String::from("cdd9917") || short_id(commit_max.id()) == String::from("e5a7eb5") {
        let aux = 1 + 1;
    }

    // commit_max_id.split("\n").collect()
    // PAINT
    let mut paint_string = paint(l, max_index, &commit_max);

    let parents_max: Vec<Commit> = commit_max.parents().collect();

    // SUBSTITUTE commit_max by all its parents inside the "commits" vector.
    commits.remove(max_index);
    match parents_max.len() {
        0 => {
            paint_string.push_str(&format!("\n╽"));
            status = Status::Decrease;
        },
        1 => {
            commits.insert(max_index, parents_max[0].clone());
        },
        2 => {
            status = Status::Increase;
            // paint_string.push_str(&format!("\n├─{}┐", String::from("┼─").repeat(l-1)));
            // paint_string.push_str(&format!("\n├─{}┐", String::from("│ ").repeat(l-1)));
            paint_string.push_str(&format!("\n{}├─┐", String::from("│ ").repeat(l-1)));
            commits.insert(max_index, parents_max[0].clone());
            commits.insert(max_index + 1, parents_max[1].clone());
        },
        _ => { panic!("AAHHH! There is a commit with more than 2 parents!!! I'm so scared... HINT: Use the case above and apply it to general") }
    }


    let mut binding = commits.clone();
    let (dedup, duplicates) = binding.partition_dedup_by(|a, b| a.id() == b.id());

    let mut reduces_string = String::new();

    let dupl_len = duplicates.len();
    if dupl_len > 0 {
        // reduces_string = String::from("\nXXXXXXXXXXXXXXXXXXXX");
        // reduces_string.push_str(&format!("\n{}├─┘", String::from("│ ").repeat(l-2)));
        reduces_string.push_str(&format!("\n{}├─{}┘", String::from("│ ").repeat(l-(dupl_len + 1)), String::from("──").repeat(dupl_len - 1)));
        /*   ─
        for (i, dup) in duplicates.iter().enumerate() {
            let mut first_encounter_done = false;
            for (j, c) in commits.iter().enumerate() {
                if first_encounter_done {
                    if c.id() == dup.id() {
                        reduces_string.push_str("┘ ");
                        // paint_string.push_str(&String::from("\nwawawaXXXXXXW"));
                        break;
                    } else {
                        reduces_string.push_str("───");
                    }
                } else {
                    if c.id() == dup.id() {
                        first_encounter_done = true;
                        reduces_string.push_str("\n├─");
                    } else {
                        reduces_string.push_str("  ");
                    }
                }
            }
        }
        */
    }
    paint_string.push_str(&reduces_string);

    match status {
        Status::Same => {
        },
        Status::Increase => {
        },
        Status::Decrease => {
        }
    }

    let vec_str = paint_branch(dedup.to_vec(), vec![]);

    output.push((paint_string, commit_max.id()));

    [output, vec_str].concat()
}

pub fn paint_commit_track(commit: Commit) -> Vec<(String, Oid)> {
    paint_branch(vec![commit], vec![])
}

