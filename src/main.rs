use std::fs::File;
use std::io::{BufReader, BufWriter, Read, Write};
use std::num::IntErrorKind;

fn get_input(querry: &str) -> String{ //получение ввода
    print!("{querry}");
    std::io::stdout().flush().unwrap();


    let mut buffer = String::new();
    std::io::stdin().read_line(&mut buffer).unwrap();

    buffer.trim().to_owned()
}


fn procces_file_data (data:&Vec<u8>, key:u8) -> Vec<u8> //шифровние
{
    let mut proccesed_data = Vec::with_capacity(data.len());

    for byte in data{
        proccesed_data.push(byte ^ key);
    }

    proccesed_data
}

fn main() {

    println!("\x1b[35m
            _ _  .  _ _
           / / // _\\ / /

           \x1b[0m");
    loop {
        

        let input_file_name = get_input("Enter file name to process: ");

        let input_file = match File::open(&input_file_name) {
            Ok(file) => file,
            Err(e) => {
                println!("Can't open file \"{input_file_name}\" : {e}");
                continue
            }
        };

        let key = match get_input("\nEnter a key for file encryption/decrypion: ")
            .parse::<u8>(){
            Ok(key) => key,
            Err(e) => {
                match e.kind() {
                    IntErrorKind::Empty => println!("Key mustn't be empty."),
                    IntErrorKind::InvalidDigit => println!("Enter number."),
                    IntErrorKind::PosOverflow => println!("Number must be in range 0->255."),
                    _ => println!("Error getting key!")
                }
                println!();
                continue
            }
        };


        let mut reader = BufReader::new(input_file);
        let mut input_data = Vec::new();


        if let Err(err) = reader.read_to_end(&mut input_data)
        {
            println!("Error: {err}");
            continue
        }


        let proccessed_data = procces_file_data(&input_data, key);
        let output_file_name = get_input("File name to output: ");
        let output_file = match File::create(&output_file_name) {
            Ok(file) => file,
            Err(e) => {
                println!("\nCan't create file \"{output_file_name}\": {e}");
                continue
            }
        };

        let mut writer = BufWriter::new(output_file);

        if let Err(err) = writer.write_all(&proccessed_data)
        {
            println!("\nError writing to output file: {err}\n\n");
            continue
        }


        print!("{esc}c", esc = 27 as char); //system cls
    }
}