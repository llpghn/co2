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
    Temp_xlsb = 0xFC,
    Temp_lsb = 0x88,
    Temp_msb = 0x89,
  }

  const ADDR_BME280: u16 = 0x76;

  // Helper functions
  fn bcd2dec(bcd: u8) -> u8 {
    (((bcd & 0xF0) >> 4) * 10) + (bcd & 0x0F)
  }

  fn dec2bcd(dec: u8) -> u8 {
    ((dec / 10) << 4) | (dec % 10)
  }

  /*
  var2 = (((((adc_T >> 4) - (dig_T1)) * ((adc_T >> 4) - (dig_T1)))>> 12) * (dig_T3))>>14;
  */


  pub fn read_temp_register() -> Result< u8 , Box<dyn Error>> {
    println!("--- Try to read the Temp ---");
    let mut i2c = I2c::new()?;
    let temp_msb: [u8; 1] = [0xFA];
    let temp_lsb: [u8; 1] = [0xFB];
    let temp_xlsb: [u8; 1] = [0xFC];
    i2c.set_slave_address(ADDR_BME280)?;
    
    let written_bytes: usize = i2c.write(&temp_msb)?;
    println!("Total Bytes send: {}", written_bytes);
    let mut res_temp_msb : [u8; 1] = [0];
    let read_bytes: usize = i2c.read(&mut res_temp_msb)?;

    let written_bytes: usize = i2c.write(&temp_lsb)?;
    println!("Total Bytes send: {}", written_bytes);
    let mut res_temp_lsb : [u8; 1] = [0];
    let read_bytes: usize = i2c.read(&mut res_temp_lsb)?;

    let written_bytes: usize = i2c.write(&temp_xlsb)?;
    println!("Total Bytes send: {}", written_bytes);
    let mut res_temp_xlsb : [u8; 1] = [0];
    let read_bytes: usize = i2c.read(&mut res_temp_xlsb)?;

    let my_temp: i32 =  (res_temp_msb[0] as i32 << 12) | (res_temp_lsb[0] as i32 << 4) | (res_temp_xlsb[0] >> 4);

    println!("Read {} bytes: {}", read_bytes, my_temp);

    Ok(())
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