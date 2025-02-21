//Exercício adicional - Inserindo uma string e a invertendo

//Importando a biblioteca de entrada e saída IO
use std::io;
use reverse_string::reverse; 


fn main() {
    // Solicita ao usuário que insira uma string
    println!("Digite uma string para inverter:");

    // Cria uma nova string para armazenar a entrada do usuário
    let mut input = String::new();

    // Lê a entrada do usuário
    std::io::stdin()
        .read_line(&mut input)
        .expect("Falha ao ler a linha");

    // Remove a nova linha do final da string
    let input = input.trim();

    // Inverte a string e imprime o resultado
    let reversed = reverse(input);
    println!("String invertida: {}", reversed);
}