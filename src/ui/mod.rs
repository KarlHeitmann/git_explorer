use crossterm::event::{self, Event, KeyCode};
use std::io::Stdout;
use std::io;

use tui::{
    backend::CrosstermBackend,
    Terminal
};

#[derive(Copy, Clone, Debug)]
pub enum MenuItem {
    Home,
    Nodes,
    Edit,
    SubSearch
}

impl From<MenuItem> for usize {
    fn from(input: MenuItem) -> usize {
        match input {
            MenuItem::Home => 0,
            MenuItem::Nodes => 1,
            MenuItem::Edit => 2,
            MenuItem::SubSearch => 3,
        }
    }
}

use tui::{
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Span, Spans},
    widgets::{
        Block, BorderType, Borders, Paragraph, Tabs,
    },
};

pub fn get_layout_chunks(size: Rect) -> Vec<Rect> {
    Layout::default()
        .direction(Direction::Vertical)
        .margin(2)
        .constraints(
            [
                Constraint::Length(3),
                Constraint::Min(2),
                Constraint::Length(3),
            ]
            .as_ref(),
        )
        .split(size)
}

pub fn draw_status_bar<'layout>() -> Paragraph<'layout> {
    let (title, color) = ("NORMAL MODE +++FILTER MODE CONTAIN+++", Color::LightCyan);

    Paragraph::new(title)
        .style(Style::default().fg(color))
        .alignment(Alignment::Center)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .style(Style::default().fg(Color::White))
                .title("Status")
                .border_type(BorderType::Plain),
        )
}

pub fn draw_menu_tabs<'a>(menu_titles: &'a Vec<&'a str>, active_menu_item: MenuItem) -> Tabs<'a> {
    let menu = menu_titles
        .iter()
        .map(|t| {
            let (first, rest) = t.split_at(1);
            Spans::from(vec![
                Span::styled(
                    first,
                    Style::default()
                        .fg(Color::Yellow)
                        .add_modifier(Modifier::UNDERLINED),
                ),
                Span::styled(rest, Style::default().fg(Color::White)),
            ])
        })
        .collect();

    Tabs::new(menu)
        .select(active_menu_item.into())
        .block(Block::default().title("Menu").borders(Borders::ALL))
        .style(Style::default().fg(Color::White))
        .highlight_style(Style::default().fg(Color::Yellow))
        .divider(Span::raw("|"))
}

pub fn render_home<'a>(data: &'a Vec<String>) -> Paragraph<'a> {
    let home = Paragraph::new(data.iter().map(|d| Spans::from(vec![Span::raw(d)])).collect::<Vec<Spans>>())
    .alignment(Alignment::Left)
    .block(
        Block::default()
            .borders(Borders::ALL)
            .style(Style::default().fg(Color::White))
            .title("Home")
            .border_type(BorderType::Plain),
    );
    home
}

pub fn explorer_wrapper(terminal: &mut Terminal<CrosstermBackend<Stdout>>, data: Vec<String>) -> Result<(), Box<dyn std::error::Error>> {
    let stdout = io::stdout();
    let backend = CrosstermBackend::new(stdout);
    let menu_titles = vec!["Home", "Quit"];
    let active_menu_item = MenuItem::Home;
    let mut terminal = Terminal::new(backend)?;
    terminal.clear()?;
    loop {
        terminal.draw(|rect| {
            // let explorer = &mut explorer;
            let chunks = get_layout_chunks(rect.size());

            let status_bar = draw_status_bar();

            let tabs = draw_menu_tabs(&menu_titles, active_menu_item);

            rect.render_widget(tabs, chunks[0]);
            rect.render_widget(render_home(&data), chunks[1]);
            rect.render_widget(status_bar, chunks[2]);
        })?;

        // let terminal = &mut terminal;
        if let Event::Key(key) = event::read()? {
            if key.code == KeyCode::Char('q') {
                break;
            }
        }
    }

    Ok(())
}


