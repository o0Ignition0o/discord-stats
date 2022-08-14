use anyhow::Context;
use discord::{Discord, State};
use std::env;

fn main() -> anyhow::Result<()> {
    let bot_key: String =
        env::var("BOT_API_KEY").expect("provide a bot key as BOT_API_KEY env variable");

    let discord = Discord::from_bot_token(bot_key.as_str())?;

    let (mut connection, ready) = discord.connect()?;

    std::thread::spawn(move || {
        let mut state = State::new(ready);
        loop {
            connection.sync_calls(&state.all_private_channels());
            connection.sync_servers(&state.all_servers());
            match connection.recv_event() {
                Ok(event) => {
                    state.update(&event);
                }
                Err(e) => {
                    dbg!(e);
                }
            }
        }
    });

    std::thread::sleep(std::time::Duration::from_secs(5));

    let servers = discord.get_servers()?;

    let server = servers[0].clone();

    let channels = discord
        .get_server_channels(server.id)
        .context("couldn't get server channels")?;

    println!("Server {}", server.name);

    println!("-------------------------");

    println!(
        "Channels \n\n{}\n\n",
        channels
            .iter()
            .map(|c| c.name.clone())
            .collect::<Vec<_>>()
            .join("\n")
    );

    println!("-------------------------");

    let members = discord
        .get_server_members(server.id)
        .context("couldn't get server members")?;

    println!(
        "Members \n\n{}\n\n",
        members
            .iter()
            .map(|m| m.display_name())
            .collect::<Vec<_>>()
            .join("\n")
    );

    println!("-------------------------");
    Ok(())
}
