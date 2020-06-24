use enet::*;

#[path = "./gamepacket.rs"]
mod gamepacket;

pub struct Actions {
  pub items_dat: Option<Vec<u8>>,
}

impl Actions {
  fn refresh_item_data(self, peer: &mut Peer<()>, channel: &u8) {
    gamepacket::raw(&self.items_dat.unwrap())
      .send(peer, channel);
  }

  fn enter_game(self, peer: &mut Peer<()>, channel: &u8) {
    gamepacket::new()
      .string("OnRequestWorldSelectMenu")
      .string("default|\nadd_button|Showing:  `wWorlds``|_catselect_|0.6|3529161471|\n")
      .send(peer, channel);
  }

  pub fn match_it(self, action: &str, peer: &mut Peer<()>, channel: &u8) {
    match action {
      "refresh_item_data" => self.refresh_item_data(peer, channel),
      "enter_game" => self.enter_game(peer, channel),
      _ => println!("unhandled action: {}", action)
    }
  }
}