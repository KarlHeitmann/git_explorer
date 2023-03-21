use git2::{Repository, Commit, Oid, Time, Branches, Branch, BranchType};
use std::fmt::{Display, Formatter, Result as FmtResult};
use crate::utils::short_id;

#[derive(Clone)]

pub struct GraphNode {
    pub grapheme: String,
    pub oid: Oid,
    pub branch_shorthand: Option<String>,
    pub summary: String,
    // TODO: add commit summary
}

impl GraphNode {
    pub fn id(&self) -> Oid {
        self.oid
    }
}
 
impl Display for GraphNode {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        let (grapheme, oid, branch_shorthand, summary) = (&self.grapheme, &self.oid, &self.branch_shorthand, &self.summary);
        let branch_shorthand = match branch_shorthand {
            Some(branch_shorthand) => { format!("[{}]", branch_shorthand) },
            None => { String::new() },
        };
        write!(f, "{} ({}) {} {}", grapheme, short_id(*oid), branch_shorthand, summary)
    }
}

