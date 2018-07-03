use ggez::graphics;
use ggez::graphics::{DrawParam, Image, Point2, Rect};
use ggez::Context;
use main_state::PIXELS_PER_TILE;
use main_state::SCALE_FACTOR;
use std::collections::hash_map::HashMap;
use std::path::Path;

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub enum SpriteName {
    Archer,
    Wizard,
    Warrior,
    Scholar,
    Necromancer,
    Defender,
    Monk,
    Champion,
    King,
    Bodyguard,
    UndeadArcher,
    UndeadWizard,
    UndeadWarrior,
    UndeadScholar,
    UndeadNecromancer,
    UndeadDefender,
    UndeadMonk,
    UndeadChampion,
    UndeadKing,
    UndeadBodyguard,
    Floor,
}

#[derive(PartialEq, Eq, Hash, Clone, Copy)]
pub enum SpriteSheet {
    Main,
    BricksAndTiles,
}

#[derive(Clone, Copy)]
struct Sprite {
    pub sheet: SpriteSheet,
    pub src: Rect,
}

pub struct Sprites {
    sprite_sheets: HashMap<SpriteSheet, Image>,
    sprites: HashMap<SpriteName, Sprite>,
}

impl Sprites {
    pub fn new(ctx: &mut Context) -> Self {
        let mut sprite_sheets = HashMap::new();
        sprite_sheets.insert(
            SpriteSheet::Main,
            Image::new(ctx, Path::new("/Spiderdave_main.png")).unwrap(),
        );
        sprite_sheets.insert(
            SpriteSheet::BricksAndTiles,
            Image::new(ctx, Path::new("/Spiderdave_bricks_and_tiles.png")).unwrap(),
        );
        let mut new_sprites = Self {
            sprite_sheets,
            sprites: HashMap::new(),
        };
        new_sprites.sprite_default(SpriteName::Archer, SpriteSheet::Main, 0, 2);
        new_sprites.sprite_default(SpriteName::Wizard, SpriteSheet::Main, 1, 2);
        new_sprites.sprite_default(SpriteName::Warrior, SpriteSheet::Main, 2, 2);
        new_sprites.sprite_default(SpriteName::Scholar, SpriteSheet::Main, 3, 2);
        new_sprites.sprite_default(SpriteName::Necromancer, SpriteSheet::Main, 4, 2);
        new_sprites.sprite_default(SpriteName::Defender, SpriteSheet::Main, 5, 2);
        new_sprites.sprite_default(SpriteName::Monk, SpriteSheet::Main, 6, 2);
        new_sprites.sprite_default(SpriteName::Champion, SpriteSheet::Main, 7, 2);
        new_sprites.sprite_default(SpriteName::King, SpriteSheet::Main, 8, 2);
        new_sprites.sprite_default(SpriteName::Bodyguard, SpriteSheet::Main, 9, 2);
        new_sprites.sprite_default(SpriteName::UndeadArcher, SpriteSheet::Main, 0, 3);
        new_sprites.sprite_default(SpriteName::UndeadWizard, SpriteSheet::Main, 1, 3);
        new_sprites.sprite_default(SpriteName::UndeadWarrior, SpriteSheet::Main, 2, 3);
        new_sprites.sprite_default(SpriteName::UndeadScholar, SpriteSheet::Main, 3, 3);
        new_sprites.sprite_default(SpriteName::UndeadNecromancer, SpriteSheet::Main, 4, 3);
        new_sprites.sprite_default(SpriteName::UndeadDefender, SpriteSheet::Main, 5, 3);
        new_sprites.sprite_default(SpriteName::UndeadMonk, SpriteSheet::Main, 6, 3);
        new_sprites.sprite_default(SpriteName::UndeadChampion, SpriteSheet::Main, 7, 3);
        new_sprites.sprite_default(SpriteName::UndeadKing, SpriteSheet::Main, 8, 3);
        new_sprites.sprite_default(SpriteName::UndeadBodyguard, SpriteSheet::Main, 9, 3);
        new_sprites.sprite_default(SpriteName::Floor, SpriteSheet::BricksAndTiles, 5, 1);
        new_sprites
    }

    fn sprite_default(
        &mut self,
        name: SpriteName,
        sheet: SpriteSheet,
        x_tile: u32,
        y_tile: u32,
    ) {
        self.make_sprite(name, sheet, x_tile, y_tile, 1, 1);
    }

    fn make_sprite(
        &mut self,
        name: SpriteName,
        sheet: SpriteSheet,
        x_tile: u32,
        y_tile: u32,
        w_tile: u32,
        h_tile: u32,
    ) {
        let image = self.sprite_sheets.get(&sheet).unwrap();
        let x = ((x_tile + 1) + (x_tile * PIXELS_PER_TILE)) as f32;
        let y = ((y_tile + 1) + (y_tile * PIXELS_PER_TILE)) as f32;
        let w = (w_tile * PIXELS_PER_TILE) as f32;
        let h = (h_tile * PIXELS_PER_TILE) as f32;
        let src = Rect::fraction(
            x,
            y,
            w,
            h,
            &Rect::new(0.0, 0.0, image.width() as f32, image.height() as f32),
        );
        let sprite = Sprite { sheet, src };
        self.sprites.insert(name, sprite);
    }

    fn make_sprite_ex(
        &mut self,
        name: SpriteName,
        sheet: SpriteSheet,
        x: u32,
        y: u32,
        w: u32,
        h: u32,
    ) {
        let image = self.sprite_sheets.get(&sheet).unwrap();
        let src = Rect::fraction(
            x as f32,
            y as f32,
            w as f32,
            h as f32,
            &Rect::new(0.0, 0.0, image.width() as f32, image.height() as f32),
        );
        let sprite = Sprite { sheet, src };
        self.sprites.insert(name, sprite);
    }

    fn draw_sprite(&self, ctx: &mut Context, sprite: Sprite, dest: Point2) {
        let image = self.sprite_sheets.get(&sprite.sheet).unwrap();
        graphics::draw_ex(
            ctx,
            image,
            DrawParam {
                src: sprite.src,
                dest,
                scale: Point2::new(SCALE_FACTOR, SCALE_FACTOR),
                ..Default::default()
            },
        ).unwrap();
    }

    pub fn draw(&self, ctx: &mut Context, sprite_name: SpriteName, dest: Point2) {
        let sprite = self.sprites.get(&sprite_name).unwrap();
        self.draw_sprite(ctx, *sprite, dest);
    }
}
