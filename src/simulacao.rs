use crate::traits::Cinematica;
use crate::traits::Simulavel;

// Agrupa a partícula e os instantes de tempo a simular
pub struct Simulacao<T: Cinematica>{
    pub objeto: T,
    pub tempo: Vec<f64>, // lista de instantes em segundos (s)
}

impl<T: Cinematica> Simulavel for Simulacao<T>{
    fn simular_posicao(&self) -> Vec<f64>{
        // calcula x(t) para cada instante da lista
        // resultado: [12.0, 32.0, 45.75, 212.0] metros
        // t=0: 12 + 0 + 0 = 12.0 m
        // t=1: 12 + 15 + 5 = 32.0 m
        // t=1.5: 12 + 22.5 + 11.25 = 45.75 m
        // t=5: 12 + 75 + 125 = 212.0 m
        let mut lista_posicao: Vec<f64> = Vec::new();
        for i in &self.tempo{
            let resultado = self.objeto.posicao_em(*i);
            lista_posicao.push(resultado);
        }
        lista_posicao
    }
    fn simular_velocidade(&self) -> Vec<f64>{
        // calcula v(t) para cada instante da lista
        // resultado: [15.0, 25.0, 30.0, 65.0] m/s
        // t=0: 15 + 0 = 15.0 m/s
        // t=1: 15 + 10 = 25.0 m/s
        // t=1.5: 15 + 15 = 30.0 m/s
        // t=5: 15 + 50 = 65.0 m/s
        let mut lista_velocidade: Vec<f64> = Vec::new();
        for i in &self.tempo{
            let resultado = self.objeto.velocidade_em(*i);
            lista_velocidade.push(resultado);
        }
        lista_velocidade
    }
}