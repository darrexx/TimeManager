use chrono::{DateTime, TimeZone, Utc};
use crossbeam::channel::Sender;
use diesel::{
    r2d2::{ConnectionManager, Pool},
    SqliteConnection,
};
use serde::{Deserialize, Serialize};
use tauri::{AppHandle, Manager, State};

use crate::{
    azure_devops::{api::update_workitem_to_in_progress, client::AzureDevopsClient},
    db::activity::{
        create_activity, create_activity_time, get_activity, update_activity, update_activity_time,
    },
    kimai::{api::post_timesheet, client::KimaiClient},
    state::{
        models::{ConfigState, FrontendState, TimerState},
        state::set_start_state,
    },
};

use super::timer::TimerCommand;

#[tauri::command]
pub async fn start_timer(
    app_handle: AppHandle,
    sender_state: State<'_, Sender<TimerCommand>>,
    timer_state: State<'_, TimerState>,
    db: State<'_, Pool<ConnectionManager<SqliteConnection>>>,
    config: State<'_, ConfigState>,
    frontend_state: State<'_, FrontendState>,
    activity_name: String,
) -> Result<(), ()> {
    start_timer_internal(
        &app_handle,
        sender_state,
        timer_state,
        db,
        config,
        frontend_state,
        activity_name,
        None,
    )
    .await
}

#[tauri::command]
pub async fn start_timer_with_workitem(
    app_handle: AppHandle,
    sender_state: State<'_, Sender<TimerCommand>>,
    timer_state: State<'_, TimerState>,
    db: State<'_, Pool<ConnectionManager<SqliteConnection>>>,
    config: State<'_, ConfigState>,
    frontend_state: State<'_, FrontendState>,
    workitem_name: String,
    workitem_id: i64,
) -> Result<(), ()> {
    start_timer_internal(
        &app_handle,
        sender_state,
        timer_state,
        db,
        config,
        frontend_state,
        workitem_name,
        Some(workitem_id),
    )
    .await
}

async fn start_timer_internal(
    app_handle: &AppHandle,
    sender_state: State<'_, Sender<TimerCommand>>,
    timer_state: State<'_, TimerState>,
    db: State<'_, Pool<ConnectionManager<SqliteConnection>>>,
    config: State<'_, ConfigState>,
    frontend_state: State<'_, FrontendState>,
    activity_name: String,
    workitem_id: Option<i64>,
) -> Result<(), ()> {
    let mut frontend_state = frontend_state.lock().await;
    frontend_state.current_activity = Some(activity_name.clone());

    let mut timer = timer_state.lock().await;
    let connection = &mut db.get().unwrap();

    let existing_activity = get_activity(connection, &activity_name);

    set_start_state(&mut timer, &activity_name, &existing_activity);

    if let Some(activity) = existing_activity {
        create_activity_time(connection, activity.id);
        let existing_duration = activity.duration.unwrap_or(0);
        sender_state
            .send(TimerCommand::Start(existing_duration as u64))
            .unwrap();
    } else {
        let id = create_activity(connection, &activity_name, workitem_id);
        create_activity_time(connection, id);
        sender_state.send(TimerCommand::Start(0)).unwrap();
    }

    let config = config.lock().await;
    if config.devops_config.automatically_update_workitems && workitem_id.is_some() {
        if let Some(client) = app_handle.try_state::<AzureDevopsClient>() {
            update_workitem_to_in_progress(
                client.get(),
                &config.devops_config.base_url,
                &config.devops_config.organization,
                &config.devops_config.project,
                workitem_id.unwrap(),
            )
            .await
            .unwrap();
        }
    }

    Ok(())
}

#[tauri::command]
pub async fn stop_timer(
    sender_state: State<'_, Sender<TimerCommand>>,
    timer_state: State<'_, TimerState>,
    db: State<'_, Pool<ConnectionManager<SqliteConnection>>>,
    kimai_client: State<'_, KimaiClient>,
    config: State<'_, ConfigState>,
    kimai: KimaiCommandArguments,
) -> Result<(), ()> {
    sender_state.send(TimerCommand::Stop).unwrap();

    let mut timer = timer_state.lock().await;
    timer.running = false;

    let connection = &mut db.get().unwrap();

    if timer.start_time.is_none() || timer.activity_name.is_none() {
        return Ok(());
    }
    let activity_duration = get_activity_duration(&timer.start_time, &timer.activity_duration);
    let activity_name = timer.activity_name.clone().unwrap();

    let updated_activity = update_activity(connection, &activity_name, activity_duration);
    let updated_activity_time = update_activity_time(connection, updated_activity);

    if kimai.use_kimai {
        let config = config.lock().await;

        let start_time = DateTime::from(
            Utc.timestamp_millis_opt(updated_activity_time.start_time)
                .single()
                .unwrap(),
        );
        let end_time = DateTime::from(
            Utc.timestamp_millis_opt(updated_activity_time.end_time.unwrap())
                .single()
                .unwrap(),
        );

        post_timesheet(
            &kimai_client.get(),
            &config.kimai_config.base_url,
            kimai.project,
            kimai.activity,
            start_time,
            end_time,
            activity_name,
        )
        .await
        .unwrap();
    }

    timer.start_time = None;

    Ok(())
}

#[derive(Serialize, Deserialize)]
pub struct KimaiCommandArguments {
    pub use_kimai: bool,
    pub project: i32,
    pub activity: i32,
}

fn get_activity_duration(
    start_timer: &Option<chrono::DateTime<Utc>>,
    existing_duration: &Option<chrono::Duration>,
) -> chrono::Duration {
    let now = chrono::Utc::now();
    now - start_timer.unwrap_or(now)
        + existing_duration.unwrap_or(chrono::Duration::milliseconds(0))
}

#[tauri::command]
pub async fn reset_timer(
    sender_state: State<'_, Sender<TimerCommand>>,
    timer_state: State<'_, TimerState>,
) -> Result<(), ()> {
    sender_state.send(TimerCommand::Reset).unwrap();

    let mut timer = timer_state.lock().await;
    timer.start_time = Some(chrono::Utc::now());
    Ok(())
}
