use crate::Message as PsycheDailyMessage;
use iced::{
    alignment, button, pane_grid, scrollable, Button, Color, Column, Container,
    Element, Length, Scrollable, Text,
};
use iced_audio::{
    h_slider, knob, tick_marks, v_slider, xy_pad, FloatRange, FreqRange,
    IntRange, LogDBRange, Normal, VSlider,
};

use super::audio_mixer::channel_fader::ChannelFader;

#[derive(Debug)]
pub struct Pane {
    pub is_pinned: bool,
    pub pin_button: button::State,
    pub content: Content,
    pub controls: Controls,
}

impl Pane {
    pub fn new(id: usize) -> Self {
        Self {
            is_pinned: false,
            pin_button: button::State::new(),
            content: Content::new(id),
            controls: Controls::new(),
        }
    }
}

#[derive(Debug)]
pub struct Content {
    pub id: usize,
    pub scroll: scrollable::State,
    pub split_horizontally: button::State,
    pub split_vertically: button::State,
    pub close: button::State,
    pub pane_name: String,
    // TODO audio_bus: -> Shows the audio output signal in decibels,
    pub float_range: FloatRange,
    pub int_range: IntRange,
    pub db_range: LogDBRange,
    pub freq_range: FreqRange,
    pub h_slider_state: h_slider::State,
    pub v_slider_state: v_slider::State,
    pub knob_state: knob::State,
    pub xy_pad_state: xy_pad::State,
    pub center_tick_mark: tick_marks::Group,
    pub channel_fader: ChannelFader,
}

#[derive(Debug, Clone, Copy)]
pub struct Controls {
    pub close: button::State,
}

impl Content {
    pub fn new(id: usize) -> Self {
        // Initalize each range:
        let float_range = FloatRange::default_bipolar();
        let int_range = IntRange::new(0, 10);
        let db_range = LogDBRange::new(-12.0, 12.0, 0.5.into());
        let freq_range = FreqRange::default();

        Content {
            id,
            scroll: scrollable::State::new(),
            split_horizontally: button::State::new(),
            split_vertically: button::State::new(),
            close: button::State::new(),
            pane_name: "Composition".to_string(),
            // AUDIO UI
            // Add the ranges.
            float_range,
            int_range,
            db_range,
            freq_range,

            // Initialize the state of the widgets with a normalized parameter
            // that has a value and a default value.
            h_slider_state: h_slider::State::new(int_range.normal_param(5, 5)),
            v_slider_state: v_slider::State::new(
                db_range.default_normal_param(),
            ),
            knob_state: knob::State::new(
                freq_range.normal_param(1000.0, 1000.0),
            ),
            xy_pad_state: xy_pad::State::new(
                float_range.default_normal_param(),
                float_range.default_normal_param(),
            ),

            // Add a tick mark at the center position with the tier 2 size
            center_tick_mark: tick_marks::Group::center(tick_marks::Tier::Two),
            channel_fader: ChannelFader::new(),
        }
    }
    pub fn view(
        &mut self,
        pane: pane_grid::Pane,
        total_panes: usize,
        is_pinned: bool,
        has_sample_creator_open: bool,
        pane_name: String,
    ) -> Element<PsycheDailyMessage> {
        let Content {
            scroll,
            split_horizontally,
            split_vertically,
            close,
            pane_name,
            h_slider_state,
            ..
        } = self;

        // Create each parameter widget, passing in the current state of the widget.
        // let h_slider_widget =
        //     HSlider::new(&mut self.h_slider_state, Message::HSliderInt)
        //         // Add the tick mark group to this widget.
        //         .tick_marks(&self.center_tick_mark);

        let v_slider_widget = VSlider::new(
            &mut self.channel_fader.v_slider_state,
            PsycheDailyMessage::VSliderDB,
        )
        .tick_marks(&self.channel_fader.center_tick_mark)
        .height(Length::Units(150));

        // let knob_widget = Knob::new(
        //     &mut self.knob_state,
        //     Message::KnobFreq,
        //     || None,
        //     || None,
        // );

        // let xy_pad_widget =
        //     XYPad::new(&mut self.xy_pad_state, Message::XYPadFloat);

        let button = |state, label, message, style| {
            Button::new(
                state,
                Text::new(label)
                    .width(Length::Fill)
                    .horizontal_alignment(alignment::Horizontal::Center)
                    .size(16),
            )
            .width(Length::Fill)
            .padding(8)
            .on_press(message)
            .style(style)
        };

        let mut controls = Column::new()
            .spacing(5)
            .max_width(150)
            // .push(button(
            //     split_horizontally,
            //     "Split horizontally",
            //     Message::Split(pane_grid::Axis::Horizontal, pane),
            //     style::Button::Primary,
            // ))
            // .push(button(
            //     split_vertically,
            //     "Composition +",
            //     Message::Split(pane_grid::Axis::Vertical, pane),
            //     style::Button::Primary,
            // ))
            ;

        if total_panes > 1 && !is_pinned {
            // controls = controls.push(button(
            //     close,
            //     "Close",
            //     Message::Close(pane),
            //     style::Button::Destructive,
            // ));
        }

        let mut content = Scrollable::new(scroll)
            .width(Length::Fill)
            .height(Length::Fill)
            .spacing(10)
            // .align_items(iced::Alignment::Start)
            .push(controls);

        if self.id == 0 && !has_sample_creator_open {
            content = content.push(button(
                split_horizontally,
                "Create new sample",
                PsycheDailyMessage::OpenCreateNewSamplePane(
                    pane_grid::Axis::Horizontal,
                    pane,
                ),
                style::Button::Primary,
            ))
        }

        if self.id == 1 {
            content = content.push(v_slider_widget)
        }

        Container::new(content)
            .width(Length::Fill)
            .height(Length::Fill)
            .padding(5)
            // .center_y()
            .into()
    }
}

impl Controls {
    fn new() -> Self {
        Self {
            close: button::State::new(),
        }
    }

    pub fn view(
        &mut self,
        pane: pane_grid::Pane,
        total_panes: usize,
        is_pinned: bool,
    ) -> Element<PsycheDailyMessage> {
        let mut button =
            Button::new(&mut self.close, Text::new("Close").size(14))
                .style(style::Button::Control)
                .padding(3);
        if total_panes > 1 && !is_pinned {
            button = button.on_press(PsycheDailyMessage::Close(pane));
        }
        button.into()
    }
}

pub mod style {
    use crate::ui::colors::{ACTIVE, HOVERED, PANE_ID_COLOR_FOCUSED, SURFACE};
    use iced::{button, container, Background, Color, Vector};

    pub struct PsycheDaily;

    impl container::StyleSheet for PsycheDaily {
        fn style(&self) -> container::Style {
            container::Style {
                ..Default::default()
            }
        }
    }

    pub struct TitleBar {
        pub is_focused: bool,
    }

    impl container::StyleSheet for TitleBar {
        fn style(&self) -> container::Style {
            let pane = Pane {
                is_focused: self.is_focused,
            }
            .style();

            container::Style {
                text_color: if self.is_focused {
                    Some(Color::from_rgba(120., 120., 120., 0.5))
                } else {
                    Some(Color::from_rgba(255., 255., 100., 0.9))
                },
                background: Some(pane.border_color.into()),
                ..Default::default()
            }
        }
    }

    pub struct Pane {
        pub is_focused: bool,
    }

    impl container::StyleSheet for Pane {
        fn style(&self) -> container::Style {
            container::Style {
                background: Some(Background::Color(SURFACE)),
                border_width: 1.0,
                border_color: if self.is_focused {
                    Color::from_rgba(0.5, 0.5, 0.5, 1.)
                } else {
                    Color::from_rgba(0.5, 0.5, 0.5, 0.)
                },
                ..Default::default()
            }
        }
    }

    pub enum Button {
        Primary,
        Destructive,
        Control,
        Pin,
    }

    impl button::StyleSheet for Button {
        fn active(&self) -> button::Style {
            let (background, text_color) = match self {
                Button::Primary => (Some(ACTIVE), Color::WHITE),
                Button::Destructive => {
                    (None, Color::from_rgb8(0xFF, 0x47, 0x47))
                }
                Button::Control => (Some(PANE_ID_COLOR_FOCUSED), Color::WHITE),
                Button::Pin => (Some(ACTIVE), Color::WHITE),
            };

            button::Style {
                text_color,
                background: background.map(Background::Color),
                border_radius: 5.0,
                shadow_offset: Vector::new(0.0, 0.0),
                ..button::Style::default()
            }
        }

        fn hovered(&self) -> button::Style {
            let active = self.active();

            let background = match self {
                Button::Primary => Some(HOVERED),
                Button::Destructive => Some(Color {
                    a: 0.2,
                    ..active.text_color
                }),
                Button::Control => Some(PANE_ID_COLOR_FOCUSED),
                Button::Pin => Some(HOVERED),
            };

            button::Style {
                background: background.map(Background::Color),
                ..active
            }
        }
    }
}
