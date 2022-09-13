mod lib;
use std::io;

// A função utilizada é f(x) = (x/2)² - sin x
// Calculo realizado através do método da bissecção

// método que calcula o F(a)
fn fa(a: f32) -> f32 {
    let funcao_a = f32::powf(a / 2.0, 2.0) - a.sin();
    return funcao_a;
}

// método que calcula o F(b)
fn fb(b: f32) -> f32 {
    let funcao_b = f32::powf(b / 2.0, 2.0) - b.sin();
    return funcao_b;
}

// método que calcula o xk1.5
fn xk(a: f32, b: f32) -> f32 {
    let xk = (a + b) / 2.0;
    return xk;
}

// método que calcula o F(xk)
fn fxk(xk: f32) -> f32 {
    let funcao_xk: f32 = f32::powf(xk / 2.0, 2.0) - xk.sin();
    return funcao_xk;
}

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
    // lib::false_position::ft(); Importar um modulo com funções

    loop {
        println!("{} interação", i);
        println!("a = {}", a);
        println!("b = {}", b);
        let mut raiz = xk(a, b);
        let mut fa = fa(a);
        let mut fb = fb(b);
        let mut fxk = fxk(raiz);

        if (fa * fxk) < 0.0 {
            b = raiz;
        } else {
            a = raiz;
        }
        println!("Raiz: {}", raiz);
        println!("F(a): {}", fa);
        println!("F(b): {}", fb);
        println!("F(xk): {} \n", fxk);
        i += 1.0;

        if i >= 10.0 || f32::abs(fxk) < tolerancia {
            break;
        }
    }
}
