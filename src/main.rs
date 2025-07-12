use std::time::Duration;
use tokio::time::sleep;

#[tokio::main]
async fn main() {
    let mut task_list = Vec::new();
    for i in 0..13 {
        task_list.push(
            tokio::spawn(async move {
                sleep(Duration::from_secs(i)).await;
                println!("Task {} done", i);
            })
        )
    };
    for x in task_list {
        let _ = x.await;
    }
}