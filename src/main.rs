use chrono::{DateTime,Utc};
use std::time::SystemTime;
use std::env;
use std::io::Write;
use std::fs::OpenOptions;
use std::{thread, time};
use rand::{self,Rng};

fn main() {

    let verbose_logs = env::var("RUST_LOG");
    let should_log = verbose_logs.unwrap() == "1";
    let file_path = "/var/log/badrust.log";
    let mut file = OpenOptions::new()
      .write(true)
      .append(true)
      .open(file_path)
      .unwrap();

    let one_second = time::Duration::from_millis(1000);

    loop {
        let now = SystemTime::now();
        let now: DateTime<Utc> = now.into();
        let timestamp = now.to_rfc3339();
        let mut rng = rand::thread_rng();
        let token_value:i64 = rng.gen_range(1111111111..9999999999);
        let message = timestamp.to_string() + " token: " + &token_value.to_string() + "\n";

        if should_log {
          file.write_all(message.as_bytes());
        }
        thread::sleep(one_second);
    }
}
