use std::rc::Rc;
use std::collections::HashMap;
use zmq;
use url::Url;

use frame::{Unit, Frame};

pub struct Client {
    url: Url,
    socket: Option<zmq::Socket>,
    state: ServerState,
    message_sent: bool,
}

impl Client {
    pub fn new(server_url: Url) -> Client {
        Client {
            url: server_url,
            socket: None,
            state: ServerState::default(),
            message_sent: false,
        }
    }

    pub fn connect(&mut self) -> String {
        // TODO Error handling

        let context = zmq::Context::new();
        let socket = context.socket(zmq::REQ).unwrap();
        socket.connect(self.url.as_str()).unwrap();

        let hello = "protocol=16, micro_mode=False";
        socket.send_str(&hello, 0).unwrap();

        let reply = socket.recv_string(0).unwrap().unwrap();
        self.socket = Some(socket);

        reply
    }

    pub fn receive(&mut self) -> Result<String, &'static str> {
        if !self.message_sent {
            return Err("trying to receive without sending anything");
        }

        if let Some(socket) = self.socket.as_ref() {
            socket.poll(zmq::POLLIN, 30_000).map_err(|_| "poll timeout")?;

            let data = socket.recv_string(0).map_err(|_| "read error")?;
            let message = data.map_err(|_| "malformed UTF")?;

            self.state.parse(&message)?;
            self.state.update()?;

            self.message_sent = false;

            Ok(message)
        } else {
            Err("not connected")
        }
    }
}

#[derive(Debug, Default)]
pub struct ServerState {
    /// Number of frames from order to execution
    lag_frames: i32,

    /// 2D map data. 255 where not available
    map_data: Vec<u8>,
    map_size: (i32, i32),

    /// 2D, build tile resolution. FIXME BitVec
    buildable_data: Vec<bool>,
    buildable_size: (i32, i32),

    /// Name of the current map
    map_name: String,

    player_id: i32,
    neutral_id: i32,
    is_replay: bool,

    // game state
    // Frame* frame; // this will allow for easy reset (XXX)
    // std::string frame_string;
    // std::vector<int> deaths;
    frame: Rc<Frame>,

    deaths: Vec<i32>,
    frame_from_bwapi: i32,

    /// if micro mode
    battle_frame_count: i32,

    /// did the game end?
    game_ended: bool,

    /// did we won the game?
    game_won: bool,

    /// if micro mode
    battle_just_ended: bool,
    battle_won: bool,
    waiting_for_restart: bool,
    last_battle_ended: i32,

    /// if with image
    image_mode: String,

    /// position of screen {x, y} in pixels. {0, 0} is
    screen_position: (i32, i32),

    visibility: Vec<u8>,
    visibility_size: (i32, i32),

    image: Vec<u8>,
    image_size: (i32, i32),

    /// Alive units in this frame. Used to detect end-of-battle in micro mode. If
    /// the current frame is the end of a battle, this will contain all units that
    /// were alive when the battle ended (which is not necessarily the current
    /// frame due to frame skipping on the serv side). Note that this map ignores
    /// onlyConsiderUnits_.
    /// Maps unit id to player id
    alive_units: HashMap<i32, i32>,

    /// Like aliveUnits, but containing only units of types in onlyConsiderUnits.
    /// If onlyConsiderUnits is empty, this map is invalid.
    alive_units_considered: HashMap<i32, i32>,

    // Bots might want to use this map instead of frame->units because:
    // - Unknown unit types are not present (e.g. map revealers)
    // - Units reported as dead are not present (important if the server performs
    //   frame skipping. In that case, frame->units will still contain all units
    //   that have died since the last update.
    // - In micro mode and with frame skipping, deaths are only applied until the
    //   battle is considered finished, i.e. it corresponds to aliveUnits.
    units: HashMap<i32, Vec<Unit>>,
}

impl ServerState {
    pub fn parse(&mut self, message: &str) -> Result<(), &'static str> {
        for (k, v) in parse_table(&message) {
            match k {
                "frame" => self.frame = Rc::new(Frame::parse_from(v)?),

                "deaths" => self.deaths = v[1 .. v.len()-1] // strip quotes
                    .split(',')
                    .filter(|e| e.len() > 0) // a,,b
                    .map(|x| x.parse().unwrap_or(0))
                    .collect(),

                // TODO
                _ => {}
            }

        }

        Ok(())
    }

    pub fn update(&self) -> Result<(), &'static str> {
        unimplemented!()
    }
}

fn parse_table(input: &str) -> HashMap<&str, &str> {
    input[1 .. input.len()-1] // strip quotes
        .split(',')
        .filter(|e| e.len() > 0)         // a,,b
        .map(|pair|
            pair
                .split('=')
                .filter(|e| e.len() > 0) // a= or =b
                .collect::<Vec<_>>()
        )
        .filter(|e| e.len() == 2)        // a=b=c
        .map(|p| (p[0], p[1]))
        .collect()
}

fn parse_list(input: &str) -> Vec<&str> {
    input[1 .. input.len()-1] // strip quotes
        .split(',')
        .filter(|e| e.len() > 0) // a,,b
        .collect()
}
