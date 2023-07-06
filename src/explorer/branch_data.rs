use std::fmt::{Display, Formatter, Result as FmtResult};
use git2::{Oid, Error, Branch, BranchType};

#[derive(Clone)]
enum BranchKind {
    Local,
    Remote,
}

impl From<BranchType> for BranchKind {
    fn from(branch_type: BranchType) -> Self {
        match branch_type {
            BranchType::Local => BranchKind::Local,
            BranchType::Remote => BranchKind::Remote,
        }
    }
}


#[derive(Clone)]
pub struct BranchData {
    oid: Oid,
    shorthand: String,
    kind: Option<BranchKind>,
}

// impl From<(Branch<'_>, BranchType)> for BranchData {
impl From<Result<(Branch<'_>, BranchType), Error>> for BranchData {
    // fn from(b: (Branch, BranchType)) -> Self {
    fn from(b: Result<(Branch, BranchType), Error>) -> Self {
        match b {
            Ok((branch, branch_type)) => {
                let reference = branch.get();
                let shorthand = reference.shorthand().unwrap().to_string();
                let oid = reference.target().unwrap();
                let kind = BranchKind::from(branch_type);
                Self {
                    oid,
                    shorthand,
                    // kind: Some(kind.into()),
                    kind: Some(kind),
                }
            },
            Err(e) => {
                Self {
                    oid: Oid::zero(),
                    shorthand: String::new(),
                    kind: None,
                }
            }
        }
    }
}

impl BranchData {
    // pub fn new(branch: Result<((Branch, BranchType), <E>)>) -> Self {
    pub fn new(branch: Result<(Branch, BranchType), Error>) -> Self {
        match branch {
            Ok((branch, branch_type)) => {
                let reference = branch.get();
                let shorthand = reference.shorthand().unwrap().to_string();
                let oid = reference.target().unwrap();
                let kind = BranchKind::from(branch_type);
                Self {
                    oid,
                    shorthand,
                    // kind: Some(kind.into()),
                    kind: Some(kind),
                }
            },
            Err(e) => {
                Self {
                    oid: Oid::zero(),
                    shorthand: String::new(),
                    kind: None,
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

