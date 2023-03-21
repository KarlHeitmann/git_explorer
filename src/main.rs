#![feature(iter_collect_into)]
#![feature(slice_partition_dedup)]

use git2::{ Repository, BranchType };
use crossterm::terminal::{disable_raw_mode, enable_raw_mode};


use tui::{
    backend::CrosstermBackend,
    // backend::{ CrosstermBackend, Backend },
    Terminal
};

use std::io;
use std::env;

mod ui;
mod utils;
mod explorer;

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

    let args: Vec<String> = env::args().collect();
    let stop_condition = args.get(1);

    let repo = match Repository::open(".") {
        Ok(repo) => repo,
        Err(e) => panic!("failed to open: {}", e),
    };

    enable_raw_mode().expect("can run in raw mode");

    let stdout = io::stdout();
    // execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    terminal.clear()?;

    match stop_condition {
        Some(stop_condition) => {
            let mut branches = repo.branches(Some(BranchType::Local)).unwrap();
            match branches.find(|b| b.as_ref().unwrap().0.get().shorthand().unwrap().to_string().contains(stop_condition)) {
                Some(Ok((branch, _))) => {
                    let reference = branch.get();
                    let oid = reference.target().unwrap();
                    let shorthand = reference.shorthand().unwrap();
                    ui::explorer_wrapper(&mut terminal, &repo, Some((oid, shorthand.to_string())))?
                },
                _ => ui::explorer_wrapper(&mut terminal, &repo, None)?,
            };
            
        }
        None => ui::explorer_wrapper(&mut terminal, &repo, None)?
    }

    disable_raw_mode()?;
    terminal.show_cursor()?;

    // test_info(&repo);

    Ok(())
}
