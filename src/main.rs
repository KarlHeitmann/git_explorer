#![feature(iter_collect_into)]
#![feature(slice_partition_dedup)]
use git2::{Repository, BranchType};
use crossterm::terminal::{enable_raw_mode, disable_raw_mode};

use tui::{
    backend::CrosstermBackend,
    Terminal
};


use std::io;

mod ui;
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

// fn main() {
fn main() -> Result<(), Box<dyn std::error::Error>> {


    let repo = match Repository::open(".") {
        Ok(repo) => repo,
        Err(e) => panic!("failed to open: {}", e),
    };
    let head = match repo.head() {
        Ok(repo) => repo,
        Err(e) => panic!("failed to get head: {}", e),
    };

    enable_raw_mode().expect("can run in raw mode");

    let stdout = io::stdout();
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    terminal.clear()?;

    //let folder = if cli.folder.is_none() { String::from(".") } else { cli.folder.unwrap() };

    ui::explorer_wrapper(&mut terminal)?;

    disable_raw_mode()?;
    terminal.show_cursor()?;


    // test_info(&repo);
    // graph::paint_commit_track(head.peel_to_commit().unwrap(), 0);
    let data = graph::paint_commit_track(head.peel_to_commit().unwrap());
    println!("{}", data.join("\n"));

    Ok(())
}
