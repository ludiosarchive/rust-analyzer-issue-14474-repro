#![feature(
    generators,
    proc_macro_hygiene,
    stmt_expr_attributes,
)]

use aes::cipher::KeyIvInit;
use aes::cipher::generic_array::GenericArray;
use futures::stream::{Stream, BoxStream};
use futures_async_stream::stream;

type Aes128Ctr = ctr::Ctr64BE<aes::Aes128>;

async fn empty_stream() -> impl Stream<Item = ()> {
    futures::stream::empty()
}

fn stream_file() -> BoxStream<'static, ()> {
    Box::pin(
        #[stream]
        async move {
            let stream = empty_stream().await;
            let key = GenericArray::from_slice(&[0; 16]);
            let nonce = GenericArray::from_slice(&[0; 16]);
            let cipher = Aes128Ctr::new(key, nonce);

            yield ();
        }
    )
}

fn main() {
    stream_file();
}
