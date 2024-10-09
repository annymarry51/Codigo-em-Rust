//Exemplo de uma função com retorno:
fn maior(x: i32, y: i32) -> i32{
    if x>y {
        x
    }
    else{
      y
    }
}

fn main() {
    let x = 35;
    let y = 20;
    let maior = maior(x,y);
    println!("O maior número é {}", maior);
}
