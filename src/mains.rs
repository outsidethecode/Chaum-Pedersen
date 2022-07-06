use mod_exp::mod_exp;


fn main() {

    let p : i32 = 23;
    let q : i32 = 11;
    let g : i32 = 4;
    let h : i32 = 9;

    let x : i32 = 6;
    let y1 : i32 = mod_exp(g, x, p);
    let y2 : i32 = mod_exp(h, x, p);

    println!("y1={}", y1);
    println!("y2={}", y2);

    let k : i32 = 7;

    let r1 : i32 =  mod_exp(g, k, p);
    let r2 : i32 =  mod_exp(h, k, p);
    println!("r1={}", r1);
    println!("r2={}", r2);

    let c : i32 = 4;
    let s : i32 = k-c*x % q;
    println!("s={}", s);

    let rr1 : i32 = mod_exp(g.pow(s as u32) * y1.pow(c as u32), 1, p);
    let rr2 : i32 = mod_exp(h.pow(s as u32) * y2.pow(c as u32), 1, p);
    println!("rr1={}", rr1);
    println!("rr2={}", rr2);
}