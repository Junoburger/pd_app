mod artist;
pub mod audio;
mod composition;
mod synth;
use artist::{get_artists, Artist};
use audio::output_test_runner;
use composition::Composition;
use synth::synth;

use iced::{
    alignment, button, keyboard, pane_grid, scrollable, window, Alignment,
    Application, Button, Color, Column, Command, Container, Element, Length,
    PaneGrid, Row, Scrollable, Settings, Space, Subscription, Text,
};
use iced_aw::{graphics::icons::icon_to_char, Icon, ICON_FONT};
use iced_native::{event, subscription, Event};

use iced_audio::{
    h_slider, knob, tick_marks, v_slider, xy_pad, FloatRange, FreqRange,
    HSlider, IntRange, Knob, LogDBRange, Normal, VSlider, XYPad,
};

// Test drawing a audio i/o meter with level control -- START
//

// Test drawing a audio i/o meter with level control -- END
// static ICON: &[u8] = include_bytes!("../resources/sqr.png");
// const ICON_HEIGHT: u32 = 250;
// const ICON_WIDTH: u32 = 250;

fn main() -> iced::Result {
    // let image = image::load_from_memory(ICON).unwrap();
    // let icon = iced::window::Icon::from_rgba(
    //     image.as_bytes().to_vec(),
    //     ICON_HEIGHT,
    //     ICON_WIDTH,
    // );

    let settings = Settings {
        window: window::Settings {
            // icon: Some(icon.unwrap()),
            min_size: Some((400, 200)),
            ..window::Settings::default()
        },
        ..Settings::default()
    };

    PsycheDaily::run(settings)
}

#[derive(Debug, Clone, Copy)]
enum Message {
    CreateCompositionPressed,
    Split(pane_grid::Axis, pane_grid::Pane),
    SplitFocused(pane_grid::Axis),
    FocusAdjacent(pane_grid::Direction),
    Clicked(pane_grid::Pane),
    Dragged(pane_grid::DragEvent),
    Resized(pane_grid::ResizeEvent),
    TogglePin(pane_grid::Pane),
    Close(pane_grid::Pane),
    CloseFocused,
    // AUDIO UI
    HSliderInt(Normal),
    VSliderDB(Normal),
    KnobFreq(Normal),
    XYPadFloat(Normal, Normal),

    // AUDIO BACKEND
    OpenAudioDefaultChannel,
}

struct PsycheDaily {
    compositions: Vec<Composition>,
    artists: Vec<Artist>,
    is_composition_mode: bool,
    create_composition_button: button::State,
    panes: pane_grid::State<Pane>,
    panes_created: usize,
    focus: Option<pane_grid::Pane>,
    // The ranges handle converting the input/output of a parameter to and from
    // a usable value.
    //
    // There are 4 built-in options available for a range:
    //
    // * FloatRange - a linear range of f32 values
    // * IntRange - a discrete range of i32 values. This will cause the widget
    // to "step" when moved.
    // * LogDBRange - a logarithmic range of decibel values. Values around 0 dB
    // will increment slower than values farther away from 0 dB.
    // * FreqRange - a logarithmic range of frequency values. Each octave in
    // the 10 octave spectrum (from 20 Hz to 20480 Hz) is spaced evenly.
    //
    float_range: FloatRange,
    int_range: IntRange,
    db_range: LogDBRange,
    freq_range: FreqRange,

    // The states of the widgets that will control the parameters.
    h_slider_state: h_slider::State,
    v_slider_state: v_slider::State,
    knob_state: knob::State,
    xy_pad_state: xy_pad::State,

    // A group of tick marks with their size and position.
    center_tick_mark: tick_marks::Group,

    output_text: String,

    // Open a audio I/O stream for a default channel
    open_audio_io: button::State,
}

impl iced::Application for PsycheDaily {
    type Message = Message;
    type Executor = iced::executor::Default;
    type Flags = ();

    fn new(_flags: ()) -> (Self, Command<Message>) {
        // Initalize each range:
        let float_range = FloatRange::default_bipolar();
        let int_range = IntRange::new(0, 10);
        let db_range = LogDBRange::new(-12.0, 12.0, 0.5.into());
        let freq_range = FreqRange::default();

        let (panes, _) = pane_grid::State::new(Pane::new(0));
        (
            Self {
                compositions: vec![],
                artists: vec![],
                is_composition_mode: false,
                create_composition_button: button::State::new(),
                panes,
                panes_created: 1,
                focus: None,
                // AUDIO UI
                // Add the ranges.
                float_range,
                int_range,
                db_range,
                freq_range,

                // Initialize the state of the widgets with a normalized parameter
                // that has a value and a default value.
                h_slider_state: h_slider::State::new(
                    int_range.normal_param(5, 5),
                ),
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
                center_tick_mark: tick_marks::Group::center(
                    tick_marks::Tier::Two,
                ),

                output_text: "Move a widget!".into(),
                open_audio_io: button::State::new(),
            },
            Command::none(),
        )
    }

    fn title(&self) -> String {
        String::from("Psyche Daily")
    }
    fn update(&mut self, message: Message) -> Command<Message> {
        match message {
            Message::CreateCompositionPressed => {
                self.is_composition_mode = true
            }
            Message::Split(axis, pane) => {
                let result = self.panes.split(
                    axis,
                    &pane,
                    Pane::new(self.panes_created),
                );

                if let Some((pane, _)) = result {
                    self.focus = Some(pane);
                }

                self.panes_created += 1;
            }
            Message::SplitFocused(axis) => {
                if let Some(pane) = self.focus {
                    let result = self.panes.split(
                        axis,
                        &pane,
                        Pane::new(self.panes_created),
                    );

                    if let Some((pane, _)) = result {
                        self.focus = Some(pane);
                    }

                    self.panes_created += 1;
                }
            }
            Message::FocusAdjacent(direction) => {
                if let Some(pane) = self.focus {
                    if let Some(adjacent) =
                        self.panes.adjacent(&pane, direction)
                    {
                        self.focus = Some(adjacent);
                    }
                }
            }
            Message::Clicked(pane) => {
                self.focus = Some(pane);
            }
            Message::Resized(pane_grid::ResizeEvent { split, ratio }) => {
                self.panes.resize(&split, ratio);
            }
            Message::Dragged(pane_grid::DragEvent::Dropped {
                pane,
                target,
            }) => {
                self.panes.swap(&pane, &target);
            }
            Message::Dragged(_) => {}
            Message::TogglePin(pane) => {
                if let Some(Pane { is_pinned, .. }) = self.panes.get_mut(&pane)
                {
                    *is_pinned = !*is_pinned;
                }
            }
            Message::Close(pane) => {
                if let Some((_, sibling)) = self.panes.close(&pane) {
                    self.focus = Some(sibling);
                }
            }
            Message::CloseFocused => {
                if let Some(pane) = self.focus {
                    if let Some(Pane { is_pinned, .. }) = self.panes.get(&pane)
                    {
                        if !is_pinned {
                            if let Some((_, sibling)) = self.panes.close(&pane)
                            {
                                self.focus = Some(sibling);
                            }
                        }
                    }
                }
            }
            // AUDIO UI
            // Retrieve the value by mapping the normalized value of the parameter
            // to the corresponding range.
            //
            // Now do something useful with that value!
            Message::HSliderInt(normal) => {
                // Integer parameters must be snapped to make the widget "step" when moved.
                self.h_slider_state.snap_visible_to(&self.int_range);

                let value = self.int_range.unmap_to_value(normal);
                self.output_text = format!("HSliderInt: {}", value);
            }
            Message::VSliderDB(normal) => {
                let value = self.db_range.unmap_to_value(normal);
                self.output_text = format!("VSliderDB: {:.3}", value);
            }
            Message::KnobFreq(normal) => {
                let value = self.freq_range.unmap_to_value(normal);
                self.output_text = format!("KnobFreq: {:.2}", value);
            }
            Message::XYPadFloat(normal_x, normal_y) => {
                let value_x = self.float_range.unmap_to_value(normal_x);
                let value_y = self.float_range.unmap_to_value(normal_y);
                self.output_text =
                    format!("XYPadFloat: x: {:.2}, y: {:.2}", value_x, value_y);
            }
            Message::OpenAudioDefaultChannel => {
                output_test_runner();
                // synth();
            }
        }

        Command::none()
    }

    fn subscription(&self) -> Subscription<Message> {
        subscription::events_with(|event, status| {
            if let event::Status::Captured = status {
                return None;
            }

            match event {
                Event::Keyboard(keyboard::Event::KeyPressed {
                    modifiers,
                    key_code,
                }) if modifiers.command() => handle_hotkey(key_code),
                _ => None,
            }
        })
    }

    fn view(&mut self) -> Element<Message> {
        // Create each parameter widget, passing in the current state of the widget.
        // let h_slider_widget =
        //     HSlider::new(&mut self.h_slider_state, Message::HSliderInt)
        //         // Add the tick mark group to this widget.
        //         .tick_marks(&self.center_tick_mark);

        // let v_slider_widget =
        //     VSlider::new(&mut self.v_slider_state, Message::VSliderDB)
        //         .tick_marks(&self.center_tick_mark);

        // let knob_widget = Knob::new(
        //     &mut self.knob_state,
        //     Message::KnobFreq,
        //     || None,
        //     || None,
        // );

        // let xy_pad_widget =
        //     XYPad::new(&mut self.xy_pad_state, Message::XYPadFloat);

        // //
        let content: Element<_> = Column::new()
            .max_width(300)
            .max_height(500)
            .spacing(20)
            .padding(20)
            .align_items(Alignment::Center)
            // .push(h_slider_widget)
            // .push(v_slider_widget)
            // .push(knob_widget)
            // .push(xy_pad_widget)
            .push(
                Container::new(Text::new(&self.output_text))
                    .width(Length::Fill),
            )
            .into();

        let focus = self.focus;
        let total_panes = self.panes.len();

        let pane_grid = PaneGrid::new(&mut self.panes, |id, pane| {
            // let is_focused = focus == Some(id);

            // let text = if pane.is_pinned { "Unpin" } else { "Pin" };
            // let pin_button =
            //     Button::new(&mut pane.pin_button, Text::new(text).size(14))
            //         .on_press(Message::TogglePin(id))
            //         .style(style::Button::Pin)
            //         .padding(3);
            // let title = Row::with_children(vec![
            //     // pin_button.into(),
            //     // Text::new("Pane").into(), <<-- should probably showcontent title [e.g composition-name]
            //     Text::new(pane.content.id.to_string())
            //         .color(if is_focused {
            //             PANE_ID_COLOR_FOCUSED
            //         } else {
            //             PANE_ID_COLOR_UNFOCUSED
            //         })
            //         .into(),
            // ])
            // .spacing(5)

            // let title_bar = pane_grid::TitleBar::new(title)
            //     .controls(pane.controls.view(id, total_panes, pane.is_pinned))
            //     .padding(1)
            //     .style(style::TitleBar { is_focused });

            pane_grid::Content::new(pane.content.view(
                id,
                total_panes,
                pane.is_pinned,
            ))
            // .title_bar(title_bar) // <<-- // TODO: Title bar should probably be something like tabs with project-name
            // .style(style::Pane { is_focused })
        })
        .width(Length::Fill)
        .height(Length::Fill)
        .spacing(10)
        .on_click(Message::Clicked)
        .on_drag(Message::Dragged)
        .on_resize(10, Message::Resized);

        let mut wrapper = Row::new().height(Length::Fill).width(Length::Fill);

        let column_1: Column<Message> = Column::new()
            .height(Length::Fill)
            .padding(10)
            // .push(Space::new(Length::Units(50), Length::Units(0)))
            .push(
                Button::new(
                    &mut self.create_composition_button,
                    Text::new(format!(
                        "Composition {}",
                        icon_to_char(Icon::ClipboardPlus),
                    ))
                    .font(ICON_FONT),
                )
                .on_press(Message::CreateCompositionPressed)
                .style(style::Button::Primary),
            )
            .push(Space::new(Length::Units(0), Length::Units(10)))
            .push(
                Button::new(
                    &mut self.open_audio_io,
                    Text::new("Open audio channel"),
                )
                .on_press(Message::OpenAudioDefaultChannel)
                .style(style::Button::Pin),
            );

        let mut column_2: Column<Message> = Column::new().height(Length::Fill);
        // .push(iced::widget::Space::with_height(Length::Fill))
        if self.is_composition_mode == true {
            column_2 = column_2.push(content);
        }

        wrapper = wrapper.push(column_1).push(column_2);

        Container::new(wrapper)
            .width(Length::Fill)
            .height(Length::Fill)
            .padding(0)
            .into()
    }
}

const PANE_ID_COLOR_UNFOCUSED: Color = Color::from_rgb(
    0xFF as f32 / 255.0,
    0xC7 as f32 / 255.0,
    0xC7 as f32 / 255.0,
);
const PANE_ID_COLOR_FOCUSED: Color = Color::from_rgb(
    0xFF as f32 / 255.0,
    0x47 as f32 / 255.0,
    0x47 as f32 / 255.0,
);

fn handle_hotkey(key_code: keyboard::KeyCode) -> Option<Message> {
    use keyboard::KeyCode;
    use pane_grid::{Axis, Direction};

    let direction = match key_code {
        KeyCode::Up => Some(Direction::Up),
        KeyCode::Down => Some(Direction::Down),
        KeyCode::Left => Some(Direction::Left),
        KeyCode::Right => Some(Direction::Right),
        _ => None,
    };

    match key_code {
        KeyCode::V => Some(Message::SplitFocused(Axis::Vertical)),
        KeyCode::H => Some(Message::SplitFocused(Axis::Horizontal)),
        KeyCode::W => Some(Message::CloseFocused),
        _ => direction.map(Message::FocusAdjacent),
    }
}

#[derive(Debug)]
struct Pane {
    pub is_pinned: bool,
    pub pin_button: button::State,
    pub content: Content,
    pub controls: Controls,
}

#[derive(Debug)]
struct Content {
    id: usize,
    scroll: scrollable::State,
    split_horizontally: button::State,
    split_vertically: button::State,
    close: button::State,
}

#[derive(Debug)]
struct Controls {
    close: button::State,
}

impl Pane {
    fn new(id: usize) -> Self {
        Self {
            is_pinned: false,
            pin_button: button::State::new(),
            content: Content::new(id),
            controls: Controls::new(),
        }
    }
}

impl Content {
    fn new(id: usize) -> Self {
        Content {
            id,
            scroll: scrollable::State::new(),
            split_horizontally: button::State::new(),
            split_vertically: button::State::new(),
            close: button::State::new(),
        }
    }
    fn view(
        &mut self,
        pane: pane_grid::Pane,
        total_panes: usize,
        is_pinned: bool,
    ) -> Element<Message> {
        let Content {
            scroll,
            split_horizontally,
            split_vertically,
            close,
            ..
        } = self;

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

        // let slider =
        //     iced_audio::Knob::new(&mut self.audio_knob, test(), test(), test());

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
            controls = controls.push(button(
                close,
                "Close",
                Message::Close(pane),
                style::Button::Destructive,
            ));
        }

        let content = Scrollable::new(scroll)
            .width(Length::Fill)
            .spacing(10)
            .align_items(iced::Alignment::Start)
            .push(controls);

        // TODO: draw a rectangle for the base of displaying an audio signal [!wave_form]

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
    ) -> Element<Message> {
        let mut button =
            Button::new(&mut self.close, Text::new("Close").size(14))
                .style(style::Button::Control)
                .padding(3);
        if total_panes > 1 && !is_pinned {
            button = button.on_press(Message::Close(pane));
        }
        button.into()
    }
}

mod style {
    use crate::PANE_ID_COLOR_FOCUSED;
    use iced::{button, container, Background, Color, Vector};

    const SURFACE: Color = Color::from_rgb(
        0xF2 as f32 / 255.0,
        0xF3 as f32 / 255.0,
        0xF5 as f32 / 255.0,
    );

    const ACTIVE: Color = Color::from_rgb(
        0x72 as f32 / 255.0,
        0x89 as f32 / 255.0,
        0xDA as f32 / 255.0,
    );

    const HOVERED: Color = Color::from_rgb(
        0x67 as f32 / 255.0,
        0x7B as f32 / 255.0,
        0xC4 as f32 / 255.0,
    );

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
                    Some(Color::from_rgba(0., 0., 0., 0.5))
                } else {
                    Some(Color::from_rgba(0., 0., 0., 0.9))
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
                border_width: 2.0,
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
