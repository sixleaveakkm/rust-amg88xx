
use i2cdev::linux::LinuxI2CDevice;
use std::thread;
use std::time::Duration;
use amg88xx::amg88xx::*;


#[cfg(any(target_os = "linux", target_os = "android"))]
fn main() {
	// device is the path of your i2c-addr under /dev.
	let device = "/dev/i2c-1";

	// set device addr, SLAVE_ADDR_PRIMARY is default addr 0x69. SLAVE_ADDR_ALT is 0x68
	let amg88xx_i2cdev = LinuxI2CDevice::new(device, SLAVE_ADDR_PRIMARY).unwrap();

	// new set the device to normal mode without interrupt and fps 10
	let mut amg88xx = AMG88XX::new(amg88xx_i2cdev).unwrap();

	// loop get 8*8 pixels temperature
	loop {
		let matrix = amg88xx.pixels().unwrap();
		println!(
			"Temperature: {:?} ",
			matrix
		);
		thread::sleep(Duration::from_millis(100));
	}
}

#[cfg(not(any(target_os = "linux", target_os = "android")))]
fn main() {}
