use std::{thread, time};

pub(super)fn test_sleep() {
    sleep(1000);
}

fn sleep(millis: u64) {
    let ten_millis = time::Duration::from_millis(millis);
    let now = time::Instant::now();

    thread::sleep(ten_millis);

    assert!(now.elapsed() >= ten_millis);
}

