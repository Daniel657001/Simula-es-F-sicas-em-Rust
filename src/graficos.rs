use plotters::prelude::*;

pub fn grafico_funcao_onda(massa: f64, velocidade: f64) {

    let root = BitMapBackend::new("funcao_onda.png", (800, 600)).into_drawing_area();
    root.fill(&WHITE).unwrap();
        let mut chart = ChartBuilder::on(&root)
        .caption("Função de Onda |ψ|²", ("sans-serif", 30))
        .margin(10)
        .x_label_area_size(30)
        .y_label_area_size(30)
        .build_cartesian_2d(-10.0f64..10.0f64, 0.0f64..1.2f64)
        .unwrap();

    chart.configure_mesh().draw().unwrap();
    let momento = massa * velocidade;

    chart.draw_series(LineSeries::new(
        (-1000..=1000).map(|xi| {
            let x = xi as f64 / 100.0;
            let psi = (momento * x / 1.055e-34).cos().powi(2) + (momento * x / 1.055e-34).sin().powi(2);
            (x, psi)
        }),
        &RED,
    )).unwrap();

    root.present().unwrap();
}

