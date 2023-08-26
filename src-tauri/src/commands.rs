use crate::{
    azure_devops::{
        api::{
            get_my_workitems_for_current_iteration, get_workitems_by_ids,
            update_workitem_to_in_progress,
        },
        client::{configure_devops_httpclient, AzureDevopsClient},
        error::AzureDevopsError,
        models::Workitem,
    },
    config::{Config, FrontendConfig},
    db::{
        activity::{
            create_activity, get_activity, get_all_activities, get_last_activities, update_activity,
        },
        models::Activity,
    },
    state::{set_start_state, ConfigState, TimerState},
    timer::TimerCommand,
};
use chrono::Utc;
use crossbeam::channel::Sender;
use diesel::{
    r2d2::{ConnectionManager, Pool},
    SqliteConnection,
};
use tauri::{AppHandle, Manager, Menu, State};

#[tauri::command]
pub async fn start_timer(
    app_handle: AppHandle,
    sender_state: State<'_, Sender<TimerCommand>>,
    timer_state: State<'_, TimerState>,
    db: State<'_, Pool<ConnectionManager<SqliteConnection>>>,
    config: State<'_, ConfigState>,
    activity_name: String,
) -> Result<(), ()> {
    start_timer_internal(
        &app_handle,
        sender_state,
        timer_state,
        db,
        config,
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
    workitem_name: String,
    workitem_id: i64,
) -> Result<(), ()> {
    start_timer_internal(
        &app_handle,
        sender_state,
        timer_state,
        db,
        config,
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
    activity_name: String,
    workitem_id: Option<i64>,
) -> Result<(), ()> {
    let mut timer = timer_state.lock().await;
    let connection = &mut db.get().unwrap();

    let existing_activity = get_activity(connection, &activity_name);

    set_start_state(&mut timer, &activity_name, &existing_activity);

    if let Some(activity) = existing_activity {
        let existing_duration = activity.duration.unwrap_or(0);
        sender_state
            .send(TimerCommand::Start(existing_duration as u64))
            .unwrap();
    } else {
        create_activity(connection, &activity_name, workitem_id);
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

    update_activity(connection, &activity_name, activity_duration);

    timer.start_time = None;

    Ok(())
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

#[tauri::command]
pub fn get_activity_history(db: State<Pool<ConnectionManager<SqliteConnection>>>) -> Vec<Activity> {
    let connection = &mut db.get().unwrap();
    match get_last_activities(connection) {
        Some(activities) => activities,
        None => vec![],
    }
}

#[tauri::command]
pub fn get_activities(db: State<Pool<ConnectionManager<SqliteConnection>>>) -> Vec<Activity> {
    let connection = &mut db.get().unwrap();
    match get_all_activities(connection) {
        Some(activities) => activities,
        None => vec![],
    }
}

#[allow(clippy::too_many_arguments)]
#[tauri::command]
pub async fn save_devops_config(
    app_handle: AppHandle,
    config: State<'_, ConfigState>,
    url: String,
    user: String,
    pat: String,
    organization: String,
    project: String,
    team: String,
) -> Result<(), ()> {
    let devops_client = configure_devops_httpclient(&user, &pat);
    app_handle.manage(AzureDevopsClient(devops_client));

    let mut config = config.lock().await;
    config.devops_config.base_url = url;
    config.devops_config.user = user;
    config.devops_config.pat = pat;
    config.devops_config.organization = organization;
    config.devops_config.project = project;
    config.devops_config.team = team;

    confy::store("timemanager", None, Config::from(&config)).unwrap();

    Ok(())
}

#[tauri::command]
pub async fn get_workitems(
    app_handle: AppHandle,
    config: State<'_, ConfigState>,
) -> Result<Vec<Workitem>, AzureDevopsError> {
    if let Some(client) = app_handle.try_state::<AzureDevopsClient>() {
        let config = config.lock().await;

        let workitem_ids = get_my_workitems_for_current_iteration(
            client.get(),
            &config.devops_config.base_url,
            &config.devops_config.organization,
            &config.devops_config.project,
            &config.devops_config.team,
        )
        .await?;

        let workitems = get_workitems_by_ids(
            client.get(),
            &config.devops_config.base_url,
            &config.devops_config.organization,
            &config.devops_config.project,
            workitem_ids,
        )
        .await?;

        Ok(workitems)
    } else {
        Err(AzureDevopsError::Unauthorized)
    }
}

#[tauri::command]
pub async fn get_config(config: State<'_, ConfigState>) -> Result<Config, String> {
    let config = config.lock().await;

    Ok(Config::from(&config))
}

#[tauri::command]
pub async fn set_config(config: FrontendConfig) -> Result<(), ()> {
    confy::store("timemanager", None, Config::from(config)).unwrap();

    Ok(())
}

#[tauri::command]
pub async fn toggle_popout(app_handle: AppHandle, active: bool) -> Result<(), ()> {
    if active {
        std::thread::spawn(move || {
            tauri::WindowBuilder::new(
                &app_handle,
                "popout",
                tauri::WindowUrl::App("popout".into()),
            )
            .title("popout")
            .transparent(true)
            .decorations(false)
            .resizable(false)
            .inner_size(165f64, 45f64)
            .always_on_top(true)
            .menu(Menu::new())
            .skip_taskbar(true)
            .build()
            .unwrap();
        });
    } else {
        app_handle.get_window("popout").unwrap().close().unwrap();
    }
    Ok(())
}
