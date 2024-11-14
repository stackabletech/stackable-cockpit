use std::cmp::max;

use ratatui::{
    prelude::*,
    widgets::{Block, Cell, Paragraph, Row, Table, Wrap},
};
use stackable_cockpit::{platform::stacklet::Stacklet, utils::k8s::StackletConditionsExt};

use crate::cmds::control_center::state::Model;

pub fn render_stacklets(model: &mut Model, area: Rect, buf: &mut Buffer) {
    let [table_area, details_area] = Layout::vertical([Constraint::Fill(70), Constraint::Fill(30)])
        .spacing(1)
        .areas(area);

    render_table(model, table_area, buf);
    render_details(model, details_area, buf);
}

fn render_table(model: &mut Model, area: Rect, buf: &mut Buffer) {
    let mut rows = Vec::with_capacity(model.stacklets.len());

    let healthy_style = Style::default().fg(Color::Indexed(153)).bg(Color::Black);
    let unhealthy_style = Style::default().fg(Color::LightRed).bg(Color::Black);
    let reconciliation_paused_style = Style::default().fg(Color::Gray).bg(Color::Black);

    if model.stacklets_table_state.selected().is_none() {
        model.stacklets_table_state.select(Some(0));
    }

    let selected = model
        .stacklets_table_state
        .selected()
        .expect("There must always be a row selected in the stacklets table");

    for (index, stacklet) in model.stacklets.iter().enumerate() {
        let stacklet_healthy = stacklet.conditions.is_stacklet_healthy();
        let reconciliation_paused = stacklet.conditions.is_reconciliation_paused();

        let mut style = match (reconciliation_paused, stacklet_healthy) {
            (true, _) => reconciliation_paused_style,
            (false, true) => healthy_style,
            (false, false) => unhealthy_style,
        };

        if index == selected {
            style = style.reversed();
        }

        let namespace_cell = Cell::new(stacklet.namespace.clone().unwrap_or_default());
        let product_cell = Cell::new(stacklet.product.as_str());
        let name_cell = Cell::new(stacklet.name.as_str());
        let endpoint_lines = stacklet
            .endpoints
            .iter()
            .map(|(name, endpoint)| Line::from(format!("{name}: {endpoint}")))
            .collect::<Vec<_>>();
        let num_endpoint_lines = max(endpoint_lines.len(), 1);
        let endpoints_cell = Cell::new(endpoint_lines);

        rows.push(
            Row::new(vec![
                namespace_cell,
                product_cell,
                name_cell,
                endpoints_cell,
            ])
            .style(style)
            .height(
                u16::try_from(num_endpoint_lines)
                    .expect("The height of endpoint lines needs to fit in a u16")
                    .saturating_add(1),
            ),
        );
    }

    let widths = calculate_row_widths(&model.stacklets);

    let table = Table::new(rows, widths)
        // ...and they can be separated by a fixed spacing.
        .column_spacing(2)
        // You can set the style of the entire Table.
        .style(Style::new())
        // It has an optional header, which is simply a Row always visible at the top.
        .header(
            Row::new(vec!["Namespace", "Product", "Name", "Endpoints"])
                .style(Style::new().bold())
                // To add space between the header and the rest of the rows, specify the margin
                .bottom_margin(1),
        )
        // We don't use the `row_highlight_style` function, as we need a more flexible way.
        // .row_highlight_style(healthy_style.reversed())
        .block(Block::bordered().title("Stacklets"));

    StatefulWidget::render(table, area, buf, &mut model.stacklets_table_state);
}

fn render_details(model: &mut Model, area: Rect, buf: &mut Buffer) {
    let maybe_selected_stacklet = model
        .stacklets_table_state
        .selected()
        .and_then(|selected| model.stacklets.get(selected));

    // It might be the case stacklets have not been loaded yet
    let paragraph = if let Some(stacklet) = maybe_selected_stacklet {
        Paragraph::new(format!("{:?}", stacklet))
    } else {
        Paragraph::new("")
    };
    paragraph
        .wrap(Wrap { trim: false })
        .block(Block::bordered().title("Stacklet details"))
        .render(area, buf);
}

fn calculate_row_widths(stacklets: &[Stacklet]) -> [Constraint; 4] {
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
        Constraint::Fill(100),
    ]
}

fn longest_string<'a>(strings: impl IntoIterator<Item = &'a String>) -> Option<usize> {
    strings.into_iter().map(|s| s.len()).max()
}
