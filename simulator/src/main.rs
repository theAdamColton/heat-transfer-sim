/**
 * Adam Colton 2021
 *
 * Runs the simulator on a variety of starting conditions
 */
mod entities;

use entities::*;

fn main() {
        let steelpipe = Pipe {
            length: 10f64,
            mass: 5f64,
            rad_int: 0.05f64,
            rad_ext: 0.07f64,
            temp: 1f64,
            specific_heat: 452f64,
            thermal_cond: 35f64,
        };

        let tank = Tank {
            mass: 40f64,
            height: 3f64,
            radius: 1f64,
            specific_heat: 400f64,
            liq_mass: 1f64,
            liq_specific_heat: 4182f64,
            liq_avgr_temp: 22f64,
            temp: 22f64,
        };


        let res = calculate_pipe_heat_rate(&steelpipe, &water);
        println!("{}", res);

}
