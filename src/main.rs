use ggez::{
    Context,
    ContextBuilder,
    GameResult,
    GameError,
    conf::{WindowSetup, WindowMode},
    event::{self, EventHandler},
    graphics::{
        self,
        Font,
        Text,
        Rect,
        Color,
        DrawMode,
        DrawParam,
        set_window_position,
    },
};
use winit::dpi::PhysicalPosition;
use mint::Point2;

const PI: f32 = 3.14159265;
const TWO_PI: f32 = 2. * PI;
const HALF_PI: f32 = PI / 2.;

struct Game {
    text: Text,
    theta: f32,
    rect: graphics::Mesh,
}

impl Game {
    fn new(ctx: &mut Context) -> Self {
        Self {
            text: Text::new(("GGEZ Test!", Font::default(), 300.)),
            theta: 0.,
            rect: graphics::Mesh::new_rectangle(
                ctx,
                DrawMode::fill(),
                center_square(500.),
                Color::CYAN,
            ).unwrap(),
        }
    }

    fn draw_rect(&mut self, ctx: &mut Context) -> GameResult<()> {
        graphics::draw(
            ctx,
            &self.rect,
            DrawParam::new()
                .dest([1200., 900.])
                .rotation(self.theta),
        )
    }

    fn draw_wave(&mut self, ctx: &mut Context) -> GameResult<()> {
        let mut points = vec![];
        for i in 0..2400 {
            let i = i as f32;
            points.push(new_point2(i, (self.theta + i / 200.).sin() * 900. + 900.));
        }
        let line = graphics::Mesh::new_line(
            ctx,
            &points[..],
            10.,
            Color::MAGENTA
        )?;
        graphics::draw(ctx, &line, ([0., 0.],))
    }
}

impl EventHandler<GameError> for Game {
    fn update(&mut self, _ctx: &mut Context) -> GameResult<()> {
        self.theta += 0.04;
        if self.theta > TWO_PI {
            self.theta = 0.;
        }
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        graphics::clear(ctx, Color::BLACK);
        self.draw_wave(ctx)?;
        self.draw_rect(ctx)?;
        graphics::draw(ctx, &self.text, ([50., 50.],))?; // draw text
        graphics::present(ctx)
    }
}

fn center_rect(w: f32, h: f32) -> Rect {
    Rect {
        x: -w / 2.,
        y: -h / 2.,
        w,
        h,
    }
}

fn center_square(size: f32) -> Rect {
    center_rect(size, size)
}

fn new_point2(x: f32, y: f32) -> Point2<f32> {
    Point2 {
        x,
        y,
    }
}

fn main() -> GameResult{
    let (mut ctx, event_loop) = ContextBuilder::new("GGEZ Test", "KermitPurple")
        .window_setup(WindowSetup{
            title: String::from("GGEZ Test"),
            ..Default::default()
        })
        .window_mode(WindowMode{
            width: 2400.,
            height: 1800.,
            ..Default::default()
        })
        .build()?;
    set_window_position(&ctx, PhysicalPosition::new(20, 20))?;
    let game = Game::new(&mut ctx);
    event::run(ctx, event_loop, game);
}

