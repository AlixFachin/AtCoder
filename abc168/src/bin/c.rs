use std::f64::consts::PI;

use proconio::input;

const DEBUG: bool = false;

fn theta_hr(h: f64, m: f64) -> f64 {
    return ( PI/6.0 )*(h + m/60.0);
}

fn theta_min(m: f64) -> f64 {
    return m*PI/30.0;
}

#[test]
fn test_angles() {
    assert_eq!(theta_hr(0.0,0.0),0.0);
    assert_eq!(theta_hr(6.0,0.0),PI);
    assert_eq!(theta_hr(9.0,0.0),3.0*PI/2.0);
    assert_eq!(theta_hr(3.0,0.0),PI/2.0);
    assert_eq!(theta_hr(3.0,15.0),PI/2.0 + PI/(6.0*4.0));

    assert_eq!(theta_min(0.0),0.0);
    assert_eq!(theta_min(30.0),PI);
    assert_eq!(theta_min(45.0),3.0*PI/2.0)
    
}

fn main() {
    input! {
        a: f64,
        b: f64,
        h: f64,
        m: f64,
    }

  


    let t_a = theta_hr(h, m);
    let t_b = theta_min(m);

    let x_a = a*(t_a.sin());
    let y_a = a*(t_a.cos());
    let x_b = b*(t_b.sin());
    let y_b = b*(t_b.cos());

    if DEBUG {
        println!("a:{},b:{},h:{},m:{}",a,b,h,m);
        println!("x_a:{},y_a:{},x_b:{},y_b:{}",x_a, y_a, x_b, y_b);
    }

    let d = ((y_b - y_a)*(y_b - y_a) + (x_b-x_a)*(x_b-x_a)).sqrt();

    println!("{:.20}",d)

}
