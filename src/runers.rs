use std::io::{self, Write , BufRead}; //el bufread es para leer el archivo de Prueebas.TXT liena por linea
use std::fs::{OpenOptions , File}; // Importar OpenOptions para manejar modos de apertura para el .txt de los corredores
use std::process::Command;





//// ESTRUCTURAS PARA EL CORREDOR: el corredor tiene una estructura anida Timeruners anidadd
#[derive(Debug)] 
pub struct Runers{
    pub name:String,
    pub time_400: Timeruners,
    pub time_800: Timeruners,
    pub time_1500: Timeruners
}
//el tiempo que debe tener cada uno minutos y segundos
#[derive(Debug)] 
pub struct Timeruners {
    pub minutes:i64,
    pub seconds:i64
}
//no solo sirve declara las Structuras como publicos, tambien sus variables dentro





// FUNCIONES USADAS DURANTE EL PROGRAMA

pub fn exit() -> bool{
    let mut input_exit = String::new();
    println!("");
    println!("Hay otro corredor ? (SI/NO) : ");
    io::stdin().read_line(&mut input_exit).expect("ERRROR");
    input_exit.trim().to_uppercase() == "NO"

}
//esta funcion se hara cargo de pedir informacion
pub fn read_time(prompt: &str) -> Timeruners{
    let mut input:String = String::new();
    println!("{}",prompt); //mostrar la info

    println!("Minutos: ");
    io::stdin().read_line(&mut input).expect("ERROR");
    let minutes:i64 = input.trim().parse().expect("ERROR"); 

    input.clear();

    println!("Segundos: ");
    io::stdin().read_line(&mut input).expect("ERROR");
    let seconds:i64 = input.trim().parse().expect("ERROR"); 

    // Deben tener los mismos nomnbres que lo de las estructura
    Timeruners{ minutes , seconds}
}



//este se hara cargo de pasarle los datos a la estructura
pub fn create_runners(name:String , time_400:Timeruners , time_800:Timeruners , time_1500:Timeruners ) -> Runers{
    Runers{
        name,
        time_400,
        time_800,
        time_1500
    }
}



// Función para guardar la información del corredor en un archivo y si el archivo no esta se crea
pub fn save_runner_to_file(runner: &Runers, filename: &str) -> std::io::Result<()> {
    let mut file = OpenOptions::new()
        .append(true)
        .create(true)
        .open(filename)?;

    writeln!(
        file,
        "{},{},{},{},{},{},{}",
        runner.name,
        runner.time_400.minutes, runner.time_400.seconds,
        runner.time_800.minutes, runner.time_800.seconds,
        runner.time_1500.minutes, runner.time_1500.seconds
    )?;

    Ok(())
}


// Función para convertir minutos y segundos a segundos totales
fn to_seconds(time: &Timeruners) -> i64 {
    time.minutes * 60 + time.seconds 
}


// Función para leer los corredores desde Pruebas.txt
// Función para leer los corredores desde Pruebas.txt
pub fn read_runners(filename: &str) -> Vec<Runers> {
    let file = File::open(filename).expect("No se pudo abrir el archivo");
    let reader = io::BufReader::new(file);
    // se crear un leector bon el buf para el txt Prueebas
    let mut runners = Vec::new();
    //se crea un vector para pasar los valores del los correodores

    // se recorre lo que este leyendo el txt
    for line in reader.lines() {
        let line = line.expect("Error al leer la línea");
        let parts: Vec<&str> = line.split(',').collect();
        //pica en slcie los datos, empezando con el nomnbre (espacio 0) desde el 1 hasta el 6 
        let name = parts[0].trim().to_string();
        let time_400 = Timeruners {
            minutes: parts[1].trim().parse().unwrap(),
            seconds: parts[2].trim().parse().unwrap(),
        };
        let time_800 = Timeruners {
            minutes: parts[3].trim().parse().unwrap(),
            seconds: parts[4].trim().parse().unwrap(),
        };
        let time_1500 = Timeruners {
            minutes: parts[5].trim().parse().unwrap(),
            seconds: parts[6].trim().parse().unwrap(),
        };
        //se hace el pusc de todos los de nombre y tiempo para el vector
        runners.push(Runers {
            name,
            time_400,
            time_800,
            time_1500,
        });
    }

    runners
}


// Función para escribir los resultados en Corredores.txt
pub fn write_results(runners: &[Runers], filename: &str) -> std::io::Result<()> {
    let mut file = OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true) // Sobrescribir el archivo si ya existe
        .open(filename)?;

    // Definir anchos fijos para las columnas
    let name_width = 20;
    let time_width = 8; // Suficiente para "MM:SS"
    writeln!(
        file,
        "{:<name_width$}\t{:<time_width$}\t{:<time_width$}\t{:<time_width$}",
        "Nombre", "400m", "800m", "1500m",
        name_width = name_width,
        time_width = time_width
    )?;

    // Escribir los datos de cada corredor con anchos fijos
    for runner in runners {
        writeln!(
            file,
            "{:<name_width$}\t{:>time_width$}:{:02}\t{:>time_width$}:{:02}\t{:>time_width$}:{:02}",
            runner.name,
            runner.time_400.minutes, runner.time_400.seconds,
            runner.time_800.minutes, runner.time_800.seconds,
            runner.time_1500.minutes, runner.time_1500.seconds,
            name_width = name_width,
            time_width = time_width
        )?;
    }


    // Calcular el porcentaje de corredores con menos de 50 segundos en 400m
    let count_400 = runners.iter().filter(|r| to_seconds(&r.time_400) < 50).count();
    let percentage_400 = (count_400 as f32 / runners.len() as f32) * 100.0;

    // Encontrar el mejor tiempo en 1500m
    let best_1500 = runners
        .iter()
        .min_by_key(|r| to_seconds(&r.time_1500))
        .unwrap();

    // Contar cuántos corredores completaron los 800m en menos de 2 minutos (120 segundos)
    let count_800 = runners.iter().filter(|r| to_seconds(&r.time_800) < 120).count();

    // Escribir los resultados
    writeln!(file, "\nPorcentaje de corredores con menos de 50 segundos en 400m: {:.2}%", percentage_400)?;
    writeln!(file, "Mejor tiempo en 1500m: {} - {}:{}", best_1500.name, best_1500.time_1500.minutes, best_1500.time_1500.seconds)?;
    writeln!(file, "Corredores con menos de 2 minutos en 800m: {}", count_800)?;

    Ok(())
}


//funcion para limpiar la pantalla luego dar vueltas en el ciclo
pub fn clear_screen() {
    if cfg!(target_os = "windows") {
        // Comando para limpiar la pantalla en Windows
        Command::new("cmd")
            .args(&["/c", "cls"])
            .status()
            .expect("No se pudo limpiar la pantalla");
    } else {
        // Comando para limpiar la pantalla en Unix/Linux/MacOS
        Command::new("clear")
            .status()
            .expect("No se pudo limpiar la pantalla");
    }
}