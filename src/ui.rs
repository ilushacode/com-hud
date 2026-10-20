use winpane::{Color, Hud, RectElement, TextElement};

pub const HUD_WIDTH: u32 = 280;
pub const HOT_CORNER_SIZE: i32 = 10;

const FONT_SIZE: f32 = 13.0;
const LINE_HEIGHT: f32 = 22.0;
const PADDING_TOP: f32 = 10.0;
const PADDING_BOTTOM: f32 = 10.0;

/// Вычисляет высоту HUD на основе количества отображаемых строк.
pub fn compute_height(line_count: usize) -> u32 {
    let count = line_count.max(1) as f32;
    (PADDING_TOP + count * LINE_HEIGHT + PADDING_BOTTOM).ceil() as u32
}

/// Отрисовывает фон и текст виджета.
pub fn draw_widget(hud: &Hud, lines: &[String], height: u32) {
    // Фон
    hud.set_rect("bg", RectElement {
        x: 0.0, y: 0.0,
        width: HUD_WIDTH as f32,
        height: height as f32,
        fill: Color::rgba(18, 18, 24, 230), // 90% непрозрачности
        corner_radius: 0.0,
        ..Default::default()
    });

    // Текст
    for (i, line) in lines.iter().enumerate() {
        hud.set_text(&format!("line{}", i), TextElement {
            text: line.clone(),
            x: 12.0,
            y: PADDING_TOP + (i as f32) * LINE_HEIGHT,
            font_size: FONT_SIZE,
            color: Color::rgba(255, 255, 255, 230), // 90% непрозрачности
            ..Default::default()
        });
    }

    // Удаляем старые строки, если портов стало меньше
    for i in lines.len()..10 {
        hud.remove(&format!("line{}", i));
    }
}