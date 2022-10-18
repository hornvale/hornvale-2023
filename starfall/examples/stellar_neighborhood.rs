use plotters::prelude::*;
const OUT_FILE_NAME: &'static str = "examples/output-stellar-neighborhood.svg";

use starfall::astronomy::stellar_neighborhood::constants::STELLAR_NEIGHBORHOOD_RADIUS;
use starfall::astronomy::stellar_neighborhood::constraints::Constraints as StellarNeighborhoodConstraints;

fn main() -> Result<(), Box<dyn std::error::Error>> {
  let area = SVGBackend::new(OUT_FILE_NAME, (1024, 760)).into_drawing_area();

  area.fill(&WHITE)?;

  let constraints = StellarNeighborhoodConstraints::habitable();
  let radius = constraints.radius.unwrap_or(STELLAR_NEIGHBORHOOD_RADIUS);

  let x_axis = (-radius..radius).step(1.0);
  let y_axis = (-radius..radius).step(1.0);
  let z_axis = (-radius..radius).step(1.0);

  let mut rng = rand::thread_rng();
  let stellar_neighborhood = constraints.generate(&mut rng).unwrap();
  let points = stellar_neighborhood
    .neighbors
    .iter()
    .map(|neighbor| neighbor.coordinates)
    .collect::<Vec<(f64, f64, f64)>>();
  let mut chart = ChartBuilder::on(&area)
    .caption(format!("Stellar Neighborhood"), ("sans", 20))
    .build_cartesian_3d(x_axis.clone(), y_axis.clone(), z_axis.clone())?;
  chart.with_projection(|mut pb| {
    pb.yaw = 0.5;
    pb.scale = 1.0;
    pb.into_matrix()
  });
  chart
    .configure_axes()
    .light_grid_style(BLACK.mix(0.10))
    .max_light_lines(3)
    .draw()?;
  chart.draw_series(PointSeries::of_element(points, 5, &RED, &|c, s, st| {
    return EmptyElement::at(c)
      + Circle::new((0, 0), s, st.filled())
      + Text::new(
        format!("({:.1}, {:.1}, {:.1})", c.0, c.1, c.2),
        (10, 0),
        ("sans-serif", 10).into_font(),
      );
  }))?;
  chart.configure_series_labels().border_style(&BLACK).draw()?;
  area.present().expect("Unable to write result to file!");
  println!("Result has been saved to {}", OUT_FILE_NAME);
  Ok(())
}

#[test]
fn entry_point() {
  main().unwrap()
}
