use std::fs::File;
use std::io::{self, BufRead};
use std::path::{Path, self};
use std::fs::OpenOptions;
use std::io::Write;

fn main() {
    if let Ok(lines) = read_lines("./top50.csv") {
        for line in lines {
            if let Ok(ip) = line {
                println!("{}\n", ip);
            }
        }
    }
    let file = abrir_archivo(path);
    (&file);
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn agregar_texto() {
    let mut file = OpenOptions::new()
        .write(true)  
        .append(true)
        .create(true)
        .open("top50.csv")
        .unwrap();
    if let Err(e) = writeln!(file, "\n51,Not Allowed,TV Girl,Alternative,88
    \n52,The Less I Know The Better,Tame Impala, Alternative,97") {
        eprintln!("Couldn't write to file: {}", e);
    }
}

fn abrir_archivo(p: &Path) -> File{

    let mut binding = OpenOptions::new();
    let binding = binding.append(true);
    let file = match binding.open(p){
        Err(why) => panic!("No se puede abrir el archivo"),
        Ok(file) => file,
    };
    return file
}
