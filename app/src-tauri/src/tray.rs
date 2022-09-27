use tauri::{CustomMenuItem, SystemTray, SystemTrayMenu, SystemTrayMenuItem};

pub fn create_tray() -> SystemTray {
  let show = CustomMenuItem::new("show".to_string(), "Show");
  let quit = CustomMenuItem::new("quit".to_string(), "Quit");
  let hide = CustomMenuItem::new("hide".to_string(), "Hide");
  let take_out = CustomMenuItem::new("take_out".to_string(), "Take Out");
  let tray_menu = SystemTrayMenu::new()
    .add_item(quit)
    .add_item(take_out)
    .add_native_item(SystemTrayMenuItem::Separator)
    .add_item(hide)
    .add_native_item(SystemTrayMenuItem::Separator)
    .add_item(show);

  SystemTray::new().with_menu(tray_menu)
}
