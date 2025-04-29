use std::collections::HashMap;

fn main() {
    //Criação do HashMaps
    let mut estoque = HashMap::new();

    //1. Inserção de valores
    estoque.insert("banana", 100);
    estoque.insert("maça", 50);
    estoque.insert("pepino", 2);
    estoque.insert("caqui", 20);

    println!("{:?}", estoque);


    //2. Acessar de forma segura os valores do Hashmap
    if let Some(qtde) = estoque.get("maça") {
        println!("Temos {:?} maças em estoque!", qtde);
    }

    //3. Atualizando estoque com entry()
    estoque.entry("pepino").and_modify(|qtde| * qtde += 100);
    if let Some(qtde) = estoque.get("pepino"){
        println!("Temos {:?} pepino em estoque!", qtde);
    }

    //4. Deletando o caqui
    estoque.remove("caqui");
    println!("{:?})", estoque);

    //5. Filtrar todas as frutas acima de 100 unidade
    estoque.retain(|fruta, &mut qtd| qtd >100);
    println!("{:?}", estoque);

    //6. Limpeza total!
    estoque.clear();
    println!("{:?}", estoque);

}