use std::env;
use kafka::producer::{Producer, Record};

fn main() {
    let args: Vec<String> = env::args().collect();

    let hosts = vec!["localhost:29092".to_owned()];

    let mut producer =
      Producer::from_hosts(hosts)
        .create()
        .unwrap();
   
    for i in 0..10 {
      let buf = format!("{i}");
      producer.send(&Record::from_value(&args[1], buf.as_bytes())).unwrap();
      println!("Sent: {i}");
    }
}