fn main() {
    let texto = String::from("Buenas tardes !!!");
    let palabra = primera_palabra(&texto);
    println!("{palabra}");
    let palabra2 = segunda_palabra(&texto);
    println!("{palabra2}");
}

fn primera_palabra(parte: &str) -> &str {
    let bytes = parte.as_bytes();

    for (i, &byte) in bytes.iter().enumerate() {
        if byte == b' '{
            return &parte[0..i];
        }
    }
    &parte[..]
}

fn segunda_palabra(parte: &str) -> &str{
    let bytes = parte.as_bytes();
    let mut inicio = 0;

    for (i, &byte) in bytes.iter().enumerate() {
        if byte == b' '{
            if inicio > 0{
                return &parte[inicio..i];
            } else {
                inicio = i + 1;
            }
        }
    }
    if inicio > 0{
        return &parte[inicio..]
    }else {
        ""
    }
}
