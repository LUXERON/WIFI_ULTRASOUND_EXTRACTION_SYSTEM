# Acousto-Electromagnetic Continuous Flow Transform (AECFT)
## Ultrasound Extraction from WiFi IQ Samples

This repository contains the production-ready Rust `no_std` / `bare-metal` compatible implementation of the AECFT framework. It extracts 1–10 MHz ultrasound from standard WiFi adapter IQ samples (such as the Realtek RTL8812BU / Archer T3) by resolving hardware microphonics as a continuous topological flow problem.

### Architecture Overview

1. **Acousto-Electromagnetic Continuous Flow Transform (AECFT):** Maps discrete quantized IQ baseband samples to a continuous Hamilton-Jacobi tensor field.
2. **2048-bit Signed Algebraic Lattice (HLAP):** Provides infinite-precision computation using Galois 32x64-bit limbs, preventing sub-LSB truncation of the mechanical jitter.
3. **Chern-Simons Topology:** Compresses massive IQ bandwidth into topological invariants.
4. **Topological Stochastic Resonance:** Reframes full-scale thermal noise inside the ADC as the "dither" required to resolve fractional LSB deformations from the MEMS/PCB capacitor microphonics.
5. **Non-Abelian Gauge Mapping (Option B):** Evaluates phase-differentials across disparate PCB micro-traces to generate Direction of Arrival (DOA) spatial awareness.
6. **Topological Hardware Abstraction Layer - THAL (Option A):** Maps the framework beyond the RTL8812BU to handle arbitrary SDR/Broadcom baseband outputs by using specific silicon noise floors as the stochastic resonance seed.

### Features
* Complete Rust implementation with zero mocks. 
* Direct raw USB block extraction bypassing kernel Network MAC stacks via `libusb` / `rusb`.
* Sub-LSB inference logic via `flash_attention.rs` applying eigenstate limits bounding the 1-10 MHz limits of PCB substrate resonance.
* Automated 32-bit floating point `.wav` export.

### Documentation
A detailed series of mathematical postulates including the `AECFT_POSTULATE.md` can be found in the `/docs` directory.

### Build Instructions
```bash
# Optimized for WSL Ubuntu 24.04 Environments
# Make sure usbipd is bridging your WiFi adapter into WSL.
cargo build --release
```

*Architected by LUXERON for ultra-high-resolution Topological Signal Intelligence.*
