use std::path::Path;

pub fn print_help() {
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
            -m: [f -or- d ] f -> file d -> directory
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

pub fn print_out() {
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
pub fn check_if_absolute(p: &Path) -> bool {
    if p.is_absolute() {
        return true;
    }
    false
}
