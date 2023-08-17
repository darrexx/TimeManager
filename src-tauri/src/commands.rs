use crossbeam::channel::Sender;
use tauri::State;

use crate::{timer::TimerCommand, TimerState};

#[tauri::command]
pub fn start_timer(sender_state: State<Sender<TimerCommand>>, timer_state: State<TimerState>) {
    sender_state.send(TimerCommand::Start).unwrap();

    let mut timer = timer_state.lock().unwrap();
    timer.running = true;
    timer.start_time = Some(chrono::Utc::now());
}

#[tauri::command]
pub fn stop_timer(sender_state: State<Sender<TimerCommand>>, timer_state: State<TimerState>) {
    sender_state.send(TimerCommand::Stop).unwrap();

    let mut timer = timer_state.lock().unwrap();
    timer.running = false;
    timer.start_time = None;
}

#[tauri::command]
pub fn reset_timer(sender_state: State<Sender<TimerCommand>>, timer_state: State<TimerState>) {
    sender_state.send(TimerCommand::Reset).unwrap();

    let mut timer = timer_state.lock().unwrap();
    timer.start_time = Some(chrono::Utc::now());
}
