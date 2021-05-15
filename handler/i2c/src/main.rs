
//use paho_mqtt as mqtt;

mod lib_mqtt;       // Handler für MQTT Kommunikation
mod bme280;         // Struct for handling Kommunikation with BME280-Sensor

/////////////////////////////////////////////////////////////////////////////

enum sms{
    collectData,
    sendData,
    //sleep,
}

struct CollectedData {
    temperature: f32,
    humidity: f32, 
    pressure: f32,
}

impl CollectedData {
    fn get_temperature_message(&self, top_mc: &mut String, message: &mut String) {
        let mut my_topic = String::from("/sensor/value/temperature");
        *topic = my_topic;
        let mut my_message = self.temperature.to_string();
        *message = my_message;
    }
    fn get_humidity_message(&self, topic: &mut String, message: &mut String) {
        let mut my_topic = String::from("/sensor/value/humidity");
        *topic = my_topic;
        let mut my_message = self.humidity.to_string();
        *message = my_message;
    }
    fn get_pressure_message(&self, topic: &mut String, message: &mut String) {
        let mut my_topic = String::from("/sensor/value/pressure");
        *topic = my_topic;
        let mut my_message = self.pressure.to_string();
        *message = my_message;
    }
}


fn main() {
    //let args: Vec<String> = env::args().collect();                              // get parameter from CLI

    env_logger::init();                                                         // Initialize the logger from the env
    // Initialize 
    let state = sms::collectData;
    let cli = lib_mqtt::lib_mqtt::connect_to_broker();
    let mut to_send = CollectedData{
        temperature: 0.0,
        humidity: 0.0, 
        pressure: 0.0,
    };

    
    //while(1){
    /*
        match state{
            sms::collectData => {
                let measurements = bme280::bme280::get();
                to_send.temperature = measurements.temperature;
                to_send.humidity = measurements.humidity;
                to_send.pressure = measurements.pressure;
                state == sms::sendData;
            }
            sms::sendDate => {

            }
        }
        */
            
    //}
    


    //while(1){}

    // Load Data from Sensor
    let measurements = bme280::bme280::get();
    to_send.temperature = measurements.temperature;
    to_send.humidity = measurements.humidity;
    to_send.pressure = measurements.pressure;
    
    let mut topic = String::from("Not set");
    let mut message = String::from("Not set");
    to_send.get_temperature_message(&mut topic, &mut message);
    
    lib_mqtt::lib_mqtt::send_message(&cli, &topic, &message);


    //println!("- Topic: {}", topic);
    //println!("- Message: {}", message);
    println!("Measurement for temperature: {}", measurements.temperature);
    //
    //lib_mqtt::lib_mqtt::send_msg_temp(&cli);
    // Disconnect from the broker
    //cli.disconnect(None).unwrap();

    



}