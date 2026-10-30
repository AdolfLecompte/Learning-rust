fn main() {
    let r1 = Rectangulo {
        ancho: dbg!(30),
        alto: 50,
    };

    println!("el area es {}", area(&r1));
    println!("ancho {} y alto {}", r1.ancho, r1.alto);
    dbg!(&r1);
    println!("r1 es {r1:?}");
    println!("r1 es {r1:#?}");
}

#[derive(Debug)]
struct Rectangulo {
    alto: u32,
    ancho: u32,
}

fn area (r: &Rectangulo) -> u32{
    r.ancho * r.alto
}
