use std::future::Future;
use futures_util::{Sink, Stream};
use server_fn::client::browser::BrowserClient;
use server_fn::client::Client;
use server_fn::error::FromServerFnError;
use server_fn::request::browser::BrowserRequest;
use server_fn::response::browser::BrowserResponse;

pub struct AddAuthHeaderClient;

impl<E, IS, OS> Client<E, IS, OS> for AddAuthHeaderClient
where
    E: FromServerFnError,
    IS: FromServerFnError,
    OS: FromServerFnError,
{
    type Request = BrowserRequest;
    type Response = BrowserResponse;

    fn send(req: Self::Request) -> impl Future<Output=Result<Self::Response, E>> + Send {
        let headers = req.headers();
        headers.append("Authorization", "Bearer ");
        <BrowserClient as Client<E, IS, OS>>::send(req)
    }

    fn open_websocket(
        path: &str,
    ) -> impl Future<
        Output = Result<
            (
                impl Stream<
                    Item = Result<server_fn::Bytes, server_fn::Bytes>,
                > + Send
                + 'static,
                impl Sink<server_fn::Bytes> + Send + 'static,
            ),
            E,
        >,
    > + Send {
        <BrowserClient as Client<E, IS, OS>>::open_websocket(path)
    }

    fn spawn(future: impl Future<Output = ()> + Send + 'static) {
        <BrowserClient as Client<E, IS, OS>>::spawn(future)
    }
}