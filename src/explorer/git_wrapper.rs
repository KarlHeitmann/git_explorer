use git2::{Reference, Error, Repository, Branches, Branch, BranchType, ReferenceType};
use crate::explorer::branch_data::BranchData;
use std::process;

pub struct GitWrapper {
    path: Option<String>,
    pub repo: Repository,
}

impl GitWrapper {
    pub fn new(path: Option<String>) -> Self {
        let repo = match path.clone() {
            Some(p) => {
                match Repository::open(&p) {
                    Ok(repo) => repo,
                    Err(e) => { error!("failed to open '{}': {}", p, e); process::exit(0x0100); },
                }
            },
            None => {
                match Repository::open(".") {
                    Ok(repo) => repo,
                    Err(e) => { error!("failed to open current directory: {}", e); process::exit(0x0100); },
                }
            }
        };
        Self {
            path,
            repo,
        }
    }

    pub fn head(&self) -> Result<Reference, Error> {
        self.repo.head()
    }

    pub fn branches(&self) -> Result<Branches, Error> {
        self.repo.branches(Some(BranchType::Local))
    }

    pub fn branches_data(&self, stop_condition: Option<BranchData>) -> Vec<Option<BranchData>> {
        let mut stop_conditions: Vec<Option<BranchData>> = vec![stop_condition];
        match self.repo.head() {
            Ok(head) => {
                for branch in self.repo.branches(Some(BranchType::Local)).unwrap() {
                    // let branch_data = BranchData::new(branch);
                    let branch_data = BranchData::from(branch);
                    let b_string = branch_data.shorthand();
                    let head = head.shorthand().unwrap().to_string();
                    if head.contains(b_string) || b_string.contains(&head) {
                        stop_conditions.push(Some(branch_data));
                    }
                }
            },
            Err(_) => {
                for branch in self.repo.branches(Some(BranchType::Local)).unwrap() {
                    let branch_data = BranchData::new(branch);
                    stop_conditions.push(Some(branch_data));
                }
            }
        };
        stop_conditions
    }
}
