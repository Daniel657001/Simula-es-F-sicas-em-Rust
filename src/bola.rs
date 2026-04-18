use crate::traits::Cinematica;

pub struct Bola{
    pub posicao: f64,    // x₀ — posição inicial em metros (m)
    pub velocidade: f64, // v₀ — velocidade inicial em metros por segundo (m/s)
    pub aceleracao: f64, // a — aceleração em metros por segundo² (m/s²)
    pub massa: f64,
}


impl Cinematica for Bola{
        fn posicao_em(&self, t: f64) -> f64 {
        // Lei do Movimento Uniformemente Acelerado (MUA)
        // x = x₀ + v₀t + ½at²
        // x₀ = posição inicial
        // v₀t = distância percorrida pela velocidade inicial (movimento uniforme)
        // ½at² = distância extra causada pela aceleração (efeito quadrático do tempo)
        let d_final = self.posicao + (self.velocidade * t) + 0.5*(self.aceleracao * t.powi(2));
        d_final
    }
    fn velocidade_em(&self, t: f64) -> f64 {
        // Lei da velocidade no MUA
        // v = v₀ + at
        // v₀ = velocidade inicial
        // at = variação de velocidade causada pela aceleração
        // resultado: 15 + 10×3 = 45 m/s
        let velocidade_final = self.velocidade + (self.aceleracao * t);
        velocidade_final
    }
    fn energia_cinetica(&self) -> f64{
        // Ec = ½mv²
        // energia que a partícula tem por estar em movimento
        // quanto maior a massa e velocidade, maior a energia cinética
        // resultado: ½ × 5 × 15² = 562.5 J
        let energia_cinetic = 0.5 * (self.massa * self.velocidade.powi(2));
        energia_cinetic
    }
    fn energia_potencial(&self, h: f64) -> f64{
        // Ep = mgh
        // energia que a partícula tem pela sua altura (h) no campo gravitacional
        // g = 9.8 m/s² (aceleração gravitacional da Terra)
        // resultado: 5 × 9.8 × 5 = 245 J
        let energia_potencia = self.massa * 9.8 * h;
        energia_potencia
    }
    fn energia_mecanica(&self, h: f64) -> f64{
        // Em = Ec + Ep
        // energia total do sistema — num sistema sem atrito é sempre conservada
        // (Princípio da Conservação da Energia)
        // resultado: 562.5 + 245 = 807.5 J
        let energia_mecanic = self.energia_cinetica() + self.energia_potencial(h);
        energia_mecanic
    }
    fn forca(&self) -> f64{
        // Segunda Lei de Newton: F = ma
        // a força é o que causa a aceleração numa massa
        // resultado: 5 × 10 = 50 N
        let forca = self.massa * self.aceleracao;
        forca
    }

}