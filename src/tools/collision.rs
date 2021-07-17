use crate::{ball::{Ball, HorizontalDirection, VerticalDirection}, paddle::{Paddle, PaddleDirection}};

pub fn check_collision(ball: &mut Ball, paddle: &mut Paddle) {

	if ball.pos_y >= paddle.pos_y && ball.pos_x >= paddle.pos_x && ball.pos_x <= paddle.pos_x + 200.0 {
		ball.direction.v_direction = VerticalDirection::UP;
		match paddle.direction {
			PaddleDirection::LEFT => ball.direction.h_direction = HorizontalDirection::LEFT,
			PaddleDirection::RIGHT => ball.direction.h_direction = HorizontalDirection::RIGHT,
			PaddleDirection::STILL => ball.direction.h_direction = HorizontalDirection::STILL,
		}
	}


	if ball.pos_x >= 800.0 {
		ball.direction.h_direction = HorizontalDirection::LEFT;
	}
	
	if ball.pos_x <= 0.0 {
		ball.direction.h_direction = HorizontalDirection::RIGHT;
	}

	if ball.pos_y <= 0.0 {
		ball.direction.v_direction = VerticalDirection::DOWN;
		match ball.direction.h_direction {
			HorizontalDirection::LEFT => ball.direction.h_direction = HorizontalDirection::RIGHT,
			HorizontalDirection::RIGHT => ball.direction.h_direction = HorizontalDirection::LEFT,
			HorizontalDirection::STILL => ball.direction.h_direction = HorizontalDirection::STILL,
		}
	}

}