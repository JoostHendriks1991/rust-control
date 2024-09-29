use automatica::{
    plots::bode::Bode,
    units::{RadiansPerSecond, ToDecibel},
    Complex, Tf, Tfz,
};

use gnuplot::{AxesCommon, Figure};

#[allow(clippy::non_ascii_literal)]
fn main() {
    // let tf = Tf::new(Poly::<f64>::one(), Poly::new_from_roots(&[-1.]));
    let tf = Tf::new([-0.75_f64, 0.25], [0.75, 0.75, 1.]);

    println!("T:\n{}", tf);

    let c = tf.eval(&Complex::new(0., 1.));
    println!("\nEvaluation at i:");
    println!(
        "{} = {:.3}dB, {:.3}°",
        c,
        c.norm().to_db(),
        c.arg().to_degrees()
    );

    println!("\nBode Plot:");
    let b = Bode::new(tf, RadiansPerSecond(0.1), RadiansPerSecond(10.0), 0.1);
    let mut f: Vec<f64> = Vec::new();
    let mut m: Vec<f64> = Vec::new();
    let mut ph: Vec<f64> = Vec::new();
    for g in b.into_iter().into_db_deg() {
        f.push(g.angular_frequency().0);
        m.push(g.magnitude());
        ph.push(g.phase());
    }

    let mut fg = Figure::new();
    fg.set_multiplot_layout(2, 1);
    fg.axes2d()
        .set_title("Bode plot", &[])
        .set_x_label("Frequency", &[])
        .set_y_label("Magnitude", &[])
        .lines(f.clone(), m.clone(), &[]);
    fg.axes2d()
        .set_x_label("Frequency", &[])
        .set_y_label("Phase", &[])
        .lines(f.clone(), ph.clone(), &[]);
    fg.show_and_keep_running().unwrap();

    let k = 0.5;
    let tfz = Tfz::new([1. - k], [-k, 1.]);
    println!("\nDiscrete function T:\n{}\n", tfz);
    let pz = Bode::new_discrete(tfz, RadiansPerSecond(0.01), 0.1);
    f.clear();
    m.clear();
    ph.clear();
    for g in pz.into_iter().into_db_deg() {
        f.push(g.angular_frequency().0);
        m.push(g.magnitude());
        ph.push(g.phase());
    }

    let mut fg = Figure::new();
    fg.set_multiplot_layout(2, 1);
    fg.axes2d()
        .set_title("Bode plot", &[])
        .set_x_label("Frequency", &[])
        .set_y_label("Magnitude", &[])
        .lines(f.clone(), m.clone(), &[]);
    fg.axes2d()
        .set_x_label("Frequency", &[])
        .set_y_label("Phase", &[])
        .lines(f.clone(), ph.clone(), &[]);
    fg.show_and_keep_running().unwrap();
}
