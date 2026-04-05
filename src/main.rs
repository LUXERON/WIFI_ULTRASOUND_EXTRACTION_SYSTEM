pub mod capture;
pub mod compression;
pub mod math;
pub mod cft;
pub mod audio;

use anyhow::Result;

fn main() -> Result<()> {
    println!("Initializing CURILEXA ALPHA WiFi Ultrasound AECFT Extraction Engine...");
    println!("Target: Archer T3 (Realtek RTL8812BU)");

    // Phase 1: Hardware capture
    println!("[1/5] Establishing USB Monitor Mode interception...");
    let mut interceptor = capture::archer_t3::ArcherT3Interceptor::new()?;
    
    // Sniff 1 raw CSI block for demonstration
    let raw_buf = interceptor.read_raw_block()?;
    println!("      Captured raw bounds of {} bytes.", raw_buf.len());
    let iq_stream = capture::iq_stream::extract_iq_from_buffer(&raw_buf)?;
    println!("      Extracted {} IQ samples.", iq_stream.len());

    // Phase 2: Topological Chern-Simons invariant compression
    println!("[2/5] Compressing topological IQ manifold boundaries...");
    let invariant = compression::chern_simons::compress_iq_manifold(&iq_stream);
    println!("      Winding #: {}, Gauge: {}", invariant.winding_number, invariant.gauge_field_magnitude);
    
    // Phase 3 & 4: HLAP 2048-bit continuous embeddings & Flash Attention Eigenstates
    println!("[3/5] & [4/5] Embedding into 2048-bit lattice and extracting eigenstates...");
    let hamiltonian_states = cft::hamiltonian::embed_into_manifold(&[invariant]);
    let flash_attention = cft::flash_attention::FlashAttentionMatrix::new(hamiltonian_states);
    
    let wave_collapse = flash_attention.collapse_wave_function();
    let ultrasound_pcm = cft::eigen_extract::extract_ultrasound_signature(&wave_collapse);
    
    // Phase 5: High-fidelity write
    println!("[5/5] Synthesizing continuous WAV output...");
    let writer_config = audio::pcm_writer::PcmWriter::default();
    audio::wav_export::export_ultrasound_wav("archer_t3_ultrasound_isolated.wav", &ultrasound_pcm, &writer_config)?;
    
    println!("SUCCESS: Extraction complete. Generated 'archer_t3_ultrasound_isolated.wav'.");
    Ok(())
}
