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

#[derive(Debug, Clone, Copy)]
enum TeamColor {
    Red,
    Green,
    Blue,
    Yellow,
}

// impl From<Color> for TeamColor {
//     fn from(color: Color) -> Self {
//         match color {
//             Color::Red => TeamColor::Red,
//             Color::Green => TeamColor::Green,
//             Color::Blue => TeamColor::Blue,
//             Color::Yellow => TeamColor::Yellow,
//             _ => panic!("Unrecognied color")
//         }
//     }
// }

impl Into<Color> for TeamColor {
    fn into(self) -> Color {
        match self {
            TeamColor::Red => Color::Red,
            TeamColor::Green => Color::Green,
            TeamColor::Blue => Color::Blue,
            TeamColor::Yellow => Color::Yellow,
        }
    }
}

#[derive(Debug, Clone)]
struct Box {
    square: Rectangle,
    color: TeamColor,
}

#[derive(Debug, Clone)]
struct Board
{
    boxes: Vec<Box>,
    playground: Rect,
}

// impl Box {
//     fn get_coords(&self) -> vec<Points> {
//         let points = Vec::new();
//         let x_start = self.square.x as u32;
//         let x_end = self.square.x as u32 + self.square.width as u32;
//         let y_start = self.square.y as u32;
//         let y_end = self.square.y as u32 + self.square.height as u32;        
//         for i in x_start..x_end+1 {
//             points
//         }
//     }
// }

impl Board
{
    fn new() -> Self {
        let line_color = Color::White;
        let mut board = Board {
            boxes: Vec::new(),
            playground: Rect::new(0, 0, 150, 150),
        };
        for i in (0..=150).step_by(10) {
            if i <= 150 {
                for j in (0..=150).step_by(10) {
                    board.boxes.push(
                        Box {
                            square: Rectangle {
                                x: i as f64,
                                y: j as f64,
                                width: 10.0,
                                height: 10.0,
                                color: line_color,
                            },
                            color: TeamColor::Red
                        }
                    );
                }
            };
        };
        board
    }
}

impl Widget for Board {
    fn render(self, area: Rect, buf: &mut Buffer) {
        // let boxes = self.boxes.clone();
        // let canvas = Canvas::default()
        // .block(Block::default().borders(Borders::ALL).title("Ludo Board"))
        // .paint(move |ctx| {
        //     for i in &boxes {
        //         ctx.draw(&i.square);
        //     }
        // })
        // .x_bounds([0.0, 150.0])
        // .y_bounds([0.0, 150.0])
        // .background_color(Color::Red);
        // canvas.render(area, buf);
        /////////////////////////////////// 
            
        for i in &self.boxes {
            Canvas::default()
            .block(Block::default().borders(Borders::ALL).title("Ludo Board"))
            .paint(|ctx| {
                ctx.draw(&i.square);
            })
            .x_bounds([0.0, 150.0])
            .y_bounds([0.0, 150.0])
            .background_color(i.color.into())
            .render(area, buf);
        }
    }
}

fn main() -> Result<(), io::Error> {
    let stdout = io::stdout().into_raw_mode()?;
    let backend = TermionBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    terminal.clear()?;
    terminal.hide_cursor()?;

    let a = Board::new();

    terminal.draw(|mut f| {
        let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Percentage(80), Constraint::Percentage(20)].as_ref())
        .split(f.size());
        f.render_widget(a, chunks[0]);
    })
}

// let canvas = Canvas::default()
// .block(Block::default().borders(Borders::ALL).title("Ludo Board"))
// .paint(move |ctx| {
//     for i in &boxes {
//         ctx.draw(i);
//     }
// })
// .x_bounds([0.0, 150.0])
// .y_bounds([0.0, 150.0])
// .background_color(Color::Red);
// canvas.render(area, buf);


// for i in &self.boxes {
//     Canvas::default()
//     .block(Block::default().borders(Borders::ALL).title("Ludo Board"))
//     .paint(move |ctx| {
//         ctx.draw(&i.square);
//     })
//     .background_color(i.color.into())
//     .render(area, buf);
// }