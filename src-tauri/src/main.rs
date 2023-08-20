// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use commands::{get_activity_history, reset_timer, start_timer, stop_timer};
use crossbeam::{channel::bounded, sync::Parker};
use diesel::r2d2::{ConnectionManager, Pool};
use diesel::sqlite::SqliteConnection;
use state::Timer;
use std::{sync::Mutex, thread};
use timer::{run_timer, timer_handler, TimerCommand};

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

    tauri::Builder::default()
        .manage(command_sender)
        .manage(database_pool)
        .manage(timer_state)
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
