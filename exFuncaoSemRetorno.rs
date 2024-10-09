//Exemplo de função em Rust sem retorno:

fn maior(x: i32, y: i32){
    if x>y {
        println!("O maior número é {}", x);
    }
    else{
        println!("O maior número é {}", y);
    }
}

fn main() {
    let x = 10;
    let y = 11;
    maior(x,y);
}
