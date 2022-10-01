mod artist;
pub mod audio;
mod composition;
mod synth;
use std::{
    collections::HashMap,
    sync::{
        atomic::{AtomicBool, Ordering},
        mpsc, Arc, Condvar, Mutex,
    },
    thread,
};

use artist::{get_artists, Artist};
use audio::output_test_runner;
use composition::Composition;
// use synth::synth;
mod ui;

use cpal::traits::StreamTrait;
use iced::{
    button, keyboard,
    pane_grid::{self, Split},
    window::{self, Icon},
    Application, Button, Color, Column, Command, Container, Element, Length,
    PaneGrid, Row, Settings, Space, Subscription, Text,
};
use iced_audio::Normal;
use iced_aw::graphics::icons::icon_to_char;
// use iced_aw::{graphics::icons::icon_to_char, Icon, ICON_FONT};
use iced_native::{event, subscription, Event};

use ui::{
    colors::{PANE_ID_COLOR_FOCUSED, PANE_ID_COLOR_UNFOCUSED},
    components::{
        audio_mixer::level_meter,
        panes::{style, Pane},
    },
};

static ICON: &[u8] = include_bytes!("../resources/sqr.png");
const ICON_HEIGHT: u32 = 250;
const ICON_WIDTH: u32 = 250;

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
            min_size: Some((600, 400)),
            transparent: true,
            ..window::Settings::default()
        },
        ..Settings::default()
    };

    PsycheDaily::run(settings)
}

struct PsycheDaily {
    panes: pane_grid::State<Pane>,
    panes_created: usize,
    focus: Option<pane_grid::Pane>,
    // Open a audio I/O stream for a default channel
    start_new_composition: button::State,

    has_sample_creator_open: bool,
    is_composition_mode: bool,
    toggle_sidepanel: button::State,
    pane_names: HashMap<String, pane_grid::Pane>,
    switch_on: bool,
}

#[derive(Debug, Clone, Copy)]
pub enum Message {
    // Pane Grid related
    Split(pane_grid::Axis, pane_grid::Pane),
    SplitFocused(pane_grid::Axis),
    FocusAdjacent(pane_grid::Direction),
    Clicked(pane_grid::Pane),
    Dragged(pane_grid::DragEvent),
    Resized(pane_grid::ResizeEvent),
    TogglePin(pane_grid::Pane),
    Close(pane_grid::Pane),
    CloseFocused,

    ////
    CreateCompositionPressed,
    OpenCreateNewSamplePane(pane_grid::Axis, pane_grid::Pane),
    // AUDIO BACKEND
    OpenAudioDefaultChannel,

    // ------
    TestToggle,
    DB(Normal),
    ////
}

impl iced::Application for PsycheDaily {
    type Message = Message;
    type Executor = iced::executor::Default;
    type Flags = ();

    fn new(_flags: ()) -> (Self, Command<Message>) {
        let (panes, _) = pane_grid::State::new(Pane::new(0));
        (
            Self {
                is_composition_mode: false,
                start_new_composition: button::State::new(),
                panes,
                panes_created: 1, // FIXME should be 0 on init
                focus: None,
                has_sample_creator_open: false,
                toggle_sidepanel: button::State::new(),
                pane_names: HashMap::new(),
                switch_on: false,
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
                // TODO: Figure out a way to have a pane align along the opposite axis with an on-click

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
            Message::OpenCreateNewSamplePane(axis, pane) => {
                let result = self.panes.split(
                    axis,
                    &pane,
                    Pane::new(self.panes_created),
                );

                if let Some((pane, _)) = result {
                    self.focus = Some(pane);
                }

                self.panes_created += 1; // TODO: keep track of named panes in a vector/hashmap
                self.has_sample_creator_open = !self.has_sample_creator_open;
            }
            // AUDIO BE
            Message::OpenAudioDefaultChannel => {
                // let (tx, rx) = mpsc::channel();
                self.switch_on = !self.switch_on;
                // let x = Arc::new(AtomicBool::new(self.switch_on));
                println!("BOOL {}", self.switch_on);

                let pair = Arc::new((Mutex::new(false), Condvar::new()));
                let pair2 = pair.clone();

                // let (tx, rx) = mpsc::channel();

                if self.switch_on {
                    thread::spawn(move || {
                        output_test_runner();
                        // match rx.try_recv() {
                        //     Ok(_) | Err(mpsc::TryRecvError::Disconnected) => {
                        //         println!("Terminating.");
                        //     }
                        //     Err(mpsc::TryRecvError::Empty) => {}
                        // }
                    })
                    .join()
                    .unwrap()
                };

                // let _ = tx.send(());

                // synth();
            }
            Message::TestToggle => {
                println!("Toggle");
            }
            Message::DB(normal) => {
                //  self.output_text =
                // info_text_db("VSliderDB", self.db_range.unmap_to_value(normal));

                // TODO: Update output text in sample_creator pane
            }
        }

        Command::none()
    }

    fn view(&mut self) -> Element<Message> {
        //         column![
        //     text("Bezier tool example").width(Length::Shrink).size(50),
        //     self.bezier.view(&self.curves).map(Message::AddCurve),
        //     button("Clear").padding(8).on_press(Message::Clear),
        // ]

        // TODO: On press -> should open a new panel (for now -> might become a modal)
        //     .push(Text::new("Show composition swim lanes"))
        let focus = self.focus;
        let total_panes = self.panes.len();

        let mut pane_grid = PaneGrid::new(&mut self.panes, |id, pane| {
            let is_focused = focus == Some(id);
            let has_sample_creator_open = self.has_sample_creator_open;

            let mut pane_name = format!("{}", pane.content.pane_name);

            if has_sample_creator_open == true && pane.content.id == 1 {
                pane_name = "Sample creator".to_string();
            }

            // let x = pane_grid::Node::splits(pane_grid).map(|split| split);

            let title = Row::with_children(vec![
                // Text::new("Pane").into(), <<-- should probably showcontent title [e.g composition-name]
                Text::new(&pane_name)
                    .color(if is_focused {
                        PANE_ID_COLOR_FOCUSED
                    } else {
                        PANE_ID_COLOR_UNFOCUSED
                    })
                    .into(),
            ])
            .spacing(5);

            let title_bar = pane_grid::TitleBar::new(title)
                .controls(pane.controls.view(id, total_panes, pane.is_pinned))
                .padding(1)
                .style(style::TitleBar { is_focused });

            pane_grid::Content::new(pane.content.view(
                id,
                total_panes,
                pane.is_pinned,
                has_sample_creator_open,
                pane_name,
            ))
            .title_bar(title_bar) // <<-- // TODO: Title bar should probably be something like tabs with project-name
            .style(style::Pane { is_focused })
        })
        .width(Length::Fill)
        .height(Length::Fill)
        .spacing(10)
        // .on_click(Message::Clicked)
        .on_drag(Message::Dragged)
        .on_resize(10, Message::Resized);

        pane_grid = pane_grid.on_click(Message::Clicked);

        let mut wrapper = Row::new().height(Length::Fill).width(Length::Fill);

        let mut column_1: Column<Message> = Column::new().height(Length::Fill);
        // .push(Space::new(Length::Units(50), Length::Units(0)));

        // Show option to creation a new composition when not already in composition mode
        if !self.is_composition_mode {
            column_1 = column_1.push(
                Button::new(
                    &mut self.start_new_composition,
                    Text::new("Composition +"),
                )
                .on_press(Message::CreateCompositionPressed)
                .style(style::Button::Primary),
            )
        }

        // Show sidebar with composition relatd options when in composition mode
        if self.is_composition_mode {
            column_1 = column_1.push(
                Button::new(
                    &mut self.toggle_sidepanel,
                    Text::new(icon_to_char(iced_aw::Icon::ArrowBarLeft))
                        .font(iced_aw::ICON_FONT), // TODO Add sample browser
                )
                .on_press(Message::TestToggle)
                .style(style::Button::Control),
            )
            // .push(Text::new(&self.output_text))
        }

        let mut column_2: Column<Message> = Column::new().height(Length::Fill);

        // Show composition panes
        if self.is_composition_mode == true {
            column_2 = column_2.push(pane_grid);
        }

        wrapper = wrapper.push(column_1).push(column_2);

        Container::new(wrapper)
            .width(Length::Fill)
            .height(Length::Fill)
            .padding(0)
            .into()
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
}

// -------------------
// -------------------
// -------------------
// -------------------
// -------------------
// -------------------
// -------------------
// TODO Figure out where handle_hotkey belongs

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

pub fn toggle_audio_io_switch(switch_on: bool) {}

// -----------------------
