use std::io::{self, BufReader, Read, Seek};
use std::fs::File;
use std::path::Path;

static MAX_RRN: i32 = 97;
static HEADER_SIZE_TYPE1: i32 = 182;

struct FileHeader {
    status: char,
    rrn: i32,
    offset: i64,
    prox_rrn: i32,
    prox_offset: i64,
    nro_reg_rem: i32
}

struct Vehicle {
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

fn initialize_vehicle() -> Vehicle {
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

fn print_vehicle_full(V: Vehicle, f_type: u8) {

    println!("Removido: {}", V.removido);
    println!("tamanho_registro: {}", V.tamanho_registro);
    println!("prox_rrn: {}", V.rrn);
    println!("prox_offset: {}", V.offset);
    println!("ID: {}", V.id);
    println!("Ano de fabricação: {}", V.ano);
    println!("Quantidade de carros: {}", V.qtt);
    println!("Estado: {}", V.sigla);
    println!("tam_cidade: {}", V.tam_cidade);
    println!("Cod5: {}", V.codC5);
    println!("Cidade: {}", V.cidade);
    println!("tam_marca: {}", V.tam_marca);
    println!("Cod6: {}", V.codC6);
    println!("Marca: {}", V.marca);
    println!("tam_modelo: {}", V.tam_modelo);
    println!("Cod7: {}", V.codC7);
    println!("Modelo: {}", V.modelo);
    println!("");

}

fn read_header_from_bin(file: &File, f_type: u8)
                -> Result<Box<FileHeader>, io::Error> {
    
    let f_header = Box::new(FileHeader {
                                        status: '0',
                                        rrn: -1,
                                        offset: -1,
                                        prox_rrn: 0,
                                        prox_offset: 0,
                                        nro_reg_rem: 0
                                    });

    Ok(f_header)
}

fn read_reg_from_bin_type1(mut file_bin_r: &File, V: &mut Vehicle, rrn: i32) -> Result<(), io::Error> {

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
    V.removido = u8::from_le_bytes(buf_c) as char;
    if V.removido == '1'{
        return Ok(()); // if the register is removed, return
    }

    // Reads 'rrn'
    reader.read_exact(&mut buf_i32)?;
    V.rrn = i32::from_le_bytes(buf_i32);

    // Reads 'id'
    reader.read_exact(&mut buf_i32)?;
    V.id = i32::from_le_bytes(buf_i32);

    // Reads 'ano'
    reader.read_exact(&mut buf_i32)?;
    V.ano = i32::from_le_bytes(buf_i32);

    // Reads 'qtt'
    reader.read_exact(&mut buf_i32)?;
    V.qtt = i32::from_le_bytes(buf_i32);

    // Reads 'sigla'
    reader.read_exact(&mut buf_c_2)?;
    match std::str::from_utf8(&buf_c_2){
        Ok(string) => V.sigla = string.to_string(),
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
                V.tam_cidade = i32::from_le_bytes(buf_i32); 

                reader.read_exact(&mut buf_c)?;
                V.codC5 = u8::from_le_bytes(buf_c) as char; 

                // Reads 'cidade'
                buf_string = vec![0; V.tam_cidade as usize];
                reader.read_exact(&mut buf_string)?;
                match std::str::from_utf8(&buf_string){
                    Ok(string) => V.cidade = string.to_string(),
                    Err(e) => return Ok(())
                }

                byte_counter += 1 + 4 + (V.cidade.len() as i32);
            },

            '1' => {
                reader.read_exact(&mut buf_i32)?;
                V.tam_marca = i32::from_le_bytes(buf_i32); 

                reader.read_exact(&mut buf_c)?;
                V.codC6 = u8::from_le_bytes(buf_c) as char; 

                // Reads 'cidade'
                buf_string = vec![0; V.tam_marca as usize];
                reader.read_exact(&mut buf_string)?;
                match std::str::from_utf8(&buf_string){
                    Ok(string) => V.marca = string.to_string(),
                    Err(e) => return Ok(())
                }

                byte_counter += 1 + 4 + (V.marca.len() as i32);
            },

            '2' => {
                reader.read_exact(&mut buf_i32)?;
                V.tam_modelo = i32::from_le_bytes(buf_i32); 

                reader.read_exact(&mut buf_c)?;
                V.codC7 = u8::from_le_bytes(buf_c) as char; 

                // Reads 'cidade'
                buf_string = vec![0; V.tam_modelo as usize];
                reader.read_exact(&mut buf_string)?;
                match std::str::from_utf8(&buf_string){
                    Ok(string) => V.modelo = string.to_string(),
                    Err(e) => return Ok(())
                }

                byte_counter += 1 + 4 + (V.modelo.len() as i32);
            },

            _ => (),

        }
            

    };
    /*
    */
        

    println!("removido: {}", V.removido as char);
    println!("prox_rrn: {}", V.rrn as i32);
    println!("id: {}", V.id as i32);
    println!("ano: {}", V.ano as i32);
    println!("qtt: {}", V.qtt);
    println!("sigla: {}", V.sigla);
    println!("cidade: {}", V.cidade);
    println!("marca: {}", V.marca);
    println!("modelo: {}", V.modelo);
    println!("");


    Ok(())

}

pub fn read_all_reg_from_bin(filename_in_bin: &Path, f_type: u8) -> Result<(), io::Error> {
    
    let mut file_bin_r = File::open(filename_in_bin)?;

    let mut V = initialize_vehicle();

    if f_type == 1{
        let mut rrn = 0;
        loop {
            match read_reg_from_bin_type1(&file_bin_r, &mut V, rrn) {
                Ok(_) => {},
                Err(e) => break,
            };

            rrn += 1;
        }
    };

    Ok(())
}
