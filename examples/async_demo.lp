# LPLang — Async Demo

use std::time::{sleep, Duration}

async fn task(id: u32, delay_ms: u64) {
    print("Task {id} starting")
    sleep(Duration::from_millis(delay_ms)).await
    print("Task {id} done")
}

async fn main() {
    nursery! {
        spawn task(1, 100);
        spawn task(2, 200);
        spawn task(3, 50);
    }
    print("All tasks complete")
}