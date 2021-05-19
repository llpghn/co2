//pub mod bme280{

  use linux_embedded_hal::{Delay, I2cdev};
  use bme280::BME280;

  pub struct Data{
    pub temperature: f32,
    pub humidity: f32,
    pub pressure: f32,
  }

  pub fn loadData() -> Data{
    let i2c_bus = I2cdev::new("/dev/i2c-1").unwrap();
    // initialize the BME280 using the primary I2C address 0x76
    let mut bme280 = BME280::new_primary(i2c_bus, Delay);
    
    bme280.init().unwrap();
    
    // measure temperature, pressure, and humidity
    let measurements = bme280.measure().unwrap();
    
    let d = Data{
      temperature : measurements.temperature,
      humidity : measurements.humidity,
      pressure : measurements.pressure,
    };
    d
  }
//}