extern crate flate2;

use flate2::write::GzEncoder;
use flate2::Compression;
use std::env;
use std::env::args;
use std::fs;
use std::fs::create_dir_all;
use std::fs::File;
use std::io;
use std::io::copy;
use std::io::BufReader;
use std::path::Path;
use std::time::Instant;

#[derive(Default)]
struct Commands {
    work: String,
    input: String,
    output: String,
    level: u32,
    name: String,
}
fn main() {
    let cmd_string: Vec<String> = args().collect();
    if cmd_string.contains(&"--help".to_string()) | (cmd_string.len() == 1) {
        print_help();
        return;
    }
    if cmd_string.contains(&"--compress".to_string()) | (cmd_string.len() == 1) {
        std::process::exit(compression_logic(cmd_string));
    }
}
fn compression_logic(cmd_string : Vec<String>) -> i32{
    print_help();
    print_out();
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
        | com_struct.output.is_empty()
        | com_struct.level.eq(&0)
        | com_struct.work.is_empty()
    {
        println!("Woops! Command criteria not fullfilled");
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
            let check_abs = chekc_if_absolute(t_opth);
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
            println!("Output location at: {opth:?}");
            let mut input = BufReader::new(File::open(com_struct.input).unwrap());
            let output = File::create(opth.join(com_struct.name)).unwrap();
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
                println!("File provided!! Consider using flag `-m c`");
                return -1;
            }
            let start = Instant::now();
            comp_dir(com_struct);
            println!("Directory Archived!!");
            println!("Elapsed Time: {:?}", start.elapsed());
        }
        _ => {
            println!("Invalid flags selected!!");
        }
    }
    0
}
fn comp_dir(stru_cpy: Commands) {
    let pth = Path::new(&stru_cpy.output);
    let npth = env::current_dir().unwrap();
    let npth = npth.join(pth);
    if !npth.exists() {
        fs::create_dir_all(&npth).unwrap();
    }
    let mut name = stru_cpy.name.to_owned();
    name.push_str(".tar.gz");
    let npth = npth.join(name);
    let tar_gz = File::create(npth).unwrap();
    let enc = GzEncoder::new(tar_gz, Compression::new(stru_cpy.level));
    let mut tar = tar::Builder::new(enc);
    tar.append_dir_all("content", stru_cpy.input).unwrap();
}

// Under construction // Temporary template
fn decompression_logic(path : String) {
    //Decompression logic
    let file = fs::File::open(&path).unwrap();
    let mut archive = zip::ZipArchive::new(file).unwrap();

    for i in 0..archive.len() {
        let mut file = archive.by_index(i).unwrap();

        let outpath = match file.enclosed_name() {
            Some(path) => path.to_owned(),
            None => continue,
        };
        {
            let comment = file.comment();
            if !comment.is_empty(){
                println!("File {} comment:{} ",i,comment);
            }
        }
        if (*file.name()).ends_with('/'){
            println!("File {} extracted to {}", i, outpath.display());
            fs::create_dir_all(&outpath).unwrap();
        }
        else {
            println!("File {} extracted to {} ", i, outpath.display());
            if let Some(p) = outpath.parent() {
                if !p.exists(){
                    fs::create_dir_all(&p).unwrap();
                }
            }
            let mut outfile = fs::File::create(&outpath).unwrap();
            io::copy(&mut file, &mut outfile).unwrap();
        }
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            if let Some(mode) = file.unix_mode(){
                fs::set_permissions(&outpath, fs::Permissions::from_mode(mode)).unwrap();
            }
        }
    }
}   


fn print_help() {
    println!(
        r#"
=======================================================================================================
 ________   ___   ________   ___  ___   ________    ________   ___   ________   
|\_____  \ |\  \ |\   __  \ |\  \|\  \ |\   ___  \ |\_____  \ |\  \ |\   __  \  
 \|___/  /|\ \  \\ \  \|\  \\ \  \\\  \\ \  \\ \  \ \|___/  /|\ \  \\ \  \|\  \ 
     /  / / \ \  \\ \   ____\\ \  \\\  \\ \  \\ \  \    /  / / \ \  \\ \   ____\
    /  /_/__ \ \  \\ \  \___| \ \  \\\  \\ \  \\ \  \  /  /_/__ \ \  \\ \  \___|
   |\________\\ \__\\ \__\     \ \_______\\ \__\\ \__\|\________\\ \__\\ \__\   
    \|_______| \|__| \|__|      \|_______| \|__| \|__| \|_______| \|__| \|__|                           

    Thank you for using zipunzip, this tool is used to compress and decompress any file
        Usage: 
        
        modes [ --compress , --decompress ]

        --compress:
            -i: input file location and name (Ex: /dir/folder)
            -o: output file location (Ex: /dir/folder)
            -n: output file name
            -m: [f -or- d ] c -> file d -> directory
            -l: compression mode 1-9, 1 is lowest compression 9 is highest
            --help: this text
        --decompress:
            //logic pending

            other info:
            . = current directory
            / = root directory ( C:/ in Windows)
            Use ' ' for any directory names containing spaces

        Syntax : zipunzip -i <value> -o <value> -m <value> -l <value> (!! Order doesnot matter !!)

=======================================================================================================
    "#
    )
}

fn print_out() {
    println!(
        r#"
                _                  _   
               | |                | |  
   ___   _   _ | |_  _ __   _   _ | |_ 
  / _ \ | | | || __|| '_ \ | | | || __|
 | (_) || |_| || |_ | |_) || |_| || |_ 
  \___/  \__,_| \__|| .__/  \__,_| \__|
                    | |                
                    |_|                
    "#
    )
}
fn chekc_if_absolute(p: &Path) -> bool {
    if p.is_absolute() {
        return true;
    }
    false
}


/* TODO:
1 -> Uncompress
2 -> Tests
*/
