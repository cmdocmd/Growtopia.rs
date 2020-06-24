use enet::*;
use std::fs;
use std::env;
use std::thread;
use bytes::BufMut;
use rouille::*;

mod config;

#[path = "./handler/events.rs"]
mod events;

#[path = "./handler/gamepacket.rs"]
mod gamepacket;
pub use gamepacket::*;

fn build_items_database(path: String) -> Box<[u8]> {
  let file = fs::read(path).expect("failed to read items.dat");
  let mut data: Vec<u8> = vec![];

  data.put_uint_le(0x4, 4);
  data.put_uint_le(0x10, 4);
  data.put_int_le(-1, 4);
  data.extra(4);
  data.put_uint_le(0x8, 4);
  data.extra(36);
  data.put_uint_le(file.len() as u64, 4);

  let mut counter = 0;

  while counter < file.len() {
    data.push(file[counter]);
    counter += 1;
  }

  data.into_boxed_slice()
}

fn get_hash(items_dat: &[u8]) -> u32 {
  let mut h: u32 = 0x55555555;
  let mut counter: usize = 0;

  while counter < items_dat.len() {
    h = ((h >> 27).wrapping_add(h << 5)).wrapping_add(items_dat[counter] as u32);
    counter += 1;
  };

  h
}

fn main() {
  let server_config: config::Config = config::get();
  println!("LOADED Config. Using ip: \x1b[32m{}\x1b[0m, port: \x1b[31m{}\x1b[0m", server_config.host, server_config.port);

  let server: Enet = Enet::new().expect("failed initializing the enet server.");
  let address: Address = Address::new(server_config.host, server_config.port);

  let host: Host<()> = server.create_host::<()>(Some(&address),
    1024,
    ChannelLimit::Maximum,
    BandwidthLimit::Unlimited,
    BandwidthLimit::Unlimited).expect("failed creating enet host");

  let items_dat = build_items_database(format!("{}/src/items.dat", env::current_dir().unwrap().to_string_lossy()));

  println!("Growtopia.rs now started.");

  let mut main_server: events::Events = events::Events {
    users: None
  };

  thread::spawn(|| {
    start_http();
  });

  main_server.listen(host, &*items_dat, get_hash(&*items_dat));
}

fn start_http() {
  println!("Webserver started on different thread.");
  start_server("127.0.0.1:80", move |response| {    
    match response.url().as_str() { 
      "/growtopia/server_data.php" => {
        if response.method().to_lowercase() == "post" {
          Response::text("server|127.0.0.1\nport|17091\ntype|1\n#maint|Mainetrance message (Not used for now) -- Growtopia.rs\n\nbeta_server|127.0.0.1\nbeta_port|17091\n\nbeta_type|1\nmeta|localhost\nRTENDMARKERBS1001")
        } else {
          Response::empty_404()
        }
      },
      _ => Response::empty_404()
    }
  });
}