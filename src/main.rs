use ball::BallDirection;
use collision::check_collision;
use ggez::event;
use ggez::graphics::{self, Color, Rect};
use ggez::input::keyboard;
use ggez::{Context, GameResult};
use glam::*;
use paddle::PaddleDirection;

#[path = "./objects/ball.rs"]
mod ball;
#[path = "./objects/paddle.rs"]
mod paddle;
#[path = "./tools/collision.rs"]
mod collision;

fn main() -> GameResult {
    let cb = ggez::ContextBuilder::new("pong_rs", "Tantatorn S");
    let (ctx, event_loop) = cb.build()?;
    let state = MainState::new()?;
    event::run(ctx, event_loop, state)
}

struct MainState {
    ball: ball::Ball,
    paddle: paddle::Paddle,
}

impl MainState {
    fn new() -> GameResult<MainState> {
        let s = MainState {
            ball: ball::Ball::new(400.0, 280.0, BallDirection{
                h_direction: ball::HorizontalDirection::STILL,
                v_direction: ball::VerticalDirection::DOWN,
            }),
            paddle: paddle::Paddle::new(300.0, 500.0, paddle::PaddleDirection::STILL),
        };
        Ok(s)
    }
}

impl event::EventHandler<ggez::GameError> for MainState {
    fn update(&mut self, _ctx: &mut Context) -> GameResult {

        self.ball.move_ball();

        if keyboard::is_key_pressed(_ctx, event::KeyCode::A) {
            self.paddle.move_paddle(paddle::PaddleDirection::LEFT);
        } else if keyboard::is_key_pressed(_ctx, event::KeyCode::D) {
            self.paddle.move_paddle(paddle::PaddleDirection::RIGHT);
        } else {
            self.paddle.direction = PaddleDirection::STILL;
        }

        check_collision(&mut self.ball, &mut self.paddle);

        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        graphics::clear(ctx, [0.1, 0.2, 0.3, 1.0].into());

        let ball = self.ball.get_mesh(ctx)?;
        let paddle_1 = self.paddle.get_mesh(ctx)?;

        graphics::draw(
            ctx,
            &ball,
            graphics::DrawParam::default(),
        )?;
        graphics::draw(
            ctx, 
            &paddle_1, 
            graphics::DrawParam::default()
        )?;

        graphics::present(ctx)?;

        Ok(())
    }
}
