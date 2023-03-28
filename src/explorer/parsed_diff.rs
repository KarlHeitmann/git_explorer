use git2::{Repository, Commit, Oid, DiffHunk, DiffLine, };
use log::{error};
use tui::{
    style::{Color, Style},
    text::{Span, Spans},
};
use crate::utils::short_id;

pub struct ParsedDiff<'a> {
    commit_1_oid: Oid,
    commit_2_oid: Option<Oid>,
    pub test_lines: Vec<Spans<'a>>,
}

pub struct MyDiffLine<'a>(DiffLine<'a>);

impl From<MyDiffLine<'_>> for String {
    fn from(line: MyDiffLine) -> String {
        format!("{}:{}{}",
            line.0.new_lineno().unwrap_or_else(|| line.0.old_lineno().unwrap()),
            line.0.origin().to_string(),
            String::from_utf8(line.0.content().to_vec()).unwrap()
        )
    }
}

impl<'a> From<MyDiffLine<'_>> for Spans<'a> {
    fn from(line: MyDiffLine) -> Spans<'a> {
        let s = format!("{}:{}{}",
            line.0.new_lineno().unwrap_or_else(|| line.0.old_lineno().unwrap()),
            line.0.origin().to_string(),
            String::from_utf8(line.0.content().to_vec()).unwrap()
        );
        let style = match line.0.origin() {
            ' ' => Style::default().fg(Color::White),
            '+' => Style::default().fg(Color::Green),
            '-' => Style::default().fg(Color::Red),
            _ => Style::default().fg(Color::White),
        };
        Spans::from(vec![Span::styled(s, style)])
    }
}

pub struct MyCommit<'a>(Commit<'a>);

impl<'a> From<MyCommit<'_>> for Vec<Spans<'a>> {
    fn from(commit: MyCommit) -> Vec<Spans<'a>> {
        let parents = commit.0.parents().map(|c| short_id(c.id())).collect::<Vec<String>>().join(" - ");

        let message = &commit.0
            .message()
            .unwrap_or("NO COMMIT MESSAGE");
        let committer = format!("Committer: {}", commit.0.committer().to_string());
        let author = format!("Author: {}", commit.0.author());
        let short_id_current_commit = short_id(commit.0.id());
        let parents = format!("PARENTS: {}", parents);
        vec![
            Spans::from(vec![Span::styled(message.to_string(), Style::default().fg(Color::White))]), // TODO: message will not generate spans with new lines // TODO: Can this be replicated for ListItems? To add new lines there with multiple spans?
            Spans::from(vec![Span::styled(committer, Style::default().fg(Color::Red))]),
            Spans::from(vec![Span::styled(author, Style::default().fg(Color::White))]),
            Spans::from(vec![Span::styled(short_id_current_commit, Style::default().fg(Color::White))]),
            Spans::from(vec![Span::styled(parents, Style::default().fg(Color::White))]),
        ]
    }
}

// type MyDiffHunk<'a> = DiffHunk<'a>;
pub struct MyDiffHunk<'a>(DiffHunk<'a>);

impl From<MyDiffHunk<'_>> for String {
    fn from(hunk: MyDiffHunk) -> String {
        String::from_utf8(hunk.0.header().to_vec()).unwrap()
    }
}

impl ParsedDiff<'_> {
    pub fn new(commit_1: Commit, commit_2: Option<Oid>, repo: &Repository) -> Self {
        let commit_1_oid = commit_1.id();
        let commit_2_oid = commit_2;
        let current_commit = commit_1;
        let test_lines;

        let my_current_commit: MyCommit = MyCommit(current_commit.clone());
        let mut diff_spans: Vec<Spans> = my_current_commit.into();

        match commit_2 {
            Some(oid) => {
                let sub_tree_oid_previous = oid;
                let previous_commit = repo.find_commit(sub_tree_oid_previous).unwrap();

                let my_first_diff = repo.diff_tree_to_tree(
                    previous_commit.tree().ok().as_ref(),
                    current_commit.tree().ok().as_ref(),
                    None
                ).unwrap();

                let _foreach_result = my_first_diff.foreach(
                    &mut |delta, _| {
                        let old_file = delta.old_file();
                        let old_file = old_file.path().unwrap();
                        let new_file = delta.new_file();
                        let new_file = new_file.path().unwrap();
                        let tmp = format!("{:?} - {:?}\n", old_file, new_file);
                        true
                    },
                    None,
                    Some(&mut |_, _hunk| {
                        true
                    }),
                    Some(&mut |_, hunk, line| {
                        match hunk {
                            Some(diff_hunk) => {
                                let hunk: MyDiffHunk = MyDiffHunk(diff_hunk);
                                let hunk: String = hunk.into();
                                let line = MyDiffLine(line);
                                let style = Style::default().fg(Color::White);
                                let spans: Spans = line.into();
                                diff_spans.push(spans);
                            }
                            None => {
                                error!("NO DIFF HUNK for {:?}", hunk);
                            }
                        }
                        true
                    }),
                );
            },
            None => {}
        }
        // let t = Text::from(diff_spans);
        test_lines = diff_spans;
        Self {
            commit_1_oid,
            commit_2_oid,
            test_lines,
        }
    }
}


