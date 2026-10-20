fn main() {
    let resultado = fibonacci(50);
    println!("el n-esimo es -> {resultado}");
}

fn fibonacci(n: u32) -> u64 {
    if n == 0{
        return 0;
    }

    let mut anterior= 0;
    let mut actual= 1;

    for _ in 1..n{
        let siguiente= anterior + actual;
        anterior = actual;
        actual = siguiente;
    }
    actual
}
