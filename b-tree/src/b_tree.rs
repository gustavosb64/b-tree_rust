use std::io;
use std::fs::File;
use std::path::Path;

struct header_btree {
    status: u8,
    no_raiz: i16,
    prox_rrn: i16,
    nro_nos: i16
}

struct key {
    c: i16,
    rrn: i16,
    offset: i32
}

struct node {
    tipo_no: u8,
    nro_chaves: u16,
    key: key,
    p: [i16; 5]
}

fn initialize_btree_header() -> Box<header_btree>{
    let mut b_header = Box::new(header_btree{
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

