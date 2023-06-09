// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use discord_presence::{Client};
use once_cell::sync::Lazy;
use tauri_plugin_log::{TargetKind, Target};

// TODO: Eventually - when I get better at Rust I'm going to remove these static mutables
static mut CLIENT: Lazy<Client> = Lazy::new(|| Client::new(988215000676073532));
static mut STARTED_CLIENT: bool = false;
static mut CLIENT_CONNECTED: bool = false;

#[cfg(not(mobile))]
#[tauri::command]
fn start_discord_rpc() {
  println!("[RUST] DiscordRPC requested to be started by JS.");

  unsafe {
    if STARTED_CLIENT {
      println!("[RUST] Client was already started!");
      return;
    }
    STARTED_CLIENT = true;
    _ = CLIENT.start();
    
    CLIENT.on_ready(|_ctx| {
      CLIENT_CONNECTED = true;
      println!("[RUST] Client connected.");
    });

    CLIENT.on_error(|_ctx| {
      CLIENT_CONNECTED = false;
      println!("[RUST] Client disconnected.");
      _ = CLIENT.start();
    })
  }
  
  println!("[RUST] Attempting to connect to Client.")
}

#[cfg(not(mobile))]
#[tauri::command]
fn clear_activity() {
  unsafe {
    if STARTED_CLIENT {
      CLIENT.clear_activity().expect("failed to clear activity");
    }
  }
}

#[cfg(not(mobile))]
#[tauri::command]
fn get_rpc_state() -> bool {
  unsafe {
    return CLIENT_CONNECTED;
  }
}

#[cfg(not(mobile))]
#[tauri::command]
fn set_discord_rpc(state: String, details: String, image: String) {
  println!("[RUST] Attempting to set DiscordRPC state to \"{}\" and details to \"{}\" and to \"{}\".", state, details, image);
  if state.is_empty() { return; }
  if details.is_empty() { return; }
  if image.is_empty() { return; }

  unsafe {
    CLIENT.set_activity(|act|
      act
        .state(state)
        .details(details)
        .assets(|aset| 
          aset.large_image(image).large_text("musicplayer2")
      ))
    .expect("[RUST] Failed to set activity");
  }
  println!("[RUST] Succeeded!")
}

fn main() {
    tauri::Builder::default()
      .plugin(tauri_plugin_log::Builder::default().targets([
        Target::new(TargetKind::Stdout),
        Target::new(TargetKind::Webview),
      ]).build())
      .invoke_handler(tauri::generate_handler![set_discord_rpc, start_discord_rpc, clear_activity, get_rpc_state])
      .run(tauri::generate_context!())
      .expect("error while running tauri application");
}