// Contrato para qualquer objeto que possa ser simulado no tempo
pub trait Simulavel{
    fn simular_posicao(&self) -> Vec<f64>;   // posição em cada instante
    fn simular_velocidade(&self) -> Vec<f64>; // velocidade em cada instante
}

// Contrato para qualquer objeto com propriedades cinemáticas e energéticas
pub trait Cinematica{
    fn posicao_em(&self, t: f64) -> f64;        // x(t) em metros
    fn velocidade_em(&self, t: f64) -> f64;     // v(t) em m/s
    fn energia_cinetica(&self) -> f64;           // Ec em Joules (J)
    fn energia_potencial(&self, h: f64) -> f64;  // Ep em Joules (J)
    fn energia_mecanica(&self, h: f64) -> f64;   // Em = Ec + Ep em Joules (J)
    fn forca(&self) -> f64;                      // F em Newtons (N)
}