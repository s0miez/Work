use std::fs::File;
use std::path::Path;
use std::io::Read;
use std::fs::OpenOptions;
use std::io::Write;

#[derive(Default)]
struct Persona{
    codigo: String,
    nombre: String,
    costo: String,
    lab: String,
    comp_prin: String,
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

    f.write_all(b"Nuevo texto\n");
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
            Err(why) => panic!("El archivo no se puede abrir..."),
            Ok(file) => file,
        };
        read_file(&file)
    } else {
        create_blank_file(p);
    }
}

fn create_persona(p: &Path){

    let mut p1 = Persona{codigo: a,
                             nombre: b,
                             costo: c,
                             lab: d,
                             comp_prin: e}
                             
    let temp = format!("{}:{}:{}:{}\n", p1.codigo, 
                                              p1.nombre, 
                                              p1.costo, 
                                              p1.lab,
                                              p1.comp_prin,
                                              );
    
    let mut file = open_file_to_append(p);

    file.write_all(temp.as_bytes());
}

fn menu(p: &Path){
    let mut x = String::new();
    let stdin = io::stdin();
    
    println!("Farmacia Azul
Menu
Consultar precios (1)
Buscar medicamento (2)
Agregar medicamento (3)
");
    stdin.read_line(mut x).expect("No pudo ejecutarse dicha accion");

    if x == 1 {
        println!("{}", p);
    }
    if x == 2 {
        //Mostrar su valor 
        println!("Escriba el codigo del medicamento a buscar: ");
        let mut n = String::new();
        let stdin = io::stdin();
        stdin.read_line(mut n).expect("No pudo ejecutarse dicha accion");
        //Buscar la palabra del medicamento en el texto y dar el precio:
        let palabra: String = p.to_string();
        //let mut m = palabra[];
        if palabra.contains(n){
            println!("{}", "El precio es de: {}", n, m);
        } else {
            println!("No encontrado");
        } 
    }
    if x == 3 {
        //agregar txto en el archivo
        println!("Escriba el codigo del medicamento, su nombre, su valor, su laboratorio de presedencia y su componente principal")
        println!("Codigo: ");
        let stdin = io::stdin();
        let mut a = String::new();
        stdin.read_line(mut a).expect("No pudo ejecutarse dicha accion");
        println!("Nombre: ");
        let mut b = String::new();
        stdin.read_line(mut b).expect("No pudo ejecutarse dicha accion");
        println!("Valor: ");
        let mut c = String::new();
        stdin.read_line(mut c).expect("No pudo ejecutarse dicha accion");
        println!("Laboratorio: ");
        let mut d = String::new();
        stdin.read_line(mut d).expect("No pudo ejecutarse dicha accion");
        println!("Componente principal: ");
        let mut e = String::new();
        stdin.read_line(mut e).expect("No pudo ejecutarse dicha accion");
        
        p.write_all("{}, {}, {}, {}, {}", a, b, c, d, e);
    }
    
    
}


fn main(){
    let path = Path::new("./data/ejer2_progra_lab2.txt");
    open_file_to_read(path);
    create_persona(path);
    open_file_to_read(path);
}

