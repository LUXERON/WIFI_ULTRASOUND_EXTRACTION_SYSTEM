use crate::math::alu_2048::Signed2048;

/// Filters down the infinite precision `Signed2048` eigenstates back into
/// workable `f32` samples representing the 1-10 MHz ultrasound jitter explicitly explicitly
pub fn extract_ultrasound_signature(eigenstates: &[Signed2048]) -> Vec<f32> {
    let mut audio_samples = Vec::with_capacity(eigenstates.len());

    for eigen in eigenstates {
        // Since the microphony happens at extremely fine thresholds (lower fractional parts)
        // that 64-bit systems clip, our 2048 bit ALU has captured the mechanical jitter
        // at the 0th and 1st limb boundary explicitly.

        // Demodulate the lower limb
        // Converting u64 bounds down to -1.0..1.0 audio floats
        let raw_val = eigen.limbs[0] as f64; 
        
        let mut audio_float = (raw_val / (u64::MAX as f64)) * 2.0 - 1.0;
        if !eigen.sign {
            audio_float = -audio_float;
        }

        audio_samples.push(audio_float as f32);
    }

    audio_samples
}
