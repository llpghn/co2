
//use paho_mqtt as mqtt;

mod lib_mqtt;       // Handler für MQTT Kommunikation
mod bme280;         // Struct for handling Kommunikation with BME280-Sensor

/////////////////////////////////////////////////////////////////////////////

enum sms{
    collectData,
    sendDate,
    sleep,
}

struct CollectedData {
    temperature: f32,
    humidity: f32, 
    pressure: f32,
    
}

impl CollectedData {
    fn getTemperatureMessage(&self, topic: &mut str, message: &mut str) {
        let mut my_topic = "/sensor/value/temperature";
        topic = my_topic;
        let mut my_message = self.temperature.to_string();
        message = my_message;
    }
}

fn main() {
    //let args: Vec<String> = env::args().collect();                              // get parameter from CLI

    env_logger::init();                                                         // Initialize the logger from the env
    // Initialize 
    let state = sms::collectData;
    let cli = lib_mqtt::lib_mqtt::connect_to_broker();
    let to_send = CollectedData{
        temperature: 0.0,
        humidity: 0.0, 
        pressure: 0.0,
    }

    /*
    while(1){
        if state == sms::collectData {
            let measurements = bme280::bme280::get();
            to_send.temperature = measurements.temperature;
            to_send.humidity = measurements.humidity;
            to_send.pressure = measurements.pressure;
        }
    }
    */


    //while(1){}

    // Load Data from Sensor
    let measurements = bme280::bme280::get();
    to_send.temperature = measurements.temperature;
    to_send.humidity = measurements.humidity;
    to_send.pressure = measurements.pressure;
    
    let mut topic: &str = "Not set";
    let mut message: &str = "Not set";
    to_send::getTemperatureMessage(&mut topic, &mut message);
    
    println!("- Topic: {}", topic);
    println!("- Message: {}", message);
    println!("Measurement for temperature: {}", measurements.temperature);
    //
    //lib_mqtt::lib_mqtt::send_msg_temp(&cli);
    // Disconnect from the broker
    //cli.disconnect(None).unwrap();

    



}