use std::io;
use std::fs::File;
use std::path::Path;
//use std::io::{self, BufReader, Read, Seek};

static NODE_SIZE_TYPE1: i32 = 45;

/* DOESN'T WORK:
 * mod records; */
use super::records;

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

struct BTree {
    status: u8,
    no_raiz: i32,
    prox_rrn: i32,
    nro_nos: i32
}


/*
impl BTree {
    
    pub fn read_header_from_btree(&mut self, file_btree_r: &File) -> Result<Box<BTree>, io::Error> {
        
        file_btree_r.seek(io::SeekFrom::Start(pos_to_seek as u64))?;
        let mut reader = BufReader::new(file_btree_r);
        
        // creating buffers used for reading
        let mut buf_c   = [0_u8; 1];
        let mut buf_i32 = [0_u8; 0];
        
        //let mut b_tree = BTree 

    }

    //pub fn initialize_btree(&mut self, f_type: u8) -> Result<Box<Node>, io::Error> {
    pub fn initialize_btree(&mut self, f_type: u8) -> Result<(), io::Error> {
        
//        node = Box::new(Node);

        Ok(())

    }
}

impl BTree {

    pub fn initialize_btree_header() -> Box<BTree>{
        let mut b_header = Box::new(BTree{
                                                    status: 0,
                                                    no_raiz: -1,
                                                    prox_rrn: 0,
                                                    nro_nos: 0
                                                });

        return b_header
    }

    pub fn initialize_node (f_type: u8) {





    }

    pub fn initialize_btree (file_btree_rw: &File, b_header: &Box<BTree>, new_key: Key, f_type: u8) -> Result<(), io::Error> {

     //   new_root = initialize_node(f_type);

    }

    pub fn add_new_node_btree(file_bin_rw: &File, b_header: &Box<BTree>, id: i32, id_ref: i32, f_type: u8) -> Result<(), io::Error> {

        let new_key = Key {
                            c: id,
                            rrn: id_ref,
                            offset: -1
                         };
        
        /*
        if b_header.no_raiz == -1 {

            
        }
        */


        Ok(())
    }
}


pub fn write_btree_file_from_bin(file_bin_r: &File, filename_btree: &Path, f_type: u8) -> Result<bool, io::Error> {

    println!("filename_btree: {}", filename_btree.display());

    let file_btree_wr = File::create(filename_btree)?;
    let mut b_header = initialize_btree_header();

    let mut id: i32 = -1;

    let mut V = records::initialize_vehicle();

    if f_type == 1 { 
        
        let header_offset = NODE_SIZE_TYPE1;

        let mut counter = 0;
        loop {
            match records::read_id_from_reg_type1(&file_bin_r, &mut id, counter) {
                
                Ok(_) => {},
                Err(e) => break,
            };

            if id != -1 {
                add_new_node_btree(&file_bin_r, &b_header, id, counter, f_type); 
            }
            counter += 1;
        }
    }

    Ok(true)
}
*/

