//Exemplo básico de composição de comandos 
fn main() {
    let n1 = 20;
    let mut n2 = 0;  
    println!("Os numeros pares entre {} e {} são:", n2, n1);
    while n2 <= n1 {
        if n2 % 2 == 0 {
            println!("{}", n2);  
        }
        n2 += 1;  
    }
}
