// Copyright (C) 2016, The Gpio Util Project Developers.

use options::GpioExportOptions;
use config::GpioConfig;
use std::process::exit;
use export;

pub fn main(config: &GpioConfig, opts: &GpioExportOptions) {
    let pin = match config.get_pin(&opts.pin[..]) {
        Some(pin) => pin,
        None => {
            println!("Unable to find config entry for pin '{}'", opts.pin);
            exit(1)
        }
    };

    let symlink_root = match opts.symlink_root {
        Some(ref slr) => &slr[..],
        None => config.get_symlink_root(),
    };

    if let Err(e) = export::export(pin, Some(symlink_root)) {
        println!("Error occurred while exporting pin: {:?}", pin);
        println!("{}", e);
        exit(1);
    }
}
