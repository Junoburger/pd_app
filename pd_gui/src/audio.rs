use std::io::Error;

use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use fundsp::prelude::AudioNode;
use ringbuf::RingBuffer;

use fundsp::hacker::*;

#[derive(Debug)]
pub struct AudioData {
    latency: f32,
    input_device: String,
    output_device: String,
}

impl AudioData {
    fn from_args() -> Result<Self, Error> {
        // TODO: Check if clap if necessary on init AND figure out optimal latency
        // !OR
        //! always set default and create a switcher that iterates through available I/O

        return Ok(AudioData {
            latency: 100.0,
            input_device: "default".to_string(),
            output_device: "default".to_string(),
        });
    }
}

pub fn output_test_runner() {
    let opt = AudioData::from_args().unwrap();

    let host = cpal::default_host();
    let input_device = host
        .default_input_device()
        .expect("failed to find input device");

    let output_device = host
        .default_output_device()
        .expect("failed to find output device");

    // We'll try and use the same configuration between streams to keep it simple.
    let config: cpal::StreamConfig =
        input_device.default_input_config().unwrap().into();
    println!("{:#?}", config.sample_rate);

    // Create a delay in case the input and output devices aren't synced.
    let latency_frames = (opt.latency / 1_000.0) * config.sample_rate.0 as f32;
    let latency_samples = latency_frames as usize * config.channels as usize;

    // The buffer to share samples
    let ring = RingBuffer::new(latency_samples * 2);
    let (mut producer, mut consumer) = ring.split();

    // Fill the samples with 0.0 equal to the length of the delay.
    for _ in 0..latency_samples {
        // The ring buffer has twice as much space as necessary to add latency here,
        // so this should never fail
        producer.push(0.0).unwrap();
    }

    let input_data_fn = move |data: &[f32], _: &cpal::InputCallbackInfo| {
        let mut output_fell_behind = false;
        for &sample in data {
            if producer.push(sample).is_err() {
                output_fell_behind = true;
            }
        }
        if output_fell_behind {
            eprintln!("output stream fell behind: try increasing latency");
        }
    };

    let output_data_fn =
        move |data: &mut [f32], _: &cpal::OutputCallbackInfo| {
            let mut input_fell_behind = false;

            for sample in data {
                *sample = match consumer.pop() {
                    Some(s) => s,
                    None => {
                        input_fell_behind = true;
                        0.0
                    }
                };
            }
            if input_fell_behind {
                eprintln!("input stream fell behind: try increasing latency");
            }
        };

    // Build streams.

    let input_stream = input_device
        .build_input_stream(&config, input_data_fn, err_fn)
        .unwrap();

    let output_stream = output_device
        .build_output_stream(&config, output_data_fn, err_fn)
        .unwrap();
    println!("Successfully built streams.");

    // Play the streams.
    println!(
        "Starting the input and output streams with `{}` milliseconds of latency.",
        opt.latency
    );
    input_stream.play().unwrap();
    output_stream.play().unwrap();

    // Run for 3 seconds before closing.
    println!("Playing for 5 seconds... ");
    std::thread::sleep(std::time::Duration::from_secs(10));
    drop(input_stream);
    drop(output_stream);
    println!("Done!");
}

pub fn err_fn(err: cpal::StreamError) {
    eprintln!("an error occurred on stream: {}", err);
}
