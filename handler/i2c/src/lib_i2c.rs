/*
 * Handler for handling i2c devices connecten to the device.
 */

// Connected BME280: https://www.bosch-sensortec.com/media/boschsensortec/downloads/datasheets/bst-bme280-ds002.pdf
// Slave address: 111011X (X = 0)
//    dann mit X=0 Adressieren wir die 0x76 als Adresse.

mod lib_i2c{
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


  pub fn scan(){
    println!("Start scanning I2C-Bus");
    let i2c = I2c::new()?;
    i2c.set_slave_address(ADDR_BME280)?;
    let command: u8 = 0x76;
    let mut readBud: [u8, 3];
    i2c.block_write(command, &readBud)?;
    println!("{}-{}-{}", readBud[0], readBud[1], readBud[2]);
  }
}