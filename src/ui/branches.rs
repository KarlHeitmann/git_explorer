use tui::{
    layout::{Alignment, Rect},
    style::{Color, Style},
    terminal::Frame,
    widgets::{
        Block, Borders, Paragraph, Wrap
    },
    backend::Backend,
};

pub fn render_branches<B: Backend>(
    f: &mut Frame<B>,
    chunks: &mut Vec<Rect>,
    ) {
    let p = Paragraph::new("Branches")
        .block(Block::default().title(format!("Commit COMPLETE")).borders(Borders::ALL))
        .style(Style::default().fg(Color::White).bg(Color::Black))
        .alignment(Alignment::Left)
        .wrap(Wrap { trim: true });

    f.render_widget(p, chunks[1]);

}
