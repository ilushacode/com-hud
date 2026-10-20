mod ports;
mod ui;

use std::thread;
use std::time::Duration;

use windows::Win32::Foundation::POINT;
use windows::Win32::UI::WindowsAndMessaging::GetCursorPos;
use winpane::{Context, HudConfig, Placement};

use ports::get_com_port_display_strings;
use ui::{compute_height, draw_widget, HOT_CORNER_SIZE};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let ctx = Context::new()?;

    let hud = ctx.create_hud(HudConfig {
        placement: Placement::Position { x: 0, y: 0 },
        width: 1,
        height: 1,
        position_key: None,
    })?;

    hud.set_opacity(0.9);

    let mut is_visible = false;

    loop {
        // Логика горячего угла
        let mut point = POINT { x: 0, y: 0 };
        unsafe { GetCursorPos(&mut point) }.ok();

        let in_hot_corner = point.x >= 0
            && point.x <= HOT_CORNER_SIZE
            && point.y >= 0
            && point.y <= HOT_CORNER_SIZE;

        if in_hot_corner && !is_visible {
            is_visible = true;

            let lines = get_com_port_display_strings();
            let new_height = compute_height(lines.len());

            hud.set_size(ui::HUD_WIDTH, new_height);
            draw_widget(&hud, &lines, new_height);
            hud.show();

        } else if !in_hot_corner && is_visible {
            is_visible = false;
            hud.set_size(1, 1);
            hud.hide();
        }

        thread::sleep(Duration::from_millis(50));
    }
}