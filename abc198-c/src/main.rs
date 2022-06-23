use proconio::input;

fn main() {
    input! {
        r: f64,
        x: f64,
        y: f64,
    }

    let dis: f64 = (x * x + y * y).sqrt();

    if dis % r == 0.0 {
        println!("{}", (dis / r) as i32);
    } else {
        println!("{}", (dis / r).ceil() as i32 + 1);
    }
}
