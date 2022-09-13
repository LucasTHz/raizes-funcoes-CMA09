use std::io;

fn fa(a: f32) -> f32{
    let mut funcao_a = f32::powf(a / 2.0, 2.0) - a.sin();
    return funcao_a;
}

fn fb(b: f32) -> f32 {
    let mut funcao_b = f32::powf(b / 2.0, 2.0) - b.sin();
    return funcao_b;
}

fn xk(a: f32, b: f32) -> f32 {
    let mut xk = (a + b) / 2.0;
    return xk;
}

fn fxk(xk: f32) -> f32 {
    let mut funcao_xk: f32 = f32::powf(xk / 2.0, 2.0) - xk.sin();
    return funcao_xk;
}

fn main() {
    println!("Digite o valor de a");
    let mut a = String::new();

    io::stdin().read_line(&mut a)
        .expect("Falha ao ler entrada");

    println!("Digite o valor de b");
    let mut b = String::new();

    io::stdin().read_line(&mut b)
        .expect("Falha ao ler entrada");

    
    let mut a: f32 = a.trim().parse()
        .expect("Por favor, digite um número!");

    let mut b: f32 = b.trim().parse()
        .expect("Por favor, digite um número!");

    let mut i: f32 = 1.0;

    while i <= 10.0 {
        println!("{} interação", i);
        println!("a = {}", a);
        println!("b = {}", b);
        let mut raiz = xk(a,b);
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
    }

}
