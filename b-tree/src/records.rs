use std::io::{self, BufReader, Read, Seek, Write};
use std::fs::File;
use std::path::Path;

use super::b_tree;

static MAX_RRN: i32 = 97;
static HEADER_SIZE_TYPE1: i32 = 182;

pub struct FileHeader {
    status: char,
    rrn: i32,
    offset: i64,
    prox_rrn: i32,
    prox_offset: i64,
    nro_reg_rem: i32
}

pub struct Vehicle {
    removido: char,
    tamanho_registro: i32, //used only by type2
    rrn: i32,
    offset: i64,
    id: i32,
    ano: i32,
    tam_cidade: i32,
    codC5: char,
    cidade: String,
    qtt: i32,
    sigla: String,
    tam_marca: i32,
    codC6: char,
    marca: String,
    tam_modelo: i32,
    codC7: char,
    modelo: String
}

pub fn initialize_vehicle() -> Vehicle {
    Vehicle {
        removido: '0',
        tamanho_registro: -1,
        rrn: -1,
        offset: -1,
        id: -1,
        ano: -1,
        qtt: -1,
        sigla: "".to_string(),
        tam_cidade: 0,
        codC5: 'I',
        cidade: "".to_string(),
        tam_marca: 0,
        codC6: 'I',
        marca: "".to_string(),
        tam_modelo: 0,
        codC7: 'I',
        modelo: "".to_string(),
    }
}

fn print_vehicle_full(vehicle: &Vehicle, f_type: u8) {
    println!("Removido: {}", vehicle.removido);
    println!("tamanho_registro: {}", vehicle.tamanho_registro);
    println!("prox_rrn: {}", vehicle.rrn);
    println!("prox_offset: {}", vehicle.offset);
    println!("ID: {}", vehicle.id);
    println!("Ano de fabricação: {}", vehicle.ano);
    println!("Quantidade de carros: {}", vehicle.qtt);
    println!("Estado: {}", vehicle.sigla);
    println!("tam_cidade: {}", vehicle.tam_cidade);
    println!("Cod5: {}", vehicle.codC5);
    println!("Cidade: {}", vehicle.cidade);
    println!("tam_marca: {}", vehicle.tam_marca);
    println!("Cod6: {}", vehicle.codC6);
    println!("Marca: {}", vehicle.marca);
    println!("tam_modelo: {}", vehicle.tam_modelo);
    println!("Cod7: {}", vehicle.codC7);
    println!("Modelo: {}", vehicle.modelo);
    println!("");
}

pub fn print_vehicle(vehicle: &Vehicle, f_type: u8) {
    println!("MARCA DO VEICULO: {}", vehicle.marca);
    println!("MODELO DO VEICULO: {}", vehicle.modelo);
    println!("ANO DE FABRICACAO: {}", vehicle.ano);
    println!("NOME DA CIDADE: {}", vehicle.cidade);
    println!("QUANTIDADE DE VEICULOS: {}", vehicle.qtt);
    println!("");
}

pub fn get_status_from_header(header: &Box<FileHeader>) -> char {
    return header.status;
}

pub fn print_header(f_header: &Box<FileHeader>, f_type: u8) {
    println!("status: {}", f_header.status);
    println!("rrn: {}", f_header.rrn);
    println!("prox_rrn: {}", f_header.prox_rrn);
    println!("nro_reg_rem: {}", f_header.nro_reg_rem);
}

pub fn read_header_from_bin(mut file_bin_r: &File, f_type: u8)
                -> Result<Box<FileHeader>, io::Error> {
    
    let mut f_header = Box::new(FileHeader {
                                        status: '0',
                                        rrn: -1,
                                        offset: -1,
                                        prox_rrn: 0,
                                        prox_offset: 0,
                                        nro_reg_rem: 0
                                    });
    
    // Seek correct position for file pointer
    file_bin_r.seek(io::SeekFrom::Start(0))?;
    let mut reader = BufReader::new(file_bin_r);

    // Creates buffers for reading
    let mut buf_c = [0_u8; 1];
    let mut buf_i32 = [0_u8; 4];

    // Reads 'status'
    reader.read_exact(&mut buf_c)?;
    f_header.status = u8::from_le_bytes(buf_c) as char;

    // Reads 'rrn' (topo)
    reader.read_exact(&mut buf_i32)?;
    f_header.rrn = i32::from_le_bytes(buf_i32);

    // seeking position of new data
    let size:u64 = (HEADER_SIZE_TYPE1 - 4 - 4) as u64; //sizeof(i32)
    reader.seek(io::SeekFrom::Start(size))?;

    // reading next rrn 
    reader.read_exact(&mut buf_i32)?;
    f_header.prox_rrn = i32::from_le_bytes(buf_i32);

    // reading amount of removed records
    reader.read_exact(&mut buf_i32)?;
    f_header.nro_reg_rem = i32::from_le_bytes(buf_i32);

    Ok(f_header)
}

pub fn read_id_from_reg_type1(mut file_bin_r: &File, id: &mut i32, rrn: i32) -> Result<(), io::Error> {

    // Seek correct position for file pointer
    let pos_to_seek: i32 = MAX_RRN*rrn + HEADER_SIZE_TYPE1;
    file_bin_r.seek(io::SeekFrom::Start(pos_to_seek as u64))?;
    let mut reader = BufReader::new(file_bin_r);

    let mut buf_c = [0_u8; 1];
    let mut buf_i32 = [0_u8; 4];

    // Checks if reg is removed
    reader.read_exact(&mut buf_c)?;
    let mut is_removed = u8::from_le_bytes(buf_c) as char;
    if is_removed == '1'{
        *id = -1;
        return Ok(()); // if the register is removed, return
    }

    // Reads ID
    reader.seek(io::SeekFrom::Current(4 as i64))?;
    reader.read_exact(&mut buf_i32)?;
    *id = i32::from_le_bytes(buf_i32);

    Ok(())
}

pub fn read_reg_from_bin_type1(mut file_bin_r: &File, vehicle: &mut Vehicle, rrn: i32) -> Result<(), io::Error> {

    // Seek correct position for file pointer
    let pos_to_seek = MAX_RRN*rrn + HEADER_SIZE_TYPE1;
    file_bin_r.seek(io::SeekFrom::Start(pos_to_seek as u64))?;
    let mut reader = BufReader::new(file_bin_r);

    // Used to assert that we don't surpass the register size limit
    let mut byte_counter: i32 = 0;


    // Creates buffers for reading
    let mut buf_c = [0_u8; 1];
    let mut buf_c_2 = [0_u8; 2];
    let mut buf_i32 = [0_u8; 4];
    let mut buf_string: Vec<u8> = Vec::new();

    // Reads 'removido'
    reader.read_exact(&mut buf_c)?;
    vehicle.removido = u8::from_le_bytes(buf_c) as char;
    if vehicle.removido == '1'{
        return Ok(()); // if the register is removed, return
    }

    // Reads 'rrn'
    reader.read_exact(&mut buf_i32)?;
    vehicle.rrn = i32::from_le_bytes(buf_i32);

    // Reads 'id'
    reader.read_exact(&mut buf_i32)?;
    vehicle.id = i32::from_le_bytes(buf_i32);

    // Reads 'ano'
    reader.read_exact(&mut buf_i32)?;
    vehicle.ano = i32::from_le_bytes(buf_i32);

    // Reads 'qtt'
    reader.read_exact(&mut buf_i32)?;
    vehicle.qtt = i32::from_le_bytes(buf_i32);

    // Reads 'sigla'
    reader.read_exact(&mut buf_c_2)?;
    match std::str::from_utf8(&buf_c_2){
        Ok(string) => vehicle.sigla = string.to_string(),
        Err(e) => return Ok(())
    };
    
    for i in 0..3 {

        if byte_counter > MAX_RRN-5 { 
            return Ok(()) 
        };

        // If it fails, then the file has reached its end
        reader.seek(io::SeekFrom::Current(4 as i64))?;

        reader.read_exact(&mut buf_c)?;
        let mut aux_char = u8::from_le_bytes(buf_c) as char;

        // Returns the pointer to the position before the integer
        reader.seek(io::SeekFrom::Current(-5 as i64))?;

        match aux_char {
            
            '0' => {
                reader.read_exact(&mut buf_i32)?;
                vehicle.tam_cidade = i32::from_le_bytes(buf_i32); 

                reader.read_exact(&mut buf_c)?;
                vehicle.codC5 = u8::from_le_bytes(buf_c) as char; 

                // Reads 'cidade'
                buf_string = vec![0; vehicle.tam_cidade as usize];
                reader.read_exact(&mut buf_string)?;
                match std::str::from_utf8(&buf_string){
                    Ok(string) => vehicle.cidade = string.to_string(),
                    Err(e) => return Ok(())
                }

                byte_counter += 1 + 4 + (vehicle.cidade.len() as i32);
            },

            '1' => {
                reader.read_exact(&mut buf_i32)?;
                vehicle.tam_marca = i32::from_le_bytes(buf_i32); 

                reader.read_exact(&mut buf_c)?;
                vehicle.codC6 = u8::from_le_bytes(buf_c) as char; 

                // Reads 'cidade'
                buf_string = vec![0; vehicle.tam_marca as usize];
                reader.read_exact(&mut buf_string)?;
                match std::str::from_utf8(&buf_string){
                    Ok(string) => vehicle.marca = string.to_string(),
                    Err(e) => return Ok(())
                }

                byte_counter += 1 + 4 + (vehicle.marca.len() as i32);
            },

            '2' => {
                reader.read_exact(&mut buf_i32)?;
                vehicle.tam_modelo = i32::from_le_bytes(buf_i32); 

                reader.read_exact(&mut buf_c)?;
                vehicle.codC7 = u8::from_le_bytes(buf_c) as char; 

                // Reads 'cidade'
                buf_string = vec![0; vehicle.tam_modelo as usize];
                reader.read_exact(&mut buf_string)?;
                match std::str::from_utf8(&buf_string){
                    Ok(string) => vehicle.modelo = string.to_string(),
                    Err(e) => return Ok(())
                }

                byte_counter += 1 + 4 + (vehicle.modelo.len() as i32);
            },

            _ => (),

        }
            

    };

    Ok(())

}

pub fn read_all_reg_from_bin(filename_in_bin: &Path, f_type: u8) -> Result<(), io::Error> {
    
    let mut file_bin_r = File::open(filename_in_bin)?;

    let mut vehicle = initialize_vehicle();

    if f_type == 1 {
        let mut rrn = 0;
        loop {
            match read_reg_from_bin_type1(&file_bin_r, &mut vehicle, rrn) {
                Ok(_) => {},
                Err(e) => break,
            };

            rrn += 1;
        }
    };

    Ok(())
}

pub fn initialize_reg_type1(mut file_bin_w: &File) -> Result<(), io::Error> {
    /* TODO
     *  How to properly reference to MAX_RRN?
     */
    const arr_len: usize = 97;
    let arr_c: [char; arr_len as usize] = ['$'; arr_len as usize];
    
    for value in arr_c {
        write!(file_bin_w, "{}", &value)?;
    }

    Ok(())
}

pub fn  write_reg_in_bin_type1(file_bin_w: &File, vehicle: &Vehicle) {

    initialize_reg_type1(file_bin_w);


}

pub fn add_new_reg_type1(mut file_bin_rw: &File, vehicle: Vehicle, rrn: &mut i32, f_header: &mut Box<FileHeader>) -> Result<i32, io::Error>{ 
    
    let mut flag_stack: u8 = 0; // tells whether there were space reuse
    
    if f_header.rrn == -1 {
        *rrn = f_header.prox_rrn;
    }
    else { 
        *rrn = f_header.rrn;
        flag_stack = 1;
    };

    let mut ref_offset: i32 = (*rrn)*MAX_RRN + HEADER_SIZE_TYPE1;
    file_bin_rw.seek(io::SeekFrom::Start(ref_offset as u64))?;

    // Creates buffers for reading
    let mut buf_c = [0_u8; 1];
    let mut buf_i32 = [0_u8; 4];

    let mut reader = BufReader::new(file_bin_rw);

    if flag_stack != 0 {
        let mut is_removed: char = '0';
        let mut new_stack_top: i32 = -1;
        
        // if the record doesn't is marked as removed, returns
        reader.read_exact(&mut buf_c)?;
        is_removed = u8::from_le_bytes(buf_c) as char;
        if is_removed != '1' {
            return Ok(-1);
        }

        reader.read_exact(&mut buf_i32)?;
        new_stack_top = i32::from_le_bytes(buf_i32);
        
        (*f_header).rrn = new_stack_top;
        (*f_header).nro_reg_rem = f_header.nro_reg_rem - 1;

        // Returns pointer to the start of the record
        // necessary for the write function
        file_bin_rw.seek(io::SeekFrom::Start(ref_offset as u64))?;
    }

    write_reg_in_bin_type1(file_bin_rw, &vehicle);

    Ok(0)
}

pub fn search_reg_in_btree(file_bin_r: &File, file_btree_r: &File, id:i32, mut btree: b_tree::BTree, f_header:Box<FileHeader>, f_type:u8) -> i32{

    let ref_rrn: i32 = btree.search_index_in_b_tree(file_bin_r, file_btree_r, id, &f_header, f_type); 

    if ref_rrn == -1 {
        println!("Registro inexistente.");
        return 1;
    }

    let mut vehicle = initialize_vehicle();
    let _ = read_reg_from_bin_type1(file_bin_r, &mut vehicle, ref_rrn);
    print_vehicle(&vehicle, f_type);

    return 0;
}

pub fn add_new_reg_using_btree(file_bin_rw: &File, file_btree_rw: &File, f_type: u8, f_header: &mut Box<FileHeader>, mut btree: b_tree::BTree, id: i32, ano: i32, qtt: i32, sigla: String, cidade: String, marca: String, modelo: String) -> i32 {
    
    let i_f_type = f_type;

    // check if the record doesn't already exists
    let mut ref_rrn: i32 = btree.search_index_in_b_tree(file_bin_rw, file_btree_rw, id, f_header, i_f_type);
    if ref_rrn != -1 {
        return -1; 
    }

    let mut vehicle = initialize_vehicle();

    vehicle.id = id;
    vehicle.ano = ano;
    vehicle.qtt = qtt;
    vehicle.sigla = sigla;
    vehicle.cidade = cidade;
    vehicle.marca = marca;
    vehicle.modelo = modelo;

    if f_type == 1 {
        let rrn: i32 = -1;
        add_new_reg_type1(file_bin_rw, vehicle, &mut ref_rrn, f_header);
    }
    
    return 0

}
