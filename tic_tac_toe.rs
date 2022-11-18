use std::fs::File;
use std::path::Path;
use std::io::Read;
use std::fs::OpenOptions;
use std::io::Write;

#[derive(Default)]
struct Personaje1{
    personaje1: String,
    fichap1: String,
    fila1: u8,
    columna1: u8,
}
struct Personaje2{
    personaje2: String,
    fichap2: String,
    fila2: u8,
    columna2: u8,
}

fn personas() {
    println!("Tres en raya!
    El juego se trata de competir contra tu oponente hasta
    alcanzar realizar una linea recta de 3 de tus fichas consecutivas");
    println!(" Bienvenidos!
    Jugador 1: escriba su nombre: ");
    let stdin = io::stdin();
    let mut a = String::new();
    stdin.read_line(mut a).expect("No pudo ejecutarse dicha accion");
    println!("Escoja su tipo de ficha (X o O): ");
    let mut n = String::new();
    stdin.read_line(mut n).expect("No pudo ejecutarse dicha accion");
    println!("Jugador 2: escriba su nombre: ")
    let mut b = String::new();
    stdin.read_line(mut b).expect("No pudo ejecutarse dicha accion");

    if n == "X" || n == "x" {
        let mut m = "O";
    } else {
        let mut m = "X";
    }
}

fn crear_pers(p: &Path) {
    let mut p1 = Personaje1{personaje1: a,
                            fichap1: n,
                            fila1: x,
                            columna1: xx,
    }
    let mut p2 = Personaje2{personaje2: b,
                            fichap2: m,
                            fila2: y,
                            columna2: yy,
    }

    let mut file = open_file_to_append(p);
    file.write_all(temp.as_bytes());
}

fn tablero() {
    let mut line1 = Vec::new();
    line1.push("1");
    line1.push(" ");
    line1.push("2");
    line1.push(" ");
    line1.push("3");
    line1.push(" ");
 
    let mut fila1 = Vec::new();
    fila1.push("º");
    fila1.push("|");
    fila1.push("º");
    fila1.push("|");
    fila1.push("º");
    fila1.push("1");

    let mut fila2 = Vec::new();
    fila2.push("º");
    fila2.push("|");
    fila2.push("º");
    fila2.push("|");
    fila2.push("º");
    fila2.push("2");

    let mut fila3 = Vec::new();
    fila3.push("º");
    fila3.push("|");
    fila3.push("º");
    fila3.push("|");
    fila3.push("º");
    fila3.push("3");

    let mut vect = Vec::new();
    vect.push(line1);
    vect.push(fila1);
    vect.push(fila2);
    vect.push(fila3);

    println!("{}", vect);
}

fn mod_tiradas() {
    
    // corregir

    loop{
        println!("jugador 1 coordenada fila (1, 2 o 3): ");
        let mut x = String::new();
        let stdin = io::stdin();
        stdin.read_line(mut x).expect("No pudo ejecutarse dicha accion");
        let mut x = x;

        if x =< 3 || x >= 1 {
            
            //Aqui reemplazar 

        } else {
            println!("Por favor ingresar un valor dentro de las coordenadas! ");
            let mut x = String::new();
            stdin.read_line(mut x).expect("No pudo ejecutarse dicha accion");
        }
        println!("jugador 1 coordenada columna (1, 2 o 3): ");
        let mut xx = String::new();
        stdin.read_line(mut xx).expect("No pudo ejecutarse dicha accion");
        let mut xx = xx-1;

        if xx =< 3 || xx >= 1 {
            
            //Aqui reemplazar 

        } else {
            println!("Por favor ingresar un valor dentro de las coordenadas! ");
            let mut xx = String::new();
            stdin.read_line(mut xx).expect("No pudo ejecutarse dicha accion");
        }

        println!("Jugador 2 coordenada fila (1, 2 o 3): ");
        let mut y = String::new();
        stdin.read_line(mut y).expect("No pudo ejecutarse dicha accion");
        let mut y = y;

        if y =< 3 || y >= 1 {
            
            //Aqui reemplazar 

        } else {
            println!("Por favor ingresar un valor dentro de las coordenadas! ");
            let mut y = String::new();
            stdin.read_line(mut y).expect("No pudo ejecutarse dicha accion");
        }

        println!("Jugador 2 coordenada columna (1, 2 o 3): ");
        let mut yy = String::new();
        stdin.read_line(mut y).expect("No pudo ejecutarse dicha accion");
        let mut yy = yy-1;

        if yy =< 3 || yy >= 1 {
            
            //Aqui reemplazar 

        } else {
            println!("Por favor ingresar un valor dentro de las coordenadas! ");
            let mut yy = String::new();
            stdin.read_line(mut y).expect("No pudo ejecutarse dicha accion");
        }
        // sacar el loop
    }
}



fn read_file(mut f: &File){
    let mut text = String::new();
    f.read_to_string(&mut text).unwrap();
    println!("{}", &text);
}

fn create_blank_file(p: &Path){
    let file = File::create(p).expect("El archivo no pudo crearse");
    println!("El archivo fue creado");
}

fn add_new_content(mut f: &File){
    f.write_all(b"Medicamentos\n");
}

fn open_file_to_append(p: &Path) -> File{

    let mut binding = OpenOptions::new();
    let binding = binding.append(true);
    let file = match binding.open(p){
        Err(why) => panic!("No se puede abrir el archivo"),
        Ok(file) => file,
    };
    return file
}

fn open_file_to_read(p: &Path){
    if Path::new(p).exists(){
        let file = match File::open(&p){
            Err(why) => panic!("El archivo no se puede abrir"),
            Ok(file) => file,
        };
        read_file(&file);
    } else {
        create_blank_file(p);
    }
}



fn main(){
    let path = Path::new("./data/ejer_progra_lab4_3enRaya.txt");

    open_file_to_read(path);
    personas();
    crear_pers(path);
    tablero();
    mod_tiradas();
    let file = open_file_to_append(path);
    add_new_content(&file);
    open_file_to_read(path);


}
