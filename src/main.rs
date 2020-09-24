extern crate diesel;
#[cfg(test)]
extern crate mockall;

mod data;
mod domain;
mod ui;
use crate::data::repositories::repositories::DBKeepRepository;
use crate::domain::entities::Keep;
use crate::domain::usecases::usecases::*;
use crate::ui::models::KeepItems;
use diesel::prelude::*;
use diesel::sqlite::SqliteConnection;
use std::error::Error;
use std::io::stdin;
use std::{env, io};
use termion::clear;
use termion::event::Key;
use termion::input::TermRead;
use termion::raw::IntoRawMode;
use tui::backend::TermionBackend;
use tui::layout::{Constraint, Direction, Layout};
use tui::style::{Color, Modifier, Style};
use tui::text::Spans;
use tui::widgets::{Block, Borders, List, ListItem};
use tui::Terminal;
use webbrowser;

fn handle_create(url: String, keep_repository: DBKeepRepository) {
    create_keep(Keep { url: url }, keep_repository);
}

fn handle_delete(url: String, keep_repository: DBKeepRepository) {
    delete_keep(Keep { url: url }, keep_repository);
}

type StdOut = termion::raw::RawTerminal<std::io::Stdout>;
type Backend = TermionBackend<StdOut>;
type TypeTerminal = Terminal<Backend>;

#[allow(unused)]
fn handle_get(keep_repository: DBKeepRepository, mut terminal: TypeTerminal) {
    println!("{}", clear::All);
    let result = get_all_keeps(keep_repository);
    let mut keep_items = KeepItems::new(result.clone());

    for c in stdin().keys() {
        terminal.draw(|f| {
            let chunks = Layout::default()
                .direction(Direction::Horizontal)
                .margin(1)
                .constraints([Constraint::Percentage(80)].as_ref())
                .split(f.size());
            let size = f.size();
            let items: Vec<ListItem> = keep_items
                .items
                .iter()
                .map(|keep| {
                    let span = Spans::from(keep.url.clone());
                    ListItem::new(span)
                })
                .collect();

            let list_items = List::new(items)
                .block(Block::default().borders(Borders::ALL).title("Keepy TUI"))
                .highlight_style(
                    Style::default()
                        .bg(Color::LightGreen)
                        .add_modifier(Modifier::BOLD),
                )
                .highlight_symbol("✨");
            f.render_stateful_widget(list_items, chunks[0], &mut keep_items.state);
        });

        //for c in stdin().keys() {
        match c.unwrap() {
            Key::Char('q') => {
                break;
            }
            Key::Down => keep_items.next(),
            Key::Up => keep_items.previous(),
            Key::Left => keep_items.unselect(),
            Key::Right => { 
                let url_to_open = result[keep_items.state.selected().unwrap()].url.as_str();
                webbrowser::open(url_to_open).is_ok(); 
            }
            _ => {}
        }
        //}
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let stdout = io::stdout().into_raw_mode()?;
    let backend = TermionBackend::new(stdout);
    let terminal = Terminal::new(backend)?;
    let db_conn = SqliteConnection::establish("keeps.db").unwrap();
    let keep_repository = DBKeepRepository { db_conn: db_conn };
    let args: Vec<String> = env::args().collect();
    if args.len() > 1 {
        let command: String = args[1].clone();
        match command.as_str() {
            "--create" | "-c" => handle_create(args[2].clone(), keep_repository),
            "--delete" | "-d" => handle_delete(args[2].clone(), keep_repository),
            _ => handle_get(keep_repository, terminal),
        }
    } else {
        handle_get(keep_repository, terminal)
    }
    Ok(())
}
