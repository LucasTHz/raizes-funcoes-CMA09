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

  pub fn xk(a: f32, b: f32) -> f32 {
      let dividendo = (a * fb(b)) - (b * fa(a));
      return dividendo / (fb(b) - fa(a));
  }

  pub fn fxk(xk: f32) -> f32 {
      let funcao_xk: f32 = f32::powf(xk / 2.0, 2.0) - xk.sin();
      return funcao_xk;
  }
}