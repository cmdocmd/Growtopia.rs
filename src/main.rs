use enet::*;
use std::fs;
use std::env;
use std::thread;
use rouille::*;

mod config;

#[path = "./handler/events.rs"]
mod events;

fn build_items_database(path: String) -> Box<[u8]> {
  let file = fs::read(path).expect("failed to read items.dat");
  file.into_boxed_slice()
}

fn main() {
  let server_config: config::Config = config::get();
  println!("LOADED Config. Using ip: {}, port: {}", server_config.host, server_config.port);

  let server: Enet = Enet::new().expect("failed initializing the enet server.");
  let address: Address = Address::new(server_config.host, server_config.port);

  let host: Host<()> = server.create_host::<()>(Some(&address),
    1024,
    ChannelLimit::Maximum,
    BandwidthLimit::Unlimited,
    BandwidthLimit::Unlimited).expect("failed creating enet host");

  let items_dat = build_items_database(format!("{}/src/items.dat", env::current_dir().unwrap().to_string_lossy()));

  println!("Growtopia.rs now started.");

  let main_server: events::Events = events::Events {
    host,
    items_dat,
    items_dat_hash: 0x55555555
  };

  thread::spawn(|| {
    start_http();
  });

  main_server.listen();
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