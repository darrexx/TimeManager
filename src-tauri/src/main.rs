// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use azure_devops::client::{configure_devops_httpclient, AzureDevopsClient};
use config::models::{AzureDevopsConfig, Config};
use crossbeam::channel::bounded;
use diesel::r2d2::{ConnectionManager, Pool};
use diesel::sqlite::SqliteConnection;
use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};
use kimai::client::{configure_kimai_httpclient, KimaiClient};
use state::models::{Frontend, Timer};
use tauri::async_runtime::Mutex;
use tauri_helper::configure::ConfigureTauri;
use timer::timer::TimerCommand;

use crate::activity::commands::{get_activities, get_activity_times, get_history};
use crate::azure_devops::commands::get_workitems;
use crate::config::commands::save_devops_config;
use crate::kimai::commands::{get_customers, get_kimai_activities, get_projects};
use crate::state::commands::{get_config, get_frontend_state, set_config};
use crate::tauri_helper::commands::toggle_popout;
use crate::timer::commands::{reset_timer, start_timer, start_timer_with_workitem, stop_timer};

mod activity;
mod azure_devops;
mod config;
mod db;
mod kimai;
mod reqwest_helper;
mod schema;
mod state;
mod tauri_helper;
mod timer;

const MIGRATIONS: EmbeddedMigrations = embed_migrations!();

fn main() {
    let config: Config = confy::load("timemanager", None).unwrap();

    let devops_httpclient_pool = AzureDevopsClient(configure_devops_httpclient(
        &config.devops_config.user,
        &config.devops_config.pat,
    ));

    let kimai_httpclient_pool = KimaiClient(configure_kimai_httpclient(
        &config.kimai_config.user,
        &config.kimai_config.token,
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
        .manage(frontend_state)
        .manage(kimai_httpclient_pool);

    if config.devops_config != AzureDevopsConfig::default() {
        builder = builder.manage(devops_httpclient_pool);
    }

    let config_state = Mutex::new(config);

    builder
        .manage(config_state)
        .setup_tauri(command_receiver)
        .invoke_handler(tauri::generate_handler![
            start_timer,
            stop_timer,
            reset_timer,
            save_devops_config,
            get_workitems,
            start_timer_with_workitem,
            get_config,
            set_config,
            toggle_popout,
            get_activities,
            get_frontend_state,
            get_activity_times,
            get_history,
            get_projects,
            get_customers,
            get_kimai_activities
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

fn run_db_migrations(conn: &mut impl MigrationHarness<diesel::sqlite::Sqlite>) {
    conn.run_pending_migrations(MIGRATIONS)
        .expect("Could not run migrations");
}
