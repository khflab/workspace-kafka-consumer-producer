use std::env;
use kafka::consumer::{Consumer, FetchOffset, GroupOffsetStorage};

fn main() {
    let args: Vec<String> = env::args().collect();

    let mut consumer =
        Consumer::from_hosts(vec!("localhost:29092".to_owned()))
            .with_topic(args[1].to_owned())
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