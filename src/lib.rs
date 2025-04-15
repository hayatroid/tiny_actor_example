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
