use std::sync::Arc;

use anyhow::Result;
use async_trait::async_trait;
use tokio::net::ToSocketAddrs;
use tokio::fs;
use tokio::io::{AsyncReadExt, AsyncSeekExt, AsyncWriteExt};

//use clap::Parser;
use russh::keys::*;
use russh::*;
//use russh_sftp::Sftp;
use russh_sftp::{client::SftpSession, protocol::OpenFlags};

#[derive(Default)]
pub struct SessionState {
    pub session: Option<Session>,
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
        let auth_res = session.authenticate_password(user, password).await?;

        if !auth_res {
            anyhow::bail!("Authentication failed");
        }

        Ok(Self { session })
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
                        Err(err) => Ok("error utf8".to_string()),
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

    pub async fn send_file(&mut self, local_path: String, remote_path: String) -> Result<String> {
        let channel = self.session.channel_open_session().await.unwrap();
        channel.request_subsystem(true, "sftp").await.unwrap();
        let sftp = SftpSession::new(channel.into_stream()).await.unwrap();



        //送信元のファイルパス
        println!("Local file path {}", local_path);
        println!("Remote file path {}", remote_path);

        // ファイルを書き込む
        let remote_file_flags = OpenFlags::CREATE | OpenFlags::TRUNCATE | OpenFlags::WRITE | OpenFlags::READ;
        let mut remote_file = sftp.open_with_flags(remote_path, remote_file_flags)
            .await
            .unwrap();
        let mut local_file = fs::File::open(local_path).await?;
        println!("Copy Start");
        // バッファを用意 (例: 65KB)
        let mut buffer = vec![0u8; 65536];
        // ローカルファイルを読み取り、リモートファイルに書き込む
        loop {
            let bytes_read = local_file.read(&mut buffer).await?;
            if bytes_read == 0 {
                break; // EOF
            }
            remote_file.write_all(&buffer[..bytes_read]).await?;
        }
        //io::copy(&mut local_file, &mut remote_file).await?;
        println!("Copy Completed!");

        Ok("SendCompleted".to_string())
    }

    pub async fn close(&mut self) -> Result<()> {
        self.session
            .disconnect(Disconnect::ByApplication, "", "English")
            .await?;
        Ok(())
    }
}
