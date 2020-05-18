
use i2cdev::linux::LinuxI2CDevice;
use std::thread;
use std::time::Duration;
use amg88xx::amg88xx::*;

#[cfg(any(target_os = "linux", target_os = "android"))]
fn main() {
	let device = "/dev/i2c-1";
	let amg88xx_i2cdev = LinuxI2CDevice::new(device, SLAVE_ADDR_PRIMARY).unwrap();

	let mut amg88xx = AMG88XX::new(amg88xx_i2cdev).unwrap();

	loop {
		let matrix = amg88xx.pixels().unwrap();
		println!(
			"Temperature: {:?} ",
			matrix
		);
		thread::sleep(Duration::from_millis(100));
	}
}
