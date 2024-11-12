use ratatui::prelude::*;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};
use tui_logger::{TuiLoggerLevelOutput, TuiLoggerSmartWidget, TuiWidgetState};

pub fn init_logging() {
    tracing_subscriber::registry()
        .with(tui_logger::tracing_subscriber_layer())
        .init();

    tui_logger::init_logger(log::LevelFilter::Trace).unwrap();
    tui_logger::set_default_level(log::LevelFilter::Info);
    tracing::error!("Ich war hier");
}

pub fn render_logging(area: Rect, state: &TuiWidgetState, buf: &mut Buffer) {
    TuiLoggerSmartWidget::default()
        .style_error(Style::default().fg(Color::Red))
        .style_debug(Style::default().fg(Color::Green))
        .style_warn(Style::default().fg(Color::Yellow))
        .style_trace(Style::default().fg(Color::Magenta))
        .style_info(Style::default().fg(Color::Cyan))
        .output_separator(':')
        .output_timestamp(Some("%H:%M:%S".to_string()))
        .output_level(Some(TuiLoggerLevelOutput::Abbreviated))
        .output_target(true)
        .output_file(false)
        .output_line(false)
        .state(state)
        .render(area, buf);
}
