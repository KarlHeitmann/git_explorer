#![feature(iter_collect_into)]
use git2::{Repository, BranchType};

mod graph;

// #[!warn(dead_code)]
fn test_info(repo: &Repository) {
    let head = match repo.head() {
        Ok(repo) => repo,
        Err(e) => panic!("failed to get head: {}", e),
    };

    println!("{:?}", head.name());
    println!("{:?}", head.shorthand());
    println!("{:?}", head.is_branch());

    for branch in repo.branches(Some(BranchType::Local)).unwrap() {
        let b = branch.unwrap();
        println!("{:?}", b.0.get().name());
        println!("{:?}", b.0.get().shorthand());
    }
    let state = repo.state();
    println!("{:?}", state);

    let workdir = repo.workdir();
    println!("{:?}", workdir);
 
    let my_first_diff = repo.diff_index_to_workdir(None, None).unwrap();

    let foreach_result = my_first_diff.foreach(
		&mut |_, _| true,
		None,
		Some(&mut |_, hunk| {
            let a = String::from_utf8(hunk.header().to_vec()).unwrap();
            println!("{:?}", a);
			true
		}),
		Some(&mut |_, _hunk, line| {
            let mut a = line.origin().to_string();
            let b = String::from_utf8(line.content().to_vec()).unwrap();
            a.push_str(&b);
            println!("{:?}", a);
			true
		}),
	);

    println!("{:?}", foreach_result);
}

fn main() {

    let repo = match Repository::open(".") {
        Ok(repo) => repo,
        Err(e) => panic!("failed to open: {}", e),
    };
    let head = match repo.head() {
        Ok(repo) => repo,
        Err(e) => panic!("failed to get head: {}", e),
    };

    // test_info(&repo);
    // graph::paint_commit_track(head.peel_to_commit().unwrap(), 0);
    graph::paint_commit_track(head.peel_to_commit().unwrap());

}
