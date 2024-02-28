// this is a backup of the original file, in case I mess up the original file

use nih_plug::prelude::*;
use std::sync::Arc;

// This is a shortened version of the gain example with most comments removed, check out
// https://github.com/robbert-vdh/nih-plug/blob/master/plugins/examples/gain/src/lib.rs to get
// started

struct Sidebox {
    params: Arc<SideboxParams>,
}


#[derive(Params)]
struct SideboxParams {
    #[id = "gain"]
    pub gain: FloatParam,

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
            gain: FloatParam::new(
                "Gain",
                util::db_to_gain(0.0),
                FloatRange::Skewed {
                    min: util::db_to_gain(-30.0),
                    max: util::db_to_gain(30.0),
                    // This makes the range appear as if it was linear when displaying the values as
                    // decibels
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
                IntRange::Linear { min: (0), max: (1) } // add more modes here
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
        AudioIOLayout {
            main_input_channels: NonZeroU32::new(1),
            main_output_channels: NonZeroU32::new(1),

            aux_input_ports: &[new_nonzero_u32(1)],

            ..AudioIOLayout::const_default()
        },
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
        true
    }

    fn reset(&mut self) {
        // Reset buffers and envelopes here. This can be called from the audio thread and may not
        // allocate. You can remove this function if you do not need it.
    }

    fn process(
        &mut self,
        buffer: &mut Buffer,
        _aux: &mut AuxiliaryBuffers,
        _context: &mut impl ProcessContext<Self>,

    ) -> ProcessStatus {

        // how do i get a stereo sidechain signal from the AuxiliaryBuffers?
        let sidechain_signal = _aux.get(0).unwrap();
    
        for (channel_samples, sidechain_samples) in  { // ?
        // maybe buffer.iter_samples().zip(sidechain_signal.iter_samples()) {
            let gain = self.params.gain.smoothed.next();
            let mode = self.params.mode.smoothed.next();
    
            // Apply sidechain operation
            for (sample, sidechain_sample) in channel_samples.zip(sidechain_samples) {
                match mode as i32 {
                    0 => *sample += gain * *sidechain_sample,
                    1 => *sample *= gain * *sidechain_sample,
                    _ => (), // Do nothing for other modes
                }
            }

            // Apply output gain
            let gain = self.params.gain.smoothed.next();
            for sample in channel_samples {
                *sample *= gain;
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
