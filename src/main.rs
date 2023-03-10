#![feature(iter_collect_into)]
#![feature(slice_partition_dedup)]
// use git2::{Repository, BranchType};
use git2::Repository;
use crossterm::terminal::{enable_raw_mode, disable_raw_mode};

use tui::{
    backend::CrosstermBackend,
    Terminal
};

use std::io;

mod ui;
mod utils;
mod graph;

    /*
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
}
    */

fn main() -> Result<(), Box<dyn std::error::Error>> {


    let repo = match Repository::open(".") {
        Ok(repo) => repo,
        Err(e) => panic!("failed to open: {}", e),
    };

    enable_raw_mode().expect("can run in raw mode");

    let stdout = io::stdout();
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    terminal.clear()?;


    ui::explorer_wrapper(&mut terminal, &repo, repo.head().unwrap().peel_to_commit().unwrap())?;

    disable_raw_mode()?;
    terminal.show_cursor()?;

    // test_info(&repo);

    Ok(())
}
