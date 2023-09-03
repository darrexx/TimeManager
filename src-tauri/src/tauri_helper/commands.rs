use tauri::{AppHandle, Manager, Menu, State};

use crate::state::models::FrontendState;

#[tauri::command]
pub async fn toggle_popout(
    app_handle: AppHandle,
    frontend_state: State<'_, FrontendState>,
    active: bool,
) -> Result<(), ()> {
    let mut state = frontend_state.lock().await;
    state.popout_active = active;

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
