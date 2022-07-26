use futures::future::{BoxFuture, FutureExt};
use std::fmt::Debug;

const THRESHOLD: usize = 3;

/// TODO: Remove `Clone` requirement for `T`
pub fn batch_executor<'a, T, U, F>(input: Vec<T>, func: F) -> BoxFuture<'a, Vec<U>>
where
    T: Send + Clone + Debug + 'a,
    U: Send + Debug + 'a,
    F: Send + Copy + Fn(T) -> U + 'a,
{
    async move {
        if input.len() > THRESHOLD {
            // TODO: Replace with consuming `chunks`. Remove `v.to_vec()` call
            let inputs: Vec<Vec<_>> = input.chunks(THRESHOLD).map(|v| v.to_vec()).collect();
            println!("inputs: {:#?}", inputs);

            let mut futures = Vec::new();
            for input in inputs {
                let future = batch_executor(input, func);

                futures.push(future);
            }
            let res = futures::future::join_all(futures).await;
            println!("res: {:#?}", res);

            res.into_iter().flatten().collect()
        } else {
            let mut results = Vec::new();
            for item in input {
                let res = func(item);

                results.push(res);
            }

            results
        }
    }
    .boxed()
}
