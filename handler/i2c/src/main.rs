
//use paho_mqtt as mqtt;

//mod lib_mqtt;     // Handler für MQTT Kommunikation
mod bme280;         // Struct for handling Kommunikation with BME280-Sensor

/////////////////////////////////////////////////////////////////////////////



fn main() {
    //let args: Vec<String> = env::args().collect();                              // get parameter from CLI

    env_logger::init();                                                         // Initialize the logger from the env
    let sens = bme280::BME280::new("/dev/i2c-1");
    sens::loadValues();
    println!("Temperature: {}", send.temperature);
        
    //let cli = lib_mqtt::lib_mqtt::connect_to_broker();
    //lib_mqtt::lib_mqtt::send_msg_temp(&cli);
    // Disconnect from the broker
    //cli.disconnect(None).unwrap();

    



}