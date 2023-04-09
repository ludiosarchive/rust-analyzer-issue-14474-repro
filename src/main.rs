#![feature(
    generators,
    proc_macro_hygiene,
    stmt_expr_attributes,
)]

use aes::cipher::KeyIvInit;
use aes::cipher::generic_array::GenericArray;
use futures::stream::Stream;
use futures_async_stream::stream;

type Aes128Ctr = ctr::Ctr64BE<aes::Aes128>;

fn empty_stream() -> impl Stream<Item = ()> {
    futures::stream::empty()
}

fn stream_file() -> impl Stream<Item = ()> {
    #[stream]
    async move {
        let stream = empty_stream();
        let key = GenericArray::from_slice(&[0; 16]);
        let cipher = Aes128Ctr::new(key, key);

        yield ();
    }
}

fn main() {
    stream_file();
}
