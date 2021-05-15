
pub mod lib_mqtt{

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

  pub fn send_message(client: &mqtt::Client, topic: &String, payload: &String ){
    //println!("{}-{}", topic, payload);
    let msg = mqtt::MessageBuilder::new()
      .topic(topic)
      .payload(payload)
      .qos(1)
      .finalize();
    if let Err(e) = client.publish(msg) {
      println!("Error sending message: {:?}", e);
    }
  }

  pub fn send_msg_temp(client: &mqtt::Client) {
    // Create a message and publish it
    let msg = mqtt::MessageBuilder::new()
      .topic("/sensor/value/temperature")
      .payload("Hello synchronous world!")
      .qos(1)
      .finalize();
    if let Err(e) = client.publish(msg) {
      println!("Error sending message: {:?}", e);
    }
  }

}
