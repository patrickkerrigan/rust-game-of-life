#![windows_subsystem = "windows"]
use allegro::*;
use allegro_primitives::*;
use allegro::BlendOperation::Add;
use allegro::BlendMode::{Alpha, InverseAlpha};
use allegro::DisplayOption::{SampleBuffers, Samples, Vsync};
use allegro::DisplayOptionImportance::Suggest;
use rand::{thread_rng, Rng};
use crate::cell::{Cell, ProtoCell};

mod cell;

const WIDTH: usize = 300;
const HEIGHT: usize = 150;
const CELL_SIZE: i32 = 7;

type Generation = [[Option<Cell>; WIDTH]; HEIGHT];


fn main()
{
    allegro::run(user_main)
}

fn user_main()
{
    let core = Core::init().unwrap();
    let prim = PrimitivesAddon::init(&core).unwrap();

    core.set_new_display_option(SampleBuffers, 1, Suggest);
    core.set_new_display_option(Samples, 16, Suggest);
    core.set_new_display_option(Vsync, 1, Suggest);
    core.set_new_display_flags(OPENGL);

    let display = Display::new(&core, WIDTH as i32 * CELL_SIZE, HEIGHT as i32 * CELL_SIZE).unwrap();
    core.set_blender(Add, Alpha, InverseAlpha);
    display.set_window_title("Game of Life");

    let timer = Timer::new(&core, 1.0 / 5.0).unwrap();

    core.install_keyboard().unwrap();

    let queue = EventQueue::new(&core).unwrap();
    queue.register_event_source(display.get_event_source());
    queue.register_event_source(timer.get_event_source());
    queue.register_event_source(core.get_keyboard_event_source().unwrap());

    let mut generation = seed();

    let mut redraw = true;
    let mut paused = false;
    timer.start();
    'exit: loop
        {
            match queue.wait_for_event() {
                DisplayClose{..} => break 'exit,

                KeyDown{keycode: k, ..} if k == KeyCode::Escape => break 'exit,
                KeyDown{keycode: k, ..} if k == KeyCode::R => generation = seed(),
                KeyDown{keycode: k, ..} if k == KeyCode::P => paused = !paused,
                KeyDown{keycode: k, ..} if k == KeyCode::PadPlus => timer.set_speed(timer.get_speed() / 2.0),
                KeyDown{keycode: k, ..} if k == KeyCode::PadMinus => timer.set_speed(timer.get_speed() * 2.0),

                TimerTick{..} => redraw = true,

                _ => ()
            }

            if redraw && !paused && queue.is_empty() {
                core.clear_to_color(Color::from_rgb(0, 0, 0));
                draw_cells(&prim, &generation);
                core.flip_display();
                generation = tick(generation);
                redraw = false;
            }
        }
}

#[inline]
fn draw_cells(prim: &PrimitivesAddon, cells: &Generation) {
    for j in 0..HEIGHT {
        for i in 0..WIDTH {
            if let Some(ref cell) = cells[j][i] {
                prim.draw_filled_circle(
                    CELL_SIZE as f32 * i as f32 + 3.0,
                    CELL_SIZE as f32 * j as f32 + 3.0,
                    (CELL_SIZE - 1) as f32 / 2.0,
                    Color::from_rgb(cell.red, cell.green, cell.blue)
                );
            }
        }
    }
}


fn seed() -> Generation {
    let mut generation = [[None; WIDTH]; HEIGHT];

    for j in 0..HEIGHT {
        for i in 0..WIDTH {
            generation[j][i] = if thread_rng().gen_weighted_bool(6) {
                Some(Cell {
                    red: thread_rng().gen_range(50, 255),
                    green: thread_rng().gen_range(50, 255),
                    blue: thread_rng().gen_range(50, 255)
                })
            } else {
                None
            };
        }
    }

    generation
}

#[inline]
fn tick(previous_gen: Generation) -> Generation {
    let mut generation = [[None; WIDTH]; HEIGHT];

    for j in 0..HEIGHT {
        for i in 0..WIDTH {
            let mut neighbours: u16 = 0;
            let mut proto_cell = ProtoCell::new();

            if j > 0 {
                process_cell(&mut neighbours, &mut proto_cell, &previous_gen[j - 1][i]);
                if i > 0 {process_cell(&mut neighbours, &mut proto_cell, &previous_gen[j - 1][i - 1]);}
                if i < WIDTH - 1 {process_cell(&mut neighbours, &mut proto_cell, &previous_gen[j - 1][i + 1]);}
            }
            if j < HEIGHT - 1 {
                process_cell(&mut neighbours, &mut proto_cell, &previous_gen[j + 1][i]);
                if i > 0 {process_cell(&mut neighbours, &mut proto_cell, &previous_gen[j + 1][i - 1]);}
                if i < WIDTH - 1 {process_cell(&mut neighbours, &mut proto_cell, &previous_gen[j + 1][i + 1]);}
            }
            if i > 0 {process_cell(&mut neighbours, &mut proto_cell, &previous_gen[j][i - 1]);}
            if i < WIDTH - 1 {process_cell(&mut neighbours, &mut proto_cell, &previous_gen[j][i + 1]);}

            generation[j][i] = match (&previous_gen[j][i], neighbours) {
                (p@Some(_), n) if n == 2 || n == 3 => *p,
                (None, n) if n == 3 => Some(proto_cell.into_cell(neighbours)),
                _ => None
            }
        }
    }

    generation
}

#[inline]
fn process_cell(n: &mut u16, proto_cell: &mut ProtoCell, cell: &Option<Cell>) {
    match cell {
        Some(c) => {
            *n += 1;
            proto_cell.add(c);
        },
        None => ()
    }
}
