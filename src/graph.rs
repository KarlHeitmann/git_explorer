use git2::Commit;

pub fn paint_commit_track(commit: Commit, offset: u8) {
    match commit.summary() {
        Some(message) => println!("{} {}", " ".repeat(offset.into()), message.trim()),
        None => {},
    }

    let mut i = 0;
    let parents: Vec<Commit> = commit.parents().collect();
    for parent in parents {
        paint_commit_track(parent, offset + i);
        i = i + 1;
    }
}

