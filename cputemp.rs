// SPDX-License-Identifier: GPL-2.0

// Copyright (C) 2024 Google LLC.

//! Rust misc device to get CPU temperature.

use kernel::{
    c_str,
    ioctl::_IO,
    miscdevice::{MiscDevice, MiscDeviceOptions, MiscDeviceRegistration},
    prelude::*,
    thermal,
};

const RUST_MISC_DEV_CPUTEMP: u32 = _IO('R' as u32, 9);

module! {
    type: RustCPUTempModule,
    name: "rust_cputemp",
    author: "Charalampos Mitrodimas",
    description: "Rust misc device CPU Temp",
    license: "GPL",
}

struct RustCPUTempModule {
    _miscdev: Pin<KBox<MiscDeviceRegistration<RustCPUTemp>>>,
}

impl kernel::Module for RustCPUTempModule {
    fn init(_module: &'static ThisModule) -> Result<Self> {
        pr_info!("Initialising Rust Misc Device CPU Temp\n");

        let options = MiscDeviceOptions {
            name: c_str!("rust-misc-device"),
        };

        Ok(Self {
            _miscdev: KBox::pin_init(
                MiscDeviceRegistration::<RustCPUTemp>::register(options),
                GFP_KERNEL,
            )?,
        })
    }
}

struct RustCPUTemp;

#[vtable]
impl MiscDevice for RustCPUTemp {
    type Ptr = KBox<Self>;

    fn open() -> Result<Self::Ptr> {
        pr_info!("Opening Rust Misc Device CPU Temp\n");
        Ok(KBox::new(RustCPUTemp, GFP_KERNEL)?)
    }

    fn ioctl(
        _device: <Self::Ptr as kernel::types::ForeignOwnable>::Borrowed<'_>,
        cmd: u32,
        _arg: usize,
    ) -> Result<isize> {
        let mut temp: i32 = 0;

        match cmd {
            RUST_MISC_DEV_CPUTEMP => {
                pr_info!("Hello from Rust Misc Device\n");

                let tz = unsafe {
                    thermal::thermal_zone_get_zone_by_name(
                        c_str!("x86_pkg_temp").as_ptr() as *const i8
                    )
                };
                unsafe { thermal::thermal_zone_get_temp(tz, &mut temp) };

                Ok((temp / 1000) as isize)
            }
            _ => Err(ENOIOCTLCMD),
        }
    }
}

impl Drop for RustCPUTemp {
    fn drop(&mut self) {
        pr_info!("Exiting the Rust Misc Device CPU Temp\n");
    }
}
