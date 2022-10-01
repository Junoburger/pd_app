use iced::Renderer;
use iced_audio::{tick_marks, v_slider, LogDBRange, VSlider};

use crate::Message;

#[derive(Debug)]
pub struct ChannelFader {
    pub v_slider_state: v_slider::State,
    pub db_range: LogDBRange,
    pub center_tick_mark: tick_marks::Group,
}

impl ChannelFader {
    pub fn new() -> Self {
        let db_range = LogDBRange::new(-12.0, 12.0, 0.5.into());

        ChannelFader {
            v_slider_state: v_slider::State::new(
                db_range.default_normal_param(),
            ),
            db_range,
            center_tick_mark: tick_marks::Group::center(tick_marks::Tier::Two),
        }
    }
}
