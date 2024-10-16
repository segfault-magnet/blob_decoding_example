use fuel_block_committer_encoding::{
    blob::{self, Blob},
    bundle::{self, Bundle},
};

fn main() {
    let hex_str = std::fs::read_to_string("./data.txt")
        .unwrap()
        .strip_prefix("0x")
        .unwrap()
        .to_owned();

    let blob_raw_data = hex::decode(hex_str).unwrap();

    let blob: Blob = blob_raw_data.into_boxed_slice().try_into().unwrap();

    let blob_decoder = blob::Decoder::default();
    let blob_header = blob_decoder.read_header(&blob).unwrap();
    eprintln!("{blob_header:?}");

    let bundle_bytes = blob_decoder.decode(&[blob]).unwrap();
    let bundle: Bundle = bundle::Decoder::default().decode(&bundle_bytes).unwrap();

    println!("{:?}", bundle);
}
