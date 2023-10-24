use std::fs::File;
use std::path::Path;
use std::io::{self, BufReader, Read, Seek};

static NODE_SIZE_TYPE1: i32 = 45;

/* DOESN'T WORK:
 * mod records; */
use super::records;

#[derive(Copy, Clone, Debug)]
struct Key {
    c: i32,
    rrn: i32,
    offset: i64
}

struct Node {
    tipo_no: char,
    nro_chaves: i32,
    key: [Key; 4],
    p: [i32; 5]
}

pub struct BTree {
    status: char,
    no_raiz: i32,
    prox_rrn: i32,
    nro_nos: i32
}

impl Default for Key {
    fn default() -> Self {
        let key = Key {
                        c: -1, 
                        rrn: -1, 
                        offset: -1, 
                      };
        key
    }
}

impl Default for Node {
    fn default() -> Self {
        let default_key:Key = Key::new();
        let arr_keys: [Key; 4] = [default_key; 4];

        let mut node = Node{ 
                                tipo_no: '0',
                                nro_chaves: 0,
                                key: arr_keys, 
                                p: [-1; 5]
                            };
        node
    }
}

impl Key {

    fn new() -> Self {
        Default::default()
    }

    fn print(&mut self) {
        println!("c: {}, rrn: {}, offset: {}", self.c, self.rrn, self.offset);
    }

}

impl Node {

    pub fn new() -> Node {
        Default::default()
    }

    fn print(&mut self) {
        println!("----------------------------");
        println!("tipo_no: {}", self.tipo_no);
        println!("nro_chaves: {}", self.nro_chaves);
        println!("key: {:?}", self.key);
        println!("P: {:?}", self.p);
        println!("----------------------------");
    }
}

impl BTree {

    pub fn new(mut file_btree_r: &File) -> Result<Self, io::Error> {
        let mut b_tree = BTree{ 
                                status: '1',
                                no_raiz: -1,
                                prox_rrn: -1,
                                nro_nos: 0,
                              };


        file_btree_r.seek(io::SeekFrom::Start(0))?;
        let mut reader = BufReader::new(file_btree_r);
        
        // creating buffers used for reading
        let mut buf_c   = [0_u8; 1];
        let mut buf_i32 = [0_u8; 4];

        // reads status
        reader.read_exact(&mut buf_c)?;
        b_tree.status = u8::from_le_bytes(buf_c) as char;

        // reads no_raiz
        reader.read_exact(&mut buf_i32)?;
        b_tree.no_raiz = i32::from_le_bytes(buf_i32);

        // reads prox_rrn
        reader.read_exact(&mut buf_i32)?;
        b_tree.prox_rrn = i32::from_le_bytes(buf_i32);

        // reads nro_nos
        reader.read_exact(&mut buf_i32)?;
        b_tree.nro_nos = i32::from_le_bytes(buf_i32);

        Ok(b_tree)
    }

    pub fn get_status_from_btree(&mut self) -> char {
        self.status
    }

    pub fn print_btree_header(&mut self) {
        println!("B-Tree status: {}", self.status);
        println!("B-Tree no_raiz: {}", self.no_raiz);
        println!("B-Tree prox_rrn: {}", self.prox_rrn);
        println!("B-Tree nro_nos: {}", self.nro_nos);
    }

    fn search_in_page_b_tree(&mut self, mut file_btree_r: &File, cur_node: Node, src_id: i32, f_type: u8) -> Result<i32, io::Error>{
        
        for i in 0..cur_node.nro_chaves {

            let idx: usize = i as usize;
            
            // if current key is equal to the ID we are searching for, it
            // returns its reference
            if cur_node.key[idx].c == src_id {
                let mut Pr: i32 = -1;
                
                if f_type == 1 { 
                    Pr = cur_node.key[idx].rrn;
                }
                return Ok(Pr);
            }
            
            // if current key is larger than the ID we are searching for, 
            // the search is continued to the left of the key 
            if cur_node.key[idx].c > src_id {

                let mut new_node: Node = match self.read_node_from_b_tree(file_btree_r, cur_node.p[idx], f_type) {
                    Ok(Some(node)) => node,
                    Ok(None) => return Ok(-1),
                    Err(e) => return Err(e),
                };

                return self.search_in_page_b_tree(file_btree_r, new_node, src_id, f_type);
            }
        }

        // if the current key is neither smaller nor equal to the ID we
        // are searching for, the search continues to the right of the
        // current key (values larger than the key)
        let mut new_node: Node = match self.read_node_from_b_tree(file_btree_r, cur_node.p[cur_node.nro_chaves as usize], f_type) {
            Ok(Some(node)) => node,
            Ok(None) => return Ok(-1),
            Err(e) => return Err(e),
        };

        return self.search_in_page_b_tree(file_btree_r, new_node, src_id, f_type);

    }

    fn read_node_from_b_tree(&mut self, mut file_btree_r: &File, rrn_b_tree: i32, f_type: u8) -> Result<Option<Node>, io::Error>{
        
        if rrn_b_tree == -1{
            return Ok(None);
        }

        // initializing node
        let mut node = Node::new();

        // header offset 
        let mut offset:u64 = NODE_SIZE_TYPE1 as u64;
        offset += offset*(rrn_b_tree as u64);

        file_btree_r.seek(io::SeekFrom::Start(offset))?;
        let mut reader = BufReader::new(file_btree_r);

        // creating buffers used for reading
        let mut buf_c   = [0_u8; 1];
        let mut buf_i32 = [0_u8; 4];
        let mut buf_i64 = [0_u8; 8]; // for offset, not used

        // reads tipo_no
        reader.read_exact(&mut buf_c)?;
        node.tipo_no = u8::from_le_bytes(buf_c) as char;
        
        // reads nro_chaves
        reader.read_exact(&mut buf_i32)?;
        node.nro_chaves = i32::from_le_bytes(buf_i32);
        
        // reads each key
        for i in 0..3 {
            reader.read_exact(&mut buf_i32)?;
            node.key[i].c = i32::from_le_bytes(buf_i32);

            reader.read_exact(&mut buf_i32)?;
            node.key[i].rrn = i32::from_le_bytes(buf_i32);
        }

        // reads each reference for other nodes
        for i in 0..4 {
            reader.read_exact(&mut buf_i32)?;
            node.p[i] = i32::from_le_bytes(buf_i32);
        }

        Ok(Some(node))
    }

    pub fn search_index_in_b_tree(&mut self, mut file_bin_r: &File, mut file_btree_r: &File, src_id: i32, f_header: &Box<records::FileHeader>, f_type: u8) -> i32 {

        /* TODO
         *  check these 'unwrap' better later */
        let mut node: Node = self.read_node_from_b_tree(file_btree_r, self.no_raiz, f_type).unwrap_or(None).unwrap_or_default();

        let mut ref_offset: i32 = match self.search_in_page_b_tree(file_btree_r, node, src_id, f_type) { 
            Ok(offset) => offset,
            Err(e) => -1,
        };

        ref_offset
    }
}
