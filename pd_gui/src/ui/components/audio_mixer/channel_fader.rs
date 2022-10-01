use iced::{Column, Element, Length, Renderer, Row, Text};
use iced_audio::{
    text_marks, tick_marks, v_slider, LogDBRange, Normal, VSlider,
};

use crate::{composition::Composition, PsycheDaily};

#[derive(Debug, Clone)]
pub enum ChannelFaderMessage {
    DB(Normal),
}

#[derive(Debug)]
pub struct ChannelFader {
    db_range: LogDBRange,
    channel_fader_db_state: v_slider::State,
    db_tick_marks: tick_marks::Group,
    db_text_marks: text_marks::Group,
    output_text: String,
}

impl Default for ChannelFader {
    fn default() -> Self {
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

            output_text: String::from("Move a widget"),
        }
    }
}

impl ChannelFader {
    pub fn update(&mut self, message: ChannelFaderMessage) {
        match message {
            ChannelFaderMessage::DB(normal) => {
                self.output_text =
                    self.db_range.unmap_to_value(normal).to_string()
            }
        }
    }

    pub fn view(&mut self, _debug: bool) -> Element<ChannelFaderMessage> {
        let v_slider_db = VSlider::new(
            &mut self.channel_fader_db_state,
            ChannelFaderMessage::DB,
        )
        .tick_marks(&self.db_tick_marks)
        .text_marks(&self.db_text_marks);

        // push the widgets into rows
        let v_slider_row = Row::new().spacing(20).max_height(400).push(
            Column::new()
                .max_width(120)
                .height(Length::Fill)
                .spacing(10)
                .push(Text::new("Log DB Range"))
                .push(v_slider_db),
        );

        let content = Column::new()
            .spacing(20)
            .padding(20)
            .push(v_slider_row)
            .push(Text::new(&self.output_text).size(16));

        Composition::container().push(content).into()
    }
}
