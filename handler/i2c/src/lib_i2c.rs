/*
 * Handler for handling i2c devices connecten to the device.
 */

// Connected BME280: https://www.bosch-sensortec.com/media/boschsensortec/downloads/datasheets/bst-bme280-ds002.pdf
// Slave address: 111011X (X = 0)
//    dann mit X=0 Adressieren wir die 0x76 als Adresse.

pub mod lib_i2c{
  /// Scans the I2C-Bus for other connected devices.
  use rppal::i2c::I2c;
  use std::error::Error;

  const ADDR_BME280: u16 = 0x76;

  // Helper functions
  fn bcd2dec(bcd: u8) -> u8 {
    (((bcd & 0xF0) >> 4) * 10) + (bcd & 0x0F)
  }

  fn dec2bcd(dec: u8) -> u8 {
    ((dec / 10) << 4) | (dec % 10)
  }


  pub fn scan() -> Result<(), Box<dyn Error>> {
    println!("Start scanning I2C-Bus");
    let mut i2c = I2c::new()?;
    i2c.set_slave_address(ADDR_BME280)?;
    let command: [u8; 1] = [0x76];

    i2c.write(&command)?;
    println!("Have send the command!");

    let mut reg = [0u8; 2] = [0, 0];
    i2c.read(&mut reg)?;
    Ok(())
  }
}