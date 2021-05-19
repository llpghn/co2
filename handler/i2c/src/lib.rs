/*
 * Haupt-Bibliotheks-Datei
 * 
 * Enthält die `mainloop` für die State-Machine und führt diese aus.
*/

//
mod lib_mqtt;       // Handler für MQTT Kommunikation
mod lib_bme280;         // Struct for handling Kommunikation with BME280-Sensor
use std::{thread, time};

enum sms{
  CollectData,
  SendData,
  Sleeping,
}

struct CollectedData {
  temperature: f32,
  humidity: f32, 
  pressure: f32,
}

impl CollectedData {
  fn get_temperature_message(&self, topic: &mut String, message: &mut String) {
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

pub fn mainloop(){
  env_logger::init();                                                         // Initialize the logger from the env
  // Initialize 
  let mut state = sms::CollectData;
  let cli = lib_mqtt::connect_to_broker();
  let mut to_send = CollectedData{
      temperature: 0.0,
      humidity: 0.0, 
      pressure: 0.0,
  };

  loop {
      match state{
          sms::CollectData => {
            let measurements = bme280::get();  
            //let measurements = bme280::bme280::get();
              to_send.temperature = measurements.temperature;
              to_send.humidity = measurements.humidity;
              to_send.pressure = measurements.pressure;
              state = sms::SendData;
          }
          sms::SendData => {
              let mut topic = String::from("Not set");
              let mut message = String::from("Not set");
              to_send.get_temperature_message(&mut topic, &mut message);
              lib_mqtt::send_message(&cli, &topic, &message);
              to_send.get_humidity_message(&mut topic, &mut message);
              lib_mqtt::send_message(&cli, &topic, &message);
              to_send.get_pressure_message(&mut topic, &mut message);
              lib_mqtt::send_message(&cli, &topic, &message);
              state = sms::Sleeping;
          }
          sms::Sleeping => {
              let one_second = time::Duration::from_millis(10000);
              thread::sleep(one_second);
              state = sms::CollectData;
          }
      }
      
          
  }
  cli.disconnect(None).unwrap();
}