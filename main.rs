use std::f64::consts::PI;

fn main() {
    let lut_0 = create_lut();
    println!("{:?}", lut_0);
    println!("_ __ _ __ _ __ _ __ _ __ _ __ _ __ _ __ _ __ _ __ _ __ _ __ _ __ _ __ _ __ _ __ _ __ _ __ _ __ _ __ _ __ _ __ _ __ _ __ _ __ _\n");
    compare_result()
}

// calculate the difference between two values in percentage
fn dif_per(a:f64,b:f64) -> f64 {
    let diff:f64 = ((a - b)/b) * 100.0;
    return diff;
}

// formula given in the instruction
fn formula_0(l:f64, l_0:f64, u:f64, u_0:f64, unk:f64) -> f64 {
    let val = l_0 + ((u_0 - l_0) * (unk - l as f64)) / (u as f64 - l as f64);
    return val;
}

// create a lookup table for sin
const N: usize = 360;
fn create_lut() -> [f64; N] {
    let mut arr:[f64;N] = [0.0; N];  
    for i in 0..N {
        let sin_val:f64 = ((i as f64) * (PI as f64/180 as f64)).sin();
        arr[i] = sin_val;

    }
    return arr;
}

// convert neg deg to pos with the same quadrat
// make degree to be in the range between 0 and 360
const F: usize = 10;
fn check_neg() -> [f64; F] {
    let arr_1:[f64;F] = [0.0, 0.234, 5.856, 10.100, 19.34, 63.235, -145.7, -65.89, 390.0, -3218.321];
    let mut arr_1_checked:[f64;F] = [0.0; F];
    let mut count = 0;
    for i in arr_1.iter() {
        let mut varia = i.clone();
        while varia < 0.0{
            varia += 360.0;
        }
        while varia >= 360.0{
            varia -= 360.0;
        }
        arr_1_checked[count] = varia;
        count = count + 1;
    }
    return arr_1_checked;
}

// compare the result of the formula with the result of the lookup table
// and print the result
fn compare_result() {
    let lut_1 = create_lut();
    let arr_1 = check_neg();
    for i in arr_1 {
        let mut sin_c_val:f64 = (180.0 /2.0)- (i as f64 * (180.0 / 180.0));
        while sin_c_val < 0.0 {
            sin_c_val += 360.0;
        }
        let upper_0:usize = i as usize + 1;
        let lower_0:usize = i as usize;
        let upper_1:f64 = lut_1[upper_0];
        let lower_1:f64 = lut_1[lower_0];
        let upper_0_cos:usize = sin_c_val as usize + 1;
        let lower_0_cos:usize = sin_c_val as usize;
        let upper_1_cos:f64 = lut_1[upper_0_cos];
        let lower_1_cos:f64 = lut_1[lower_0_cos];
        let prd_va:f64 = formula_0(lower_0 as f64, lower_1,upper_0 as f64,upper_1,i);
        let prd_va_cos:f64 = formula_0(lower_0_cos as f64, lower_1_cos,upper_0_cos as f64,upper_1_cos, sin_c_val);
        let prd_va_tan:f64 = prd_va / prd_va_cos;
        let act_sin:f64 = (i * PI as f64/180.0).sin();
        let act_cos:f64 = (i * PI as f64/180.0).cos();
        let act_tan:f64 = (i * PI as f64/180.0).tan();
        println!("upper index:{}  lower index:{}", upper_0, lower_0);
        println!("upper val:{}  lower val:{}", upper_1, lower_1);
        println!("approximated val(sin):{:.16}   |   actual value(sin):{:.16}   |  diff:{:.8}%\n", prd_va, act_sin, dif_per(prd_va, act_sin));
        println!("upper index:{}  lower index:{}", upper_0_cos, lower_0_cos);
        println!("upper val:{}  lower val:{}", upper_1_cos, lower_1_cos);
        println!("approximated value(cos): {:.16}   |   actual value(cos):{:.16}   | diff:{:.8}%\n", prd_va_cos, act_cos ,dif_per(prd_va_cos, act_cos));
        println!("approximated value(tan): {:.16}   |   actual value(tan):{:.16}   | diff:{:.8}%", prd_va_tan, act_tan ,dif_per(prd_va_tan, act_tan));
        println!("- - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -")    

    }
}




