use futures::{StreamExt, stream::FuturesUnordered};
use tiny_actor_example::spawn_add_actor;

#[tokio::main]
async fn main() {
    let add_actor_refs: Vec<_> = (0..5).map(|_| spawn_add_actor()).collect();
    let mut futures = FuturesUnordered::new();
    let mut index = 0;
    for i in 0..10 {
        for j in 0..10 {
            let add_actor_ref = add_actor_refs[index].clone();
            futures.push(async move { (i, j, add_actor_ref.send((i, j)).await) });
            index = (index + 1) % 5;
        }
    }
    while let Some((i, j, k)) = futures.next().await {
        println!("{} + {} = {}", i, j, k);
    }
}
