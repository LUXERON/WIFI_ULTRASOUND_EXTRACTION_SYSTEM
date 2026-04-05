/// High fidelity sample output matching native ultrasound bandwidths (e.g. 10 MHz SR).
pub struct PcmWriter {
    pub sample_rate: u32,
    pub channels: u16,
}

impl Default for PcmWriter {
    fn default() -> Self {
        Self {
            sample_rate: 10_000_000, // 10 MHz resolution for perfect ultrasound capture
            channels: 1, // Mono
        }
    }
}
