use enet::*;
use bytes::{Buf, BufMut};

trait ExtraBytes {
  fn extra_bytes(&mut self, amount: usize) -> &Self;
}

impl ExtraBytes for Vec<u8> {
  fn extra_bytes(&mut self, amount: usize) -> &Self {
    let bytes: &[u8] = &b"\x00".repeat(amount);
    self.put(bytes);

    drop(bytes);

    self
  }
}

pub struct GamePacket {
  data: Vec<u8>,
  index: u8,
  len: usize
}

impl GamePacket {
  pub fn send(self, peer: &mut Peer<()>, channel: &u8) {
    (*peer).send_packet(Packet::new(&self.data, PacketMode::ReliableSequenced).unwrap(), *channel).expect("failed sending packet");
  }

  pub fn string(mut self, string: &str) -> Self {
    let mut data: Vec<u8> = self.data;
    data.put_uint_le(self.index as u64, 1);
    data.put_uint_le(0x2, 1);
    data.put_uint_le(string.len() as u64, 4);
    data.put(string.as_bytes());

    self.index += 1;
    self.len = data.len();
    self.data = data;

    self.data[60] = self.index;

    self
  }
}

pub fn new() -> GamePacket {
  let types: (u8, u8, u8) = (0x4, 0x1, 0x8);
  let mut data: Vec<u8> = vec![];

  data.put_uint_le(types.0 as u64, 4);
  data.put_uint_le(types.1 as u64, 4);
  data.put_int_le(-1, 4);
  data.extra_bytes(4);
  data.put_uint_le(types.2 as u64, 4);
  data.extra_bytes(12);
  data.put_uint_le(0, 4);
  data.extra_bytes(25);

  GamePacket {
    data: data,
    len: 61,
    index: 0
  }
}

pub fn raw(p_type: u8, strings: (&str, &[&str])) -> GamePacket {
  let mut data: Vec<u8> = vec![];

  data.put_uint_le(p_type as u64, 4);
  data.put(strings.1.join(strings.0).as_bytes());

  GamePacket {
    len: data.len(),
    data,
    index: 0
  }
}

pub fn decode(packet: &mut Packet) -> (String, u8) {
  let mut data: &[u8] = packet.data();
  let p_type: u8 = data.get_int_le(4) as u8;

  if p_type > 3 { panic!("cannot decode packet type 4") }
  let mut vec_data: Vec<u8> = data.to_vec();
  drop(data);

  vec_data.pop();

  (String::from_utf8_lossy(&vec_data).to_string(), p_type)
}

impl GamePacket {

}