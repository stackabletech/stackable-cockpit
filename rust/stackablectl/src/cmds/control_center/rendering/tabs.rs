use ratatui::{
    prelude::*,
    style::palette::tailwind,
    widgets::{Block, Padding, Tabs},
};
use strum::{Display, EnumCount, EnumIter, FromRepr, IntoEnumIterator};

use crate::cmds::control_center::{logging::render_logs, state::Model};

use super::stacklets::render_stacklets;

#[derive(Default, Clone, Copy, Display, FromRepr, EnumIter, EnumCount)]
pub enum SelectedTab {
    #[default]
    #[strum(to_string = "Stacklets")]
    Stacklets,
    #[strum(to_string = "Logs")]
    Logs,
}

impl SelectedTab {
    pub fn render_header(&self, model: &Model, area: Rect, buf: &mut Buffer) {
        let titles = SelectedTab::iter().map(SelectedTab::title);
        let highlight_style = (Color::default(), model.selected_tab.palette().c700);
        let selected_tab_index = model.selected_tab as usize;
        Tabs::new(titles)
            .highlight_style(highlight_style)
            .select(selected_tab_index)
            .padding("", "")
            .divider(" ")
            .render(area, buf);
    }

    pub fn render_body(&self, model: &mut Model, area: Rect, buf: &mut Buffer) {
        let tab_body_block = self.block();

        match self {
            SelectedTab::Stacklets => {
                render_stacklets(model, tab_body_block.inner(area), buf);
            }
            SelectedTab::Logs => {
                render_logs(model, area, buf);
            }
        }
        tab_body_block.render(area, buf);
    }

    const fn palette(&self) -> tailwind::Palette {
        match self {
            Self::Stacklets => tailwind::BLUE,
            Self::Logs => tailwind::EMERALD,
            // Self::Tab3 => tailwind::INDIGO,
            // Self::Tab4 => tailwind::RED,
        }
    }

    /// A block surrounding the tab's content
    fn block(self) -> Block<'static> {
        Block::bordered()
            .border_set(symbols::border::PROPORTIONAL_TALL)
            .padding(Padding::horizontal(1))
            .border_style(self.palette().c700)
    }

    /// Return tab's name as a styled `Line`
    fn title(self) -> Line<'static> {
        format!("  {self}  ")
            .fg(tailwind::SLATE.c200)
            .bg(self.palette().c900)
            .into()
    }

    pub fn previous(self) -> Self {
        let current = self as usize;

        if current == 0 {
            Self::from_repr(Self::COUNT - 1).expect("Tab enum variant must be known")
        } else {
            Self::from_repr(current - 1).expect("Tab enum variant must be known")
        }
    }

    pub fn next(self) -> Self {
        let current = self as usize;
        Self::from_repr(current.saturating_add(1) % Self::COUNT)
            .expect("Tab enum variant must be known")
    }
}
