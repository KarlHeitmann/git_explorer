use git2::{Commit, Oid};

fn paint_branch(commits: Vec<Commit>) {
    let l = commits.len();
    match l {
        1 => {
            let commit = commits[0].to_owned();
            unsafe {
                let id = commit.id().to_string();
                println!("● ({}) {} ", id.get_unchecked(0..6), commit.summary().unwrap());
            }
            let parents: Vec<Commit> = commit.parents().collect();
            match parents.len() {
                0 => { println!("---No parent---") },
                1 => {
                    println!("│");
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
            println!("many many many");
            // paint_branch_aux(commit);
        },
    }
}

pub fn paint_commit_track(commit: Commit) {
    paint_branch(vec![commit]);
}

