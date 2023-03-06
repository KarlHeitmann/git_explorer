use git2::{Commit, Oid};

fn find_common_node() { todo!(); }

fn paint_branch_aux(commit: Commit, oid: Oid, offset: u8) {
    if commit.id() != oid {
        println!("│ │");
        // commit.id().to
        unsafe {
            let id = commit.id().to_string();
            println!("│ ● ({}) {} ", id.get_unchecked(0..6), commit.summary().unwrap());
        }
        match commit.parent(0) {
            Ok(parent) => paint_branch_aux(parent, oid, offset),
            Err(e) => println!("ERROR: commit {} has error on getting parent: {}", commit.id(), e),
        }
    } else {
        println!("├─┘");
    }
}

fn paint_branch(commit: Commit, oid: Option<Oid>, offset: u8) {
    match oid {
        Some(oid) => {
            paint_branch_aux(commit, oid, 0);
        },
        None => {
            unsafe {
                let id = commit.id().to_string();
                println!("● ({}) {} ", id.get_unchecked(0..6), commit.summary().unwrap());
            }
            let parents: Vec<Commit> = commit.parents().collect();
            match parents.len() {
                0 => { println!("---No parent---") },
                1 => {
                    println!("│");
                    paint_branch(parents[0].clone(), None, 0)
                },
                2 => {
                    let parent_1 = &parents[0];
                    let parent_2 = &parents[1];
                    println!("├─┐");
                    paint_branch(parent_2.clone(), Some(parent_1.id()), 0);
                    paint_branch(parent_1.clone(), None, 0);
                },
                _ => {},
            }
        },
    }
}

pub fn paint_commit_track(commit: Commit) {
    paint_branch(commit, None, 0);
}

