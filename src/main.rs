#![allow(dead_code)]

use std::io::Write;

#[derive(Clone, Copy, Debug, PartialEq)]
enum State {
    Alive,
    Dead(u8),
}

impl Default for State {
    fn default() -> Self {
        State::Dead(0)
    }
}

struct Dimensions {
    width: usize,
    height: usize,
}

struct World {
    current: Vec<Vec<State>>,
    next: Vec<Vec<State>>,
    dimensions: Dimensions,
}

impl World {
    pub fn swap_buffers(&mut self) {
        std::mem::swap(&mut self.current, &mut self.next);
    }

    pub fn get_from_next_mut(&mut self, x: isize, y: isize) -> &mut State {
        &mut self.next[y.rem_euclid(self.dimensions.height as isize) as usize]
            [x.rem_euclid(self.dimensions.width as isize) as usize]
    }

    pub fn get_from_current(&self, x: isize, y: isize) -> &State {
        &self.current[y.rem_euclid(self.dimensions.height as isize) as usize]
            [x.rem_euclid(self.dimensions.width as isize) as usize]
    }

    pub fn from_dimensions(dimensions: Dimensions) -> Self {
        let mut current = vec![vec![State::default(); dimensions.width]; dimensions.height];
        for row in current.iter_mut() {
            for cell in row.iter_mut() {
                *cell = if rand::random() {
                    State::Alive
                } else {
                    State::Dead(0)
                };
            }
        }
        World {
            current,
            next: vec![vec![State::default(); dimensions.width]; dimensions.height],
            dimensions,
        }
    }

    pub fn compute_one_step(&mut self) {
        for y in 0..self.dimensions.height {
            for x in 0..self.dimensions.width {
                let mut alive_neighbors = 0;
                for dy in -1..=1 {
                    for dx in -1..=1 {
                        if dx == 0 && dy == 0 {
                            continue;
                        }
                        if *self.get_from_current(x as isize + dx, y as isize + dy) == State::Alive
                        {
                            alive_neighbors += 1;
                        }
                    }
                }
                *self.get_from_next_mut(x as isize, y as isize) = match (
                    self.get_from_current(x as isize, y as isize),
                    alive_neighbors,
                ) {
                    (State::Alive, 2 | 3) => State::Alive,
                    (State::Alive, _) => State::Dead(0),
                    (State::Dead(_), 3) => State::Alive,
                    (State::Dead(cycles), _) => State::Dead(std::cmp::min(u8::MAX - 1, cycles + 1)),
                };
            }
        }
    }
}

struct TermSize {
    height: u16,
    width: u16,
}

fn main() {
    let mut size = unsafe { std::mem::zeroed::<TermSize>() };
    unsafe { libc::ioctl(libc::STDOUT_FILENO, libc::TIOCGWINSZ.into(), &mut size) };

    let mut world = World::from_dimensions(Dimensions {
        width: size.width as usize,
        height: size.height as usize,
    });

    let mut output_lock = std::io::stdout().lock();
    write!(output_lock, "?25l").unwrap();

    loop {
        world.compute_one_step();
        for y in 0..world.dimensions.height {
            for x in 0..world.dimensions.width {
                let current = world.get_from_current(x as isize, y as isize);
                match current {
                    State::Alive => {
                        write!(output_lock, "\x1B[48;5;15m ").unwrap();
                    }
                    State::Dead(cycles) => {
                        write!(
                            output_lock,
                            "\x1B[48;2;{red};{green};{blue}m ",
                            red = (u8::MAX - cycles) / 10,
                            green = (u8::MAX - cycles) / 10,
                            blue = u8::MAX - cycles,
                        )
                        .unwrap();
                    }
                }
            }
        }
        write!(output_lock, "\x1b[1;1H").unwrap();
        world.swap_buffers();
    }
}
