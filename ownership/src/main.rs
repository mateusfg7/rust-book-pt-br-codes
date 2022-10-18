fn main() {
    /*
     * let mut s = String::from("olá");
     * s.push_str(", mundo!"); // push_str() adiciona um literal à String
     * println!("{s}"); /* olá, mundo! */
     */

    /*
     * // Copia os dados na pilha mas mantém o ponteiro do valor na heap
     *
     * let mut a = String::from("Mateus");
     *
     * a.push_str(" Felipe"); /* success */
     * let b = a; // move `a` para `b` na pilha
     * a.push_str(" Gonçalves"); /* error */
     *
     * println!("{b}");
     */

    /*
     * // Copia profunda dos dados na pilha e do valor na heap
     *
     * let mut a = String::from("Mateus");
     *
     * a.push_str(" Felipe");
     * let b = a.clone(); // `clone()` clona `a` tanto da pilha quando na heap
     * a.push_str(" Gonçalves");
     *
     * println!("a: {a} | b: {b}");
     */

    /*
     * let a = String::from("texto");
     *
     * toma_posse(a);
     *
     * let x = 5;
     *
     * faz_uma_copia(x);
     */

    /*
     * let mut s = String::from("texto");
     *
     * let r1 = &mut s;
     * let r2 = &mut s; /* ERROR: cannot create two mutable borrow in the same scope */
     * println!("{r1} / {r2}")
     */

    /* FIXING PREVIOUS CODE */

    /*
     * let mut s = String::from("texto");
     *
     * {
     *     let r1 = &mut s;
     * } // here, `r1` is out of scope
     *
     * let r2 = &mut s;
     */

    /*
     * // Não podemos ter uma referência imutável enquanto temos uma mutável
     * let mut s = String::from("texto");
     *
     * let r1 = &s; // sem problema
     * let r2 = &s; // sem problema
     * let r3 = &mut s; // PROBLEMA GRANDE
     *
     * println!("{r1} - {r2} - {r3}")
     */

    // STRING SLICES

    /*
     * let s = String::from("texto longo");
     *
     * let texto = &s[0..5];
     * let longo = &s[6..11];
     */

    /*
     * let s = String::from("texto");
     *
     * let slice = &s[0..2];
     * let slice = &s[..2]; // the same
     */
}

/*
 * fn toma_posse(uma_string: String) {
 *     println!("{uma_string}")
 * }
 *
 * fn faz_uma_copia(um_inteiro: i32) {
 *     println!("{um_inteiro}")
 * }
 */

/*
 * fn retorna_primeira_palavra(s: &String) -> usize {
 *     let bytes = s.as_bytes();
 *
 *     for (index, &item) in bytes.iter().enumerate() {
 *         if item == b' ' {
 *             return index;
 *         }
 *     }
 *
 *     s.len()
 * }
 */
