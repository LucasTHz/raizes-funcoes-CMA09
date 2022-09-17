mod lib;
use std::io;
#[macro_use] extern crate prettytable;
use prettytable::{Table};

fn main() {
    println!("Digite o valor de a");
    let mut a = String::new();
    
    io::stdin().read_line(&mut a).expect("Falha ao ler entrada");
    
    println!("Digite o valor de b");
    let mut b = String::new();
    
    io::stdin().read_line(&mut b).expect("Falha ao ler entrada");
    
    let mut a: f32 = a.trim().parse().expect("Por favor, digite um número!");
    
    let mut b: f32 = b.trim().parse().expect("Por favor, digite um número!");
    
    let mut i: f32 = 1.0;
    let tolerancia: f32 = 0.0001;
    let mut table = Table::new();
    let mut table1 = Table::new();

    let mut a_falsa_position: f32 = a;
    let mut b_falsa_position: f32 = b;
    
    println!("\n Metodo da bissecao");
    table.add_row(row!["a", "b", "xk", "f(xk)", "f(a)", "f(b)"]);
    loop {
        let raiz = lib::bissecao::xk(a, b); // Importar um modulo com funções
        let fa = lib::bissecao::fa(a); // Importar um modulo com funções
        let fb = lib::bissecao::fb(b); // Importar um modulo com funções
        let fxk = lib::bissecao::fxk(raiz); // Importar um modulo com funções
        table.add_row(row![a, b, raiz, fxk, fa, fb]);
        
        if (fa * fxk) < 0.0 {
            b = raiz;
        } else {
            a = raiz;
        }
        
        i += 1.0;
        
        if i >= 10.0 || f32::abs(fxk) < tolerancia {
            table.printstd();
            break;
        }
    }

    i = 1.0;
    println!("\n Metodo da Falsa posicao");
    table1.add_row(row!["a", "b", "xk", "f(xk)", "f(a)", "f(b)"]);
    loop {
        let raiz_falsa = lib::false_position::xk(a_falsa_position, b_falsa_position); // Importar um modulo com funções
        let fa = lib::false_position::fa(a_falsa_position); // Importar um modulo com funções
        let fb = lib::false_position::fb(b_falsa_position); // Importar um modulo com funções
        let fxk = lib::false_position::fxk(raiz_falsa); // Importar um modulo com funções
        table1.add_row(row![a_falsa_position, b_falsa_position, raiz_falsa, fxk, fa, fb]);

        if (fa * fxk) < 0.0 {
            b_falsa_position = raiz_falsa;
        } else {
            a_falsa_position = raiz_falsa;
        }

        i += 1.0;

        if i >= 10.0 || f32::abs(fxk) < tolerancia {
            table1.printstd();
            break;
        }
    }
}
