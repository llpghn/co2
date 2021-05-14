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
mod lib_i2c;
/////////////////////////////////////////////////////////////////////////////

const SENSOR_PUB_TEMPERATURE: &str = "/sensor/value/temperature";



fn main() {
    //let args: Vec<String> = env::args().collect();                              // get parameter from CLI

    env_logger::init();                                                         // Initialize the logger from the env
    
    //let cli = lib_mqtt::lib_mqtt::connect_to_broker();
    //lib_mqtt::lib_mqtt::send_msg_temp(&cli);
    // Disconnect from the broker
    //cli.disconnect(None).unwrap();

    lib_i2c::lib_i2c::scan();

}