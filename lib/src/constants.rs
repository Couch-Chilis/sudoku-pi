use bevy::prelude::*;

// Base colors.
pub const COLOR_BAMBOO_SHOOT: Color = Color::srgb(204. / 255., 188. / 255., 42. / 255.);
pub const COLOR_CREAM: Color = Color::srgb(1., 254. / 255., 248. / 255.);
pub const COLOR_MUSTARD: Color = Color::srgb(229. / 255., 216. / 255., 97. / 255.);
pub const COLOR_EGGSHELL: Color = Color::srgb(251. / 255., 249. / 255., 230. / 255.);
pub const COLOR_MAIN: Color = Color::srgb(169. / 255., 154. / 255., 55. / 255.);
pub const COLOR_MAIN_DARKER: Color = Color::srgb(139. / 255., 126. / 255., 45. / 255.);
pub const COLOR_MAIN_DARKEST: Color = Color::srgb(92. / 255., 84. / 255., 30. / 255.);
//pub const COLOR_MAIN_LIGHT: Color = Color::rgb(185. / 255., 168. / 255., 60. / 255.);
pub const COLOR_POP_DARK: Color = Color::srgb(213. / 255., 11. / 255., 72. / 255.);
pub const COLOR_POP_FOCUS: Color = Color::srgb(245. / 255., 55. / 255., 112. / 255.);
//pub const COLOR_ORANGE: Color = Color::rgb(236. / 255., 117. / 255., 5. / 255.);

// Button colors.
pub const COLOR_BUTTON_TEXT: Color = Color::WHITE;
pub const COLOR_BUTTON_TEXT_PRESS: Color = COLOR_POP_DARK;
pub const COLOR_BUTTON_BACKGROUND: Color = COLOR_MAIN;
pub const COLOR_BUTTON_BACKGROUND_SELECTED: Color = COLOR_POP_DARK;
pub const COLOR_BUTTON_BACKGROUND_PRESS: Color = Color::WHITE;

pub const COLOR_SECONDARY_BUTTON_TEXT: Color = COLOR_MAIN;
pub const COLOR_SECONDARY_BUTTON_TEXT_SELECTED: Color = COLOR_POP_DARK;
pub const COLOR_SECONDARY_BUTTON_TEXT_PRESS: Color = COLOR_CREAM;
pub const COLOR_SECONDARY_BUTTON_BACKGROUND: Color = COLOR_CREAM;
pub const COLOR_SECONDARY_BUTTON_BACKGROUND_PRESS: Color = COLOR_POP_DARK;
pub const COLOR_SECONDARY_BUTTON_BORDER: Color = COLOR_MAIN;
pub const COLOR_SECONDARY_BUTTON_BORDER_SELECTED: Color = COLOR_POP_DARK;
pub const COLOR_SECONDARY_BUTTON_BORDER_PRESS: Color = COLOR_CREAM;

pub const COLOR_TERNARY_BUTTON_TEXT: Color = COLOR_MAIN;
pub const COLOR_TERNARY_BUTTON_TEXT_SELECTED: Color = COLOR_POP_DARK;
pub const COLOR_TERNARY_BUTTON_TEXT_PRESS: Color = Color::WHITE;
pub const COLOR_TERNARY_BUTTON_BACKGROUND: Color = Color::WHITE;
pub const COLOR_TERNARY_BUTTON_BACKGROUND_PRESS: Color = COLOR_POP_DARK;
pub const COLOR_TERNARY_BUTTON_BORDER: Color = COLOR_MAIN;
pub const COLOR_TERNARY_BUTTON_BORDER_SELECTED: Color = COLOR_POP_DARK;
pub const COLOR_TERNARY_BUTTON_BORDER_PRESS: Color = Color::WHITE;

// Timer colors.
pub const COLOR_TIMER_BORDER: Color = COLOR_MAIN;
pub const COLOR_TIMER_TEXT: Color = COLOR_MAIN_DARKEST;

// Score color.
pub const COLOR_SCORE_TEXT: Color = COLOR_POP_DARK;

// Wheel colors.
pub const COLOR_WHEEL_TOP_TEXT: Color = Color::WHITE;

// Hint color.
pub const COLOR_HINT: Color = Color::srgb(163. / 255., 217. / 255., 1.);

// Board colors.
pub const COLOR_BOARD_LINE_THICK: Color = COLOR_MAIN_DARKEST;
pub const COLOR_BOARD_LINE_MEDIUM: Color = Color::srgb(185. / 255., 178. / 255., 129. / 255.);
pub const COLOR_BOARD_LINE_THIN: Color = Color::srgb(238. / 255., 235. / 255., 215. / 255.);

// Cell colors
pub const COLOR_CELL_SELECTION: Color = COLOR_BAMBOO_SHOOT;
pub const COLOR_CELL_SAME_NUMBER: Color = COLOR_MUSTARD;
pub const COLOR_CELL_HIGHLIGHT: Color = COLOR_EGGSHELL;

// Cell size.
pub const CELL_SIZE: f32 = 0.111111;

// Highscores.
pub const MAX_NUM_HIGHSCORES: usize = 5;
