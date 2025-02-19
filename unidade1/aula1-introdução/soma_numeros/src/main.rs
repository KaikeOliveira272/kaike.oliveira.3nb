//Exercício Soman

// Importa a biblioteca padrão de entrada e saída (io) para ler dados do teclado
use std::io;

//Declara a função principal do programa 
fn main() {


//let serve para declarar uma variável 
//mut torna a variável mútavelprintln!("Digite o primeiro número:");
let mut num1 = String::new();
let mut num2 = String::new();


println!("Digite o 1° número: ");


io::stdin().read_line(&mut num1).expect("Erro ao ler entrada");

println!("Digite o 2° número: "); 

io::stdin().read_line(&mut num2).expect("Erro ao ler entrada");

//:i32 (inteiro de 64 bits, .parse() para convertes numx em i32, .expect("Digite um número válido") em caso de falha)
let num1: i32 = num1.trim().parse().expect("Digite um número válido");
let num2: i32 = num2.trim().parse().expect("Digite um número válido");

let soma = num1 + num2;

println!("A soma de {} e {} é igual a: {}", num1, num2, soma);
}