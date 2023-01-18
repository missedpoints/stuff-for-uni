//This program was written by Yazeed M. Hakami, for BE1610.
use std::num; // random library call, really didn't need this in the end but it was good having it!
use std::f64;
use std::io;
const euler: f64 = std::f64::consts::E; // Euler's number, "e"

fn main() {

    let capacitence: f64 = 0.001; // Maximum capacitence on the capacitor. It's denoted on the side, by the "1000uF".
    let resistence: f64 = 100000.0; // Luv me resistors, simple as. (this is in Ohms.)

    println!("Please input your desired voltage in decimal form.");

    let mut voltage = String::new();
    io::stdin().read_line(&mut voltage).unwrap();
    let voltage: f64 = voltage.trim().parse().unwrap(); // Voltage applied by power source during lab. (it was 5 during the practical test)

    println!("Please input your desired time increment in integer form."); // 
    println!("Output will be given in Amperes.");
    let mut step = String::new();
    io::stdin().read_line(&mut step).unwrap();
    let step: f64 = step.trim().parse().unwrap();

    for i in (0..=(voltage*(10 as f64)) as i64).step_by(step as usize) {
        let Ic: f64 = (voltage)/resistence * euler.powf(-i as f64/(resistence*capacitence));
        println!("Discharge capacitance at point {} is {}",i,Ic*10.0_f64.powf(5 as f64).abs()); // Multiplied by 10^5 so I don't get some teensy tiny number, my eyes are fragile!
        //Should also add that because I'm multiplying by 10^5, actual results are 10^-5
    }

    for i in (0..=(voltage*(10 as f64)) as i64).step_by(step as usize) {
        let Vc: f64 = voltage*(1.0 - euler.powf(-i as f64/(resistence*capacitence)));
        println!("Charge capacitance at point {} is {}",i,Vc.abs());
    }
    //Decided to do this in code instead of by hand because of my extensive background in programming, and because it was significantly easier than doing the same operation 10 times ;)
    
    // All outputed data is in Ampere.

}