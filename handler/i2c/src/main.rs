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

use std::error::Error;
use std::thread;
use std::time::Duration;

use rppal::i2c::I2c;

/////////////////////////////////////////////////////////////////////////////

const SENSOR_PUB_TEMPERATURE: &str = "/sensor/value/temperature";
const ADDR_DS3231: u16 = 0x76;

fn bcd2dec(bcd: u8) -> u8 {
    (((bcd & 0xF0) >> 4) * 10) + (bcd & 0x0F)
}

fn dec2bcd(dec: u8) -> u8 {
    ((dec / 10) << 4) | (dec % 10)
}


fn main() {
    //let args: Vec<String> = env::args().collect();                              // get parameter from CLI

    env_logger::init();                                                         // Initialize the logger from the env
    
    //let cli = lib_mqtt::lib_mqtt::connect_to_broker();
    //lib_mqtt::lib_mqtt::send_msg_temp(&cli);
    // Disconnect from the broker
    //cli.disconnect(None).unwrap();

    let mut i2c = I2c::new();

    // Set the I2C slave address to the device we're communicating with.
    i2c.set_slave_address(ADDR_DS3231);
    let command: u8 = 0x76;
    let mut readBud: [u8; 3];
    i2c.block_write(command, &readBud);
    println!("{}-{}-{}", readBud[0], readBud[1], readBud[2]);

}