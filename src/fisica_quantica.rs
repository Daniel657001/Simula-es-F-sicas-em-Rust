use crate::particula::Particula;
use num_complex::Complex;
use std::f64::consts::PI;


pub fn energia_fotao(hz: f64) -> f64{
    let energia_foton = 6.626e-34 * hz;
    energia_foton
}

pub fn principio_incerteza(p: &Particula, delta_x: f64) -> (f64, f64, bool){
    let momento_real = p.massa * p.velocidade;
    let delta_p = 1.055e-34 / (2.0 * delta_x);
    let respeita = (delta_x *  momento_real) >= 1.055e-34 / 2.0;
    (delta_x * momento_real, delta_p, respeita)
}

pub fn funcao_onda(p: &Particula) -> Complex<f64>{
    let real = ((p.massa * p.velocidade) * p.posicao / 1.055e-34).cos();
    let imaginaria = ((p.massa * p.velocidade) * p.posicao / 1.055e-34).sin();
    Complex::new(real, imaginaria).norm_sqr().into()
}

pub fn equacao_schrodinger(p: &Particula) ->f64{
    let energia = (p.massa * p.velocidade).powi(2) / (2.0 * p.massa);
    energia
}

pub fn energia_caixa(p: &Particula,tamanho: f64,n: u32) ->f64{
    let n_f = n as f64;
    let energy_box = n_f.powi(2) * PI.powi(2) * (1.055e-34_f64).powi(2) / (2.0*p.massa * tamanho.powi(2));
    energy_box
}

pub fn efeito_tunel(p: &Particula, largura: f64, energia_barreira: f64) -> (f64, f64){
    let kappa = (2.0 * p.massa * (energia_barreira-equacao_schrodinger(p))).sqrt() / 1.055e-34;
    let probabilidade = (-2.0 * kappa * largura).exp();
    (probabilidade, kappa)
}