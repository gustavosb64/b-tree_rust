use std::io;
use std::fs::File;
use std::path::Path;

struct HeaderBTree {
    status: u8,
    no_raiz: i32,
    prox_rrn: i32,
    nro_nos: i32
}

struct Key {
    c: i32,
    rrn: i32,
    offset: i64
}

struct Node {
    tipo_no: u8,
    nro_chaves: i32,
    key: Key,
    p: [i32; 5]
}

fn initialize_btree_header() -> Box<HeaderBTree>{
    let mut b_header = Box::new(HeaderBTree{
                                                status: 0,
                                                no_raiz: -1,
                                                prox_rrn: 0,
                                                nro_nos: 0
                                            });

    return b_header
}

pub fn write_btree_file_from_bin(file_bin_r: &File, filename_btree: &Path, f_type: u8) -> Result<bool, io::Error> {

    println!("Still testing");

    let file_btree_wr = File::open(filename_btree)?;

    let mut b_header = initialize_btree_header();

    Ok(true)
}

