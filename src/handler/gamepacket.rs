use enet::*;
use bytes::{Buf, BufMut};
use std::collections::HashMap;

#[allow(dead_code)]
pub struct DecodedTextPacket {
  pub data: HashMap<String, String>,
  pub p_type: u8
}

pub trait ExtraBytes {
  fn extra(&mut self, amount: usize) -> &Self;
}

impl ExtraBytes for Vec<u8> {
  fn extra(&mut self, amount: usize) -> &Self {
    let bytes: &[u8] = &b"\x00".repeat(amount);
    self.put(bytes);

    self
  }
}

#[allow(dead_code)]
pub struct GamePacket {
  data: Vec<u8>,
  index: u8,
  len: usize
}

#[allow(dead_code)]
impl GamePacket {
  pub fn send(self, peer: &mut Peer<()>, channel: &u8) {
    (*peer).send_packet(Packet::new(&self.data, PacketMode::ReliableSequenced).unwrap(), *channel).expect("failed sending packet");
  }

  pub fn string(mut self, string: &str) -> Self {
    self.data.put_uint_le(self.index as u64, 1);
    self.data.put_uint_le(0x2, 1);
    self.data.put_uint_le(string.len() as u64, 4);
    self.data.put(string.as_bytes());

    self.index += 1;
    self.len = self.data.len();

    self.data[60] = self.index;

    self
  }

  pub fn int(mut self, int: i64) -> Self {
    self.data.put_uint_le(self.index as u64, 1);
    self.data.put_uint_le(0x9, 1);
    self.data.put_int_le(int, 4);

    self.index += 1;
    self.len = self.data.len();

    self.data[60] = self.index;

    self
  }
  
    pub fn intx(mut self, int: i64) -> Self {
    self.data.put_uint_le(self.index as u64, 1);
    self.data.put_uint_le(0x5, 1);
    self.data.put_uint_le(int, 4);

    self.index += 1;
    self.len = self.data.len();

    self.data[60] = self.index;

    self
  }
  
}

#[allow(dead_code)]
pub fn new() -> GamePacket {
  let types: (u8, u8, u8) = (0x4, 0x1, 0x8);
  let mut data: Vec<u8> = vec![];

  data.put_uint_le(types.0 as u64, 4);
  data.put_uint_le(types.1 as u64, 4);
  data.put_int_le(-1, 4);
  data.extra(4);
  data.put_uint_le(types.2 as u64, 4);
  data.extra(12);
  data.put_uint_le(0, 4);
  data.extra(25);

  GamePacket {
    data: data,
    len: 61,
    index: 0
  }
}

#[allow(dead_code)]
pub fn raw_str(p_type: u8, strings: (&str, &[&str])) -> GamePacket {
  let mut data: Vec<u8> = vec![];

  data.put_uint_le(p_type as u64, 4);
  data.put(strings.1.join(strings.0).as_bytes());

  GamePacket {
    len: data.len(),
    data,
    index: 0
  }
}

#[allow(dead_code)]
pub fn raw(data: &[u8]) -> GamePacket {
  GamePacket {
    len: data.len(),
    data: data.to_vec(),
    index: 0
  }
}

#[allow(dead_code)]
fn to_map(p_str: &str) -> HashMap<String, String> {
  let mut map: HashMap<String, String> = HashMap::new();
  let keys: Vec<&str> = p_str.split("\n").collect();

  for i in keys.iter() {
    let key: Vec<&str> = i.split("|").collect();
    if key.len() <=  1 {
      continue;
    }
    map.insert(key[0].to_string(), key[1].to_string());
  }

  map
}

#[allow(dead_code)]
pub fn decode(packet: &mut Packet) -> DecodedTextPacket {
  let mut data: &[u8] = &packet.data()[..];
  let p_type: u8 = data.get_int_le(4) as u8;

  data = &data[..data.len() - 1];

  DecodedTextPacket {
    data: to_map(&*String::from_utf8_lossy(data)),
    p_type
  }
}
