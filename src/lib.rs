pub mod amg88xx {
	use i2cdev::core::I2CDevice;
	use iota::iota;

	/// AMG88XX control register address
	iota! {
	 pub const REG_PCTL: u8 = iota; // 0x00
	     , REG_RST              // 0x01
	     , REG_FPSC
	     , REG_INTC
	     , REG_STAT
	     , REG_SCLR             // 0x05
	     , REG__RESERVED_       // 0x06 reserved
	     , REG_AVE              // 0x07
	     , REG_INTHL            // 0x08
	     , REG_INTHH            // 0x09
	     , REG_INTLL            // 0x0A
	     , REG_INTLH            // 0x0B
	     , REG_IHYSL            // 0x0C
	     , REG_IHYSH            // 0x0D
	     , REG_TTHL             // 0x0E
	     , REG_TTHH             // 0x0F
	}

	#[cfg(test)]
	mod tests {
	    use super::*;

	    #[test]
	    fn test_iota() {
	        assert_eq!(REG_PCTL, 0);
		    assert_eq!(REG_RST, 1);
		    assert_eq!(REG_FPSC, 2);
		    assert_eq!(REG_INTC, 3);
		    assert_eq!(REG_STAT, 4);
		    assert_eq!(REG_SCLR, 5);
		    assert_eq!(REG_AVE, 7);
	    }
	}

	/// data offset register address
	const REG_INT_OFFSET: u8 = 0x10;
	const REG_PIXEL_OFFSET: u8 = 0x80;

	/// amg88xx i2c address
	pub const SLAVE_ADDR_PRIMARY: u16 = 0x69;
	pub const SLAVE_ADDR_ALT: u16 = 0x68;

	/// Operation Modes
	pub const PCTL_NORMAL_MODE: u8 = 0x00;
	pub const PCTL_SLEEP_MODE: u8 = 0x10;
	pub const PCTL_STAND_BY_60: u8 = 0x20;
	pub const PCTL_STAND_BY_10: u8 = 0x21;

	/// sw resets
	pub const RST_FLAG_RESET: u8 = 0x30;
	pub const RST_INITIAL_RESET: u8 = 0x3F;

	/// frame rates
	pub const FPS_10: u8 = 0x00;
	pub const FPS_1: u8 = 0x01;

	/// interrupt enables
	pub const INT_DISABLED: u8 = 0x00;
	pub const INT_ENABLED: u8 = 0x01;

	/// int modes
	pub const DIFFERENCE: u8 = 0x00;
	pub const ABSOLUTE_VALUE: u8 = 0x01;

	/// pixels length
	const PIXEL_ARRAY_WIDTH: usize = 8;
	const PIXEL_ARRAY_HEIGHT: usize = 8;

	/// temperature data conversion
	const PIXEL_TEMP_CONVERSION: f32 = 0.25;
	const THERMISTOR_CONVERSION: f32 = 0.0625;


	#[derive(Debug)]
	pub struct AMG88XX<T: I2CDevice + Sized> {
		pub i2cdev: T,
	}

	impl<T> AMG88XX<T> where T: I2CDevice + Sized {
		/// Create an amg88xx sensor with path and address set from i2cdev
		/// amg88xx is the to normal mode, without interrupt and fps is set to 10
		pub fn new(mut i2cdev: T) -> Result<AMG88XX<T>, T::Error> {
			i2cdev.smbus_write_block_data(REG_PCTL, &[PCTL_NORMAL_MODE])?;
			i2cdev.smbus_write_block_data(REG_RST, &[RST_INITIAL_RESET])?;
			i2cdev.smbus_write_byte_data(REG_INTC, INT_DISABLED)?;
			i2cdev.smbus_write_byte_data(REG_FPSC, FPS_10)?;

			Ok(AMG88XX{
				i2cdev
			})
		}

		/// Get temperature from amg88xx
		pub fn temperature(&mut self) -> Result<f32, T::Error> {
			let high = self.i2cdev.smbus_read_byte_data(REG_TTHH)?;
			let low: u8 = self.i2cdev.smbus_read_byte_data(REG_TTHL)?;
			let raw: u16 = (high as u16) << 8 | low as u16;
			Ok((raw as f32) * THERMISTOR_CONVERSION)
		}

		/// Gain 8*8 pixels temperature data from amg88xx
		pub fn pixels(&mut self) -> Result<Vec<Vec<f32>>, T::Error> {
			let mut pixels = vec![vec![0f32; PIXEL_ARRAY_WIDTH]; PIXEL_ARRAY_HEIGHT];
			for row in 0..PIXEL_ARRAY_HEIGHT {
				for col in 0..PIXEL_ARRAY_WIDTH {
					let i = row * PIXEL_ARRAY_HEIGHT + col;
					let addr = REG_PIXEL_OFFSET + (i << 1) as u8;
					let temp = self.i2cdev.smbus_read_word_data(addr)?;
					pixels[row][col] = temp as f32 * PIXEL_TEMP_CONVERSION;
				}
			}
			Ok(pixels)
		}
	}
}
