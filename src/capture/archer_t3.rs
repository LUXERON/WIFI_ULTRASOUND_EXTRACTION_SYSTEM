use anyhow::{Context, Result};
use rusb::{Context as UsbContext, Device, DeviceHandle, Direction, TransferType, UsbContext as _};
use std::time::Duration;

const TP_LINK_VID: u16 = 0x2357;
const ARCHER_T3_PID: u16 = 0x011e; // Often seen as Realtek 8812BU

/// Configuration for hardware interception.
pub struct InterceptConfig {
    pub chunk_size: usize,
    pub timeout: Duration,
}

impl Default for InterceptConfig {
    fn default() -> Self {
        Self {
            chunk_size: 1024 * 64, // 64KB bulk transfer blocks
            timeout: Duration::from_millis(100),
        }
    }
}

pub struct ArcherT3Interceptor {
    handle: DeviceHandle<UsbContext>,
    endpoint_in: u8,
    config: InterceptConfig,
}

impl ArcherT3Interceptor {
    /// Attempts to locate and open the Archer T3 USB device.
    pub fn new() -> Result<Self> {
        let mut context = UsbContext::new().context("Failed to initialize libusb")?;
        
        let (device, mut handle) = Self::find_device(&mut context)?;
        
        // Detach kernel driver if necessary (WSL2 requires usbipd configuration beforehand)
        if handle.kernel_driver_active(0).unwrap_or(false) {
            handle.detach_kernel_driver(0).context("Could not detach kernel driver")?;
        }

        handle.claim_interface(0).context("Failed to claim interface 0")?;

        let endpoint_in = Self::find_bulk_in_endpoint(&device)?;

        Ok(Self {
            handle,
            endpoint_in,
            config: InterceptConfig::default(),
        })
    }

    fn find_device(ctx: &mut UsbContext) -> Result<(Device<UsbContext>, DeviceHandle<UsbContext>)> {
        for device in ctx.devices()?.iter() {
            let desc = device.device_descriptor()?;
            if desc.vendor_id() == TP_LINK_VID && desc.product_id() == ARCHER_T3_PID {
                let handle = device.open()?;
                return Ok((device, handle));
            }
        }
        anyhow::bail!("Archer T3 (VID: 0x{:04x}, PID: 0x{:04x}) not found on USB bus", TP_LINK_VID, ARCHER_T3_PID);
    }

    fn find_bulk_in_endpoint(device: &Device<UsbContext>) -> Result<u8> {
        let config_desc = device.active_config_descriptor()?;
        for interface in config_desc.interfaces() {
            for interface_desc in interface.descriptors() {
                for endpoint_desc in interface_desc.endpoint_descriptors() {
                    if endpoint_desc.direction() == Direction::In
                        && endpoint_desc.transfer_type() == TransferType::Bulk
                    {
                        return Ok(endpoint_desc.address());
                    }
                }
            }
        }
        anyhow::bail!("Could not find a Bulk IN endpoint on the Archer T3");
    }

    /// Pulls a raw bulk transfer block from the hardware.
    pub fn read_raw_block(&mut self) -> Result<Vec<u8>> {
        let mut buffer = vec![0u8; self.config.chunk_size];
        let bytes_read = self.handle.read_bulk(self.endpoint_in, &mut buffer, self.config.timeout)?;
        buffer.truncate(bytes_read);
        Ok(buffer)
    }
}

impl Drop for ArcherT3Interceptor {
    fn drop(&mut self) {
        let _ = self.handle.release_interface(0);
        let _ = self.handle.attach_kernel_driver(0);
    }
}
