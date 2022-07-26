use std::fmt::Debug;

const THRESHOLD: usize = 3;

/// TODO: Remove `Clone` requirement for `T`
pub async fn batch_executor<T, U, F>(input: Vec<T>, func: F) -> Vec<U>
where
    T: Clone + Debug,
    U: Debug,
    F: Fn(T) -> U,
{
    if input.len() > THRESHOLD {
        // TODO: Replace with consuming `chunks`. Remove `v.to_vec()` call
        let inputs: Vec<Vec<_>> = input.chunks(THRESHOLD).map(|v| v.to_vec()).collect();
        println!("inputs: {:#?}", inputs);

        let mut futures = Vec::new();
        for input in inputs {
            let future = batch_executor_inner(input, &func);

            futures.push(future);
        }
        println!("futures count: {}", futures.len());
        let res = futures::future::join_all(futures).await;
        println!("res: {:#?}", res);

        res.into_iter().flatten().collect()
    } else {
        batch_executor_inner(input, func).await
    }
}

async fn batch_executor_inner<T, U, F>(input: Vec<T>, func: F) -> Vec<U>
where
    F: Fn(T) -> U,
{
    let mut results = Vec::new();

    for item in input {
        let res = func(item);

        results.push(res);
    }

    results
}
