use ratatui::prelude::*;
use tracing::instrument;

use super::state::Model;

mod stacklets;
pub mod tabs;

// const INFO_TEXT: [&str; 1] = ["(Esc or q) quit | (↑) move up | (↓) move down"];

#[instrument(skip(model))]
pub fn render(model: &mut Model, frame: &mut Frame) {
    let [tabs_header_area, tabs_body_area] =
        Layout::vertical([Constraint::Length(1), Constraint::Fill(100)]).areas(frame.area());

    let selected_tab = model.selected_tab;
    selected_tab.render_header(model, tabs_header_area, frame.buffer_mut());
    selected_tab.render_body(model, tabs_body_area, frame.buffer_mut());
}
