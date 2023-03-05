use git2::Commit;

pub fn paint_commit_track(commit: Commit) {
    println!("{:?}", commit.message());

    for parent in commit.parents() {
        println!("{:?}", parent.message());
    }

}

