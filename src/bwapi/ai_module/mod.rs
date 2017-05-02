use bwapi::player::Player;
use bwapi::position::Position;
use bwapi::unit::Unit;

trait AIModule {
    fn on_start(&mut self);
    fn on_end(&mut self, is_winner: bool);
    fn on_frame(&mut self);

    fn on_send_text(&mut self, text: &str);
    fn on_receive_text(&mut self, player: &Player, text: &str);
    fn on_player_left(&mut self, player: &Player);

    fn on_nuke_detected(&mut self, position: &Position);

    fn on_unit_discover(&mut self, unit: &Unit);
    fn on_unit_evade(&mut self, unit: &Unit);
    fn on_unit_show(&mut self, unit: &Unit);
    fn on_unit_hide(&mut self, unit: &Unit);
    fn on_unit_create(&mut self, unit: &Unit);
    fn on_unit_destroy(&mut self, unit: &Unit);
    fn on_unit_morph(&mut self, unit: &Unit);
    fn on_unit_renegade(&mut self, unit: &Unit);
    fn on_unit_complete(&mut self, unit: &Unit);

    fn on_save_game(&mut self, game_name: &str);
}
