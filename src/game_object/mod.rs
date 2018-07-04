use std::collections::HashSet;
use assets::SpriteName;

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash, Add, Sub, AddAssign, SubAssign, From, Into,
         Constructor, Mul, MulAssign)]
pub struct Tile {
    x: i16,
    y: i16,
}

impl Tile {
    pub fn x(&self) -> i32 {
        self.x as i32
    }

    pub fn y(&self) -> i32 {
        self.y as i32
    }
}

pub trait HasSprite {
    fn get_sprite(&self) -> SpriteName;
}

pub trait HasTile {
    fn get_tile(&self) -> Tile;

    fn get_tile_x(&self) -> i32 {
        self.get_tile().x()
    }

    fn get_tile_y(&self) -> i32 {
        self.get_tile().y()
    }
}

pub enum Side {
    Player,
    Enemy,
}

pub struct Health {
    max: u16,
    current: i16,
}

impl Health {
    pub fn new(health: u16) -> Self {
        Self {
            max: health,
            current: health as i16,
        }
    }
    pub fn take_damage(&mut self, damage: u16) {
        self.current -= damage as i16;
    }

    pub fn is_dead(&self) -> bool {
        self.current <= 0
    }
}

#[derive(HasSprite, HasTile)]
pub struct Unit {
    side: Side,
    movement: u16,
    damage: u16,
    attack_pattern: HashSet<Tile>,
    sprite: SpriteName,
    health: Health,
    tile: Tile,
}

impl Unit {
    fn melee_attack_pattern() -> HashSet<Tile> {
        let mut attack_pattern = HashSet::new();
        attack_pattern.insert(Tile::new(1, 0));
        attack_pattern.insert(Tile::new(-1, 0));
        attack_pattern.insert(Tile::new(0, -1));
        attack_pattern.insert(Tile::new(0, 1));
        attack_pattern
    }

    fn shoot_attack_pattern(range: u16) -> HashSet<Tile> {
        let mut attack_pattern = HashSet::new();
        for x in 0..range as i16 {
            for y in 0..range as i16 {
                if (x == 0) ^ (y == 0) {
                attack_pattern.insert(Tile::new(x, y));
                attack_pattern.insert(Tile::new(-x, -y));
                }
            }
        }
        attack_pattern
    }

    pub fn warrior(tile: Tile, side: Side) -> Self {
        let sprite = match side {
            Side::Enemy => SpriteName::Warrior,
            Side::Player => SpriteName::UndeadWarrior,
        };
        Self {
            side,
            movement: 3,
            damage: 4,
            attack_pattern: Self::melee_attack_pattern(),
            sprite,
            health: Health::new(10),
            tile,
        }
    }

    pub fn archer(tile: Tile, side: Side) -> Self {
        let sprite = match side {
            Side::Enemy => SpriteName::Archer,
            Side::Player => SpriteName::UndeadArcher,
        };
        Self {
            side,
            movement: 3,
            damage: 2,
            attack_pattern: Self::shoot_attack_pattern(3),
            sprite,
            health: Health::new(5),
            tile,
        }
    }
}
