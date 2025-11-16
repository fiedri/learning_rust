use std::cmp::Ordering;
use std::io;
// importamos la biblioteca de entrada/salida io que proviene de la biblioteca estandat std

use rand::Rng; //usamos la libreria rand para generar numeros aleatorios
//Rng es un trait que define métodos para generar números aleatorios
fn main() {
    println!("Guess the number!");
    let secret_number = rand::thread_rng().gen_range(1..=100);
    // generamos un numero aleatorio entre 1 y 100
    println!("Secret number is: {secret_number}");
    loop {
        println!("Please input your guess: ");
    
    //creamos una variable mutable guess de tipo String
    // las variables mutables se declaran incluyendo la palabra mut antes del nombre de la variable
    let mut guess:String = String::new();
    //new es un método asociado a la estructura String que crea una nueva instancia vacía de String
    io::stdin().read_line(&mut guess).expect("Failed to read line");
    //usamos stdin para obtener la entrada del usuario, es un metodo de io
    //read_line lee una línea de la entrada estándar y la almacena en la variable guess
    // & indica que estamos pasando una referencia mutable a guess
    //expect es un método que maneja el resultado de read_line
    // let guess: u32 = guess.trim().parse().expect("please type a number");
    let guess:u32 = match guess.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Please enter a valid number!");
            continue;
        },
    };
    // usamos tambien match para manejar el resultado de parse
    println!("You guessed: {}", guess);
    // match expression
    // usamos match para comparar guess con secret_number y devolver un mensaje apropiado
    match guess.cmp(&secret_number) {
        
        // comparamos guess con secret_number usando cmp
        // cmp devuelve un valor de la enumeración Ordering
        Ordering::Less => println!("Too small!"),
        Ordering::Greater => println!("Too big!"),
        Ordering::Equal => {
            println!("You win!");
            break;
        },
    }
    }
}
