use tiny_actor_example::spawn_add_actor;

#[tokio::main]
async fn main() {
    let add_actors = (0..5).map(|_| spawn_add_actor()).collect::<Vec<_>>();
    let mut jobs = vec![];
    let mut target = 0;
    for x in 1..=10 {
        for y in 1..=10 {
            let req = (x, y);
            let res_rx = add_actors[target].send(req);
            jobs.push((req, res_rx));
            target = (target + 1) % 5;
        }
    }
    for (req, res_rx) in jobs {
        println!("{} + {} = {}", req.0, req.1, res_rx.await.unwrap());
    }
}
