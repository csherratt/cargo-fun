extern crate crossterm;
extern crate rand;

use std::io::{Write, stdout};
use std::time::Duration;
use crossterm::style::{style, PrintStyledContent, Color};
use crossterm::style::Color::*;
use crossterm::{QueueableCommand, ExecutableCommand, terminal, cursor};
use rand::Rng;
use rand::prelude::SliceRandom;

struct Drawable {
    x: f32,
    y: f32,
    color: Color,
    what: DrawableType
}

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
enum DrawableType {
   Streamer, Balloon, Confetti
}

const BALLOON: &'static str = " .───.
;█████:
:█████;
 ╲███╱
  `─'
   )
   (
   )
   (
   )
";

const COLORS: &'static [Color] = &[Red, Green, Yellow, Blue, Magenta, Cyan];
const CONFETTI: &'static [char] = &['\\', '/', '-', '|'];

impl Drawable {
    fn draw(&self, height: i32, into: &mut std::io::Stdout) {
        let mut rng = rand::thread_rng();

        match self.what {
            DrawableType::Streamer => {
                for y in (self.y as i32 - 4)..(self.y as i32) {
                    if y < 0 || y > height {
                        continue;
                    }
                    into.queue(cursor::MoveTo(self.x as u16, y as u16)).unwrap();
                    if rng.gen_bool(0.5) {
                        into.queue(PrintStyledContent(style("(").with(self.color))).unwrap();
                    } else {
                        into.queue(PrintStyledContent(style(")").with(self.color))).unwrap();
                    }
                }
            }
            DrawableType::Balloon => {
                for (i, line) in BALLOON.lines().enumerate() {
                    let y = i as i32 + self.y as i32;
                    if y < 0 || y > height {
                        continue;
                    }
                    into.queue(cursor::MoveTo(self.x as u16, y as u16)).unwrap();
                    for char in line.chars() {
                        if char == ' ' {
                            into.queue(cursor::MoveRight(1)).unwrap();
                        } else {
                            into.queue(PrintStyledContent(style(char).with(self.color))).unwrap();
                        }
                    }
                }
            }
            DrawableType::Confetti => {
                let y = self.y as i32;
                if y < 0 || y > height {
                    return;
                }
                let index = rng.gen_range(0, CONFETTI.len());
                into.queue(cursor::MoveTo(self.x as u16, y as u16)).unwrap();
                into.queue(PrintStyledContent(style(CONFETTI[index]).with(self.color))).unwrap();
            }
        }
    }
}

fn render(_width: u16, height: u16, drawables: &Vec<Drawable>) -> crossterm::Result<()> {
    let mut stdout = stdout();

    stdout.queue(terminal::Clear(terminal::ClearType::All))?;

    for drawable in drawables {
       drawable.draw(height as i32, &mut stdout);
    }

    stdout.flush().unwrap();
    Ok(())
}

fn add_streamers(width: u16, height: u16, drawables: &mut Vec<Drawable>) {
    let mut rng = rand::thread_rng();

    for _ in 0..64 {
        let x = rng.gen_range(0, width);
        let y = rng.gen_range(-(height as f32) * 0.75, 0f32);
        let color = COLORS[rng.gen_range(0, COLORS.len())];
        drawables.push(Drawable {
            x: x as f32,
            y,
            color,
            what: DrawableType::Streamer
        });
    }
}

fn add_balloons(width: u16, height: u16, drawables: &mut Vec<Drawable>) {
    let mut rng = rand::thread_rng();

    for _ in 0..16 {
        let x = rng.gen_range(4f32, width as f32 - 4.);
        let y = rng.gen_range(0f32, height as f32);
        let color = COLORS[rng.gen_range(0, COLORS.len())];
        drawables.push(Drawable {
            x: x as f32,
            y: y + height as f32,
            color,
            what: DrawableType::Balloon
        });
    }
}

fn add_confetti(width: u16, height: u16, drawables: &mut Vec<Drawable>) {
    let mut rng = rand::thread_rng();

    for _ in 0..256 {
        let x = rng.gen_range(0, width);
        let y = rng.gen_range(-(height as f32) * 0.75, 0f32);
        let color = COLORS[rng.gen_range(0, COLORS.len())];
        drawables.push(Drawable {
            x: x as f32,
            y,
            color,
            what: DrawableType::Confetti
        });
    }
}

fn move_everything(drawables: &mut Vec<Drawable>) {
    let count = drawables.len() as f32;
    for (i, drawable) in drawables.iter_mut().enumerate() {
        let rate = 0.5f32 + 2.0 * (i as f32) / count;
        match drawable.what {
            DrawableType::Streamer | DrawableType::Confetti => {
                drawable.y += rate;
            },
            DrawableType::Balloon => {
                drawable.y -= rate;
            }
        }
    }
}

fn main() {
    let (width, height) = terminal::size().unwrap();
    let mut drawables = vec![];

    add_streamers(width, height, &mut drawables);
    add_balloons(width, height, &mut drawables);
    add_confetti(width, height, &mut drawables);

    drawables.shuffle(&mut rand::thread_rng());

    for _ in 0..50 {
        render(width, height, &drawables).unwrap();
        move_everything(&mut drawables);
        std::thread::sleep(Duration::from_millis(60));
    }

    stdout().execute(terminal::Clear(terminal::ClearType::All)).unwrap();
    stdout().execute(cursor::MoveTo(0, 0)).unwrap();
}
