pub mod bme280{

  use linux_embedded_hal::{Delay, I2cdev};
  use bme280::BME280;

  pub struct Data{
    temperature: i32,
    humidity: i32,
    pressure: i32,
  }

  pub fn get() -> Data{
    let i2c_bus = I2cdev::new("/dev/i2c-1").unwrap();
    // initialize the BME280 using the primary I2C address 0x76
    let mut bme280 = BME280::new_primary(i2c_bus, Delay);
    
    // or, initialize the BME280 using the secondary I2C address 0x77
    // let mut bme280 = BME280::new_secondary(i2c_bus, Delay);
    
    // or, initialize the BME280 using a custom I2C address
    // let bme280_i2c_addr = 0x88;
    // let mut bme280 = BME280::new(i2c_bus, bme280_i2c_addr, Delay);
    
    // initialize the sensor
    bme280.init().unwrap();
    
    // measure temperature, pressure, and humidity
    let measurements = bme280.measure().unwrap();
    
    let d = Data{
      temperature : measurements.temperature,
      humidity : measurements.humidity,
      pressure : measurements.pressure,
    };
    d
    //println!("Relative Humidity = {}%", measurements.humidity);
    //println!("Temperature = {} deg C", measurements.temperature);
    //println!("Pressure = {} pascals", measurements.pressure);
  }
}