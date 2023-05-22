use rand::Rng;
use std::cmp::Ordering;
use std::io::stdin;

pub fn get_data(comieza: i8, termina: i8) -> () {


    // Creamos una variable de tipo entero de 8 bits, el cual por medios del crate rand se realiza un numero aleatorio entre los parametros asignados en la funcion
    let numero_aleatorio: i8 = rand::thread_rng().gen_range(comieza..termina + 1);
    println!("Hola, vamos a ver si eres capaz de adivinar el numero");
    println!("Digita un numero");

    loop {
        // Creamos una nueva variable de tipo String vacio
        let mut numero_adivinar: String = String::new();

        // por medio de stdin() tomamos los datos que ingresa el usuario en consola y se gurdan en la variable previamente declarada
        stdin()
            .read_line(&mut numero_adivinar)
            .expect("No se pudo leer");
        // convertimos el dato tomado del usuario en entero de 8-bits
        let numero_adivinar: i8 = match numero_adivinar.trim().parse() {
            Ok(numero) => numero,
            Err(_) => {
                println!("Escribe un numero");
                continue;
            }
        };
        // por medio de match comparamos los datos ingresados por el usuario y retorna un mensaje segun su categoria
        match numero_adivinar.cmp(&numero_aleatorio) {
            Ordering::Less => println!("Muy bajo"),
            Ordering::Greater => println!("Muy alto"),
            Ordering::Equal => {
                println!("Felicitones");
                break;
            }
        }
    }
}
