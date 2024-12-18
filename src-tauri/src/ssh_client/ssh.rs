use std::sync::Arc;

use anyhow::Result;
use async_trait::async_trait;
use tokio::io::AsyncWriteExt;
use tokio::net::ToSocketAddrs;

//use clap::Parser;
use russh::keys::*;
use russh::*;


#[derive(Default)]
pub struct SessionState {
    pub seission: Option<Session>,
}

struct Client {}

// More SSH event handlers
// can be defined in this trait
// In this example, we're only using Channel, so these aren't needed.
#[async_trait]
impl client::Handler for Client {
    type Error = russh::Error;
//
    async fn check_server_key(
        &mut self,
        _server_public_key: &ssh_key::PublicKey,
    ) -> Result<bool, Self::Error> {
        Ok(true)
    }
}

pub struct Session {
    session: client::Handle<Client>,
}

impl Session {
    pub async fn connect<A: ToSocketAddrs>(
        user: String,
        password: String,
        addrs: A,
    ) -> Result<Self> {
        let config = client::Config {
            //inactivity_timeout: Some(Duration::from_secs(5)),
            ..<_>::default()
        };

        let config = Arc::new(config);
        let sh = Client {};

        let mut session = client::connect(config, addrs, sh).await?;
        let auth_res = session
            .authenticate_password(user, password)
            .await?;

        if !auth_res {
            anyhow::bail!("Authentication failed");
        }

        Ok(Self { session})
    }

    pub async fn call(&mut self, commands: Vec<&str>) -> Result<String> {
      let command = commands.join(";");

      let mut channel = self.session.channel_open_session().await?;
      println!("Call: {}", command);
      channel.exec(true, command).await?;

      let mut code = None;
      let mut stdout = tokio::io::stdout();

      loop {
          // There's an event available on the session channel
          let Some(msg) = channel.wait().await else {
              break;
          };
          match msg {
              // Write data to the terminal
              ChannelMsg::Data { ref data } => {
                return match String::from_utf8(data.to_vec()) {
                    Ok(string) => Ok(string),
                    Err(err) => Ok("error utf8".to_string())
                };
                //stdout.write_all(data).await?;
                //stdout.flush().await?;
              }
              // The command has returned an exit code
              ChannelMsg::ExitStatus { exit_status } => {
                  code = Some(exit_status);
                  // cannot leave the loop immediately, there might still be more data to receive
              }
              _ => {}
          }
      }
      println!("END");
      Ok(code.expect("program did not exit cleanly").to_string())
    }

    pub async fn close(&mut self) -> Result<()> {
        self.session
            .disconnect(Disconnect::ByApplication, "", "English")
            .await?;
        Ok(())
    }
}