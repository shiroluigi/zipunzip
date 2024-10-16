extern crate flate2;

use flate2::write::GzEncoder;
use flate2::Compression;
use std::env::args;
use std::fs::File;
use std::io::copy;
use std::io::BufReader;
// use std::process::Output;
use std::time::Instant;

#[derive(Default)]
struct Commands {
    work: String,
    input: String,
    output: String,
    level: u32,
}
fn main() {
    let mut com_struct: Commands = Commands::default();
    let cmd_string: Vec<String> = args().collect();
    if cmd_string.contains(&"--help".to_string()) | (cmd_string.len() == 1){
        print_help();
        return;
    }
    print_help();
    print_out();
    for i in 0..cmd_string.len() - 1 {
        match cmd_string[i].as_str() {
            "-m" => {
                if cmd_string[i + 1].starts_with("-") {
                    println!("Woops wrong syntax");
                    return;
                } else {
                    com_struct.work = cmd_string[i + 1].clone();
                }
            }
            "-i" => {
                if cmd_string[i + 1].starts_with("-") {
                    println!("Woops wrong syntax");
                    return;
                } else {
                    com_struct.input = cmd_string[i + 1].clone();
                }
            }
            "-o" => {
                if cmd_string[i + 1].starts_with("-") {
                    println!("Woops wrong syntax");
                    return;
                } else {
                    com_struct.output = cmd_string[i + 1].clone();
                }
            }
            "-l" => {
                if cmd_string[i + 1].starts_with("-") {
                    println!("Woops wrong syntax");
                    return;
                } else {
                    com_struct.level = cmd_string[i + 1].parse().unwrap();
                }
            }
            _ => (),
        }
    }
    if com_struct.input.is_empty()
        | com_struct.output.is_empty()
        | com_struct.level.eq(&0)
        | com_struct.work.is_empty()
    {
        println!("Woops! Command criteria not fullfilled");
    }
    match com_struct.work.as_str() {
        "c" => {
            let mut input = BufReader::new(File::open(com_struct.input).unwrap());
            let output = File::create(com_struct.output).unwrap();
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
        _ => {
            println!("Invalid flags selected!!");
        }
    }
}

fn print_help() {
    println!(
        r#"

 ________   ___   ________   ___  ___   ________    ________   ___   ________   
|\_____  \ |\  \ |\   __  \ |\  \|\  \ |\   ___  \ |\_____  \ |\  \ |\   __  \  
 \|___/  /|\ \  \\ \  \|\  \\ \  \\\  \\ \  \\ \  \ \|___/  /|\ \  \\ \  \|\  \ 
     /  / / \ \  \\ \   ____\\ \  \\\  \\ \  \\ \  \    /  / / \ \  \\ \   ____\
    /  /_/__ \ \  \\ \  \___| \ \  \\\  \\ \  \\ \  \  /  /_/__ \ \  \\ \  \___|
   |\________\\ \__\\ \__\     \ \_______\\ \__\\ \__\|\________\\ \__\\ \__\   
    \|_______| \|__| \|__|      \|_______| \|__| \|__| \|_______| \|__| \|__|                           

    Thank you for using zipunzip, this tool is used to compress and decompress any file
        Usage: 

            -i: input file location and name
            -o: output file location and name
            -m: c or d , c -> compress and d -> decompress
            -l: compression mode 1-9, 1 is lowest compression 9 is highest
            --help: This text

        Syntax : zipunzip -i <value> -o <value> -m <value> -l <value> (!! Order doesnot matter !!)
    "#
    )
}

fn print_out(){
    println!(r#"
                _                  _   
               | |                | |  
   ___   _   _ | |_  _ __   _   _ | |_ 
  / _ \ | | | || __|| '_ \ | | | || __|
 | (_) || |_| || |_ | |_) || |_| || |_ 
  \___/  \__,_| \__|| .__/  \__,_| \__|
                    | |                
                    |_|                
    "#)
}
