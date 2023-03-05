use git2::{Commit, Oid};

fn paint_branch_aux(commit: Commit, oid: Oid, offset: u8) {
    if commit.id() != oid {
        println!("| {}", commit.summary().unwrap());
        paint_branch_aux(commit.parent(0).unwrap(), oid, offset);
    }
}

fn paint_branch(commit: Commit, oid: Option<Oid>, offset: u8) {
    match oid {
        Some(oid) => {
            paint_branch_aux(commit, oid, 0);
        },
        None => {
            println!("{}", commit.summary().unwrap());
            let parents: Vec<Commit> = commit.parents().collect();
            match parents.len() {
                0 => {},
                1 => paint_branch(parents[0].clone(), None, 0),
                2 => {
                    let parent_1 = &parents[0];
                    let parent_2 = &parents[1];
                    paint_branch(parent_2.clone(), Some(parent_1.id()), 0);
                    paint_branch(parent_1.clone(), None, 0);
                },
                _ => {},
            }
        },
    }
}

pub fn paint_commit_track(commit: Commit) {
    match commit.summary() {
        // Some(message) => println!("{} {}", " ".repeat(offset.into()), message.trim()),
        Some(message) => println!("{}", message.trim()),
        None => {},
    }

    let parents: Vec<Commit> = commit.parents().collect();
    match parents.len() {
        0 => {},
        1 => {
            println!("1 parent!");
            paint_branch(parents[0].clone(), None, 0);
        },
        2 => {
            // NOTE left parent (parents[0]) is the far away commit, right parent (parents[1]) is the closes commit
            println!("2 PARENTS!");
            let parent_1 = &parents[0];
            let parent_2 = &parents[1];
            paint_branch(parent_1.clone(), None, 0);
            paint_branch(parent_2.clone(), Some(parent_1.id()), 0);
            // println!("{} - {}", parent_1.summary().unwrap(), parent_2.summary().unwrap());
            // println!("{} - {}", parent_1.id(), parent_2.id());

        }
        _ => {},
    }
    /*
    let mut i = 0;
    let parents: Vec<Commit> = commit.parents().collect();
    for parent in parents {
        paint_commit_track(parent, offset + i);
        i = i + 1;
    }
    */
}

