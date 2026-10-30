fn main() {
    let mut palabra = String::from("Hola");
    agregar_signo(&mut palabra);
    println!("{palabra}")
}

fn agregar_signo(signo: &mut String ){
    signo.push_str(" Mundo!!!");
}
