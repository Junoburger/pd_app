use std::{sync::mpsc, thread};

use iced::Application;
#[path = "./app.rs"]
mod app;
use crate::app::{audio, PsycheDaily};
use iced::{window, Settings};

fn main() -> iced::Result {
    let (tx, rx) = mpsc::channel();

    //    let safe_state = Arc::new(Mutex::new(self.switch_on));
    //     thread::spawn(move || {
    //         let val = *safe_state.lock().unwrap();

    //         tx.send(val).unwrap();
    //     });

    thread::spawn(move || {
        let received_state = rx.recv().unwrap();

        println!("RECEIVED = {:?} ", received_state);
        audio::open_audio_io(received_state);
    });

    // PsycheDaily::get_state(self);

    let settings = Settings {
        window: window::Settings {
            // icon: Some(icon.unwrap()),
            min_size: Some((300, 200)),
            transparent: true,
            ..window::Settings::default()
        },
        ..Settings::default()
    };

    // run the app
    PsycheDaily::run(settings)
}

// impl PsycheDaily {
//     pub fn get_state(&self) {
//         println!("{:?}", self.switch_on);
//     }
// }
