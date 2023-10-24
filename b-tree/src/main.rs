use std::io;
use std::str;
use std::path::Path;
use std::fs::File;

mod b_tree;
mod records;

fn readline() -> String{
    
    // Create a mutable String to store the user's input.
    let mut input = String::new();

    // Read a line of text from stdin and store it in the 'input' variable.
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    return input;
}

fn create_index_btree(v_input: Vec<&str>) -> Result<File, io::Error> {

    // Put input information in their respective variables
    let mut f_type: u8 = 1;
    match v_input[0] {
        "tipo1" => f_type = 1,
        "tipo2" => f_type = 2,
        _ => panic!(),
    };
    let filename_bin = Path::new(v_input[1]);
    let filename_btree = Path::new(v_input[2]);

    println!("{} - {} - {}", f_type, filename_bin.display(), filename_btree.display());

    // Attempts to open the designated file
    let file_bin_r = File::open(&filename_bin)?;

//    let _ = records::read_all_reg_from_bin(filename_bin, f_type);

//    let _ = b_tree::write_btree_file_from_bin(&file_bin_r, filename_btree, f_type);
    /*
    match b_tree::write_btree_file_from_bin(&file_bin_r, filename_btree, f_type){
        Ok(_) => println!("blablalb"),
        Err(e) => println!("{}",e),
    }
    */

//    let _ = read_header_from_bin(file_bin_r, f_type)

    Ok(file_bin_r)
}

//fn select_from_btree(v_input: Vec<&str>) -> Result<File, io::Error> {
fn select_from_btree(v_input: Vec<&str>) -> Result<(), io::Error> {
    
    println!("SELECT");

    // Put input information in their respective variables
    let mut f_type: u8 = 1;
    match v_input[0] {
        "tipo1" => f_type = 1,
        "tipo2" => f_type = 2,
        _ => panic!(),
    };
    let filename_bin = Path::new(v_input[1]);
    let idx_search = Path::new(v_input[4]); //v_input[3] is "id"

    println!("{} - {} - {}", f_type, filename_bin.display(), idx_search.display());

    // Opening binary file
    let file_bin_r = File::open(&filename_bin)?;
    let f_header = records::read_header_from_bin(&file_bin_r, f_type)?;

    // TODO: Raise error
    if records::get_status_from_header(&f_header) != '1' {
        println!("File processing failed.");
        return Ok(());
    }

    let filename_btree = Path::new(v_input[2]);
    let file_btree_r = File::open(&filename_btree)?;

    // How to work with impl properly?
    let mut b_tree = b_tree::BTree::new(&file_btree_r)?;
    //b_tree.print_btree_header();
    
    let id:i32 = (v_input[4]).parse().unwrap();
    let ret: i32 = records::search_reg_in_btree(&file_bin_r, &file_btree_r, id, b_tree, f_header, f_type);

    Ok(())
}

fn insert_using_btree(v_input: Vec<&str>) -> Result<(), io::Error> {

    println!("INSERT");

    // Put input information in their respective variables
    let mut f_type: u8 = 1;
    match v_input[0] {
        "tipo1" => f_type = 1,
        "tipo2" => f_type = 2,
        _ => panic!(),
    };
    let filename_bin = Path::new(v_input[1]);
    let filename_btree = Path::new(v_input[2]);
    let n_insertions = (v_input[3]).parse().unwrap();

    println!("{} - {} - {} - {}", f_type, filename_bin.display(), filename_btree.display(), n_insertions);

    let f_bin_rw = File::open(&filename_bin)?;
    let f_header = records::read_header_from_bin(&f_bin_rw, f_type)?;

    // TODO: Raise error
    if records::get_status_from_header(&f_header) != '1' {
        println!("File processing failed.");
        return Ok(());
    }
    
    /* TODO: 
     *  Set status 
    */
     

    let f_btree_rw = File::open(&filename_btree)?;
    let mut btree = b_tree::BTree::new(&f_btree_rw)?;
    if btree.get_status_from_btree() != '1' {
        println!("File processing failed.");
        return Ok(());
    }

    /*
    TODO
        Format input correctly. Currently, it doesn't reads quoted names
        that contains whitespaces.
    */
    let input = readline();
    let v_search_input: Vec<&str> = input.split_whitespace().collect();

    let mut id: i32 = -1; 
    let mut ano: i32 = -1; 
    let mut qtt: i32 = -1; 
    let mut sigla: String = "".to_string(); 
    let mut cidade: String = "".to_string(); 
    let mut marca: String = "".to_string(); 
    let mut modelo: String = "".to_string();

    for i in 0..n_insertions {
        id  = (v_search_input[0]).parse().unwrap();
        ano = (v_search_input[1]).parse().unwrap();
        qtt = (v_search_input[2]).parse().unwrap();
        sigla  = String::from(v_search_input[3].trim_matches('"'));
        cidade = String::from(v_search_input[4].trim_matches('"'));
        marca  = String::from(v_search_input[5].trim_matches('"'));
        modelo = String::from(v_search_input[6].trim_matches('"'));
    }

    println!("{} - {} - {} - {} - {} - {} - {}", 
               id, ano, qtt, sigla, cidade, marca, modelo);

    records::add_new_reg_using_btree(&f_bin_rw, &f_btree_rw, f_type, &f_header, btree, id, ano, qtt, sigla, cidade, marca, modelo);
    Ok(())
}

fn main() {

    let input = readline();
    let v_input: Vec<&str> = input.split_whitespace().collect();

    /*
    // Print the input back to the console.
    println!("{}", input);
    for part in v_input){
        println!("{}", part);
    }
    */

    // Stores the operation to be executed
    let operation = v_input[0];

    match operation{
        "9" => 
            match create_index_btree(v_input.iter().skip(1).cloned().collect()) {
                Ok(_) => (),
                Err(e) => println!("{}", e),
            },
        "10" => 
            match select_from_btree(v_input.iter().skip(1).cloned().collect()) {
                Ok(_) => (),
                Err(e) => println!("{}", e),
            },
        "11" => 
            match insert_using_btree(v_input.iter().skip(1).cloned().collect()) {
                Ok(_) => (),
                Err(e) => println!("{}", e),
            },
        _ => println!("invalid"),
    }

    return ();

}
