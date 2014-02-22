extern mod cairo;

fn main() {
  use cairo;
  use cairo::surface;
  use cairo::surface::Surface;

  let (width, height) = (500.0, 500.0);
  let mut s = Surface::image(surface::format::ARGB32, width as i32, height as i32);

  let mut cairo = cairo::Cairo::new(&mut s);

  cairo.save();
  cairo.set_source_rgb(0.3, 0.3, 1.0);
  cairo.paint();
  cairo.restore();

  cairo.move_to(0.0, 0.0);
  cairo.line_to(2.0 * width / 6.0, 2.0 * height / 6.0);
  cairo.line_to(3.0 * width / 6.0, 1.0 * height / 6.0);
  cairo.line_to(4.0 * width / 6.0, 2.0 * height / 6.0);
  cairo.line_to(6.0 * width / 6.0, 0.0 * height / 6.0);
  cairo.close_path();
  cairo.save();
  cairo.set_line_width(6.0);
  cairo.stroke_preserve();
  cairo.set_source_rgb(0.3, 0.3, 0.3);
  cairo.fill();
  cairo.restore();

  cairo.save();
  cairo.set_line_width(6.0);
  cairo.arc(1.0 * width / 6.0, 3.0 * height / 6.0, 0.5 * width / 6.0, 0.0 * height / 6.0, 2.0 * std::f64::consts::PI);
  cairo.stroke_preserve();
  cairo.set_source_rgb(1.0, 1.0, 0.0);
  cairo.fill();
  cairo.restore();

  s.to_png("example2.png");
  s.finish();
}