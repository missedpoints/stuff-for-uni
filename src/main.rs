//This program was written by Yazeed M. Hakami, for BE1610.
use std::num;
use std::f64;
const euler: f64 = std::f64::consts::E;

fn main() {


    let capacitence: f64 = 0.001;
    let resistence: f64 = 100000.0;

    let voltage: f64 = 5.0;



    for i in (0..=500).step_by(50) {
        let Ic: f64 = (voltage)/resistence * euler.powf(-i as f64/(resistence*capacitence));
        println!("Discharge capacitance at point {} is {}",i,Ic*10.0_f64.powf(5 as f64));
    }
    for i in (0..=500).step_by(50) {
        let Vc: f64 = voltage*(1.0 - euler.powf(-i as f64/(resistence*capacitence)));
        println!("Charge capacitance at point {} is {}",i,Vc);
    }
}