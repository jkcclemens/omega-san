extern crate discord;
extern crate dotenv;

use std::env::var;
use discord::Discord;
use discord::model::Event;

const GAS: [&'static str; 8] = [
  "v1s", "d1s",
  "v2s", "d2s",
  "v3s", "d3s",
  "v4s", "d4s",
  "v5s", "d5s",
  "v6s", "d6s",
  "v7s", "d7s",
  "v8s", "d8s",
  "v9s", "d9s",
  "v10s", "d10s",
  "v11s", "d11s",
  "v12s", "d12s"
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
      let content: String = m.content
        .chars()
        .filter(|x| x.is_whitespace() || x.is_alphanumeric())
        .flat_map(char::to_lowercase)
        .collect();
      let bad = content.split_whitespace().find(|x| GAS.contains(&x));
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
