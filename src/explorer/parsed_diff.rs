use git2::{Repository, Commit, Oid};
use tui::{
    style::{Color, Style},
    text::{Span, Spans},
};

use crate::utils::short_id;

pub struct ParsedDiff<'a> {
    commit_1_oid: Oid,
    commit_2_oid: Option<Oid>,
    // pub test_lines: Vec<Text<'a>>,
    pub test_lines: Vec<Spans<'a>>,
    // pub test_lines: Vec<Text<'a>>,
    // pub test_lines: Text<'a>,
    // pub test_lines: Vec<Paragraph<'a>>,
    pub detail: String,
}

impl ParsedDiff<'_> {
    pub fn new(commit_1: Commit, commit_2: Option<Oid>, repo: &Repository) -> Self {
        let commit_1_oid = commit_1.id();
        let commit_2_oid = commit_2;
        let current_commit = commit_1;
        // let mut test_lines = vec![];
        let test_lines;

        let mut detail = String::new();
        let parents = current_commit.parents().map(|c| short_id(c.id())).collect::<Vec<String>>().join(" - ");

        // let message = current_commit.message().clone().unwrap_or("NO COMMIT MESSAGE");
        let message = &current_commit
            // .to_owned()
            .message()
            // .clone()
            .unwrap_or("NO COMMIT MESSAGE");
        let committer = format!("Committer: {}", current_commit.committer().to_string());
        let author = format!("Author: {}", current_commit.author());
        let short_id_current_commit = short_id(current_commit.id());
        let parents = format!("PARENTS: {}", parents);
        let mut diff_spans = vec![
            Spans::from(vec![Span::styled(message.to_string(), Style::default().fg(Color::White))]), // TODO: message will not generate spans with new lines // TODO: Can this be replicated for ListItems? To add new lines there with multiple spans?
            Spans::from(vec![Span::styled(committer, Style::default().fg(Color::Red))]),
            Spans::from(vec![Span::styled(author, Style::default().fg(Color::White))]),
            Spans::from(vec![Span::styled(short_id_current_commit, Style::default().fg(Color::White))]),
            Spans::from(vec![Span::styled(parents, Style::default().fg(Color::White))]),
        ];

        // test_lines.push(t);
        //
        //
        // test_lines.push(Paragraph::from(Text::styled(diff_spans.clone(), Style::default().fg(Color::White))));
        //
        /*
        let text_tmp = vec![
            Spans::from(vec![
                        Span::raw("First"),
                        Span::styled(diff_spans.clone(), Style::default().fg(Color::Magenta)),
            ])
        ];
        //
        test_lines.push(Paragraph::new(text_tmp));
        */
        // detail.push_str(&diff_spans);

        let mut string_0 = String::from("FD\n");
        let mut string_a = String::new();
        let mut string_b = String::new();

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
                        string_0.push_str(&tmp);
                        true
                    },
                    None,
                    Some(&mut |_, _hunk| {
                        /*
                        let s = format!("{}\n",
                            String::from_utf8(hunk.diff_spans().to_vec()).unwrap()
                        );
                        string_a.push_str(&s);
                        */
                        // string_a = String::from_utf8(hunk.diff_spans().to_vec()).unwrap();
                        true
                    }),
                    Some(&mut |_, hunk, line| {
                        match hunk {
                            Some(hunk) => {
                                let hunk = String::from_utf8(hunk.header().to_vec()).unwrap();
                                if string_a == hunk {
                                    let s = format!("{}:{}{}",
                                        line.new_lineno().unwrap_or_else(|| line.old_lineno().unwrap()),
                                        line.origin().to_string(),
                                        String::from_utf8(line.content().to_vec()).unwrap()
                                    );
                                    // test_lines.push(Text::styled(s.clone(), Style::default().fg(Color::Green)));
                                    // test_lines.push(Spans::from(vec![Span::styled(s.clone(), Style::default().fg(Color::White))]));
                                    let style = match line.origin() {
                                        ' ' => Style::default().fg(Color::White),
                                        '+' => Style::default().fg(Color::Green),
                                        '-' => Style::default().fg(Color::Red),
                                        _ => Style::default().fg(Color::White),
                                    };
                                    diff_spans.push(Spans::from(vec![Span::styled(s.clone(), style)])); // TODO: THERE IS MISSING DATA, string_a is not being added to diff_spans
                                    string_b.push_str(&s);
                                } else {
                                    let s = format!("{}{}:{}{}",
                                        hunk,
                                        line.new_lineno().unwrap_or_else(|| line.old_lineno().unwrap()),
                                        line.origin().to_string(),
                                        String::from_utf8(line.content().to_vec()).unwrap()
                                    );
                                    // test_lines.push(Text::styled(s.clone(), Style::default().fg(Color::Red)));
                                    // test_lines.push(Spans::from(vec![Span::styled(s.clone(), Style::default().fg(Color::White))]));
                                    let style = match line.origin() {
                                        ' ' => Style::default().fg(Color::White),
                                        '+' => Style::default().fg(Color::Green),
                                        '-' => Style::default().fg(Color::Red),
                                        _ => Style::default().fg(Color::White),
                                    };
                                    diff_spans.push(Spans::from(vec![Span::styled(s.clone(), style)]));
                                    string_b.push_str(&s);
                                }
                                string_a = hunk;
                            }
                            None => {
                                let s = format!("{}:{}{}",
                                    line.new_lineno().unwrap_or_else(|| line.old_lineno().unwrap()),
                                    line.origin().to_string(),
                                    String::from_utf8(line.content().to_vec()).unwrap()
                                );
                                // test_lines.push(Text::styled(s.clone(), Style::default().fg(Color::Yellow)));
                                // test_lines.push(Spans::from(vec![Span::styled(s.clone(), Style::default().fg(Color::White))]));
                                let style = match line.origin() {
                                    ' ' => Style::default().fg(Color::White),
                                    '+' => Style::default().fg(Color::Green),
                                    '-' => Style::default().fg(Color::Red),
                                    _ => Style::default().fg(Color::White),
                                };
                                diff_spans.push(Spans::from(vec![Span::styled(s.clone(), style)]));
                                string_b.push_str(&s);
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
        detail.push_str(&string_0);
        detail.push_str(&string_a);
        detail.push_str(&string_b);
        Self {
            commit_1_oid,
            commit_2_oid,
            test_lines,
            detail,
        }
    }
}


