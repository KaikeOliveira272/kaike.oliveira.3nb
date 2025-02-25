//Vetores em Rust

//Coleções dinamicas que permitem redimencionamento 

fn main(){ 

    let mut numero: Vec<i32> = vec![10,20,30];

    println!("Printando valores do Vetor: {:?}", numero);

//variável.push(Valor) para inserir valor no vetor.
    numero.push(40);
    numero.push(60);

    println!("Printando e adicionando valores no Vetor: {:?}", numero);

    numero.pop(); // Remove o último elemento
    println!("Removendo o último elemento: {:?}", numero);

    numero.remove(1); // Remove o elemento no índice 1
    println!("Removendo o elemento no índice 1: {:?}", numero);
}