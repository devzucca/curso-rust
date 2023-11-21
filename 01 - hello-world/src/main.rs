fn main() {
    println!("hola, world!");

    // Variables
    let mut my_string = "cadena de texto!"; // la palabra reservada mut => hace mutable el valor de la cadena de caracteres 
    println!("{my_string}"); // formatos validos para imprimir, ("{}", my_string), ("{my_string}") => es lo mismo

    my_string = "otra cadena de texto!"; 
    println!("{my_string}"); // formatos validos para imprimir, ("{}", my_string), ("{my_string}") => es lo mismo
    
    // my_string = 6; => esto es un error no se puede mutar el valor del tipo de dato de la variable
    
}
