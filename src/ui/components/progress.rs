use std::time::Duration;

use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Style},
    widgets::{Block, Borders, Gauge, Widget},
    Frame,
};
use ratatui_image::{
    protocol::StatefulProtocol, FilterType, Resize, StatefulImage,
};
use tracing::info;

use crate::audio::progress::TrackProgress;

pub struct ProgressWidget {}

impl ProgressWidget {
    pub fn render(
        area: Rect,
        frame: &mut Frame,
        progress: &TrackProgress,
        protocol: &mut Option<&mut Box<dyn StatefulProtocol>>,
    ) {
        let chunks = Layout::default()
            .constraints([Constraint::Max(8), Constraint::Length(1)])
            .direction(Direction::Vertical)
            .split(area);

        if let Some(protocol) = protocol {
            info!("rendering image");
            info!("{:?}", chunks[0]);
            let image = StatefulImage::new(None)
                .resize(Resize::Fit(Some(FilterType::Nearest)));
            frame.render_stateful_widget(
                image,
                Rect {
                    x: 0,
                    y: chunks[0].y,
                    width: chunks[0].width,
                    height: chunks[0].height,
                },
                protocol,
            );
        }

        let (current, total) = progress.get_progress();
        let percent = if total.as_secs() > 0 {
            current.as_secs_f64() / total.as_secs_f64()
        } else {
            0.0
        };

        let gauge = Gauge::default()
            .block(Block::default().borders(Borders::NONE))
            .gauge_style(
                Style::default()
                    .fg(Color::from_u32(0x00f7d44b))
                    .bg(Color::from_u32(0x00464646)),
            )
            .ratio(percent.min(1.0))
            .label(format!(
                "{} / {}",
                format_duration(current),
                format_duration(total)
            ));

        gauge.render(chunks[1], frame.buffer_mut());
    }
}

fn format_duration(duration: Duration) -> String {
    let total_seconds = duration.as_secs();
    let minutes = total_seconds / 60;
    let seconds = total_seconds % 60;
    format!("{:02}:{:02}", minutes, seconds)
}
