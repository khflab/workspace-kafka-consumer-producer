use std::env;
use kafka::producer::{Producer, Record};

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();

    if args.len() == 0 { 
      eprintln!("- Informe um tópico");
      std::process::exit(1);
    }

    let hosts = vec!["localhost:29092".to_owned()];

    let mut producer =
      Producer::from_hosts(hosts)
        .create()
        .unwrap();
   
    for i in 0..10 {
      let buf = format!("{i}");
      producer.send(&Record::from_value(&args[0], buf.as_bytes())).unwrap();
      println!("Sent: {i}");
    }
}