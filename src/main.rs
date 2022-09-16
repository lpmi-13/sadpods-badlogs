use chrono::Utc;
use std::io::Write;
use std::fs::OpenOptions;
use std::{thread, time};
use rand::{self,Rng};

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
        let mut rng = rand::thread_rng();
        let token_value = rng.gen_range(100000..999999);
        let message = timestamp.to_string() + " token: " + &token_value.to_string() + "\n";

        file.write_all(message.as_bytes());
        thread::sleep(one_second);
    }
}
