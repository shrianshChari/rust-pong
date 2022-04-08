use macroquad::prelude::*;

struct MainState {
    ball: Rect,
    top_paddle: Rect,
    bottom_paddle: Rect,
    x_vel: f32,
    y_vel: f32,
    top_score: i32,
    bot_score: i32,
}

impl MainState {
    fn draw(&self) {
        // Draw the ball
        draw_rectangle(
            self.ball.x,
            self.ball.y,
            self.ball.w,
            self.ball.h,
            WHITE,
        );

        // Top paddle
        draw_rectangle(self.top_paddle.x,
            self.top_paddle.y,
            self.top_paddle.w,
            self.top_paddle.h,
            WHITE);

        // Bottom paddle
        draw_rectangle(self.bottom_paddle.x,
            self.bottom_paddle.y,
            self.bottom_paddle.w,
            self.bottom_paddle.h,
            WHITE);
    
        // Drawing scores
        draw_text(&self.top_score.to_string(),
            15., self.top_paddle.y + self.top_paddle.h + 20., 36., WHITE);
        draw_text(&self.bot_score.to_string(),
            15., self.bottom_paddle.y - 15., 36., WHITE);
    }

    fn update(&mut self) {
        self.ball.x += self.x_vel;
        self.ball.y += self.y_vel;

        // We do be balling
        if self.ball.top() <= 0.0 ||
        self.ball.bottom() >= screen_height() {
            self.y_vel *= -1.;

            if self.ball.top() <= 0.0 {
                self.bot_score += 1;
            }
            if self.ball.bottom() >= screen_height() {
                self.top_score += 1;
            }

            // Resets ball position
            self.ball.x = screen_width() / 2.;
            self.ball.y = screen_height() / 2.;

        }
        if self.ball.right() >= screen_width() ||
        self.ball.left() <= 0.0 {
            self.x_vel *= -1.;

            // Preventing the ball from getting softlocked
            if self.ball.left() <= 0.0 {
                self.ball.x = 0.1;
            } else if self.ball.right() >= screen_width() {
                self.ball.x = screen_width() - self.ball.w - 0.1;
            }
        }

        if self.ball.overlaps(&self.top_paddle) ||
            self.ball.overlaps(&self.bottom_paddle) {
            self.y_vel *= -1.;
        }

        // Moving top paddle w/ arrow keys
        if is_key_down(KeyCode::Right) {
            self.top_paddle.x += 10.;
        }
        if is_key_down(KeyCode::Left) {
            self.top_paddle.x -= 10.;
        }

        // Making sure paddle doesn't go off screen
        while self.top_paddle.left() <= 0.0 {
            self.top_paddle.x += 0.1;
        }
        
        while self.top_paddle.right() >= screen_width() {
            self.top_paddle.x -= 0.1;
        }


        // Moving bottom paddle w/ A and D
        if is_key_down(KeyCode::D) {
            self.bottom_paddle.x += 10.;
        }
        if is_key_down(KeyCode::A) {
            self.bottom_paddle.x -= 10.;
        }

        // Making sure paddle doesn't go off screen
        while self.bottom_paddle.left() <= 0.0 {
            self.bottom_paddle.x += 0.1;
        }
        
        while self.bottom_paddle.right() >= screen_width() {
            self.bottom_paddle.x -= 0.1;
        }
    }
}

#[macroquad::main("Pong")]
async fn main() {
    let mut main_state = MainState {
        ball: Rect::new(screen_width() / 2.0,
            screen_height() / 2.0,
            10.0,
            10.0),
        top_paddle: Rect::new(screen_width() / 2.0,
            0.,
            100.,
            15.),
        bottom_paddle: Rect::new(screen_width() / 2.0,
            screen_height() - 15.,
            100.0,
            15.0),
        x_vel: 5.0,
        y_vel: -5.0,
        top_score: 0,
        bot_score: 0,
    };

    // Equivalent to while (true)
    loop {
        clear_background(BLACK);

        main_state.draw();
        main_state.update();

        // Goes to the next frame
        next_frame().await;
    }
}


