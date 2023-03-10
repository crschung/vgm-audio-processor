use std::{time::Duration};

use midly::{TrackEvent, MidiMessage, TrackEventKind};
use rodio::{source::{SineWave, Source}, OutputStream, Sink};

fn main() {

    let options = eframe::NativeOptions {
        initial_window_size: Some(egui::vec2(400.0, 300.0)),
        ..Default::default()
    };
    eframe::run_native(
        "RustNES",
        options,
        Box::new(|_cc| Box::new(RustNES::default())),
    )
}

///
/// Takes as an input the current MIDI track, and can be used to retrieve individual notes
/// Note: MIDIs can have multiple tracks
/// 
fn parse_midi(track: &Vec<TrackEvent>)
{
    for track_event in track{
        let kind = track_event.kind;

        match kind {
            TrackEventKind::Midi { channel, message } => match message {
                MidiMessage::NoteOff { key, vel } => todo!(),
                MidiMessage::NoteOn { key, vel } => todo!(),
                MidiMessage::Aftertouch { key, vel } => todo!(),
                MidiMessage::Controller { controller, value } => todo!(),
                MidiMessage::ProgramChange { program } => todo!(),
                MidiMessage::ChannelAftertouch { vel } => todo!(),
                MidiMessage::PitchBend { bend } => todo!(),
            }
            _ => {}

        }
    }
}

/// Tentative name taken from Github
struct RustNES {
    // Test variable for the GUI. Displays currently selected files name
    picked_path: Option<String>,
}

impl Default for RustNES {
    fn default() -> Self {
        Self {
            picked_path: None,
        }
    }
}

/// Egui base
impl eframe::App for RustNES {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("RustNES");


            if ui.button("Open file…").clicked() {

                // rfd is used to access files
                if let Some(path) = rfd::FileDialog::new().pick_file() {
                    self.picked_path = Some(path.display().to_string());
                }
            }

            if let Some(picked_path) = &self.picked_path {
                ui.horizontal(|ui| {
                    ui.label("Picked Midi:");
                    ui.monospace(picked_path);
                });
            }

            if ui.button("Play").clicked(){
                play_sine_wave(440.0)
            }
        });
    }
}


///
/// Simple rodio sink to play a sine wave
/// 
fn play_sine_wave(freq: f32){
    let source = SineWave::new(freq).take_duration(Duration::from_secs_f32(1.0));

    let (_stream, stream_handle) = OutputStream::try_default().unwrap();
    let sink = Sink::try_new(&stream_handle).unwrap();

    sink.append(source);
    sink.sleep_until_end();
}