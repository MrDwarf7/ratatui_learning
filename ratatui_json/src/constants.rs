use ratatui::style::Color;

pub const HINT_TEXT: &str = "Press <e> to enter editing mode, <q> to quit";
pub const HINT_TEXT_EDITING: &str =
    "Press <Enter> to save, <Tab> to switch between key and value, <Esc> to exit";
pub const HINT_TEXT_EXITING: &str =
    "Press <y>/<Enter> to confirm exit, <n> to cancel, <Esc> to go back";

pub const HINT_TEXT_KEY: &str = "Editing Key";
pub const HINT_TEXT_VALUE: &str = "Editing Value";

pub const GENERAL_BG_COLOR: Color = Color::Black;
pub const ACTIVE_BG_COLOR: Color = Color::Black;

pub const TEXT_COLOR: Color = Color::White;
pub const DARK_TEXT_COLOR: Color = Color::DarkGray;
pub const LIST_ITEM_TEXT_COLOR: Color = Color::LightBlue;
pub const LIST_ITEM_BG_COLOR: Color = Color::Black;

pub const PRIMARY_COLOR: Color = Color::Green;
pub const ACCENT_COLOR: Color = Color::LightBlue;
pub const ERROR_COLOR: Color = Color::Red;

pub const HINT_NORMAL_MODE: Color = Color::Green;
pub const HINT_EDITING_MODE: Color = Color::Yellow;
pub const HINT_EXITING_MODE: Color = Color::Red;
