use crate::Message as PsycheDailyMessage;

use iced::{button, pane_grid, Button, Element, Text};

#[derive(Debug)]
pub struct Pane {
    pub is_pinned: bool,
    pub pin_button: button::State,
    pub content: content::Content,
    pub controls: Controls,
}

impl Pane {
    pub fn new(id: usize) -> Self {
        Self {
            is_pinned: false,
            pin_button: button::State::new(),
            content: content::Content::new(id),
            controls: Controls::new(),
        }
    }
}

// enum PanesGridMessage {}
pub mod content {
    use iced_aw::graphics::icons::icon_to_char;

    use crate::{
        ui::components::audio_mixer::{
            level_meter::{self},
            test_canvas,
        },
        Message as PsycheDailyMessage,
    };

    #[derive(Debug)]
    pub struct Content {
        pub id: usize,
        pub scroll: iced::scrollable::State,
        pub split_horizontally: iced::button::State,
        pub split_vertically: iced::button::State,
        pub close: iced::button::State,
        pub pane_name: String,
        // TODO audio_bus: -> Shows the audio output signal in decibels,
        pub channel_fader:
            crate::ui::components::audio_mixer::channel_fader::ChannelFader,
        open_audio_io: iced::button::State,
    }

    impl Content {
        pub fn new(id: usize) -> Self {
            Content {
                id,
                scroll: iced::scrollable::State::new(),
                split_horizontally: iced::button::State::new(),
                split_vertically: iced::button::State::new(),
                close: iced::button::State::new(),
                pane_name: "Composition".to_string(),
                // AUDIO UI
                channel_fader: crate::ui::components::audio_mixer::channel_fader::ChannelFader::new(),
                open_audio_io: iced::button::State::new(),
            }
        }
        pub fn view(
            &mut self,
            pane: iced::pane_grid::Pane,
            total_panes: usize,
            is_pinned: bool,
            has_sample_creator_open: bool,
            pane_name: String,
        ) -> iced::Element<PsycheDailyMessage> {
            let Content {
                scroll,
                split_horizontally,
                // split_vertically,
                // close,
                // pane_name,
                ..
            } = self;

            let button = |state, label, message, style| {
                iced::Button::new(
                    state,
                    iced::Text::new(label)
                        .width(iced::Length::Fill)
                        .horizontal_alignment(
                            iced::alignment::Horizontal::Center,
                        )
                        .size(16),
                )
                .width(iced::Length::Fill)
                .padding(8)
                .on_press(message)
                .style(style)
            };

            let mut content = iced::Scrollable::new(scroll)
                .width(iced::Length::Fill)
                .height(iced::Length::Fill)
                .spacing(10);

            if self.id == 0 && !has_sample_creator_open {
                content = content.push(button(
                    split_horizontally,
                    "Create new sample",
                    PsycheDailyMessage::OpenCreateNewSamplePane(
                        iced::pane_grid::Axis::Horizontal,
                        pane,
                    ),
                    super::style::Button::Primary,
                ))
            };

            // pane with ID 1 is sample creator // TODO: find a better denomination to identify panes
            if self.id == 1 {
                //
                let crate::ui::components::audio_mixer::channel_fader::ChannelFader {
                    db_tick_marks,
                    db_text_marks,
                    channel_fader_db_state,
                    output_text,
                    ..
                } = &mut self.channel_fader;

                let v_slider_db = iced_audio::VSlider::new(
                    channel_fader_db_state,
                    PsycheDailyMessage::DB,
                )
                .tick_marks(db_tick_marks)
                .text_marks(db_text_marks);

                let level_meter_test_l = test_canvas::Rainbow::new();
                let level_meter_test_r = test_canvas::Rainbow::new();

                // push the widgets into rows
                let v_slider_row = iced::Row::new()
                    .spacing(20)
                    .max_height(200)
                    .push(
                        iced::Column::new()
                            .max_width(120)
                            .height(iced::Length::Fill)
                            // .spacing(10)
                            .push(v_slider_db),
                    )
                    .push(
                        iced::Row::new()
                            .max_width(120)
                            .height(iced::Length::Fill)
                            // .push(level_meter::level_meter(10., 200.))
                            .push(level_meter_test_l)
                            .spacing(10)
                            // .push(level_meter::level_meter(10., 200.)),
                            .push(level_meter_test_r),
                    );

                let channel_fader = iced::Column::new()
                    .spacing(20)
                    .padding(20)
                    .push(v_slider_row)
                    .push(iced::Text::new(output_text.to_string()).size(16));

                content = content.push(channel_fader);

                let open_audio_btn = iced::Button::new(
                    &mut self.open_audio_io,
                    iced::Text::new(icon_to_char(iced_aw::Icon::RecordCircle))
                        .font(iced_aw::ICON_FONT),
                )
                .on_press(PsycheDailyMessage::OpenAudioDefaultChannel);

                content = content.push(open_audio_btn);
            }

            iced::Container::new(content)
                .width(iced::Length::Fill)
                .height(iced::Length::Fill)
                .padding(5)
                .into()
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Controls {
    pub close: button::State,
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
