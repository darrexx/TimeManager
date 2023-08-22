// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use base64::{engine::general_purpose, Engine as _};
use commands::{get_activity_history, reset_timer, start_timer, stop_timer};
use crossbeam::{channel::bounded, sync::Parker};
use diesel::r2d2::{ConnectionManager, Pool};
use diesel::sqlite::SqliteConnection;
use reqwest::{header, Client};
use state::Timer;
use std::{sync::Mutex, thread};
use timer::{run_timer, timer_handler, TimerCommand};

mod azure_devops;
mod commands;
mod db;
mod schema;
mod state;
mod timer;

fn main() {
    let (command_sender, command_receiver) = bounded::<TimerCommand>(10); //To communicate with Handler, tx in state

    let timer_state = Mutex::new(Timer {
        running: false,
        start_time: None,
        pause_start_time: None,
        activity_name: None,
        activity_duration: None,
    });

    let database_pool = Pool::builder()
        .build(ConnectionManager::<SqliteConnection>::new("activities.db"))
        .unwrap();

    let httpclient_pool = configure_httpclient(String::from("user"), String::from("pat"));

    tauri::Builder::default()
        .manage(command_sender)
        .manage(database_pool)
        .manage(timer_state)
        .manage(httpclient_pool)
        .setup(|app| {
            let app_handle = app.handle();

            let (tick_sender, tick_receiver) = bounded::<()>(5);
            let (timer_sender, timer_receiver) = bounded::<TimerCommand>(5);

            let p = Parker::new();
            let u = p.unparker().clone();

            thread::spawn(move || {
                timer_handler(
                    &app_handle,
                    command_receiver,
                    tick_receiver,
                    timer_sender,
                    u,
                )
            });

            thread::spawn(move || run_timer(timer_receiver, tick_sender, p));

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            start_timer,
            stop_timer,
            reset_timer,
            get_activity_history,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

fn configure_httpclient(user: String, pat: String) -> Client {
    let mut headers = header::HeaderMap::new();

    let base64_auth_value = general_purpose::STANDARD_NO_PAD
        .encode(format!("{user}:{pat}"))
        .to_lowercase();
    let basic_auth_header = format!("Basic {base64_auth_value}");
    let mut auth_value = header::HeaderValue::from_str(basic_auth_header.as_str()).unwrap();
    auth_value.set_sensitive(true);
    headers.insert(header::AUTHORIZATION, auth_value);

    match Client::builder().default_headers(headers).build() {
        Ok(client) => client,
        Err(_) => panic!("Could not build httpClientPool"),
    }
}
