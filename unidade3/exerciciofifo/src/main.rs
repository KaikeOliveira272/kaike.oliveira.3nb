use std::collections::VecDeque;

fn main (){
    //Criando uma nova fila utilizando VecDeque
    let mut fila: VecDeque<i32> = VecDeque::new();

    println!("Impletação de fila FIFO utilizando o VecDeque");

    println!("Inserindo elementos na fila");
    fila.push_back(10);
    println!("Inserindo elemento 20 na fila");
    fila.push_back(20);
    println!("Inserindo elemento 30 na fila");
    fila.push_back(30);
    println!("Inserindo elemento 40 na fila");
    fila.push_back(40);


    //Verificando tamanho da fila
    println!("\n O tamanho da fila é {}", fila.len());


    // Removendo elementos da fila
    println!("\nRemovendo elementos da fila:");
    while let Some(elemento) = fila.pop_front() {
        println!("Removido: {}", elemento);
    }

    //Verificando primeiro valor da fila
     if let Some(&frente) = fila.front() {
        println!("O elemento na frente da fila é: {}", frente);
    } else {
        println!("A fila está vazia.");
    }


    //Verificar se fila está vazia
    println!("Fila está vazioazona? { }", fila.is_empty());

}
