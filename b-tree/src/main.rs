use std::io;
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
    let mut f_type: u8 = 0;
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

    let _ = records::read_all_reg_from_bin(filename_bin, f_type);

//    let _ = b_tree::write_btree_file_from_bin(&file_bin_r, filename_btree, f_type);



//    let _ = read_header_from_bin(file_bin_r, f_type)

    Ok(file_bin_r)
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
        _ => println!("invalid"),
    }

    return ();

}
