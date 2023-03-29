#![feature(iter_collect_into)]
#![feature(slice_partition_dedup)]

use explorer::branch_data::BranchData;
use git2::{ Repository, BranchType };
use crossterm::terminal::{disable_raw_mode, enable_raw_mode};

// use log::{trace, LevelFilter, SetLoggerError};
use log::{trace, LevelFilter};
use log4rs::{
    append::{
        console::{ConsoleAppender, Target},
        file::FileAppender,
    },
    config::{Appender, Config, Root},
    encode::pattern::PatternEncoder,
    // filter::threshold::ThresholdFilter,
};



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

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let _level = log::LevelFilter::Info;
    let file_path = "./log";

    // Build a stderr logger.
    let _stderr = ConsoleAppender::builder().target(Target::Stderr).build();

    // Logging to log file.
    let logfile = FileAppender::builder()
        // Pattern: https://docs.rs/log4rs/*/log4rs/encode/pattern/index.html
        .encoder(Box::new(PatternEncoder::new("{l} - {m}\n")))
        .build(file_path)
        .unwrap();

    // Log Trace level output to file where trace is the default level
    // and the programmatically specified level to stderr.
    let config = Config::builder()
        .appender(Appender::builder().build("logfile", Box::new(logfile)))
        .build(
            Root::builder()
                .appender("logfile")
                .build(LevelFilter::Trace),
        )
        .unwrap();

    // Use this to change log levels at runtime.
    // This means you can change the default log level to trace
    // if you are trying to debug an issue and need more logs on then turn it off
    // once you are done.
    let _handle = log4rs::init_config(config)?;

    trace!("\n\n================================== START APPLICATION =======================================\n");
    /*
    error!("Goes to stderr and file");
    warn!("Goes to stderr and file");
    info!("Goes to stderr and file");
    debug!("Goes to file only");
    trace!("Goes to file only");
    */

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
                Some(branch) => {
                    let branch_data = BranchData::new(branch);
                    ui::explorer_wrapper(&mut terminal, &repo, Some(branch_data))?
                }
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
