use std::cmp::max;

use ratatui::{
    prelude::*,
    widgets::{Block, Paragraph, Row, Table},
};
use stackable_cockpit::{platform::stacklet::Stacklet, utils::k8s::StackletConditionsExt};
use tracing::instrument;

use super::{logging::render_logging, state::Model};

// const INFO_TEXT: [&str; 1] = ["(Esc or q) quit | (↑) move up | (↓) move down"];

#[instrument(skip(model))]
pub fn render(model: &mut Model, frame: &mut Frame) {
    let main_layout = Layout::vertical([
        Constraint::Fill(40),
        Constraint::Fill(20),
        Constraint::Fill(40),
    ])
    .spacing(1)
    .split(frame.area());

    let table = get_table(&model.stacklets);

    model.stacklets_table_state.select(Some(2));

    frame.render_stateful_widget(table, main_layout[0], &mut model.stacklets_table_state);
    frame.render_widget(
        Paragraph::new(format!("Number of stacklets: {}", model.stacklets.len()))
            .block(Block::bordered().title("Stacklet details")),
        main_layout[1],
    );
    render_logging(
        main_layout[2],
        &mut model.logging_widget_state,
        frame.buffer_mut(),
    );
}

fn get_table<'a>(stacklets: &Vec<Stacklet>) -> Table<'a> {
    let mut rows = Vec::with_capacity(stacklets.len());

    // let stacklets = stacklets.iter();
    // .chain(stacklets.iter())
    // .chain(stacklets.iter())
    // .chain(stacklets.iter())
    // .chain(stacklets.iter())
    // .chain(stacklets.iter());

    for (_index, stacklet) in stacklets.iter().enumerate() {
        let stacklet_healthy = stacklet.conditions.is_stacklet_healthy();
        let reconciliation_paused = stacklet.conditions.is_reconciliation_paused();

        // let color = match index % 2 {
        //     0 => stacklet_table_colors.normal_row_color,
        //     _ => stacklet_table_colors.alt_row_color,
        // };
        let (fg_color, bg_color) = if stacklet_healthy {
            if reconciliation_paused {
                (Color::Gray, Color::Black)
            } else {
                (Color::Indexed(153), Color::Black)
            }
        } else {
            (Color::LightRed, Color::Black)
        };
        rows.push(
            Row::new(vec![
                stacklet.namespace.clone().unwrap_or_default(),
                stacklet.product.clone(),
                stacklet.name.clone(),
            ])
            .style(Style::new().fg(fg_color).bg(bg_color)),
        );
    }

    let widths = calculate_row_widths(stacklets);
    Table::new(rows, widths)
        // ...and they can be separated by a fixed spacing.
        .column_spacing(1)
        // You can set the style of the entire Table.
        .style(Style::new())
        // It has an optional header, which is simply a Row always visible at the top.
        .header(
            Row::new(vec!["Namespace", "Product", "Name"])
                .style(Style::new().bold())
                // To add space between the header and the rest of the rows, specify the margin
                .bottom_margin(1),
        )
        // As any other widget, a Table can be wrapped in a Block.
        .block(Block::bordered().title("Stacklets"))
}

fn calculate_row_widths(stacklets: &Vec<Stacklet>) -> [Constraint; 3] {
    let namespaces = stacklets.iter().filter_map(|s| s.namespace.as_ref());
    let product_names = stacklets.iter().map(|s| &s.product);
    let names = stacklets.iter().map(|s| &s.name);

    [
        Constraint::Length(max(
            longest_string(namespaces)
                .and_then(|l| l.try_into().ok())
                .unwrap_or_default(),
            "Namespace".len() as u16,
        )),
        Constraint::Length(max(
            longest_string(product_names)
                .and_then(|l| l.try_into().ok())
                .unwrap_or_default(),
            "Product".len() as u16,
        )),
        Constraint::Length(max(
            longest_string(names)
                .and_then(|l| l.try_into().ok())
                .unwrap_or_default(),
            "Name".len() as u16,
        )),
    ]
}

fn longest_string<'a>(strings: impl IntoIterator<Item = &'a String>) -> Option<usize> {
    strings.into_iter().map(|s| s.len()).max()
}
