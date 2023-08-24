// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use azure_devops::client::{configure_devops_httpclient, AzureDevopsClient};
use commands::{
    get_activity_history, get_workitems, reset_timer, save_devops_config, start_timer, stop_timer,
};
use config::{AzureDevopsConfig, Config};
use crossbeam::{channel::bounded, sync::Parker};
use diesel::r2d2::{ConnectionManager, Pool};
use diesel::sqlite::SqliteConnection;
use state::Timer;
use std::thread;
use tauri::async_runtime::Mutex;
use timer::{run_timer, timer_handler, TimerCommand};

mod azure_devops;
mod commands;
mod config;
mod db;
mod schema;
mod state;
mod timer;

fn main() {
    let config: Config = confy::load("timemanager", None).unwrap(); //TODO https://github.com/rust-cli/confy/issues/11

    let httpclient_pool = AzureDevopsClient(configure_devops_httpclient(
        &config.devops_config.user,
        &config.devops_config.pat,
    ));

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

    let mut builder = tauri::Builder::default()
        .manage(command_sender)
        .manage(database_pool)
        .manage(timer_state);

    if config.devops_config != AzureDevopsConfig::default() {
        builder = builder.manage(httpclient_pool);
    }

    let config_state = Mutex::new(config);

    builder
        .manage(config_state)
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
            save_devops_config,
            get_workitems
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
