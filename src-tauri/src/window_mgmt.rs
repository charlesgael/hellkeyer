use tauri::{LogicalPosition, Window};

pub fn move_window_bottom_left(win: Window) {
    let monitor = win.current_monitor().unwrap().unwrap();
    let ms = monitor.size();
    let ws = win.outer_size().unwrap();

    let pos = LogicalPosition {
        x: ms.width - ws.width,
        y: ms.height - ws.height,
    };

    win.set_position(pos).unwrap();
}
