use nih_plug::prelude::*;
use std::sync::Arc;
#[allow(unused_imports)]
use core::f32::consts::PI;

use nih_plug_egui::egui;
use nih_plug_egui::create_egui_editor;
use crate::egui::Window;

// rustfft = "6.2.0"
// dasp = "0.11.0"

// mod functions;
// use functions::EnvelopeFollower;

mod editor;
use editor::SideboxEditor;


struct Sidebox {
    params: Arc<SideboxParams>,
}

#[derive(Params)]
struct SideboxParams {

    #[id = "input gain"]
    pub input_gain: FloatParam,

    #[id = "sidechain input gain"]
    pub sidechain_input_gain: FloatParam,

    #[id = "output gain"]
    pub output_gain: FloatParam,

    #[id = "sidechain phase flip"]
    pub sidechain_phase_flip: IntParam,

    #[id = "envelope follower smoothing"]
    pub envelope_follower_smoothing: FloatParam,

    #[id = "mode"]
    pub mode: IntParam,
}

impl Default for Sidebox { // Don't change this
    fn default() -> Self {
        Self {
            params: Arc::new(SideboxParams::default()),
        }
    }
}


impl Default for SideboxParams {
    fn default() -> Self {
        Self {
            // This gain is stored as linear gain. NIH-plug comes with useful conversion functions to treat these kinds of parameters as if we were dealing with decibels. Storing this as decibels is easier to work with, but requires a conversion for every sample.
            input_gain: FloatParam::new(
                "Input gain",
                util::db_to_gain(0.0),
                FloatRange::Skewed {
                    min: util::db_to_gain(-30.0),
                    max: util::db_to_gain(30.0),
                    factor: FloatRange::gain_skew_factor(-30.0, 30.0),
                },
            )
            .with_smoother(SmoothingStyle::Logarithmic(50.0))
            .with_unit(" dB")
            .with_value_to_string(formatters::v2s_f32_gain_to_db(2))
            .with_string_to_value(formatters::s2v_f32_gain_to_db()),
            sidechain_input_gain: FloatParam::new(
                "Sidechain input gain",
                util::db_to_gain(0.0),
                FloatRange::Skewed {
                    min: util::db_to_gain(-30.0),
                    max: util::db_to_gain(30.0),
                    factor: FloatRange::gain_skew_factor(-30.0, 30.0),
                },
            )
            .with_smoother(SmoothingStyle::Logarithmic(50.0))
            .with_unit(" dB")
            .with_value_to_string(formatters::v2s_f32_gain_to_db(2))
            .with_string_to_value(formatters::s2v_f32_gain_to_db()),
            output_gain: FloatParam::new(
                "output gain",
                util::db_to_gain(0.0),
                FloatRange::Skewed {
                    min: util::db_to_gain(-30.0),
                    max: util::db_to_gain(30.0),
                    factor: FloatRange::gain_skew_factor(-30.0, 30.0),
                },
            )
            .with_smoother(SmoothingStyle::Logarithmic(50.0))
            .with_unit(" dB")
            .with_value_to_string(formatters::v2s_f32_gain_to_db(2))
            .with_string_to_value(formatters::s2v_f32_gain_to_db()),
        
            mode: IntParam::new(
                "Mode",
                0,
                IntRange::Linear { min: (0), max: (5) } // add more modes here
            ),
            sidechain_phase_flip: IntParam::new(
                "Sidechain phase flip",
                0,
                IntRange::Linear { min: (0), max: (1) }
            ),
            envelope_follower_smoothing: FloatParam::new(
                "Envelope follower smoothing",
                50.0,
                FloatRange::Linear { min: 0.0, max: 0.5 },
                
            )
        }
    }
}

impl Plugin for Sidebox {
    const NAME: &'static str = "Sidebox";
    const VENDOR: &'static str = "ghowe";
    const URL: &'static str = env!("CARGO_PKG_HOMEPAGE");
    const EMAIL: &'static str = "howe.gaged@gmail.com";

    const VERSION: &'static str = env!("CARGO_PKG_VERSION");

    const AUDIO_IO_LAYOUTS: &'static [AudioIOLayout] = &[ // from https://github.com/robbert-vdh/nih-plug/blob/master/plugins/spectral_compressor/src/lib.rs
        AudioIOLayout {
            main_input_channels: NonZeroU32::new(2),
            main_output_channels: NonZeroU32::new(2),

            aux_input_ports: &[new_nonzero_u32(2)],

            ..AudioIOLayout::const_default()
        },
        /*
        AudioIOLayout {
            main_input_channels: NonZeroU32::new(1),
            main_output_channels: NonZeroU32::new(1),

            aux_input_ports: &[new_nonzero_u32(1)],

            ..AudioIOLayout::const_default()
        },
        */
    ];

    const MIDI_INPUT: MidiConfig = MidiConfig::None;
    const MIDI_OUTPUT: MidiConfig = MidiConfig::None;

    const SAMPLE_ACCURATE_AUTOMATION: bool = true;

    // If the plugin can send or receive SysEx messages, it can define a type to wrap around those
    // messages here. The type implements the `SysExMessage` trait, which allows conversion to and
    // from plain byte buffers.
    type SysExMessage = ();
    // More advanced plugins can use this to run expensive background tasks. See the field's
    // documentation for more information. `()` means that the plugin does not have any background
    // tasks.
    type BackgroundTask = ();

    fn editor(&self) -> Option<Box<dyn Editor>> {
        Some(Box::new(create_egui_editor(
            egui::Window::new("Sidebox Editor")
                .show(true)
                .open(true)
                .default_size(egui::Vec2::new(300.0,  200.0))
                .vscroll(true)
                .resizable(true)
                .scroll(true)
                .title_bar(true)
                .collapsible(false)
                .fixed_pos(false)
                .fixed_size(false)
                .min_size(egui::Vec2::new(100.0,  100.0))
                .min_width(100.0)
                .min_height(100.0)
                .max_size(egui::Vec2::new(std::f32::INFINITY, std::f32::INFINITY))
                .max_width(std::f32::INFINITY)
                .max_height(std::f32::INFINITY)
                .visible(true)
                .auto_sized(true)
                .scroll_to_cursor(true)
                .scroll_to_cursor_on_focus(true)
                .focus_on_appearing(true)
                .allow_scrolling(true)
                .allow_collapsing(true)
                .interactive(true)
                .movable(true)
                .enabled(true)
                .content(|ui| {
                    let mut editor = SideboxEditor::new(self.params.clone());
                    editor.ui(ui);
                }),
        )))
    }

    fn params(&self) -> Arc<dyn Params> {
        self.params.clone()
    }

    fn initialize(
        &mut self,
        _audio_io_layout: &AudioIOLayout,
        _buffer_config: &BufferConfig,
        _context: &mut impl InitContext<Self>,
    ) -> bool {
        // Resize buffers and perform other potentially expensive initialization operations here.
        // The `reset()` function is always called right after this function. You can remove this
        // function if you do not need it.

        // Create globabal variables and buffers here
        true
    }

    fn reset(&mut self) {
        // Reset buffers and envelopes here. This can be called from the audio thread and may not
        // allocate. You can remove this function if you do not need it.
    }

    fn process( // process one chunk of audio
        &mut self,
        buffer: &mut Buffer,
        _aux: &mut AuxiliaryBuffers,
        _context: &mut impl ProcessContext<Self>,

    ) -> ProcessStatus {

        let aux_input = &mut _aux.inputs;
        let aux_input0 = &mut aux_input[0];

        /* AuxiliaryBuffers definition
        pub struct AuxiliaryBuffers<'a> {
            pub inputs: &'a mut [Buffer<'a>],
            pub outputs: &'a mut [Buffer<'a>],
        }
        */

        const RC: f32 = 1.0 / (2.0 * std::f32::consts::PI * 400.0);
        const DT: f32 = 1.0 / 48000.0;
    
        // Apply sidechain operation
        for (mut channel_samples, mut sidechain_samples) in buffer.iter_samples().zip(aux_input0.iter_samples()) {
            let mode = self.params.mode.smoothed.next();
            let output_gain = self.params.output_gain.smoothed.next();
            let input_gain = self.params.input_gain.smoothed.next();
            let sidechain_input_gain = self.params.sidechain_input_gain.smoothed.next();
            let sidechain_phase_flip = self.params.sidechain_phase_flip.smoothed.next();
            let smoothing = self.params.envelope_follower_smoothing.smoothed.next();

            let mut prev_output: f32 = 0.0;

            // Do processing
            match mode {
                0 => { // addition
                    for (sample, sidechain_sample) in channel_samples.iter_mut().zip(sidechain_samples.iter_mut()) {
                        *sidechain_sample *= sidechain_input_gain;
                        *sample *= input_gain;
                        *sample += *sidechain_sample;
                        *sample *= output_gain;
                    }
                }
                1 => { // mutiplication
                    for (sample, sidechain_sample) in channel_samples.iter_mut().zip(sidechain_samples.iter_mut()) {
                        *sample *= input_gain;
                        *sidechain_sample *= sidechain_input_gain;
                        *sample *= *sidechain_sample;
                        *sample *= output_gain;
                    }
                }
                2 => { // do something random and interesting
                    for (sample, sidechain_sample) in channel_samples.iter_mut().zip(sidechain_samples.iter_mut()) {
                        

                    }
                }
                3 => { // modulo
                    for (sample, sidechain_sample) in channel_samples.iter_mut().zip(sidechain_samples.iter_mut()) {
                        *sidechain_sample *= sidechain_input_gain;
                        *sample *= input_gain;
                        *sample %= *sidechain_sample;
                        *sample *= output_gain;
                    }
                }
                4 => { // absolute value multiplication
                    for (sample, sidechain_sample) in channel_samples.iter_mut().zip(sidechain_samples.iter_mut()) {
                        *sample = *sample * input_gain * (*sidechain_sample).abs() * sidechain_input_gain
                    }
                }
                5 => { // this doesnt work right now

                    for (sample, sidechain_sample) in channel_samples.iter_mut().zip(sidechain_samples) {
                        let current_output = (RC * *sidechain_sample + prev_output) / (RC + DT);
                        prev_output = current_output;
                        *sample = current_output;
                    }
                }
                    /*
                    for (sample, sidechain_sample) in channel_samples.iter_mut().zip(sidechain_samples.iter_mut()) {
                        let envelope = self.follower.as_mut().unwrap().process(*sidechain_sample);
                        *sample = *sample + (*sample * input_gain * envelope) * output_gain;
                        let envelope_follower_time = self.params.envelope_follower_time.smoothed.next();
                        self.follower.as_mut().unwrap().update_time(envelope_follower_time);
                    }*/
                
                // simple convolution
                // analog ring modulation
                // FM modulation; this is too complicated at this point because it involves processing the left and right channels separately (maybe not?)
                // convolution
                // etc

                _ => {
                        { // do nothing
                    }
                }
            }
        
            
            for sample in channel_samples { *sample *= output_gain; }
        }
    
        ProcessStatus::Normal
    }
}

impl ClapPlugin for Sidebox {
    const CLAP_ID: &'static str = "com.your-domain.sidebox";
    const CLAP_DESCRIPTION: Option<&'static str> = Some("A sidechain utility including several ways to combine sidechain signals");
    const CLAP_MANUAL_URL: Option<&'static str> = Some(Self::URL);
    const CLAP_SUPPORT_URL: Option<&'static str> = None;

    // Don't forget to change these features
    const CLAP_FEATURES: &'static [ClapFeature] = &[ClapFeature::AudioEffect, ClapFeature::Stereo];
}

impl Vst3Plugin for Sidebox {
    const VST3_CLASS_ID: [u8; 16] = *b"Exactly16Chars!!";

    // And also don't forget to change these categories
    const VST3_SUBCATEGORIES: &'static [Vst3SubCategory] =
        &[Vst3SubCategory::Fx, Vst3SubCategory::Dynamics];
}

nih_export_clap!(Sidebox);
nih_export_vst3!(Sidebox);
