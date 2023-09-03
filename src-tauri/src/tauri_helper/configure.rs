use std::thread;

use crossbeam::{
    channel::{bounded, Receiver},
    sync::Parker,
};
use tauri::{
    CustomMenuItem, Manager, Menu, Runtime, Submenu, SystemTray, SystemTrayEvent, SystemTrayMenu,
};

use crate::timer::timer::{run_timer, timer_handler, TimerCommand};

pub trait ConfigureTauri {
    fn configure_window_menu(self) -> Self;

    fn configure_tray_menu(self) -> Self;

    fn setup_tauri(self, command_receiver: Receiver<TimerCommand>) -> Self;
}

impl<R: Runtime> ConfigureTauri for tauri::Builder<R> {
    fn configure_window_menu(self) -> Self {
        //Todo no darkmode until update https://github.com/tauri-apps/muda/issues/97
        let settings = CustomMenuItem::new("settings".to_string(), "Settings...");
        let quit = CustomMenuItem::new("quit".to_string(), "Quit");
        let submenu = Submenu::new("File", Menu::new().add_item(settings).add_item(quit));
        let menu = Menu::new().add_submenu(submenu);

        self.menu(menu)
            .on_menu_event(|event| match event.menu_item_id() {
                "settings" => {
                    let handle = event.window().app_handle();
                    std::thread::spawn(move || {
                        tauri::WindowBuilder::new(
                            &handle,
                            "settings",
                            tauri::WindowUrl::App("settings".into()),
                        )
                        .title("Settings")
                        .build()
                        .unwrap();
                    });
                }
                "quit" => {
                    std::process::exit(0);
                }
                _ => {}
            })
    }

    fn configure_tray_menu(self) -> Self {
        let quit = CustomMenuItem::new("quit_tray".to_string(), "Quit");
        let tray_menu = SystemTrayMenu::new().add_item(quit);
        let system_tray = SystemTray::new().with_menu(tray_menu);

        self.system_tray(system_tray)
            .on_system_tray_event(|app, event| match event {
                SystemTrayEvent::LeftClick {
                    position: _,
                    size: _,
                    ..
                } => {
                    if let Some(window) = app.get_window("main") {
                        if window.is_minimized().unwrap_or(false) {
                            window.unminimize().unwrap();
                        }
                        window.set_focus().unwrap();
                    } else {
                        let mut window_config = app.config().tauri.windows.get(0).unwrap().clone();
                        window_config.center = true;
                        tauri::WindowBuilder::from_config(app, window_config)
                            .build()
                            .unwrap();
                    }
                }
                SystemTrayEvent::MenuItemClick { id, .. } => match id.as_str() {
                    "quit" => {
                        std::process::exit(0);
                    }
                    _ => {}
                },
                _ => {}
            })
    }

    fn setup_tauri(self, command_receiver: Receiver<TimerCommand>) -> Self {
        self.setup(|app| {
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
    }
}
