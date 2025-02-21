//Importamos a função reverse ciada no módulo "reverse_string"
use reverse_string::reverse; 

#[test] // Definir um macro para rodarmos o código individualmente 
fn palavra_invertida(){
    let input = "Teste"; //Entrada a ser revertida
    let expected = "etseT";
    assert_eq!(reverse(input), expected); // Aqui como pedido no exercicio, comparamos se o resultado da inversão é igual o "expected"
}
    #[test] // Outro teste
    fn palavra_vazio() {
        let input = " ";
        let expected = " ";
        assert_eq!(reverse(input), expected);
}
   
    #[test]
    fn palindromo() {
        let input = "radar";
        let expected = "radar";
        assert_eq!(reverse(input), expected);

}