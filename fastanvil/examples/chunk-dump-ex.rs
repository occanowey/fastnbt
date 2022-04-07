use fastanvil::{CurrentJavaChunk, RegionBuffer};
use fastnbt::from_bytes;

//
// This loads a region file, extracts a chunk from it, and uses serde to
// deserialize it into a `anvil::Chunk` object and print it.
//

fn main() {
    let args: Vec<_> = std::env::args().skip(1).collect();
    let file = std::fs::File::open(args[0].clone()).unwrap();

    let mut region = RegionBuffer::new(file);
    let data = region.load_chunk(0, 0).unwrap();

    let chunk: CurrentJavaChunk = from_bytes(data.as_slice()).unwrap();

    println!("{:?}", chunk);
}
