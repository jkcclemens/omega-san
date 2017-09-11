extern crate discord;
extern crate dotenv;

use std::env::var;
use discord::Discord;
use discord::model::Event;

const GAS: [&'static str; 4] = [
  "v1s",
  "v2s",
  "v3s",
  "v4s"
];

fn main() {
  dotenv::dotenv().ok();

  let bot_token = var("OMEGA_DISCORD_TOKEN").expect("No Discord token");

  let discord = Discord::from_bot_token(&bot_token).unwrap();

  let (mut connection, _) = discord.connect().unwrap();

  loop {
    let event = match connection.recv_event() {
      Ok(e) => e,
      Err(e) => {
        println!("error receiving event: {}", e);
        continue;
      }
    };

    if let Event::MessageCreate(ref m) = event {
      let bad = m.content.split_whitespace().find(|x| GAS.contains(&x.to_lowercase().as_str()));
      if let Some(b) = bad {
        discord.send_message(
          m.channel_id,
          &format!("Did you mean *o{}*?", &b[1..]),
          "",
          false
        ).ok();
      }
    }
  }
}