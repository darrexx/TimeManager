use std::time::Duration;

use crossbeam::{
    channel::{Receiver, Sender, TryRecvError},
    select,
    sync::{Parker, Unparker},
};
use tauri::{AppHandle, Manager, Runtime};

pub enum TimerCommand {
    Start(u64),
    Stop,
    Reset,
}

pub fn timer_handler<R: Runtime>(
    app_handle: &AppHandle<R>,
    command_receiver: Receiver<TimerCommand>,
    tick_receiver: Receiver<()>,
    timer_sender: Sender<TimerCommand>,
    unparker: Unparker,
) -> ! {
    let mut ticks_per_100millis = 0;

    loop {
        select! {
            recv(command_receiver) -> msg =>
            {
                match msg {
                    Ok(command) => match command {
                        TimerCommand::Start(existing_duration) => {
                            ticks_per_100millis = existing_duration / 100;
                            unparker.unpark();
                            timer_sender.send(command).unwrap()
                        },
                        TimerCommand::Stop => timer_sender.send(command).unwrap(),
                        TimerCommand::Reset => {
                            ticks_per_100millis = 0;
                        },
                    },
                    Err(_) =>  panic!("Could not Receive Command"),
                }
            },
            recv(tick_receiver) -> msg => {
                match msg {
                    Ok(()) => {
                        ticks_per_100millis += 1;
                        if ticks_per_100millis % 10 == 0 {
                            app_handle.emit_all("timertick", ticks_per_100millis/10).unwrap();
                        }
                    },
                    Err(_) => panic!("Could not Receive tick"),
                }

            },
        }
    }
}

pub fn run_timer(rx: Receiver<TimerCommand>, tx: Sender<()>, parker: Parker) -> ! {
    let mut running = false;

    loop {
        if !running {
            parker.park();
        }

        let recv_result = rx.try_recv();
        match recv_result {
            Ok(command) => match command {
                TimerCommand::Start(_) => running = true,
                TimerCommand::Stop => running = false,
                _ => todo!(),
            },
            Err(err) => {
                if err == TryRecvError::Disconnected {
                    panic!("command channel disconnected");
                }
            }
        }

        spin_sleep::sleep(Duration::from_millis(100));

        if running {
            tx.send(()).unwrap();
        }
    }
}
