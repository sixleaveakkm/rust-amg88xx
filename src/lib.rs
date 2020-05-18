pub mod amg88xx {
	use i2cdev::core::I2CDevice;

	// Operation Modes
	const NORMAL_MODE: u8 = 0x00;
	const SLEEP_MODE: u8 = 0x10;
	const STAND_BY_60: u8 = 0x20;
	const STAND_BY_10: u8 = 0x21;

	// sw resets
	const FLAG_RESET: u8 = 0x30;
	const INITIAL_RESET: u8 = 0x3F;

	// frame rates
	const FPS_10: u8 = 0x00;
	const FPS_1: u8 = 0x01;

	// int enables
	const INT_DISABLED: u8 = 0x00;
	const INT_ENABLED: u8 = 0x01;

	// int modes
	const DIFFERENCE: u8 = 0x00;
	const ABSOLUTE_VALUE: u8 = 0x01;

	const PIXEL_OFFSET: u8 = 0x80;

	const PIXEL_ARRAY_WIDTH: u8 = 8;
	const PIXEL_ARRAY_HEIGHT: u8 = 8;
	const PIXEL_TEMP_CONVERSION: f32 = 0.25;
	const THERMISTOR_CONVERSION: f32 = 0.0625;

	pub const SLAVE_ADDR_PRIMARY: u16 = 0x69;
	pub const SLAVE_ADDR_ALT: u16 = 0x68;

	#[derive(Debug)]
	pub struct AMG88XX<T: I2CDevice + Sized> {
		pub i2cdev: T,
	}

	#[derive(Debug)]
	pub struct AMG88XXMatrix {
		data: Vec<f64>,
	}

	impl<T> AMG88XX<T> where T: I2CDevice + Sized {
		pub fn new(mut i2cdev: T) -> Result<AMG88XX<T>, T::Error> {
			//
			i2cdev.smbus_write_block_data(0x00, &[NORMAL_MODE])?;
			i2cdev.smbus_write_block_data(0x01, &[INITIAL_RESET])?;
			i2cdev.smbus_write_byte_data(0x03, INT_DISABLED)?;
			i2cdev.smbus_write_byte_data(0x02, FPS_10)?;

			Ok(AMG88XX{
				i2cdev
			})
		}

		pub fn temperature(&mut self) -> Result<f32, T::Error> {
			let high = self.i2cdev.smbus_read_byte_data(0x0F)?;
			let low: u8 = self.i2cdev.smbus_read_byte_data(0x0E)?;
			let raw: u16 = (high as u16) << 8 | low as u16;
			Ok((raw as f32) * THERMISTOR_CONVERSION)
		}

		pub fn pixels(&mut self) -> Result<Vec<Vec<f32>>, T::Error> {
			const ARRAY_HEIGHT: usize = 8;
			const ARRAY_WIDTH: usize = 8;
			let mut pixels = vec![vec![0f32; ARRAY_HEIGHT]; ARRAY_WIDTH];
			for row in 0..ARRAY_HEIGHT {
				for col in 0..ARRAY_WIDTH {
					let i = row * ARRAY_HEIGHT + col;
					let addr = PIXEL_OFFSET + (i << 1) as u8;
					let temp = self.i2cdev.smbus_read_word_data(addr)?;
					pixels[row][col] = temp as f32 * PIXEL_TEMP_CONVERSION;
				}
			}
			Ok(pixels)
		}
	}
}
