use sdl2::{pixels::Color, rect::Rect, render::WindowCanvas};

pub const THICKNESS: u32 = 15;
pub const PADDLE_SIZE: u32 = 120;

#[derive(Clone)]
struct Vector2 {
    x: f32,
    y: f32,
}

#[derive(Clone)]
pub struct Game {
    bottom_wall: Rect,
    top_wall: Rect,
    right_wall: Rect,
    ball_pos: Vector2,
    paddle_pos: Vector2,
}

fn ball_rect(ball_pos: &Vector2) -> Rect {
    Rect::new(
        (ball_pos.x - (THICKNESS as f32 / 2.0)) as i32,
        (ball_pos.y - (THICKNESS as f32 / 2.0)) as i32,
        THICKNESS,
        THICKNESS,
    )
}

fn paddle_rect(paddle_pos: &Vector2) -> Rect {
    Rect::new(
        (paddle_pos.x - (THICKNESS as f32 / 2.0)) as i32,
        (paddle_pos.y - (PADDLE_SIZE as f32 / 2.0)) as i32,
        THICKNESS,
        PADDLE_SIZE,
    )
}

impl Game {
    pub fn new() -> Self {
        let top_wall = Rect::new(0, 0, 1024, THICKNESS);
        let bottom_wall = Rect::new(0, 768 - THICKNESS as i32, 1024, THICKNESS);
        let right_wall = Rect::new(1024 - THICKNESS as i32, 0, THICKNESS, 768);
        let ball_pos = Vector2 {
            x: 1024.0 / 2.0,
            y: 768.0 / 2.0,
        };
        let paddle_pos = Vector2 {
            x: (THICKNESS as f32),
            y: 768.0 / 2.0,
        };

        Self {
            right_wall,
            bottom_wall,
            top_wall,
            ball_pos,
            paddle_pos,
        }
    }

    pub fn render(&self, canvas: &mut WindowCanvas) {
        let walls = vec![self.top_wall, self.bottom_wall, self.right_wall];

        canvas.set_draw_color(Color::RGBA(0, 0, 255, 255));
        canvas.clear();

        canvas.set_draw_color(Color::RGBA(255, 255, 255, 255));
        canvas.fill_rects(&walls[..]).unwrap();

        canvas.fill_rect(ball_rect(&self.ball_pos));
        canvas.fill_rect(paddle_rect(&self.paddle_pos));

        canvas.present();
    }
}
