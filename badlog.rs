extern crate chrono;
use chrono::Utc;
use std::io::Write;
use std::fs::OpenOptions;
use std::{thread, time};

fn main() {
    let file_path = "/var/log/badrust.log";
    let mut file = OpenOptions::new()
      .write(true)
      .append(true)
      .open(file_path)
      .unwrap();

    let one_second = time::Duration::from_millis(1000);

    loop {
        let dt = Utc::now().naive_utc();
        let timestamp: i64 = dt.timestamp();
        let message = timestamp.to_string() + " this is a log file entry\n";

        file.write_all(message.as_bytes());
        thread::sleep(one_second);
    }
}