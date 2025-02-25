//Array em Rust

fn main(){ 

    let numeros: [i32;5] = [10,20,30,40,50]; //Criando um array, o definindo como inteiro de 32 bits, com 5 campos,

//Printando a primeira posição do array
    println!("{:#?}", numeros[2]);
//Printano a última, com o numeros.len() -1 representando a quantidade total do array -1
    println!("O último elemento é {}", numeros[numeros.len() -1])
}


