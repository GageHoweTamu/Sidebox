use nih_plug::prelude::*;
use std::sync::Arc;

#[allow(unused_imports)]
use core::f32::consts::PI;

use circular_buffer::CircularBuffer;

//use nih_plug_egui::egui;
//use nih_plug_egui::create_egui_editor;
//use crate::egui::Window;

// rustfft = "6.2.0"
// dasp = "0.11.0"

mod other_stuff;
use other_stuff::SimpleEnvelopeFollower;

// mod buffer;
// use buffer::RingBuffer;

// mod editor;
// use editor::SideboxEditor;


struct Sidebox {
    params: Arc<SideboxParams>,
}

#[derive(Params)]
struct SideboxParams { // Plugin Parameters

    #[id = "input gain"]
    pub input_gain: FloatParam,

    #[id = "sidechain input gain"]
    pub sidechain_input_gain: FloatParam,

    #[id = "output gain"]
    pub output_gain: FloatParam,

    #[id = "sidechain phase flip"]
    pub sidechain_phase_flip: IntParam,

    #[id = "envelope follower smoothing"]
    pub envelope_follower_smoothing: IntParam,

    #[id = "mode"]
    pub mode: IntParam,
}

impl Default for Sidebox {
    fn default() -> Self {
        Self {
            params: Arc::new(SideboxParams::default()),
        }
    }
}

impl Default for SideboxParams { // Parameter Definitions
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
                "Mode", 0, IntRange::Linear { min: (0), max: (7) } // 0: addition, 1: multiplication, 2: absolute value multiplication, 3: modulo, 4: simple envelope follower, 5: sidechain as modulator, 6: convolution, 7: analog ring modulation, etc
            ),
            sidechain_phase_flip: IntParam::new(
                "Sidechain phase flip", 0, IntRange::Linear { min: (0), max: (1) }
            ),
            envelope_follower_smoothing: IntParam::new(
                "Envelope follower smoothing", 10, IntRange::Linear { min: 5, max: 1000 },
            ),
        }
    }
}

impl Plugin for Sidebox { // Plugin implementation
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
    
        // Apply sidechain operation
        for (mut channel_samples, mut sidechain_samples) in buffer.iter_samples().zip(aux_input0.iter_samples()) {
            let mode = self.params.mode.smoothed.next();
            let output_gain = self.params.output_gain.smoothed.next();
            let input_gain = self.params.input_gain.smoothed.next();
            let sidechain_input_gain = self.params.sidechain_input_gain.smoothed.next();
            let sidechain_phase_flip = self.params.sidechain_phase_flip.smoothed.next();
            
            // declare circular queue for envelope follower's moving average
            let mut prev_left = CircularBuffer::<100, f32>::new();
            let mut prev_right = CircularBuffer::<100, f32>::new();

            // Do processing
            match mode {
                0 => { // addition (DONE)
                    for (sample, sidechain_sample) in channel_samples.iter_mut().zip(sidechain_samples.iter_mut()) {
                        *sidechain_sample *= sidechain_input_gain;
                        *sample *= input_gain;
                        *sample += *sidechain_sample;
                        *sample *= output_gain;
                    }
                }
                1 => { // mutiplication (DONE)
                    for (sample, sidechain_sample) in channel_samples.iter_mut().zip(sidechain_samples.iter_mut()) {
                        *sample *= input_gain;
                        *sidechain_sample *= sidechain_input_gain;
                        *sample *= *sidechain_sample;
                        *sample *= output_gain;
                    }
                }
                4 => { // Simple envelope follower (not working)
                    let mut envelope_follower = SimpleEnvelopeFollower::new(
                        self.params.envelope_follower_smoothing.smoothed.next(),
                    );
                    for (sample, sidechain_sample) in channel_samples.iter_mut().zip(sidechain_samples.iter_mut()) {
                        *sample *= input_gain;
                        *sidechain_sample *= sidechain_input_gain;
                        let envelope = envelope_follower.process(*sidechain_sample);
                        *sample *= envelope; // envelope should range from 0 to 1, where 1 is the maximum amplitude
                        *sample *= output_gain;
                    }
                }
                3 => { // modulo (DONE)
                    for (sample, sidechain_sample) in channel_samples.iter_mut().zip(sidechain_samples.iter_mut()) {
                        *sidechain_sample *= sidechain_input_gain;
                        *sample *= input_gain;
                        *sample %= *sidechain_sample;
                        *sample *= output_gain;
                    }
                }
                2 => { // absolute value multiplication
                    for (sample, sidechain_sample) in channel_samples.iter_mut().zip(sidechain_samples.iter_mut()) {
                        *sample = *sample * input_gain * (*sidechain_sample).abs() * sidechain_input_gain
                    }
                }
                5 => { // outputs sidechain as a modulator

                }
                6 => {
                        // circular buffer test (i hope this works) (Not finished)
                        let mut rolling_avg = 0.0;
                        for (sample, sidechain_sample) in channel_samples.iter_mut().zip(sidechain_samples.iter_mut()) {
                            *sidechain_sample *= sidechain_input_gain;
                            prev_outputs.push_back(*sidechain_sample);              // add the current sample
                            let num = prev_outputs.len();                           // get length of buffer
                            rolling_avg = prev_outputs.iter().sum::<f32>() / num as f32; // calculate the average
                            *sample = rolling_avg * output_gain;                    // set the output to the average
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

                _ => { // testing ground
                    for sample in buffer.iter_samples() {
                        let left_sample = sample[0];
                        let right_sample = sample[1];
                        left_sample *= input_gain;
                    }
                }
            }
        
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
