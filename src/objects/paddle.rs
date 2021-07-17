use ggez::graphics::{self, Color, Mesh, Rect};
use ggez::{Context, GameResult};
use glam::*;

pub struct Paddle {
    pos_x: f32,
    pos_y: f32,
    direction: PaddleDirection,
}

pub enum PaddleDirection {
    LEFT,
    RIGHT,
    STILL,
}

impl Paddle {
    pub fn new(pos_x: f32, pos_y: f32, direction: PaddleDirection) -> Self {
        return Paddle {
            pos_x: pos_x,
            pos_y: pos_y,
            direction: direction,
        };
    }

    pub fn get_mesh(&mut self, ctx: &mut Context) -> GameResult<Mesh> {
        return graphics::Mesh::new_rectangle(
            ctx,
            graphics::DrawMode::fill(),
            Rect::new(self.pos_x, self.pos_y, 200.0, 20.0),
            Color::RED,
        );
    }

    pub fn move_paddle(&mut self, direction: PaddleDirection) {

        self.direction = direction;

        self.check_boundary();

        match self.direction {
            PaddleDirection::LEFT => self.pos_x -= 5.0,
            PaddleDirection::RIGHT => self.pos_x += 5.0,
            PaddleDirection::STILL => (),
        }
    }

    fn check_boundary(&mut self) {
        if self.pos_x <= 0.0 {
            self.pos_x += 5.0;
        } else if self.pos_x >= 600.0 {
            self.pos_x -= 5.0;
        }
    }

}
