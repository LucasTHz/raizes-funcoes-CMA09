// A função utilizada é f(x) = (x/2)² - sin x
// Calculo realizado através do método da falsa posicao
pub mod false_position {
    pub fn fa(a: f32) -> f32 {
        let funcao_a = f32::powf(a / 2.0, 2.0) - a.sin();
        return funcao_a;
    }

    // método que calcula o F(b)
    pub fn fb(b: f32) -> f32 {
        let funcao_b = f32::powf(b / 2.0, 2.0) - b.sin();
        return funcao_b;
    }

    // método que calcula o xk
    pub fn xk(a: f32, b: f32) -> f32 {
        let dividendo = (a * fb(b)) - (b * fa(a));
        return dividendo / (fb(b) - fa(a));
    }

    // método que calcula o F(xk)
    pub fn fxk(xk: f32) -> f32 {
        let funcao_xk: f32 = f32::powf(xk / 2.0, 2.0) - xk.sin();
        return funcao_xk;
    }
}

pub mod bissecao {
    // A função utilizada é f(x) = (x/2)² - sin x
    // Calculo realizado através do método da bissecção

    // método que calcula o F(a)
    pub fn fa(a: f32) -> f32 {
        let funcao_a = f32::powf(a / 2.0, 2.0) - a.sin();
        return funcao_a;
    }

    // método que calcula o F(b)
    pub fn fb(b: f32) -> f32 {
        let funcao_b = f32::powf(b / 2.0, 2.0) - b.sin();
        return funcao_b;
    }

    // método que calcula o xk1.5
    pub fn xk(a: f32, b: f32) -> f32 {
        let xk = (a + b) / 2.0;
        return xk;
    }

    // método que calcula o F(xk)
    pub fn fxk(xk: f32) -> f32 {
        let funcao_xk: f32 = f32::powf(xk / 2.0, 2.0) - xk.sin();
        return funcao_xk;
    }
}
