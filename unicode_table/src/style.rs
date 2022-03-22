use once_cell::sync::Lazy;
use std::collections::HashMap;

#[derive(PartialEq, Clone, Debug, Eq, Hash)]
pub enum TableStyle {
    Text,
    // default style
    RegularBox,
    BoldBox,
    DoubleBox,
}

impl TableStyle {
    pub fn default_style() -> TableStyle {
        TableStyle::RegularBox
    }
}

pub struct StyleChar {
    pub vertical: &'static str,     // "│"
    pub horizontal: &'static str,   // "─"
    pub top_left: &'static str,     // "┌"
    pub top_right: &'static str,    // "┐"
    pub bottom_left: &'static str,  // "└"
    pub bottom_right: &'static str, // "┘"
    pub cross: &'static str,        // "┼"
    pub top_edge: &'static str,     // "┬"
    pub bottom_edge: &'static str,  // "┴"
    pub left_edge: &'static str,    // "├"
    pub right_edge: &'static str,   // "┤"
    pub connect_diff_style: Lazy<HashMap<TableStyle, ConnDiffStyleChar>>,
}

pub struct ConnDiffStyleChar {
    pub connect_with: &'static StyleChar,
    pub top_edge_conn_right: &'static str,    // "┲"
    pub top_edge_conn_left: &'static str,     // "┱"
    pub left_edge_conn_up: &'static str,      // "┡"
    pub left_edge_conn_down: &'static str,    // "┢"
    pub right_edge_conn_up: &'static str,     // "┦"
    pub right_edge_conn_down: &'static str,   // "┧"
    pub bottom_edge_conn_left: &'static str,  // "┹"
    pub bottom_edge_conn_right: &'static str, // "┺"
}

impl ConnDiffStyleChar {
    fn default(style_char: &'static StyleChar) -> Self {
        Self {
            connect_with: style_char,
            top_edge_conn_right: style_char.top_edge,
            top_edge_conn_left: style_char.top_edge,
            left_edge_conn_up: style_char.left_edge,
            left_edge_conn_down: style_char.left_edge,
            right_edge_conn_up: style_char.right_edge,
            right_edge_conn_down: style_char.right_edge,
            bottom_edge_conn_left: style_char.bottom_edge,
            bottom_edge_conn_right: style_char.bottom_edge,
        }
    }
}

static REGULAR_BOX: StyleChar = StyleChar {
    vertical: "│",
    horizontal: "─",
    top_left: "┌",
    top_right: "┐",
    bottom_left: "└",
    bottom_right: "┘",
    cross: "┼",
    top_edge: "┬",
    bottom_edge: "┴",
    left_edge: "├",
    right_edge: "┤",
    connect_diff_style: Lazy::new(|| {
        HashMap::from([(
            TableStyle::BoldBox,
            ConnDiffStyleChar {
                connect_with: &BOLD_BOX,
                top_edge_conn_right: "┺",
                top_edge_conn_left: "┹",
                left_edge_conn_up: "┢",
                left_edge_conn_down: "┡",
                right_edge_conn_up: "┧",
                right_edge_conn_down: "┦",
                bottom_edge_conn_left: "┻",
                bottom_edge_conn_right: "┺",
            },
        )])
    }),
};

static BOLD_BOX: StyleChar = StyleChar {
    vertical: "┃",
    horizontal: "━",
    top_left: "┏",
    top_right: "┓",
    bottom_left: "┗",
    bottom_right: "┛",
    cross: "╋",
    top_edge: "┳",
    bottom_edge: "┻",
    left_edge: "┣",
    right_edge: "┫",
    connect_diff_style: Lazy::new(|| {
        HashMap::from([(TableStyle::BoldBox, ConnDiffStyleChar::default(&BOLD_BOX))])
    }),
};

pub static TABLE_STYLE_MAP: Lazy<HashMap<TableStyle, &'static StyleChar>> = Lazy::new(|| {
    HashMap::from([
        (TableStyle::RegularBox, &REGULAR_BOX),
        (TableStyle::BoldBox, &BOLD_BOX),
    ])
});
