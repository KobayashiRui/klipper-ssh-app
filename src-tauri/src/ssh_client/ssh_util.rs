use tokio::sync::Mutex;
use tauri::State;
use super::ssh::{Session, SessionState};

#[tauri::command]
pub async fn connect_ssh(host: &str, username: &str, password: &str,  port:u16, state: State<'_, Mutex<SessionState>> ) -> Result<String, String> {
  let mut state = state.lock().await;

  let mut connect_res = Session::connect(
      username.to_string(),
      password.to_string(),
      (host, port),
  )
  .await;

  let session = match connect_res {
    Ok(ssh) => ssh,
    Err(_) => {
      return Err("Connect failed!".to_string())
    }
  };

  state.seission = Some(session);

  //info!("Connected");
  Ok("Connect SSH".to_string())
}

#[tauri::command]
pub async fn send_ssh(state: State<'_, Mutex<SessionState>> ) -> Result<(),()>{
  let mut state = state.lock().await;
  match &mut state.seission {
    Some(session) => {
      let commands = vec![
        "cd /etc",
        "ls",
      ];

      let _ = session.call(commands).await;
    },
    None => {
      println!("Error");
    }
  }
  Ok(())
}
