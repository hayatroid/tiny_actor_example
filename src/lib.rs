use tokio::sync::oneshot;

type AddRequest = (i32, i32);
type AddResponse = i32;
type AddEnvelope = (AddRequest, oneshot::Sender<AddResponse>);
