#![allow(dead_code)]

use std::io::Write;
use std::num::NonZeroU8;

#[derive(Clone, Copy, Debug, PartialEq)]
enum State {
    Alive,
    Dead(NonZeroU8),
}

impl Default for State {
    fn default() -> Self {
        State::Dead(NonZeroU8::new(1).unwrap())
    }
}

struct Dimensions {
    width: usize,
    height: usize,
}

struct World {
    buffer: *mut [State; 2],
    dimensions: Dimensions,
}

unsafe fn compute_one_step(
    buffer: *mut [State; 2],
    width: usize,
    height: usize,
    cycle: &mut u8,
    mut output_buffer: impl std::io::Write,
) {
    let total_count = width * height;
    let position_current = (*cycle & 1) as usize;
    let position_next = 1 - position_current;

    for index in 0..total_count {
        let mut alive_neighbours = 0;
        let current_cell = (*buffer
            .offset(((total_count + index - width - 1) % total_count) as isize))[position_current];
        if matches!(current_cell, State::Alive) {
            alive_neighbours += 1;
        }

        let current_cell = (*buffer.offset(((total_count + index - width) % total_count) as isize))
            [position_current];
        if matches!(current_cell, State::Alive) {
            alive_neighbours += 1;
        }

        let current_cell = (*buffer
            .offset(((total_count + index - width + 1) % total_count) as isize))[position_current];
        if matches!(current_cell, State::Alive) {
            alive_neighbours += 1;
        }

        let current_cell =
            (*buffer.offset(((total_count + index - 1) % total_count) as isize))[position_current];
        if matches!(current_cell, State::Alive) {
            alive_neighbours += 1;
        }

        let current_cell =
            (*buffer.offset(((total_count + index + 1) % total_count) as isize))[position_current];
        if matches!(current_cell, State::Alive) {
            alive_neighbours += 1;
        }

        let current_cell = (*buffer
            .offset(((total_count + index + width - 1) % total_count) as isize))[position_current];
        if matches!(current_cell, State::Alive) {
            alive_neighbours += 1;
        }

        let current_cell = (*buffer.offset(((total_count + index + width) % total_count) as isize))
            [position_current];
        if matches!(current_cell, State::Alive) {
            alive_neighbours += 1;
        }

        let current_cell = (*buffer
            .offset(((total_count + index + width + 1) % total_count) as isize))[position_current];
        if matches!(current_cell, State::Alive) {
            alive_neighbours += 1;
        }

        let next_cell =
            &mut (*buffer.offset(((total_count + index) % total_count) as isize))[position_next];

        *next_cell = match (
            (*buffer.offset(index as isize))[position_current],
            alive_neighbours,
        ) {
            (State::Alive, 2 | 3) => {
                write!(output_buffer, "\x1B[48;5;15m ").unwrap();
                State::Alive
            }
            (State::Alive, _) => {
                write!(output_buffer, "\x1B[48;5;15m ").unwrap();
                State::Dead(NonZeroU8::new(1).unwrap())
            }
            (State::Dead(cycles), 3) => {
                write!(
                    output_buffer,
                    "\x1B[48;2;{red};{green};{blue}m ",
                    red = (u8::MAX - cycles.get()) / 10,
                    green = (u8::MAX - cycles.get()) / 10,
                    blue = u8::MAX - cycles.get(),
                )
                .unwrap();
                State::Alive
            }
            (State::Dead(cycles), _) => {
                write!(
                    output_buffer,
                    "\x1B[48;2;{red};{green};{blue}m ",
                    red = (u8::MAX - cycles.get()) / 10,
                    green = (u8::MAX - cycles.get()) / 10,
                    blue = u8::MAX - cycles.get(),
                )
                .unwrap();
                State::Dead(NonZeroU8::new(std::cmp::min(u8::MAX - 1, cycles.get() + 1)).unwrap())
            }
        }
    }
    *cycle = cycle.wrapping_add(1);
}
impl World {
    pub fn from_dimensions(dimensions: Dimensions) -> Self {
        let mut buffer = (0..dimensions.height * dimensions.width)
            .map(|_| {
                if rand::random() {
                    [State::Alive, State::Dead(NonZeroU8::new(1).unwrap())]
                } else {
                    [
                        State::Dead(NonZeroU8::new(1).unwrap()),
                        State::Dead(NonZeroU8::new(1).unwrap()),
                    ]
                }
            })
            .collect::<Vec<_>>();

        let buffer_ptr = buffer.as_mut_ptr();
        std::mem::forget(buffer);

        World {
            buffer: buffer_ptr,
            dimensions,
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

    let world = World::from_dimensions(Dimensions {
        width: size.width as usize,
        height: size.height as usize,
    });

    let mut output_lock = std::io::stdout().lock();
    write!(output_lock, "\x1b[?25l").unwrap();

    let mut cycles = 0u8;
    unsafe {
        loop {
            compute_one_step(
                world.buffer,
                world.dimensions.width,
                world.dimensions.height,
                &mut cycles,
                &mut output_lock,
            );
            write!(output_lock, "\x1b[1;1H").unwrap();
        }
    }
}
