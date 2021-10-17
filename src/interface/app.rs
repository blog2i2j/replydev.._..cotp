use std::error;

use tui::backend::Backend;
use tui::layout::{Constraint, Direction, Layout};
use tui::style::{Color, Modifier, Style};
use tui::terminal::Frame;
use tui::widgets::{Block, Borders, Cell, Gauge, Row, Table};

use crate::otp::otp_element::OTPElement;
use crate::otp::otp_helper::get_otp_code;
use crate::interface::table::StatefulTable;
use crate::utils::percentage;

/// Application result type.
pub type AppResult<T> = std::result::Result<T, Box<dyn error::Error>>;

/// Application.
pub struct App {
    /// Is the application running?
    pub running: bool,
    title: String,
    pub(crate) table: StatefulTable,
    elements: Vec<OTPElement>,
    progress: u16,
}

impl App {
    /// Constructs a new instance of [`App`].
    pub fn new(elements: Vec<OTPElement>) -> Self {
        let mut title = String::from(env!("CARGO_PKG_NAME"));
        title.push_str(" v");
        title.push_str(env!("CARGO_PKG_VERSION"));
        Self { running: true, title, table: StatefulTable::new(&elements), elements, progress: percentage() }
    }

    /// Handles the tick event of the terminal.
    pub fn tick(&mut self) {
        // Update progress bar
        let new_progress = percentage();
        // Check for new cycle
        if new_progress < self.progress {
            // Update codes
            self.table.items.clear();
            let mut i = 0;
            for element in &self.elements {
                // TODO Remove unwrap and check exception
                self.table.items.push(vec![(i + 1).to_string(), element.label(), element.issuer(), get_otp_code(element).unwrap()]);
                i += 1;
            }
        }
        self.progress = new_progress;
    }

    /// Renders the user interface widgets.
    pub fn render<B: Backend>(&mut self, frame: &mut Frame<'_, B>) {
        let rects = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Percentage(95), Constraint::Percentage(5)].as_ref())
            .margin(2)
            .split(frame.size());

        let selected_style = Style::default().add_modifier(Modifier::REVERSED);
        let normal_style = Style::default().bg(Color::DarkGray);
        let header_cells = ["Id", "Label", "Issuer", "OTP"]
            .iter()
            .map(|h| Cell::from(*h).style(Style::default().fg(Color::White)));
        let header = Row::new(header_cells)
            .style(normal_style)
            .height(1)
            .bottom_margin(1);
        let rows = self.table.items.iter().map(|item| {
            let height = item
                .iter()
                .map(|content| content.chars().filter(|c| *c == '\n').count())
                .max()
                .unwrap_or(0)
                + 1;
            let cells = item.iter().map(|c| Cell::from(c.as_str()));
            Row::new(cells).height(height as u16).bottom_margin(1)
        });
        let t = Table::new(rows)
            .header(header)
            .block(Block::default().borders(Borders::TOP | Borders::BOTTOM).title(self.title.as_str()))
            .highlight_style(selected_style)
            .highlight_symbol("-> ")
            .widths(&[
                Constraint::Percentage(5),
                Constraint::Percentage(35),
                Constraint::Percentage(35),
                Constraint::Percentage(25),
            ]);

        let progress_label = format!("{}%", self.progress);
        let progress_bar = Gauge::default()
            .block(Block::default())
            .gauge_style(
                Style::default()
                    .fg(Color::DarkGray)
                    .add_modifier(Modifier::BOLD),
            )
            .percent(self.progress as u16)
            .label(progress_label);

        frame.render_stateful_widget(t, rects[0], &mut self.table.state);
        frame.render_widget(progress_bar, rects[1]);
    }
}