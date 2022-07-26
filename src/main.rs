use crate::executor::batch_executor;

mod executor;

#[tokio::main]
async fn main() {
    let input = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];

    let res = batch_executor(input, |v| v as f64 + 0.5).await;
    println!("res: {:#?}", res);
}
