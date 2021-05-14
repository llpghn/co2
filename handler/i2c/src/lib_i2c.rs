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

  struct bme260{
    addr: u16 = 0x76;
    hum_lsb : u8 = 0xFE;
    hum_msb : u8 = 0xFD;
    temp_xlsb : u8 = 0xFC;
    temp_lsb: u8 = 0xFB;
    temp_msb : u8 = 0xFA;
    press_xlsb : u8 = 0xF9;
    press_lsb : u8 = 0xF8;
    press_msb : u8 = 0xF7;
  }

  const ADDR_BME280: u16 = 0x76;

  // Helper functions
  fn bcd2dec(bcd: u8) -> u8 {
    (((bcd & 0xF0) >> 4) * 10) + (bcd & 0x0F)
  }

  fn dec2bcd(dec: u8) -> u8 {
    ((dec / 10) << 4) | (dec % 10)
  }


  pub fn scan() -> Result<(), Box<dyn Error>> {
    println!("Start scanning I2C-Bus - 101");
    let mut i2c = I2c::new()?;
    i2c.set_slave_address(bme260::addr)?;
    
    //let command: [u8; 1] = [0xF7];
    let command: [u8; 1] = [bme260::hum_msb];
    let written_bytes: usize = i2c.write(&command)?;
    println!("Total Bytes send: {}", written_bytes);

    let mut reg : [u8; 1] = [0];
    let read_bytes: usize = i2c.read(&mut reg)?;
    println!("Read {} bytes: {}", read_bytes, bcd2dec(reg[0]));
    Ok(())
  }
}