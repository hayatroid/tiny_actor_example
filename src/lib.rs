use tokio::{
    sync::{mpsc, oneshot},
    time::{Duration, sleep},
};

type AddRequest = (i32, i32);
type AddResponse = i32;
type AddEnvelope = (AddRequest, oneshot::Sender<AddResponse>);

struct AddActor {
    envelope_rx: mpsc::UnboundedReceiver<AddEnvelope>,
}

impl AddActor {
    async fn run(&mut self) {
        while let Some((req, res_tx)) = self.envelope_rx.recv().await {
            let res = self.handle(req).await;
            let _ = res_tx.send(res);
        }
    }
    async fn handle(&self, req: AddRequest) -> AddResponse {
        sleep(Duration::from_secs(1)).await;
        req.0 + req.1
    }
}

#[derive(Clone)]
pub struct AddActorRef {
    envelope_tx: mpsc::UnboundedSender<AddEnvelope>,
}

impl AddActorRef {
    pub fn send(&self, req: AddRequest) -> oneshot::Receiver<AddResponse> {
        let (res_tx, res_rx) = oneshot::channel();
        let _ = self.envelope_tx.send((req, res_tx));
        res_rx
    }
}

pub fn spawn_add_actor() -> AddActorRef {
    let (envelope_tx, envelope_rx) = mpsc::unbounded_channel();
    tokio::spawn(async { AddActor { envelope_rx }.run().await });
    AddActorRef { envelope_tx }
}
