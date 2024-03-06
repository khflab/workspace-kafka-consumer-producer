use std::env;
use kafka::consumer::{Consumer, FetchOffset, GroupOffsetStorage};

fn main() {
  let args: Vec<String> = env::args().skip(1).collect();

  if args.len() == 0 { 
    eprintln!("- Informe um tópico");
    std::process::exit(1);
  }

  let mut consumer =
    Consumer::from_hosts(vec!("localhost:29092".to_owned()))
      .with_topic(args[0].to_owned())
      .with_fallback_offset(FetchOffset::Latest)
      .with_group("salamandra-rust-dev-1".to_owned())
      .with_offset_storage(Some(GroupOffsetStorage::Kafka))
      .create()
      .unwrap();

  loop {
    for ms in consumer.poll().unwrap().iter() {
      for m in ms.messages() {
        println!("{:?}", m);
      }
      consumer.consume_messageset(ms);
    }
    consumer.commit_consumed().unwrap();
  }
}