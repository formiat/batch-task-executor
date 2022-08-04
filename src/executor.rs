use std::fmt::Debug;

const THRESHOLD: usize = 3;

/// TODO: Remove `Clone` requirement for `T`
pub async fn batch_executor<T, U, F>(input: Vec<T>, func: F) -> Vec<U>
where
    T: Clone + Debug + Send + 'static,
    U: Debug + Send + 'static,
    F: Fn(T) -> U + Copy + Send + 'static,
{
    if input.len() > THRESHOLD {
        // TODO: Replace with consuming `chunks`. Remove `v.to_vec()` call
        let inputs: Vec<Vec<_>> = input.chunks(THRESHOLD).map(|v| v.to_vec()).collect();
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
