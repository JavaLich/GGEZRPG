use ggez;
use ggez::*;

mod map;
use map::Tile;

struct MainState {
    player: Player,
    map: [[Tile; 30]; 30],
    spritesheet: graphics::Image,
    dx: f32,
}

impl MainState {
    fn new(ctx: &mut Context, spritesheet: graphics::Image) -> GameResult<MainState> {
        let mapp = map::generate(&spritesheet);
        let s = MainState {
            player: Player::new(ctx)?,
            map: mapp,
            spritesheet,
            dx: 0.0,
        };
        Ok(s)
    }
}

impl event::EventHandler for MainState {
    fn update(&mut self, _ctx: &mut Context) -> GameResult {
        self.player.update();
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        graphics::clear(ctx, [1.0, 0.2, 0.3, 1.0].into());
        self.player.draw(ctx);
        self.dx -= 1.0;
        for y in 0..30 {
            for x in 0..30 {
                graphics::draw(
                    ctx,
                    &self.spritesheet,
                    graphics::DrawParam::new()
                        .src(self.map[y][x].src_rect)
                        .dest(nalgebra::Point2::new(
                            (x * 32) as f32 + self.dx,
                            (y * 32) as f32,
                        ))
                        .scale(nalgebra::Vector2::new(2.0, 2.0)),
                )
                .expect("Failed to render tile");
            }
        }
        graphics::present(ctx)?;
        Ok(())
    }
}

pub fn main() -> GameResult {
    let mode = ggez::conf::WindowMode {
        width: 1280f32,
        height: 720f32,
        fullscreen_type: ggez::conf::FullscreenType::Windowed,
        ..Default::default()
    };
    let cb = ggez::ContextBuilder::new("RPG", "Akash Melachuri")
        .window_setup(ggez::conf::WindowSetup::default().title("RPG"))
        .window_mode(mode);
    let (ctx, event_loop) = &mut cb.build()?;
    let mut spritesheet = graphics::Image::new(ctx, "/spritesheet.png")?;
    spritesheet.set_filter(graphics::FilterMode::Nearest);
    let state = &mut MainState::new(ctx, spritesheet)?;
    event::run(ctx, event_loop, state)
}

struct Player {
    img: graphics::Image,
    x: f32,
    y: f32,
}

impl Player {
    fn new(ctx: &mut Context) -> GameResult<Player> {
        Ok(Player {
            img: graphics::Image::new(ctx, "/player.png")?,
            x: 0.0,
            y: 0.0,
        })
    }

    fn update(&mut self) {
        self.x += 0.1;
        self.y += 0.1;
    }

    fn draw(&mut self, ctx: &mut Context) {
        graphics::draw(
            ctx,
            &self.img,
            graphics::DrawParam::new()
                .dest(nalgebra::Point2::new(self.x, self.y))
                .scale(nalgebra::Vector2::new(3.0, 3.0)),
        )
        .expect("Failed to render player");
    }
}
