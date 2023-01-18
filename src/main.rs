//This program was written by Yazeed M. Hakami, for BE1610.
use std::num; // random library call, really didn't need this in the end but it was good having it!
use std::f64;
const euler: f64 = std::f64::consts::E; // Euler's number, "e"

fn main() {

    let capacitence: f64 = 0.001; // Maximum capacitence on the capacitor. It's denoted on the side, by the "1000uF".
    let resistence: f64 = 100000.0; // Luv me resistors, simple as.

    let voltage: f64 = 5.0; // Voltage applied by power source during lab.

    for i in (0..=500).step_by(50) {
        let Ic: f64 = (voltage)/resistence * euler.powf(-i as f64/(resistence*capacitence));
        println!("Discharge capacitance at point {} is {}",i,Ic*10.0_f64.powf(5 as f64)); // Multiplied by 10*5 I don't get some teensy tiny number, my eyes are fragile!
        //Should also add that because I'm multiplying by 10^5, actual results are 10^-5
    }

    for i in (0..=500).step_by(50) {
        let Vc: f64 = voltage*(1.0 - euler.powf(-i as f64/(resistence*capacitence)));
        println!("Charge capacitance at point {} is {}",i,Vc);
    }

    //Decided to do this in code instead of by hand because of my extensive background in programming, and because it was significantly easier than doing the same operation 5 times ;)

}