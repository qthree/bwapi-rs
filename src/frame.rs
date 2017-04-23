
use std::collections::HashMap;

#[derive(Debug, Default)]
pub struct Order {
    /// first frame number where order appeared
    first_frame: i32,

    /// see BWAPI::Orders::Enum
    type_: i32,
    target_id: i32,
    target_coords: (i32, i32),
}

impl PartialEq for Order {
     fn eq(&self, other: &Order) -> bool {
         self.type_ == other.type_ &&
         self.target_id == other.target_id &&
         self.target_coords == other.target_coords
     }
}

#[derive(Debug, Default)]
pub struct Unit {
    id: i32,

    coords: (i32, i32),
    health: i32,
    max_health: i32,

    shield: i32,
    max_shield: i32,
    energy: i32,

    max_cd: i32,
    ground_cd: i32,
    air_cd: i32,

    idle: bool,
    detected: bool,
    lifted: bool,
    visible: bool,

    type_: i32,
    armor: i32,
    shield_armor: i32,
    size: i32,

    pixel_coords: (i32, i32),
    pixel_size: (i32, i32),

    ground_atk: i32,
    air_atk: i32,

    ground_dmg_type: i32,
    air_dmg_type: i32,
    ground_range: i32,
    air_range: i32,

    orders: Vec<Order>,

    velocity: (f64, f64),
    player_id: i32,

    resources: i32,
}

#[derive(Debug, Default)]
pub struct Resources {
    ore: i32,
    gas: i32,
    used_psi: i32,
    total_psi: i32,
}

#[derive(Debug, Default)]
pub struct Bullet {
    type_: i32,
    coords: (i32, i32)
}

#[derive(Debug, Default)]
pub struct Action {
    action: Vec<i32>,
    uid: i32,
    aid: i32
}

#[derive(Debug, Default)]
pub struct Frame {
    units: HashMap<i32, Vec<Unit>>,
    actions: HashMap<i32, Vec<Action>>,
    resources: HashMap<i32, Resources>,
    bullets: Vec<Bullet>,
    reward: i32,
    is_terminal: bool,
}

impl Order {
    fn parse_from<'i, 's: 'i, I: Iterator<Item=&'s str>>(iter: &'i mut I) -> Result<Order, &'static str> {
        Ok(Order {
            first_frame: read_int(iter)?,

            type_: read_int(iter)?,
            target_id: read_int(iter)?,
            target_coords: (read_int(iter)?, read_int(iter)?),
        })
    }
}

impl Unit {
    fn parse_from<'i, 's: 'i, I: Iterator<Item=&'s str>>(iter: &'i mut I) -> Result<Unit, &'static str> {
        let mut result = Unit {
            id: read_int(iter)?,

            coords: (read_int(iter)?, read_int(iter)?),
            health: read_int(iter)?,
            max_health: read_int(iter)?,

            shield: read_int(iter)?,
            max_shield: read_int(iter)?,
            energy: read_int(iter)?,

            max_cd: read_int(iter)?,
            ground_cd: read_int(iter)?,
            air_cd: read_int(iter)?,

            idle: read_int(iter)? > 0,
            // detected: bool,
            // lifted: bool,
            visible: read_int(iter)? > 0,

            type_: read_int(iter)?,
            armor: read_int(iter)?,
            shield_armor: read_int(iter)?,
            size: read_int(iter)?,

            pixel_coords: (read_int(iter)?, read_int(iter)?),
            pixel_size: (read_int(iter)?, read_int(iter)?),

            ground_atk: read_int(iter)?,
            air_atk: read_int(iter)?,

            ground_dmg_type: read_int(iter)?,
            air_dmg_type: read_int(iter)?,
            ground_range: read_int(iter)?,
            air_range: read_int(iter)?,

            .. Unit::default()
        };

        let n_orders = read_int(iter)?;
        if n_orders < 0 {
            return Err("n_orders size < 0");
        }

        for _ in 0 .. n_orders {
            let order = Order::parse_from(iter)?;
            result.orders.push(order);
        }

        result.velocity = (read_float(iter)?, read_float(iter)?);

        result.player_id = read_int(iter)?;
        result.resources = read_int(iter)?;

        Ok(result)
    }
}

impl Action {
    fn parse_from<'i, 's: 'i, I: Iterator<Item=&'s str>>(iter: &'i mut I) -> Result<Action, &'static str> {
        let mut result = Action::default();

        result.uid = read_int(iter)?;
        result.aid = read_int(iter)?;

        let size   = read_int(iter)?;
        if size < 0 {
            return Err("action size < 0");
        }

        result.action = Vec::with_capacity(size as usize);
        for i in 0 .. size as usize {
            result.action[i] = read_int(iter)?;
        }

        Ok(result)
    }
}

impl Resources {
    fn parse_from<'i, 's: 'i, I: Iterator<Item=&'s str>>(iter: &'i mut I) -> Result<Resources, &'static str> {
        let result = Resources {
            ore: read_int(iter)?,
            gas: read_int(iter)?,
            used_psi: read_int(iter)?,
            total_psi: read_int(iter)?,
        };

        Ok(result)
    }
}

impl Bullet {
    fn parse_from<'i, 's: 'i, I: Iterator<Item=&'s str>>(iter: &'i mut I) -> Result<Bullet, &'static str> {
        Ok(Bullet {
            type_: read_int(iter)?,
            coords: (read_int(iter)?, read_int(iter)?),
        })
    }
}

impl Frame {
    pub fn parse_from(input: &str) -> Result<Frame, &'static str> {
        let slice = &input[1 .. input.len()-1];
        let mut data = slice
            .split(' ')
            .filter(|e| e.len() > 0);

        let mut result = Frame::default();

        result.read_units(&mut data)?;
        result.read_actions(&mut data)?;
        result.read_resources(&mut data)?;
        result.read_bullets(&mut data)?;

        result.reward = read_int(&mut data)?;
        result.is_terminal = read_int(&mut data)? > 0;

        Ok(result)
    }

    fn read_units<'i, 's: 'i, I: Iterator<Item=&'s str>>(&mut self, data: &'i mut I) -> Result<(), &'static str> {
        let n_player = read_int(data)?;
        if n_player < 0 {
            return Err("n_player < 0");
        }

        for _ in 0 .. n_player {
            let id_player = read_int(data)?;
            let n_units   = read_int(data)?;

            if n_units < 0 {
                return Err("n_units < 0");
            }

            let mut units = Vec::new();
            for _ in 0 .. n_units {
                let unit = Unit::parse_from(data)?;
                units.push(unit);
            }

            self.units.insert(id_player, units);
        }

        Ok(())
    }

    fn read_actions<'i, 's: 'i, I: Iterator<Item=&'s str>>(&mut self, data: &'i mut I) -> Result<(), &'static str> {
        let n_player = read_int(data)?;
        if n_player < 0 {
            return Err("n_player < 0");
        }

        for _ in 0 .. n_player {
            let id_player = read_int(data)?;
            let n_actions = read_int(data)?;

            if n_actions < 0 {
                return Err("n_actions < 0");
            }

            let mut actions = Vec::new();
            for _ in 0 .. n_actions {
                let unit = Action::parse_from(data)?;
                actions.push(unit);
            }

            self.actions.insert(id_player, actions);
        }

        Ok(())
    }

    fn read_resources<'i, 's: 'i, I: Iterator<Item=&'s str>>(&mut self, data: &'i mut I) -> Result<(), &'static str> {
        let n_player = read_int(data)?;
        if n_player < 0 {
            return Err("n_player < 0");
        }

        for _ in 0 .. n_player {
            let id_player = read_int(data)?;
            let resources = Resources::parse_from(data)?;
            self.resources.insert(id_player, resources);
        }

        Ok(())
    }

    fn read_bullets<'i, 's: 'i, I: Iterator<Item=&'s str>>(&mut self, data: &'i mut I) -> Result<(), &'static str> {
        let n_bullets = read_int(data)?;
        if n_bullets < 0 {
            return Err("n_bullets < 0");
        }

        for _ in 0 .. n_bullets {
            let bullet = Bullet::parse_from(data)?;
            self.bullets.push(bullet);
        }

        Ok(())
    }
}

fn read_int<'i, 's: 'i, I: Iterator<Item=&'s str>>(iter: &'i mut I) -> Result<i32, &'static str> {
    match iter.next() {
        Some(s) => Ok(s.parse().map_err(|_| "error parsing i32")?),
        None => Err("unexpected EOF while parsing i32")
    }
}

fn read_float<'i, 's: 'i, I: Iterator<Item=&'s str>>(iter: &'i mut I) -> Result<f64, &'static str> {
    match iter.next() {
        Some(s) => Ok(s.parse().map_err(|_| "error parsing f64")?),
        None => Err("unexpected EOF while parsing f64")
    }
}
