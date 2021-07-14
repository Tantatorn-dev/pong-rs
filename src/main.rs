use ggez::event;
use ggez::graphics::{self, Color};
use ggez::{Context, GameResult};
use glam::*;

fn main() -> GameResult {
    let cb = ggez::ContextBuilder::new("pong_rs", "Tantatorn S");
    let (ctx, event_loop) = cb.build()?;
    let state = MainState::new()?;
    event::run(ctx, event_loop, state)
}

struct MainState {
    ball_state: BallState,
    paddle_1_state: PaddleState,
    paddle_2_state: PaddleState,
}

struct BallState {
    pos_x: f32,
    pos_y: f32,
    direction: BallDirection,
}

enum BallDirection {
    NORTH,
    SOUTH,
    NORTHEAST,
    NORTHWEST,
    SOUTHEAST,
    SOUTHWEST,
}

struct PaddleState {
    pos_x: f32,
    pos_y: f32,
    direction: PaddleDirection,
}

enum PaddleDirection {
    LEFT,
    RIGHT,
    STILL,
}

impl MainState {
    fn new() -> GameResult<MainState> {
        let s = MainState {
            ball_state: BallState {
                pos_x: 400.0,
                pos_y: 280.0,
                direction: BallDirection::NORTH,
            },
            paddle_1_state: PaddleState {
                pos_x: 0.0,
                pos_y: 380.0,
                direction: PaddleDirection::STILL,
            },
            paddle_2_state: PaddleState {
                pos_x: 0.0,
                pos_y: 180.0,
                direction: PaddleDirection::STILL,
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
            40.0,
            2.0,
            Color::WHITE,
        )?;
        graphics::draw(ctx, &ball, (Vec2::new(self.ball_state.pos_x, self.ball_state.pos_y),))?;

        graphics::present(ctx)?;
        Ok(())
    }
}
