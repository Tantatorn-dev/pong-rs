use ggez::event;
use ggez::graphics::{self, Color, Rect};
use ggez::{Context, GameResult};
use glam::*;

#[path = "./objects/ball.rs"]
mod ball;
#[path = "./objects/paddle.rs"]
mod paddle;


fn main() -> GameResult {
    
    let cb = ggez::ContextBuilder::new("pong_rs", "Tantatorn S");
    let (ctx, event_loop) = cb.build()?;
    let state = MainState::new()?;
    event::run(ctx, event_loop, state)
}

struct MainState {
    ball_state: ball::BallState,
    paddle_state: paddle::PaddleState,
}

impl MainState {
    fn new() -> GameResult<MainState> {
        let s = MainState {
            ball_state: ball::BallState {
                pos_x: 400.0,
                pos_y: 280.0,
                direction: ball::BallDirection::NORTH,
            },
            paddle_state: paddle::PaddleState {
                pos_x: 300.0,
                pos_y: 500.0,
                direction: paddle::PaddleDirection::STILL,
            },
        };
        Ok(s)
    }
}

impl event::EventHandler<ggez::GameError> for MainState {
    fn update(&mut self, _ctx: &mut Context) -> GameResult {
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        graphics::clear(ctx, [0.1, 0.2, 0.3, 1.0].into());

        let ball = graphics::Mesh::new_circle(
            ctx,
            graphics::DrawMode::fill(),
            Vec2::new(0.0, 0.0),
            20.0,
            2.0,
            Color::WHITE,
        )?;

        let paddle_1 = graphics::Mesh::new_rectangle(
            ctx,
            graphics::DrawMode::fill(),
            Rect::new(
                self.paddle_state.pos_x,
                self.paddle_state.pos_y,
                200.0,
                20.0,
            ),
            Color::RED,
        )?;

        graphics::draw(
            ctx,
            &ball,
            (Vec2::new(self.ball_state.pos_x, self.ball_state.pos_y),),
        )?;
        graphics::draw(
            ctx,
            &paddle_1,
            graphics::DrawParam::default(),
        )?;

        graphics::present(ctx)?;
        Ok(())
    }
}
