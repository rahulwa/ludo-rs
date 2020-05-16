#![allow(dead_code)]
use std::io;
use std::io::Write;
use termion::raw::IntoRawMode;
use tui::{
    backend::{TermionBackend, Backend},
    Terminal,
    buffer::Buffer,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Style, Modifier},
    widgets::{
        canvas::{Canvas, Context, Line, Rectangle, Points},
        Widget, Block, Borders, BarChart, Clear, Table, Row
    },
};

#[derive(Debug, Clone, Copy, PartialEq)]
enum BoxColor {
    Red,
    Green,
    Blue,
    Yellow,
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum TokenColor {
    Red,
    Green,
    Blue,
    Yellow,
}

impl Into<Color> for TokenColor {
    fn into(self) -> Color {
        match self {
            TokenColor::Red => Color::Red,
            TokenColor::Green => Color::Green,
            TokenColor::Blue => Color::Blue,
            TokenColor::Yellow => Color::Yellow,
        }
    }
}

impl Into<Color> for BoxColor {
    fn into(self) -> Color {
        match self {
            BoxColor::Red => Color::LightRed,
            BoxColor::Green => Color::LightGreen,
            BoxColor::Blue => Color::LightBlue,
            BoxColor::Yellow => Color::LightYellow,
        }
    }
}

#[derive(Debug, Clone)]
struct Box {
    square: Rect,
    color: BoxColor,
}

#[derive(Debug, Clone)]
struct Board
{
    boxes: Vec<Box>,
    playground: Rect,
}


impl Board
{
    fn new(playground: Rect) -> Self {
        let line_color = Color::White;
        let mut board = Board {
            boxes: Vec::new(),
            playground,
        };
        let w = 6; // Todo, currently it panics if terminal horizontal size is less then 90
        let h = 2; // Todo, currently it panics if terminal vertical size is less then 30
        dbg!(w, h, playground);
        // 15x15 boxes
        for x in (0..90).step_by(w) {
            if x < (playground.x + playground.width) {
                for y in (0..30).step_by(h) {
                    if y < (playground.y + playground.height) {
                        board.boxes.push(
                            Box {
                                square: Rect {
                                    x,
                                    y,
                                    width: w as u16,
                                    height: h as u16,
                                },
                                color: BoxColor::Red
                            }
                        );
                    }
                }
            }
        };
        board
    }
}


fn main() -> Result<(), io::Error> {
    let stdout = io::stdout().into_raw_mode()?;
    let backend = TermionBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    terminal.clear()?;
    terminal.hide_cursor()?;

    terminal.draw(|mut f| {
        let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(80), Constraint::Percentage(20)].as_ref())
        .split(f.size());
        let block = Block::default()
        .borders(Borders::ALL);
        let a = Board::new(chunks[0]);
        println!("{:?}", &f.size());
        println!("{:?}", &chunks[0]);
        // let c = a.boxes.iter().map(|z|z.square).collect::<Vec<Rect>>();
        for i in &a.boxes {
            f.render_widget(block, i.square);
        }
    })
}
