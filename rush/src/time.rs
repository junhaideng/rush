use std::ops::Add;

use chrono::{Duration, Local};

pub(crate) fn get_current_time() -> String {
    // let now = Local::now();
    let now = Local::now().add(Duration::days(1));
    now.format("%Y-%m-%d").to_string()
}

#[test]
fn test_get_current_time() {
    let t = get_current_time();
    println!("{}", t);
}
