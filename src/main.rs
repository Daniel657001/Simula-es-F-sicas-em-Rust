mod simulacao;
mod bola;
mod traits;
mod particula;
mod fisica;
mod fisica_quantica;
mod graficos;
mod matematica;

use matematica::*;
use traits::{Cinematica, Simulavel};
use simulacao::Simulacao;
use bola::Bola;
use crate::particula::Particula;
use fisica::forca_gravitacional;
use fisica_quantica::*;


fn main() {
    let particula = Particula{posicao: 12.0, velocidade: 15.0, aceleracao: 10.0, massa: 9.1e-31, forca: 40.0};
    let simulacao = Simulacao{tempo: vec![0.0, 1.0, 1.5, 5.0], objeto: particula };
    let (produto, delta_p_minimo, respeita) = principio_incerteza(&simulacao.objeto, 1e-10);
    let bola = Bola{posicao: 5.0, velocidade: 10.0, aceleracao: 9.8, massa: 2.0};
    let simulacao_bola = Simulacao{tempo: vec![0.0, 1.0, 2.0], objeto: bola};
    let (probabilidade, kappa) = efeito_tunel(&simulacao.objeto,1e-10 ,1e-19);
    let resultado_integral = integral_riemann(0.0, 2.0, 10000, |x| {integral_riemann(0.0, 3.0, 10000, |y| 2.0 * x + y)});
    graficos::grafico_funcao_onda(5.0, 15.0);
    println!(
    "=== FÍSICA CLÁSSICA ===\nPosição (t=3s): {} m\nVelocidade (t=3s): {} m/s\nEnergia Cinética: {} J\nEnergia Potencial (h=5m): {} J\nEnergia Mecânica: {} J\nForça: {} N\nVelocidades simuladas: {:?}\nPosições simuladas: {:?}\nForça Gravitacional (r=1m): {} N\n\n=== FÍSICA QUÂNTICA ===\nEnergia do Fotão (5e14 Hz): {} J\nProduto Δx×p: {}\nΔp mínimo: {}\nPrincípio de Heisenberg respeitado: {}\nFormula de Onda{}\nEquação de Schrodinger: {}\n energia na caixa: {}\nefeito tunel: {} e o kappa {}\nResultado da integral {}",
    simulacao_bola.objeto.posicao_em(3.0),
    simulacao.objeto.velocidade_em(3.0),
    simulacao.objeto.energia_cinetica(),
    simulacao.objeto.energia_potencial(5.0),
    simulacao.objeto.energia_mecanica(5.0),
    simulacao.objeto.forca(),
    simulacao.simular_velocidade(),
    simulacao.simular_posicao(),
    forca_gravitacional(&simulacao.objeto, &simulacao.objeto, 1.0),
    energia_fotao(5e14),
    produto,
    delta_p_minimo,
    respeita,
    funcao_onda(&simulacao.objeto),
    equacao_schrodinger(&simulacao.objeto),
    energia_caixa(&simulacao.objeto, 1e-10,  1),
    probabilidade,
    kappa,
    resultado_integral,

    )
}