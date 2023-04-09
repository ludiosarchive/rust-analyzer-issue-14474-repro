use aes::cipher::KeyIvInit;
use aes::cipher::generic_array::GenericArray;
use futures::stream::Stream;

type Aes128Ctr = ctr::Ctr64BE<aes::Aes128>;

async fn empty_stream() -> impl Stream<Item = ()> {
    futures::stream::empty()
}

async fn stream_file() -> impl Stream<Item = ()> {
    let stream = empty_stream().await;
    let key = GenericArray::from_slice(&[0; 16]);
    let cipher = Aes128Ctr::new(key, key);
    stream
}

fn main() {
    stream_file();
}
