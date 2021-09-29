use ggez::graphics::{self, Color};
use ggez::{Context, GameResult};
use ggez::event;
use ggez::conf;
use glam::*;

const BALL_RADIUS: f32 = 50.0;
const SCREEN_WIDTH: f32 = 800.0;
const SCREEN_HEIGHT: f32 = 600.0;

struct GameState {
    pos_x: f32,
    pos_y: f32,
    speed_x: f32,
    speed_y: f32
}

impl GameState {
    fn new() -> GameResult<GameState> {
        let result = GameState { pos_x: 51.0, pos_y: 51.0, speed_x: 1.0, speed_y: 1.0 };
        return Ok(result);
    }
}

impl event::EventHandler<ggez::GameError> for GameState {
    fn update(&mut self, _ctx: &mut Context) -> GameResult {
        self.pos_x += self.speed_x;
        self.pos_y += self.speed_y;

        if (self.pos_x + BALL_RADIUS) > SCREEN_WIDTH || (self.pos_x - BALL_RADIUS) < 0.0 {
            self.speed_x *= -1.0;          
        }

        if (self.pos_y + BALL_RADIUS) > SCREEN_HEIGHT || (self.pos_y - BALL_RADIUS) < 0.0 {
            self.speed_y *= -1.0;
        }
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        graphics::clear(ctx, Color{r: 1.0, g: 1.0, b: 1.0, a: 0.0});

        let circle = graphics::Mesh::new_circle(
            ctx,
            graphics::DrawMode::fill(),
            mint::Point2{ x: 0.0, y: 0.0 },
            BALL_RADIUS,
            2.0,
            Color::from_rgba(50, 50, 50, 200),
        )?;

        graphics::draw(ctx, &circle, (mint::Point2{ x: self.pos_x, y:self.pos_y },))?;

        graphics::present(ctx)?;
        Ok(())
    }
}

fn main() -> GameResult {
    let cb = ggez::ContextBuilder::new("Bouncing ball", "")
        .window_mode(conf::WindowMode::default().dimensions(SCREEN_WIDTH, SCREEN_HEIGHT))
        .window_setup(conf::WindowSetup::default().title("Bouncing ball"));
    let (ctx, event_loop) = cb.build()?;

    let state = GameState::new()?;
    event::run(ctx, event_loop, state);
}
