use crate::Color;

#[derive(Clone, Debug)]
pub struct GridConfig {
    pub x_color: Color,
    pub y_color: Color,
    pub line_width: f32,
    pub minor_x_color: Color,
    pub minor_y_color: Color,
    pub minor_line_width: f32,
    pub show_x_grid: bool, // New: controls X-axis grids
    pub show_y_grid: bool, // New: controls Y-axis grids
}

impl Default for GridConfig {
    fn default() -> Self {
        Self {
            x_color: Color::LightGray,
            y_color: Color::LightGray,
            line_width: 0.5,
            minor_x_color: Color::LightGray,
            minor_y_color: Color::LightGray,
            minor_line_width: 0.3,
            show_x_grid: true, // Default: show X grids
            show_y_grid: true, // Default: show Y grids
        }
    }
}
