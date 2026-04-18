pub fn integral_riemann(a: f64, b: f64, n: u64, f: impl Fn(f64) -> f64) -> f64{
    let n_f64 = n as f64;

    let largura = (b-a)/n_f64;
    let mut x = 0.0;
    let mut area: f64 = 0.0;

    for i in 0..n{
        x = a + i as f64 * largura;
        area += f(x) * largura;
    }
    area
}