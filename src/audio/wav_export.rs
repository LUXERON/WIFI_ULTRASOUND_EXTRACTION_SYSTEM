use anyhow::Result;
use hound::{WavSpec, WavWriter, SampleFormat};
use crate::audio::pcm_writer::PcmWriter;

/// Exports the array of f32 extracted ultrasound samples to a native WAV format
/// that can be opened in DSP or audio editors.
pub fn export_ultrasound_wav(path: &str, samples: &[f32], pcm: &PcmWriter) -> Result<()> {
    let spec = WavSpec {
        channels: pcm.channels,
        sample_rate: pcm.sample_rate,
        bits_per_sample: 32, // Retaining high-fidelity 32-bit floats
        sample_format: SampleFormat::Float,
    };
    
    let mut writer = WavWriter::create(path, spec)?;
    for &sample in samples {
        // Clamp bounds just in case of eigenstate spikes
        let clamped = sample.max(-1.0).min(1.0);
        writer.write_sample(clamped)?;
    }
    
    writer.finalize()?;
    Ok(())
}
