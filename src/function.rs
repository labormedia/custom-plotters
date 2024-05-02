use crate::DrawResult;
use plotters::prelude::*;
use plotters_canvas::CanvasBackend;

pub fn draw_function(canvas_id: &str, exponent: f32) -> DrawResult<impl Fn((i32, i32)) -> Option<(f32, f32)>> {
    let backend = CanvasBackend::new(canvas_id).expect("cannot find canvas");
    let root = backend.into_drawing_area();
    let font: FontDesc = ("sans-serif", 20.0).into();

    root.fill(&WHITE)?;

    let mut chart = ChartBuilder::on(&root)
        .margin(20u32)
        .caption(format!("U=x^{}*y^{}", exponent, 1.0-exponent), font)
        .x_label_area_size(30u32)
        .y_label_area_size(30u32)
        .build_cartesian_2d(-1f32..1f32, -1.2f32..1.2f32)?;

    chart.configure_mesh().x_labels(3).y_labels(3).draw()?;

    chart.draw_series(LineSeries::new(
        (-100..=100)
            .map(|s| s as f32 / 100.0)
            .map(|s| (s, 2_f32.powf(exponent)*2_f32.powf(1.0-exponent))),
        &GREEN,
    ))?;

    root.present()?;
    Ok(chart.into_coord_trans())
}