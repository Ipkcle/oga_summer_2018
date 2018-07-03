use assets::{SpriteName, Sprites};
use ggez::event::*;
use ggez::graphics;
use ggez::graphics::Image;
use ggez::graphics::{Point2, Vector2};
use ggez::timer;
use ggez::{Context, GameResult};

pub static PIXELS_PER_TILE: u32 = 8;
pub static SCALE_FACTOR: f32 = 10.0;
pub fn screen_pixels_per_tile() -> u32 {
    (PIXELS_PER_TILE as f32 * SCALE_FACTOR) as u32
}

mod debug;
mod input;

pub struct MainState {
    screen_w: u32,
    screen_h: u32,
    sprites: Sprites,
    debug_display: debug::DebugTable,
}

impl MainState {
    pub fn new(ctx: &mut Context, screen_w: u32, screen_h: u32) -> GameResult<MainState> {
        graphics::set_default_filter(ctx, graphics::FilterMode::Nearest);
        graphics::set_background_color(ctx, graphics::Color::new(0.0, 0.0, 0.0, 1.0));
        let state = MainState {
            screen_w,
            screen_h,
            sprites: Sprites::new(ctx),
            debug_display: debug::DebugTable::new(ctx, Point2::new(0.0, 0.0)),
        };
        Ok(state)
    }

    fn draw_sprite(&self, ctx: &mut Context, name: SpriteName, tile_x: u32, tile_y: u32) {
        self.sprites.draw(
            ctx,
            name,
            Point2::new(
                (tile_x * screen_pixels_per_tile()) as f32,
                (tile_y * screen_pixels_per_tile()) as f32,
            ),
        );
    }
}

impl EventHandler for MainState {
    fn update(&mut self, ctx: &mut Context) -> GameResult<()> {
        const DESIRED_FPS: u32 = 60;
        while timer::check_update_time(ctx, DESIRED_FPS) {
            let _seconds = 1.0 / (DESIRED_FPS as f32);
        }
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        //clear the contex
        graphics::clear(ctx);

        //load the fps to the debug table
        let fps = timer::get_fps(ctx) as u16;
        self.debug_display.load("fps".to_owned(), fps.to_string());

        //draw test
        for x in (0..10) {
            for y in (0..10) {
                self.draw_sprite(ctx, SpriteName::Floor, x, y);
            }
        }

        self.draw_sprite(ctx, SpriteName::Archer, 1, 1);
        self.draw_sprite(ctx, SpriteName::Wizard, 1, 0);
        self.draw_sprite(ctx, SpriteName::UndeadWizard, 0, 1);
        self.draw_sprite(ctx, SpriteName::Bodyguard, 0, 0);

        //draw the debug table
        self.debug_display.render(ctx);

        //show context on screen
        graphics::present(ctx);

        //yeild cpu when not active
        timer::yield_now();
        Ok(())
    }

    fn key_down_event(&mut self, ctx: &mut Context, keycode: Keycode, _keymod: Mod, _repeat: bool) {
        match keycode {
            Keycode::Escape => ctx.quit().unwrap(),
            _ => (), // Do nothing
        }
    }

    fn key_up_event(&mut self, _ctx: &mut Context, keycode: Keycode, _keymod: Mod, _repeat: bool) {
        match keycode {
            _ => (), // Do nothing
        }
    }
}
