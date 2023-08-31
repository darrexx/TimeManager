// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use azure_devops::client::{configure_devops_httpclient, AzureDevopsClient};
use commands::{
    get_activities, get_activity_history, get_activity_time_history, get_activity_times,
    get_config, get_frontend_state, get_workitems, reset_timer, save_devops_config, set_config,
    start_timer, start_timer_with_workitem, stop_timer, toggle_popout,
};
use config::{AzureDevopsConfig, Config};
use crossbeam::channel::bounded;
use diesel::r2d2::{ConnectionManager, Pool};
use diesel::sqlite::SqliteConnection;
use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};
use state::{Frontend, Timer};
use tauri::async_runtime::Mutex;
use tauri_helper::ConfigureTauri;
use timer::TimerCommand;

mod azure_devops;
mod commands;
mod config;
mod db;
mod schema;
mod state;
mod tauri_helper;
mod timer;

const MIGRATIONS: EmbeddedMigrations = embed_migrations!();

fn main() {
    let config: Config = confy::load("timemanager", None).unwrap();

    let httpclient_pool = AzureDevopsClient(configure_devops_httpclient(
        &config.devops_config.user,
        &config.devops_config.pat,
    ));

    let (command_sender, command_receiver) = bounded::<TimerCommand>(10);

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

    run_db_migrations(&mut database_pool.get().unwrap());

    let frontend_state = Mutex::new(Frontend::default());

    let mut builder = tauri::Builder::default()
        .configure_window_menu()
        .configure_tray_menu()
        .manage(command_sender)
        .manage(database_pool)
        .manage(timer_state)
        .manage(frontend_state);

    if config.devops_config != AzureDevopsConfig::default() {
        builder = builder.manage(httpclient_pool);
    }

    let config_state = Mutex::new(config);

    builder
        .manage(config_state)
        .setup_tauri(command_receiver)
        .invoke_handler(tauri::generate_handler![
            start_timer,
            stop_timer,
            reset_timer,
            get_activity_history,
            save_devops_config,
            get_workitems,
            start_timer_with_workitem,
            get_config,
            set_config,
            toggle_popout,
            get_activities,
            get_frontend_state,
            get_activity_time_history,
            get_activity_times
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

fn run_db_migrations(conn: &mut impl MigrationHarness<diesel::sqlite::Sqlite>) {
    conn.run_pending_migrations(MIGRATIONS)
        .expect("Could not run migrations");
}
