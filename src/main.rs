// Importar el módulo runers.rs
mod runers;

// Importar las estructuras y funciones desde runners, y dentro de las llaves las que vallas a usar
use runers::{create_runners, read_time ,save_runner_to_file ,  read_runners , write_results , exit , clear_screen };
use std::{fs::File, io::{self, Write}};


fn main() {
    /*let mut file =  File::create("datos.txt").unwrap();
    file.write(b"Hello world").unwrap();*/

    loop {

        let mut name_runer = String::new();
    println!("Ingrese el nombre del corredor:");
    io::stdin().read_line(&mut name_runer).expect("Error cannot works");
    name_runer = name_runer.trim().to_string(); //aqui habia un problema, el espacio que deja el hip se pasaba para el txt, no se podia leer bien
    //le quito el espacio con el trim

    let asks_runers: [String; 3] = [
        String::from("Cuanto tiempo hizo en en los 400mt (minutos y segundos):"), 
        String::from("Cuanto tiempo hizo en en los 800mt (minutos y segundos):") , 
        String::from("Cuanto tiempo hizo en en los 1500mt (minutos y segundos):")];
    let time_400 = read_time(&asks_runers[0]);
    let time_800 = read_time(&asks_runers[1]);
    let time_1500 = read_time(&asks_runers[2]);

    
    let runner = create_runners(name_runer, time_400, time_800, time_1500);
    // mostrar el corredor por cmd
    println!("\nDatos del corredor:");
    println!("Nombre: {}", runner.name);
    println!("Tiempo en 400m: {} minutos y {} segundos", runner.time_400.minutes, runner.time_400.seconds);
    println!("Tiempo en 800m: {} minutos y {} segundos", runner.time_800.minutes, runner.time_800.seconds);
    println!("Tiempo en 1500m: {} minutos y {} segundos", runner.time_1500.minutes, runner.time_1500.seconds);

    //crear el archivo
    save_runner_to_file(&runner, "Pruebas.txt").expect("ERROR");
    let runners = read_runners("Pruebas.txt"); //se asgina el nombre del txt, ya esta aqui dentor de raiz del proyecto
    write_results(&runners, "Corredores.txt").expect("Error al escribir en el archivo");
    
    let mut file = File::create("datos.txt").unwrap();
    for runner in &runners {
        writeln!(file, "{:#?}", runner).unwrap(); // Escribir cada corredor en una línea
    }

    if exit() {
        break;
    }else {
        clear_screen();
    }

    }

}
