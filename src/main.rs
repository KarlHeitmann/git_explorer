use git2::{Repository, BranchType, DiffHunk};

/*
use crate::{
	error::{Error, Result},
	hash,
	sync::repository::repo,
};

use crate::{
	hash,
};
*/

#[derive(Debug, Default, Clone, Copy, PartialEq, Hash)]
struct HunkHeader {
	pub old_start: u32,
	pub old_lines: u32,
	pub new_start: u32,
	pub new_lines: u32,
}

impl From<DiffHunk<'_>> for HunkHeader {
	fn from(h: DiffHunk) -> Self {
		Self {
			old_start: h.old_start(),
			old_lines: h.old_lines(),
			new_start: h.new_start(),
			new_lines: h.new_lines(),
		}
	}
}

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
    // jfkhsjkfhsdjkfs djsd hfjkhfsjkd fhjdskfhsd

    let my_first_diff = repo.diff_index_to_workdir(None, None).unwrap();
 
    /*
    for delta in my_first_diff.deltas() {
        println!("{:?}", delta.status());
        let old_file = delta.old_file();
        let new_file = delta.new_file();
        println!("{:?}", delta);
        println!("{:?}", old_file);
        println!("{:?}", new_file);
    }
    */

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
            // c.pus();
            // let a = String::from_utf8(vec!(line.origin())).unwrap().push(String::from_utf8(line.content().to_vec()).unwrap());
            println!("{:?}", a);
			true
		}),
	);

    println!("{:?}", foreach_result);



}
