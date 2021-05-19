/*
 * Haupt-Bibliotheks-Datei
 * 
 * Enthält die `mainloop` für die State-Machine und führt diese aus.
*/

//
//use crate::lib_mqtt;       // Handler für MQTT Kommunikation
//use crate::lib_bme280;         // Struct for handling Kommunikation with BME280-Sensor

//use linux_embedded_hal::{Delay, I2cdev};
//use bme280::BME280;

use std::{thread, time};

/* - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - */

mod sens_bme{
    use linux_embedded_hal::{Delay, I2cdev};
    use bme280::BME280;

    pub struct Data{
        pub temperature: f32,
        pub humidity: f32,
        pub pressure: f32,
    }

    pub fn load_data() -> Data{
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
}

/* - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - */

mod lib_mqtt{

  use std::{process, time::Duration}; 
  pub use paho_mqtt as mqtt;


  //let cli = lib_mqtt::lib_mqtt::connect_to_broker();
  //lib_mqtt::lib_mqtt::send_msg_temp(&cli);
  // Disconnect from the broker
  //cli.disconnect(None).unwrap();

  /// Connects to the MQTT-Broker installes locally on the system.
  /// # Returns
  /// * mqtt::client from paho_mqtt Library
  pub fn connect_to_broker() -> mqtt::Client{
    // Create a client & define connect options

    let host = "tcp://localhost:1883".to_string();                              // Connect to local broker.
    let mut client = mqtt::Client::new(host).unwrap_or_else(|e| {
      println!("Error creating the client: {:?}", e);
      process::exit(1);
    });

    // Use 5sec timeouts for sync calls.
    client.set_timeout(Duration::from_secs(5));

    // Connect and wait for it to complete or fail
    if let Err(e) = client.connect(None) {
      println!("Unable to connect: {:?}", e);
      process::exit(1);
    }
    client
  }

  pub fn send_message(client: &mqtt::Client, topic: &String, message: &String ){
    //println!("{}-{}", topic, payload);
    let msg = mqtt::MessageBuilder::new()
      .topic(topic)
      .payload(message.as_bytes().to_vec())
      .qos(1)
      .finalize();
    if let Err(e) = client.publish(msg) {
      println!("Error sending message: {:?}", e);
    }
  }

}

/* - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - */

enum Sms{
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
  println!("Entering Main-Loop");
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
          Sms::CollectData => {
              let measurements = sens_bme::load_data();  
              //let measurements = bme280::bme280::get();
              to_send.temperature = measurements.temperature;
              to_send.humidity = measurements.humidity;
              to_send.pressure = measurements.pressure;
              state = Sms::SendData;
          }
          Sms::SendData => {
              let mut topic = String::from("Not set");
              let mut message = String::from("Not set");
              to_send.get_temperature_message(&mut topic, &mut message);
              lib_mqtt::send_message(&cli, &topic, &message);
              to_send.get_humidity_message(&mut topic, &mut message);
              lib_mqtt::send_message(&cli, &topic, &message);
              to_send.get_pressure_message(&mut topic, &mut message);
              lib_mqtt::send_message(&cli, &topic, &message);
              state = Sms::Sleeping;
          }
          Sms::Sleeping => {
              let one_second = time::Duration::from_millis(10000);
              thread::sleep(one_second);
              state = Sms::CollectData;
          }
      }
      
          
  }
  //cli.disconnect(None).unwrap();
}