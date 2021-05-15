// paho-mqtt/examples/sync_publish.rs
// This is a Paho MQTT Rust client sample application.
//
//! This application is a simple MQTT publisher using the
//! synchronous/blocking client interface.
//!
//! The sample demonstrates:
//!   - Use of the synchronous/blocking API
//!   - Connecting to an MQTT broker
//!   - Publishing messages
//!

/*******************************************************************************
 * Copyright (c) 2017 Frank Pagliughi <fpagliughi@mindspring.com>
 *
 * All rights reserved. This program and the accompanying materials
 * are made available under the terms of the Eclipse Public License v1.0
 * and Eclipse Distribution License v1.0 which accompany this distribution.
 *
 * The Eclipse Public License is available at
 *    http://www.eclipse.org/legal/epl-v10.html
 * and the Eclipse Distribution License is available at
 *   http://www.eclipse.org/org/documents/edl-v10.php.
 *
 * Contributors:
 *    Frank Pagliughi - initial implementation and documentation
 *******************************************************************************/

//use paho_mqtt as mqtt;


//mod lib_mqtt;
//mod lib_i2c;

use linux_embedded_hal::{Delay, I2cdev};
use bme280::BME280;
/////////////////////////////////////////////////////////////////////////////



fn main() {
    //let args: Vec<String> = env::args().collect();                              // get parameter from CLI

    env_logger::init();                                                         // Initialize the logger from the env
    
    //let cli = lib_mqtt::lib_mqtt::connect_to_broker();
    //lib_mqtt::lib_mqtt::send_msg_temp(&cli);
    // Disconnect from the broker
    //cli.disconnect(None).unwrap();

    
    // using Linux I2C Bus #1 in this example
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
    
    println!("Relative Humidity = {}%", measurements.humidity);
    println!("Temperature = {} deg C", measurements.temperature);
    println!("Pressure = {} pascals", measurements.pressure);


}