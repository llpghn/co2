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

/*
bcd2dec
*/
  #[repr(u8)]
  enum Register{
    Hum_lsb = 0xFE,
    Hum_msb = 0xFD,
    Temp_xlsb = 0xFC,
    Temp_lsb = 0x88,
    Temp_msb = 0x89,
    Press_xlsb = 0xF9,
    Press_lsb = 0xF8,
    Press_msb = 0xF7,
  }

  const ADDR_BME280: u16 = 0x76;

  // Helper functions
  fn bcd2dec(bcd: u8) -> u8 {
    (((bcd & 0xF0) >> 4) * 10) + (bcd & 0x0F)
  }

  fn dec2bcd(dec: u8) -> u8 {
    ((dec / 10) << 4) | (dec % 10)
  }

  fn read_register(mut connection: I2c, addr: u16) -> Result< u8 , Box<dyn Error>> {
    connection.set_slave_address(addr);
    
    Ok(5)
  } 

  pub fn scan() -> Result<(), Box<dyn Error>> {
    println!("Start scanning I2C-Bus - 101");
    let mut i2c = I2c::new()?;
    i2c.set_slave_address(ADDR_BME280)?;
    
    //let command: [u8; 1] = [0xF7];
    //let command: [u8; 1] = [Register::Temp_lsb as u8];
    let command: [u8; 1] = [0xD0];
    let written_bytes: usize = i2c.write(&command)?;
    println!("Total Bytes send: {}", written_bytes);

    let mut reg : [u8; 1] = [0];
    let read_bytes: usize = i2c.read(&mut reg)?;
    println!("Read {} bytes: {}", read_bytes, bcd2dec(reg[0]));
    Ok(())
  }
}