use crate::{
    azure_devops::client::{configure_devops_httpclient, AzureDevopsClient},
    config::Config,
    db::{
        activity::{create_activity, get_activity, get_last_activities, update_activity},
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
use tauri::{AppHandle, Manager, State};

#[tauri::command]
pub fn start_timer(
    sender_state: State<Sender<TimerCommand>>,
    timer_state: State<TimerState>,
    db: State<Pool<ConnectionManager<SqliteConnection>>>,
    activity_name: String,
) {
    let mut timer = timer_state.lock().unwrap();
    let connection = &mut db.get().unwrap();

    let existing_activity = get_activity(connection, &activity_name);

    set_start_state(&mut timer, &activity_name, &existing_activity);

    if let Some(activity) = existing_activity {
        let existing_duration = activity.duration.unwrap_or(0);
        sender_state
            .send(TimerCommand::Start(existing_duration as u64))
            .unwrap();
    } else {
        create_activity(connection, &activity_name);
        sender_state.send(TimerCommand::Start(0)).unwrap();
    }
}

#[tauri::command]
pub fn stop_timer(
    sender_state: State<Sender<TimerCommand>>,
    timer_state: State<TimerState>,
    db: State<Pool<ConnectionManager<SqliteConnection>>>,
) {
    sender_state.send(TimerCommand::Stop).unwrap();

    let mut timer = timer_state.lock().unwrap();
    timer.running = false;

    let connection = &mut db.get().unwrap();

    if timer.start_time.is_none() || timer.activity_name.is_none() {
        return;
    }
    let activity_duration = get_activity_duration(&timer.start_time, &timer.activity_duration);
    let activity_name = timer.activity_name.clone().unwrap();

    update_activity(connection, &activity_name, activity_duration);

    timer.start_time = None;
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
pub fn reset_timer(sender_state: State<Sender<TimerCommand>>, timer_state: State<TimerState>) {
    sender_state.send(TimerCommand::Reset).unwrap();

    let mut timer = timer_state.lock().unwrap();
    timer.start_time = Some(chrono::Utc::now());
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
pub fn save_devops_config(
    app_handle: AppHandle,
    config: State<ConfigState>,
    url: String,
    user: String,
    pat: String,
    organization: String,
    project: String,
    team: String,
) {
    let devops_client = configure_devops_httpclient(&user, &pat);
    app_handle.manage(AzureDevopsClient(devops_client));

    let mut config = config.lock().unwrap();
    config.devops_config.base_url = url;
    config.devops_config.user = user;
    config.devops_config.pat = pat;
    config.devops_config.organization = organization;
    config.devops_config.project = project;
    config.devops_config.team = team;

    confy::store("timemanager", None, Config::from(&config)).unwrap();
}

#[tauri::command]
pub async fn test_command(app_handle: AppHandle) -> Result<String, String> {
    println!("Command");

    let client = app_handle.try_state::<AzureDevopsClient>();
    match client {
        Some(_) => println!("asdf"),
        None => print!("qwer"),
    };
    Ok(String::from("a"))
}
