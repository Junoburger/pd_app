use iced_audio::{text_marks, tick_marks, v_slider, LogDBRange};

use crate::Message as PsycheDailyMessage;

#[derive(Debug)]
pub struct ChannelFader {
    pub db_range: LogDBRange,
    pub channel_fader_db_state: v_slider::State,
    pub db_tick_marks: tick_marks::Group,
    pub db_text_marks: text_marks::Group,
    pub output_text: String,
}

impl ChannelFader {
    pub fn new() -> Self {
        let db_range = LogDBRange::default();
        Self {
            db_range,
            channel_fader_db_state: v_slider::State::new(
                db_range.default_normal_param(),
            ),

            db_tick_marks: vec![
                (db_range.map_to_normal(0.0), tick_marks::Tier::One),
                (db_range.map_to_normal(1.0), tick_marks::Tier::Two),
                (db_range.map_to_normal(3.0), tick_marks::Tier::Two),
                (db_range.map_to_normal(6.0), tick_marks::Tier::Two),
                (db_range.map_to_normal(12.0), tick_marks::Tier::Two),
                (db_range.map_to_normal(-1.0), tick_marks::Tier::Two),
                (db_range.map_to_normal(-3.0), tick_marks::Tier::Two),
                (db_range.map_to_normal(-6.0), tick_marks::Tier::Two),
                (db_range.map_to_normal(-12.0), tick_marks::Tier::Two),
            ]
            .into(),

            db_text_marks: text_marks::Group::min_max_and_center(
                "-12", "+12", "0",
            ),

            output_text: String::from("Channel [n]"),
        }
    }
}

pub fn info_text_db<ID: std::fmt::Debug>(id: ID, value: f32) -> String {
    format!("id: {:?}  |  value: {:.3} dB", id, value)
}
