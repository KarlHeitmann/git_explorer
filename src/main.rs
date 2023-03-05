use git2::{Repository, BranchType};

fn main() {
    println!("Hello, world!");
    let repo = match Repository::open(".") {
        Ok(repo) => repo,
        Err(e) => panic!("failed to open: {}", e),
    };
    let head = match repo.head() {
        Ok(repo) => repo,
        Err(e) => panic!("failed to get head: {}", e),
    };
    // debug!(logger, "head found"; "head" => head.name());

    println!("{:?}", head.name());
    println!("{:?}", head.shorthand());
    println!("{:?}", head.is_branch());

    for branch in repo.branches(Some(BranchType::Local)).unwrap() {
        let b = branch.unwrap();
        println!("{:?}", b.0.get().name());
        println!("{:?}", b.0.get().shorthand());
    }
    // println!("{:?}", repo.branches());
    //
    let state = repo.state();
    println!("{:?}", state);

    let workdir = repo.workdir();
    println!("{:?}", workdir);
 
    /*
    let message = repo.message();
    println!("{:?}", message);
    */

    let my_first_diff = repo.diff_index_to_workdir(None, None).unwrap();
 
    for delta in my_first_diff.deltas() {
        println!("{:?}", delta.status());
        let old_file = delta.old_file();
        let new_file = delta.new_file();
        println!("{:?}", delta);
        println!("{:?}", old_file);
        println!("{:?}", new_file);
    }
}
