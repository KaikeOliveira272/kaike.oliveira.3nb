pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}

//Criando uma função pública 

pub fn reverse(input: &str) -> String {
    

//.chars(): Converte a string em um iterador de caracteres
//.rev(): Reverte a ordem dos caracteres.
//.collect(): Junta os caracteres novamente em uma String.
    input.chars().rev().collect()
}
