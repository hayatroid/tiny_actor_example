use tiny_actor_example::spawn_add_actor;

#[tokio::main]
async fn main() {
    let add_actor_refs: Vec<_> = (0..5).map(|_| spawn_add_actor()).collect();
    let mut futures = vec![];
    let mut index = 0;
    for i in 0..10 {
        for j in 0..10 {
            futures.push((i, j, add_actor_refs[index].send((i, j))));
            index = (index + 1) % 5;
        }
    }
    for (i, j, future) in futures {
        println!("{} + {} = {}", i, j, future.await);
    }
}
