use sdl2::event::Event;
use sdl2_game3d_sample::game::Game;

fn main() {
    let sdl2_context = sdl2::init().unwrap();
    let video_subsystem = sdl2_context.video().unwrap();
    let mut event_pump = sdl2_context.event_pump().unwrap();

    let window = video_subsystem
        .window("SDL 3D", 1024, 768)
        .position_centered()
        .build()
        .unwrap();
    let mut canvas = window.into_canvas().accelerated().build().unwrap();

    let texture_creator = canvas.texture_creator();

    let mut running = true;
    let game = Game::new();

    while running {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. } => {
                    running = false;
                }
                _ => {}
            }
        }

        game.render(&mut canvas);
    }
}
