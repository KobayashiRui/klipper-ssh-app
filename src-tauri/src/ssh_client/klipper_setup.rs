use super::ssh::{Session, SessionState};
use regex::Regex;
use serde::Serialize;
use tauri::State;
use tokio::sync::Mutex;

#[tauri::command]
pub async fn klipper_get_version(state: State<'_, Mutex<SessionState>>) -> Result<(), ()> {
    let mut state = state.lock().await;
    match &mut state.session {
        Some(session) => {
            let commands = vec!["cd /etc", "ls"];

            let _ = session.call(commands).await;
        }
        None => {
            println!("Error");
        }
    }
    Ok(())
}

#[tauri::command]
pub async fn klipper_can_interface(
    state: State<'_, Mutex<SessionState>>,
) -> Result<Vec<String>, ()> {
    let mut state = state.lock().await;
    match &mut state.session {
        Some(session) => {
            let commands = vec!["ip link"];
            let res = session.call(commands).await;
            if let Ok(command_res) = res {
                let re_res = Regex::new(r"^\d+: (\S+):");
                if let Ok(re) = re_res {
                    let device_names: Vec<String> = command_res
                        .lines()
                        .filter_map(|line| re.captures(line)) // 正規表現に一致する行を抽出
                        .map(|cap| cap[1].to_string()) // キャプチャグループからデバイス名を取得
                        .collect();
                    let can_devices: Vec<String> = device_names
                        .iter()
                        .filter(|device| device.starts_with("can"))
                        .cloned() // &String を String に変換
                        .collect();
                    return Ok(can_devices);
                }
            }
        }
        None => {
            println!("Error");
        }
    }
    Ok(Vec::new())
}

fn extract_can_device(device_command_res: String) -> Vec<String> {
    let re_res = Regex::new(r"^\d+: (\S+):");
    let mut can_devices = Vec::new();
    if let Ok(re) = re_res {
        let device_names: Vec<String> = device_command_res
            .lines()
            .filter_map(|line| re.captures(line)) // 正規表現に一致する行を抽出
            .map(|cap| cap[1].to_string()) // キャプチャグループからデバイス名を取得
            .collect();
        can_devices = device_names
            .iter()
            .filter(|device| device.starts_with("can"))
            .cloned() // &String を String に変換
            .collect();
    }
    return can_devices;
}

#[derive(Serialize)]
pub struct CanDeviceUUID {
    device_name: String,
    can_uuids: Vec<String>,
}

fn extract_canbus_uuids(uuid_command_res: String) -> Vec<String> {
    // 正規表現を使用して `canbus_uuid` の値を抽出
    let re_res = regex::Regex::new(r"^Found canbus_uuid=([a-fA-F0-9]+),");
    let mut uuid_list = Vec::new();
    if let Ok(re) = re_res {
        uuid_list = uuid_command_res
            .lines()
            .filter_map(|line| re.captures(line)) // 正規表現に一致する行を抽出
            .map(|cap| cap[1].to_string()) // `canbus_uuid` を取得
            .collect()
    }
    return uuid_list;
}

#[tauri::command]
pub async fn klipper_can_uuid_list(
    state: State<'_, Mutex<SessionState>>,
) -> Result<Vec<CanDeviceUUID>, ()> {
    let mut state: tokio::sync::MutexGuard<'_, SessionState> = state.lock().await;
    let mut can_device_uuids: Vec<CanDeviceUUID> = Vec::new();
    match &mut state.session {
        Some(session) => {
            let commands = vec!["ip link"];
            let res = session.call(commands).await;
            let can_devices = match res {
                Ok(command_res) => extract_can_device(command_res),
                Err(_) => Vec::new(),
            };

            let klipper_python_path = "~/klippy-env/bin/python";
            let klipper_path = "~/klipper";
            for can_dev in &can_devices {
                println!("Search {} ...", can_dev);
                let can_query_command = format!(
                    "{} {}/scripts/canbus_query.py {}",
                    klipper_python_path, klipper_path, can_dev
                );
                let can_uuid_res = session.call(vec![&can_query_command]).await;
                if let Ok(uuid_res) = can_uuid_res {
                    let canbus_uuids = extract_canbus_uuids(uuid_res);
                    can_device_uuids.push(CanDeviceUUID {
                        device_name: can_dev.to_string(),
                        can_uuids: canbus_uuids,
                    });
                }
            }
        }
        None => {
            println!("Error");
        }
    }
    Ok(can_device_uuids)
}
