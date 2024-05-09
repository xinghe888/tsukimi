use gst::prelude::*;

use crate::{client::network::get_song_streaming_uri, ui::provider::core_song::CoreSong};

pub struct MusicPlayer {
    pipeline: gst::Element,
}

impl MusicPlayer {
    pub fn new() -> Self {
        // Initialize GStreamer
        gst::init().unwrap();

        // Build the pipeline
        let pipeline = gst::ElementFactory::make("playbin").build().unwrap();
        // Start playing
        let bus = pipeline.bus().unwrap();
        bus.add_signal_watch();
        bus.connect_message(Some("progress"), {
            move |_bus, msg| {
                on_bus_message(msg);
            }
        });
        bus.connect_message(Some("eos"), {
            move |_bus, msg| {
                on_bus_message(msg);
            }
        });
        Self { pipeline }
    }

    pub fn playing(&self) {
        let pipeline = &self.pipeline;
        pipeline
            .set_state(gst::State::Playing)
            .expect("Unable to set the pipeline to the `Playing` state");
    }

    pub fn play(&self, core_song: &CoreSong) {
        self.stop();
        let uri = get_song_streaming_uri(&core_song.id());

        self.pipeline.set_property("uri", uri);
        self.playing();
    }

    pub fn stop(&self) {
        let pipeline = &self.pipeline;
        pipeline
            .set_state(gst::State::Null)
            .expect("Unable to set the pipeline to the `Null` state");
    }

    pub fn get_position(&self) -> gst::ClockTime {
        if self.pipeline.current_state() != gst::State::Playing {
            return gst::ClockTime::from_mseconds(0);
        }
        let pipeline = &self.pipeline;
        let position = pipeline.query_position::<gst::ClockTime>();
        if let Some(position) = position {
            position
        } else {
            gst::ClockTime::from_seconds(0)
        }
    }

    pub fn position(&self) -> f64 {
        self.get_position().mseconds() as f64 / 1000.0
    }

    pub fn pause(&self) {
        let pipeline = &self.pipeline;
        pipeline
            .set_state(gst::State::Paused)
            .expect("Unable to set the pipeline to the `Paused` state");
    }

    pub fn unpause(&self) {
        let pipeline = &self.pipeline;
        pipeline
            .set_state(gst::State::Playing)
            .expect("Unable to set the pipeline to the `Playing` state");
    }

    pub fn state(&self) -> gst::State {
        let pipeline = &self.pipeline;
        pipeline.current_state()
    }
}

fn on_bus_message(msg: &gst::Message) {
    match msg.view() {
        gst::MessageView::Eos(eos) => {
            let eos_src_name = match eos.src() {
                Some(src) => src.name(),
                None => "no_name".into(),
            };
            println!("End of stream from {}", eos_src_name);
        }
        gst::MessageView::Error(err) => {
            println!(
                "Error from {:?}: {} ({:?})",
                err.src().map(|s| s.path_string()),
                err.error(),
                err.debug()
            );
        }
        gst::MessageView::DurationChanged(duration) => {
            println!("Progress {}", duration.src().unwrap());
        }
        gst::MessageView::Progress(progress) => match progress.src() {
            Some(minutes) => {
                println!("Progress {}", minutes);
            }
            None => {
                println!("Progress None");
            }
        },
        _ => {
            println!("Message {:?}", msg.type_());
        }
    }
}
