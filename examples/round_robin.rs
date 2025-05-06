use tiny_actor_example::{AddActorRef, spawn_add_actor};

#[tokio::main]
async fn main() {
    let actors: Vec<AddActorRef> = (0..5).map(|_| spawn_add_actor()).collect();
    let mut tasks = Vec::new();
    let mut index = 0;
    for x in 1..=10 {
        for y in 1..=10 {
            let rx = actors[index].send((x, y));
            tasks.push((x, y, rx));
            index = (index + 1) % 5;
        }
    }
    for (x, y, rx) in tasks {
        println!("{} + {} = {}", x, y, rx.await.unwrap());
    }
}
