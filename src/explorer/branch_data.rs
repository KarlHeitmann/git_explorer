use std::fmt::{Display, Formatter, Result as FmtResult};
use git2::{Oid, Error, Branch, BranchType};

#[derive(Clone)]
pub struct BranchData {
    oid: Oid,
    shorthand: String,
}

impl BranchData {
    // pub fn new(branch: Result<((Branch, BranchType), <E>)>) -> Self {
    pub fn new(branch: Result<(Branch, BranchType), Error>) -> Self {
        match branch {
            Ok((branch, branch_type)) => {
                let reference = branch.get();
                let shorthand = reference.shorthand().unwrap().to_string();
                let oid = reference.target().unwrap();
                Self {
                    oid,
                    shorthand,
                }
            },
            Err(e) => {
                Self {
                    oid: Oid::zero(),
                    shorthand: String::new(),
                }
            }
        }
    }

    pub fn shorthand(&self) -> &String { &self.shorthand }

    pub fn oid(&self) -> Oid { self.oid }
}

impl Display for BranchData {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        write!(f, "{}, ", self.shorthand)
    }
}

