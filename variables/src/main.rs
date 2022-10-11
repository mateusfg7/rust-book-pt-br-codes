fn main() {
    // let x = 5;
    // println!("O valor de x é: {}", x);
    // x = 6; /* error */
    // println!("O valor de x é: {}", x);
    //
    // let mut x = 5;
    // println!("O valor de x é: {}", x);
    // x = 6;
    // println!("O valor de x é: {}", x);

    // Constantes são sempre imutáveis e em maiúscula por convenção
    // const PONTOS_MAXIMOS: u32 = 100_000;

    // Shadowing
    //
    // let x = 5;
    // let x = x + 1;
    // let x = x * 2;
    // println!("O valor de x é: {}", x);
    //
    // let espacos = "   ";
    // let espacos = espacos.len();
    //
    // let mut espacos = "   ";
    // espacos = espacos.len(); /* error */
    //

    // TIPOS DE DADOS
    // Tupla

    // let tup: (i32, f64, u8) = (500, 6.4, 1);
    //
    // let (x, y, z) = tup; // desestruturação
    //
    // println!("O valor de  é {y}");

    // let x: (i32, f64, u8) = (500, 6.4, 1);
    //
    // let quinhentos = x.0; // pegar um valor de uma tupla pelo índice com o `.`
    // let seis_ponto_quatro = x.1;
    // let um = x.2;

    // Matrizes
    // diferente das tuplas, matrizes devem ter sempre o mesmo tipo, e uma vez
    // declarado, ela não pode alterar seu tamanho.
    //
    // let a = [1, 2, 3, 4, 5];
    //
    // Uma matriz não é tão flexível quanto um vetor, o vetor tem um tipo semelhante,
    // e é permitido aumentar e diminuir seu tamanho. Um exemplo de uso da matrix é
    // quando se precisa de valores fixos, como meses de um ano.
    //
    // let _meses = [
    //     "Janeiro",
    //     "Fevereiro",
    //     "Março",
    //     "Abril",
    //     "Maio",
    //     "Junho",
    //     "Julho",
    //     "Agosto",
    //     "Setembro",
    //     "Outubro",
    //     "Novembro",
    //     "Dezembro",
    // ];

    // Para acessar um valor na matriz, basta passar o índice do valor com `[]`.
    //
    // let a = [1, 2, 3, 4, 5];
    // let primeiro = a[0];
    // let segundo = a[1];
}
