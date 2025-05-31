extern crate flate2;

use flate2::bufread::GzDecoder;
use flate2::write::GzEncoder;
use flate2::Compression;
use std::env;
use std::env::args;
use std::fs;
use std::fs::create_dir_all;
use std::fs::File;
use std::io::copy;
use std::io::BufReader;
use std::path::Path;
use std::time::Instant;
mod utilities;

#[derive(Default)]
struct Commands {
    work: String,
    input: String,
    output: String,
    level: u32,
    name: String,
}
#[derive(Default)]
struct DCommands {
    input: String,
    output: String,
}

// ###################################ENTRY POINT###################################
fn main() {
    let cmd_string: Vec<String> = args().collect();
    if cmd_string.contains(&"--help".to_string()) | (cmd_string.len() == 1) {
        utilities::print_help();
        return;
    }
    if cmd_string.contains(&"--compress".to_string()) {
        std::process::exit(compression_logic(cmd_string));
    }
    if cmd_string.contains(&"--decompress".to_string()) {
        std::process::exit(decompression_logic(cmd_string));
    }
}
// ###################################ENTRY POINT TILL HERE###################################

fn compression_logic(cmd_string: Vec<String>) -> i32 {
    utilities::print_help();
    utilities::print_out();
    let mut com_struct: Commands = Commands::default();
    for i in 0..cmd_string.len() - 1 {
        match cmd_string[i].as_str() {
            "-n" => {
                if cmd_string[i + 1].starts_with("-") {
                    println!("Woops wrong syntax");
                    return -1;
                } else {
                    com_struct.name = cmd_string[i + 1].clone();
                }
            }
            "-m" => {
                if cmd_string[i + 1].starts_with("-") {
                    println!("Woops wrong syntax");
                    return -1;
                } else {
                    com_struct.work = cmd_string[i + 1].clone();
                }
            }
            "-i" => {
                if cmd_string[i + 1].starts_with("-") {
                    println!("Woops wrong syntax");
                    return -1;
                } else {
                    com_struct.input = cmd_string[i + 1].clone();
                }
            }
            "-o" => {
                if cmd_string[i + 1].starts_with("-") {
                    println!("Woops wrong syntax");
                    return -1;
                } else {
                    com_struct.output = cmd_string[i + 1].clone();
                }
            }
            "-l" => {
                if cmd_string[i + 1].starts_with("-") {
                    println!("Woops wrong syntax");
                    return -1;
                } else {
                    com_struct.level = cmd_string[i + 1].parse().unwrap();
                }
            }
            _ => (),
        }
    }
    if com_struct.input.is_empty()
        // | com_struct.output.is_empty()
        | com_struct.level.eq(&0)
        | com_struct.work.is_empty()
    {
        println!("Woops! Command criteria not fullfilled\nExitting...");
        return -1;
    }
    if com_struct.output.is_empty(){
        com_struct.output = String::from(".");
    }
    match com_struct.work.as_str() {
        "f" => {
            let ipth = Path::new(&com_struct.input);
            println!("Input {ipth:?}");
            if ipth.is_dir() {
                println!("Directory provided!! Consider using flag `-m d` ");
                return -1;
            }
            let cpath = env::current_dir().unwrap();
            let t_opth = Path::new(&com_struct.output);
            let check_abs = utilities::check_if_absolute(t_opth);
            let opth;
            if !check_abs {
                opth = cpath.join(t_opth);
                if !opth.is_file() & !opth.exists() {
                    create_dir_all(&opth).unwrap();
                }
            } else {
                opth = Path::to_path_buf(&t_opth);
                if !opth.is_file() & !opth.exists() {
                    create_dir_all(&opth).unwrap();
                }
            }
            let mut outputname_with_ext = com_struct.name.clone();
            outputname_with_ext.push_str(".gz");
            println!("Output location at: {opth:?}");
            let mut input = BufReader::new(File::open(com_struct.input).unwrap());
            let output = File::create(opth.join(outputname_with_ext)).unwrap();
            println!("Compressing....");
            //Encoding
            let mut encoder = GzEncoder::new(output, Compression::new(com_struct.level));
            let start = Instant::now();
            copy(&mut input, &mut encoder).unwrap();
            let output = encoder.finish().unwrap();
            println!(
                "Source length: {:?}",
                input.get_ref().metadata().unwrap().len()
            );
            println!("Target length: {:?}", output.metadata().unwrap().len());
            println!("Elapsed Time: {:?}", start.elapsed());
        }
        "d" => {
            let pth = Path::new(&com_struct.input);
            if pth.is_file() {
                println!("File provided!! Consider using flag `-m f`");
                return -1;
            }
            let start = Instant::now();
            match comp_dir(com_struct) {
                Ok(_) => {
                    println!("Directory Archived!!");
                    println!("Elapsed Time: {:?}", start.elapsed());
                },
                Err(e) => {
                    eprintln!("Compression failed: {}", e);
                    std::process::exit(1);
                }
            }
        }
        _ => {
            println!("Invalid flags selected!!");
        }
    }
    0
}

// Under construction 
fn decompression_logic(cmd_string: Vec<String>) -> i32 {
    utilities::print_help();
    utilities::print_out();
    let mut com_struct: DCommands = DCommands::default();
    for i in 0..cmd_string.len() - 1 {
        match cmd_string[i].as_str() {
            "-i" => {
                if cmd_string[i + 1].starts_with("-") {
                    println!("Woops wrong syntax");
                    return -1;
                } else {
                    com_struct.input = cmd_string[i + 1].clone();
                }
            }
            "-o" => {
                if cmd_string[i + 1].starts_with("-") {
                    println!("Woops wrong syntax");
                    return -1;
                } else {
                    com_struct.output = cmd_string[i + 1].clone();
                }
            }
            _ => (),
        }
    }

    let cpath = env::current_dir().unwrap();
    let input_path = cpath.join(&com_struct.input);

    println!("{input_path:?} {}", com_struct.output);
    if input_path.is_file() {
        //Check if file ends with gz then uncompress or else donot
        if com_struct.input.contains(&".gz".to_string()) {
            //File decompression logic
            let input_file = BufReader::new(File::open(&input_path).unwrap());
            let mut output_file = File::create(&com_struct.output).unwrap();
            let mut gz = GzDecoder::new(input_file);
            copy(&mut gz, &mut output_file).unwrap();
        }
        else {
            println!("Invalid File format");
            return -1;
        }
    }
    0
}

//This folder compression is unsafe for complex files and might freeze the terminal, use on simple folder structures
/*
*
* TODO: Use walkdir insted of append_dir_all
*
*/
fn comp_dir(stru_cpy: Commands) -> Result<(), Box<dyn std::error::Error>>  {
    println!("Starting directory compression...");
    let input_path = Path::new(&stru_cpy.input);
    let abs_input_path = if input_path.is_absolute() {
        input_path.to_path_buf()
    } else {
        env::current_dir()?.join(input_path)
    };
    if !abs_input_path.is_dir() {
        print!("Path provided is not a directory!");
        return Ok(());
    }
    let pth = Path::new(&stru_cpy.output);
    let npth = env::current_dir()?;
    let npth = npth.join(pth);
    if !npth.exists() {
        fs::create_dir_all(&npth)?;
    }
    let mut name = stru_cpy.name.to_owned();
    name.push_str(".tar.gz");
    let npth = npth.join(name);
    let tar_gz = File::create(npth)?;
    let enc = GzEncoder::new(tar_gz, Compression::new(stru_cpy.level));
    let mut tar = tar::Builder::new(enc);
    tar.append_dir_all("compressed", abs_input_path)?;
    tar.finish()?;
    Ok(())
}
