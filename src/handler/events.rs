use enet::*;
use std::net::Ipv4Addr;
use std::collections::HashMap;
mod gamepacket;

pub struct Events {
  // logged in users
  pub users: Option<HashMap<String, String>>
}

impl Events {
  pub fn listen(&mut self, mut host: Host<()>, items_dat: &[u8], hash: u32) {
    loop {
      match host.service(0).expect("failed checking events") {
        Some(Event::Connect(ref mut peer)) => {
          self.on_conn(peer)
        },

        Some(Event::Receive { ref mut sender, ref channel_id, ref mut packet }) => {
          self.on_msg(sender, channel_id, packet, (&items_dat, hash));
        },
        _ => ()
      }
    }
  }

  fn on_msg(&mut self, peer: &mut Peer<()>, channel: &u8, packet: &mut Packet, others: (&[u8], u32)) {
    let decoded_packet: gamepacket::DecodedTextPacket = gamepacket::decode(packet);

    if decoded_packet.p_type != 4 {
      if decoded_packet.data.contains_key("requestedName") || decoded_packet.data.contains_key("tankIDName") {
        // login
        gamepacket::new()
          .string("OnSuperMainStartAcceptLogonHrdxs47254722215a")
          .int(others.1 as i64)
          .string("ubistatic-a.akamaihd.net")
          .string("0098/CDNContent61/cache/")
          .string("cc.cz.madkite.freedom org.aqua.gg idv.aqua.bulldog com.cih.gamecih2 com.cih.gamecih com.cih.game_cih cn.maocai.gamekiller com.gmd.speedtime org.dax.attack com.x0.strai.frep com.x0.strai.free org.cheatengine.cegui org.sbtools.gamehack com.skgames.traffikrider org.sbtoods.gamehaca com.skype.ralder org.cheatengine.cegui.xx.multi1458919170111 com.prohiro.macro me.autotouch.autotouch com.cygery.repetitouch.free com.cygery.repetitouch.pro com.proziro.zacro com.slash.gamebuster")
          .string("proto=84|choosemusic=audio/mp3/about_theme.mp3|active_holiday=0|server_tick=226933875|clash_active=0|drop_lavacheck_faster=1|isPayingUser=0|")
          .send(peer, channel);
      } else {
        let action = decoded_packet.data.get("action").unwrap();
        match action.as_str() {
          "refresh_item_data" => {
            gamepacket::raw(others.0)
              .send(peer, channel);
          }
          _ => println!("action: {}", action)
        }
      }
    }
  }

  fn on_conn(&mut self, peer: &mut Peer<()>) {
    gamepacket::raw_str(1, ("\n", &["\x00"]))
      .send(peer, &1);
  
    gamepacket::new()
      .string("OnConsoleMessage")
      .string("`6Growtopia.rs`` up soon.")
      .send(peer, &1);
  }
}

fn _ip_port(peer: &mut Peer<()>) -> (Ipv4Addr, u16) {
  let address: &String = &peer.address().ip().to_string();
  let port: u16 = peer.address().port();
  let mut ip: [u8; 4] = [0; 4];
  let mut counter: i8 = 3;

  for i in address.split(".") {
    ip[counter as usize] = i.parse::<u8>().unwrap();
    counter -= 1;
  }

  (Ipv4Addr::new(ip[0], ip[1], ip[2], ip[3]), port)
}