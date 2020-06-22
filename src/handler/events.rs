use enet::*;
use std::net::Ipv4Addr;
mod gamepacket;

pub struct Events {
  pub items_dat: Box<[u8]>,
  pub items_dat_hash: u32,
  pub host: Host<()>
}

impl Events {
  pub fn listen(mut self) {
    loop {
      match self.host.service(0).expect("failed checking events") {
        Some(Event::Connect(ref mut peer)) => on_conn(peer),

        Some(Event::Receive { ref mut sender, ref channel_id, ref mut packet }) => on_msg(sender, channel_id, packet, (&*self.items_dat, self.items_dat_hash)),
        _ => ()
      }
    }
  }
}

fn ip_port(peer: &mut Peer<()>) -> (Ipv4Addr, u16) {
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

fn on_msg(peer: &mut Peer<()>, channel: &u8, packet: &mut Packet, others: (&[u8], u32)) {
  println!("Received packet from peer: {:?}", gamepacket::decode(packet).data);

  gamepacket::raw(3, ("\n", &["action|set_url", "url|https://www.youtube.com/watch?v=dQw4w9WgXcQ", "label|`$Come back soon.``"]))
    .send(peer, &channel);
}

fn on_conn(peer: &mut Peer<()>) {
  gamepacket::raw(1, ("\n", &["\x00"]))
    .send(peer, &1);

  gamepacket::new()
    .string("OnConsoleMessage")
    .string("`6Growtopia.rs`` up soon.")
    .send(peer, &1);
}