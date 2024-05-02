use std::sync::Arc;

use nih_plug::prelude::*;

struct HueHue {
    params: Arc<HueHueParams>,
}

#[derive(Params)]
struct HueHueParams {

    #[id = "repeat_octaves"]
    repeat_ovtaves: BoolParam,

    #[id = "repeat_range"]
    repeat_range: IntParam,

    #[id = "resonant_gain"]
    resonant_gain: FloatParam,

    #[id = "Q"]
    q_factor: FloatParam,
    
    #[id = "gain"]
    gain: FloatParam,
}

impl Default for HueHue {
    fn default() -> Self {
        HueHue {
            params: Arc::new(HueHueParams::default())
        }
    }
}

impl Default for HueHueParams {
    fn default() -> Self {
        HueHueParams {
            repeat_ovtaves: BoolParam::new (
                "Repeat Octaves",
                false
            ),
            repeat_range: IntParam::new (
                "Repeat Range",
                10,
                IntRange::Linear { min: 1, max: 10 }
            ),
            resonant_gain: FloatParam::new (
                "Resonant Gain",
                util::db_to_gain(0.0),
                FloatRange::Skewed {
                    min: util::db_to_gain(-30.0),
                    max: util::db_to_gain(30.0),
                    // This makes the range appear as if it was linear when displaying the values as
                    // decibels
                    factor: FloatRange::gain_skew_factor(-30.0, 30.0),
                }
            )            
            .with_smoother(SmoothingStyle::Logarithmic(50.0))
            .with_unit(" dB")
            .with_value_to_string(formatters::v2s_f32_gain_to_db(2))
            .with_string_to_value(formatters::s2v_f32_gain_to_db()),
            q_factor: FloatParam::new (
                "Q",
                100.0,
                FloatRange::Linear { min: 0.0, max: 10000.0 }
            ),
            gain: FloatParam::new (
                "Gain",
                util::db_to_gain(0.0),
                FloatRange::Skewed {
                    min: util::db_to_gain(-30.0),
                    max: util::db_to_gain(30.0),
                    // This makes the range appear as if it was linear when displaying the values as
                    // decibels
                    factor: FloatRange::gain_skew_factor(-30.0, 30.0),
                }
            )            
            .with_smoother(SmoothingStyle::Logarithmic(50.0))
            .with_unit(" dB")
            .with_value_to_string(formatters::v2s_f32_gain_to_db(2))
            .with_string_to_value(formatters::s2v_f32_gain_to_db()),
        }
    }
}

impl Plugin for HueHue {
    const NAME: &'static str = "HueHue";
    const VENDOR: &'static str = "Boxman";
    const URL: &'static str = env!("CARGO_PKG_HOMEPAGE");
    const EMAIL: &'static str = "dhhoyt2002@gmail.com";
    const VERSION: &'static str = env!("CARGO_PKG_VERSION");

    const AUDIO_IO_LAYOUTS: &'static [AudioIOLayout] = &[AudioIOLayout {
        main_input_channels: NonZeroU32::new(2),
        main_output_channels: NonZeroU32::new(2),

        aux_input_ports: &[],
        aux_output_ports: &[],

        // Individual ports and the layout as a whole can be named here. By default these names
        // are generated as needed. This layout will be called 'Stereo', while a layout with
        // only one input and output channel would be called 'Mono'.
        names: PortNames::const_default(),
    }]; 

    const MIDI_INPUT: MidiConfig = MidiConfig::None;
    const MIDI_OUTPUT: MidiConfig = MidiConfig::None;


    const SAMPLE_ACCURATE_AUTOMATION: bool = true;
    type SysExMessage = ();
    type BackgroundTask = ();

    fn params(&self) -> Arc<dyn Params> {
        self.params.clone()
    } 


    fn process(
        &mut self,
        buffer: &mut Buffer,
        _aux: &mut AuxiliaryBuffers,
        _context: &mut impl ProcessContext<Self>,
    ) -> ProcessStatus {
        ProcessStatus::Normal
    }
}

impl ClapPlugin for HueHue {
    const CLAP_ID: &'static str = "HueHue";
    const CLAP_DESCRIPTION: Option<&'static str> = Some("A colorizer plugin");
    const CLAP_MANUAL_URL: Option<&'static str> = Some(Self::URL);
    const CLAP_SUPPORT_URL: Option<&'static str> = None;

    // Don't forget to change these features
    const CLAP_FEATURES: &'static [ClapFeature] = &[ClapFeature::AudioEffect, ];
}

impl Vst3Plugin for HueHue {
    const VST3_CLASS_ID: [u8; 16] = *b"HueHue Colorizer";

    // And also don't forget to change these categories
    const VST3_SUBCATEGORIES: &'static [Vst3SubCategory] =
        &[Vst3SubCategory::Fx, Vst3SubCategory::Dynamics];
}

nih_export_clap!(HueHue);
nih_export_vst3!(HueHue);