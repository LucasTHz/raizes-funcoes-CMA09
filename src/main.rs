mod lib;
use lib::*;
use std::io;
#[macro_use]
extern crate prettytable;
use prettytable::Table;

fn main() {
    loop {
        println!("Digite o valor de a");
        let mut a = String::new();

        // trata o erro de entrada
        io::stdin().read_line(&mut a).expect("Falha ao ler entrada");
        let mut a: f32 = match a.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Por favor, digite um número!");
                continue;
            }
        };

        println!("Digite o valor de b");
        let mut b = String::new();

        io::stdin().read_line(&mut b).expect("Falha ao ler entrada");
        let mut b: f32 = match b.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Por favor, digite um número!");
                continue;
            }
        };

        println!("Digite o valor de parada");
        let mut parada: String = String::new();

        io::stdin()
            .read_line(&mut parada)
            .expect("Falha ao ler entrada");

        let mut parada: f32 = match parada.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Por favor, digite um número!");
                continue;
            }
        };

        println!("Digite o valor de tolerancia");
        let mut tolerancia: String = String::new();

        io::stdin()
            .read_line(&mut tolerancia)
            .expect("Falha ao ler entrada");

        let tolerancia: f32 = match tolerancia.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Por favor, digite um número!");
                continue;
            }
        };

        let mut parada2: f32 = parada;
        let mut table = Table::new();
        let mut table1 = Table::new();

        let mut a_falsa_position: f32 = a;
        let mut b_falsa_position: f32 = b;

        println!("\n Metodo da bissecao");
        table.add_row(row!["a", "b", "xk", "f(xk)", "f(a)", "f(b)"]);
        loop {
            let raiz = bissecao::xk(a, b); // Importar um modulo com as funções
            let fa = bissecao::fa(a);
            let fb = bissecao::fb(b);
            let fxk = bissecao::fxk(raiz);
            table.add_row(row![a, b, raiz, fxk, fa, fb]);

            if (fa * fxk) < 0.0 {
                b = raiz;
            } else {
                a = raiz;
            }

            parada -= 1.0;

            if parada == 0.0 || f32::abs(fxk) < tolerancia {
                table.printstd();
                break;
            }
        }

        println!("\n Metodo da Falsa posicao");
        table1.add_row(row!["a", "b", "xk", "f(xk)", "f(a)", "f(b)"]);
        loop {
            // Importar um modulo com funções
            let raiz_falsa = false_position::xk(a_falsa_position, b_falsa_position);
            let fa = false_position::fa(a_falsa_position);
            let fb = false_position::fb(b_falsa_position);
            let fxk = false_position::fxk(raiz_falsa);
            table1.add_row(row![
                a_falsa_position,
                b_falsa_position,
                raiz_falsa,
                fxk,
                fa,
                fb
            ]);

            if (fa * fxk) < 0.0 {
                b_falsa_position = raiz_falsa;
            } else {
                a_falsa_position = raiz_falsa;
            }

            parada2 -= 1.0;

            if parada2 >= 10.0 || f32::abs(fxk) < tolerancia {
                table1.printstd();
                break;
            }
        }
        println!("Deseja continuar? (s/n)"); // Perguntar se o usuário deseja continuar
        let mut continuar = String::new();
        io::stdin()
            .read_line(&mut continuar)
            .expect("Falha ao ler entrada");

        if continuar.trim() == "n" {
            break;
        };
    }
}
