extern crate discord;
extern crate dotenv;
extern crate regex;
#[macro_use]
extern crate lazy_static;

use discord::Discord;
use discord::model::Event;

use regex::Regex;

use std::env::var;

lazy_static! {
  static ref GAS: Regex = Regex::new(r"\b(?:[vd](\d{1,2})s|[vdo]s(\d{1,2}))\b").expect("Regex should have worked");
}

fn main() {
  dotenv::dotenv().ok();

  let bot_token = var("OMEGA_DISCORD_TOKEN").expect("No Discord token");

  let discord = Discord::from_bot_token(&bot_token).expect("Could not create Discord with token");

  let (mut connection, _) = discord.connect().expect("Could not establish connection");

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
      if let Some(captures) = GAS.captures(&content) {
        if let Some(mat) = captures.get(1).or_else(|| captures.get(2)) {
          discord.send_message(
            m.channel_id,
            &format!("Did you mean *o{}s*?", mat.as_str()),
            "",
            false
          ).ok();
        }
      }
    }
  }
}
