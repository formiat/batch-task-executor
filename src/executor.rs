use std::fmt::Debug;

const THRESHOLD: usize = 3;

fn consuming_chunks<T>(input: Vec<T>, chunk_size: usize) -> Vec<Vec<T>> {
    let mut res = Vec::with_capacity((input.len() / chunk_size) + 1);
    res.push(Vec::with_capacity(chunk_size));

    for item in input {
        if res.last().unwrap().len() >= chunk_size {
            res.push(Vec::with_capacity(chunk_size));
        }

        res.last_mut().unwrap().push(item);
    }

    res
}

pub async fn batch_executor<T, U, F>(input: Vec<T>, func: F) -> Vec<U>
where
    T: Debug + Send + 'static,
    U: Debug + Send + 'static,
    F: Fn(T) -> U + Copy + Send + 'static,
{
    if input.len() > THRESHOLD {
        let inputs = consuming_chunks(input, THRESHOLD);
        println!("inputs: {:#?}", inputs);

        let futures: Vec<_> = inputs
            .into_iter()
            .map(|input| tokio::spawn(batch_executor_inner(input, func)))
            .collect();
        println!("futures count: {}", futures.len());

        let res = futures::future::join_all(futures).await;
        println!("res: {:#?}", res);

        res.into_iter().flat_map(Result::unwrap).collect()
    } else {
        batch_executor_inner(input, func).await
    }
}

async fn batch_executor_inner<T, U, F>(input: Vec<T>, func: F) -> Vec<U>
where
    F: Fn(T) -> U,
{
    input.into_iter().map(func).collect()
}
