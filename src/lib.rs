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
        while let Some(envelope) = self.envelope_rx.recv().await {
            let (request, response_tx) = envelope;
            let response = self.handle(request).await;
            response_tx.send(response).unwrap();
        }
    }
    async fn handle(&self, request: AddRequest) -> AddResponse {
        sleep(Duration::from_secs(1)).await;
        request.0 + request.1
    }
}

#[derive(Clone)]
pub struct AddActorRef {
    envelope_tx: mpsc::UnboundedSender<AddEnvelope>,
}

impl AddActorRef {
    pub fn send(&self, request: AddRequest) -> impl Future<Output = AddResponse> {
        let (response_tx, response_rx) = oneshot::channel();
        let envelope = (request, response_tx);
        self.envelope_tx.send(envelope).unwrap();
        async { response_rx.await.unwrap() }
    }
}

pub fn spawn_add_actor() -> AddActorRef {
    let (envelope_tx, envelope_rx) = mpsc::unbounded_channel();
    tokio::spawn(async { AddActor { envelope_rx }.run().await });
    AddActorRef { envelope_tx }
}
