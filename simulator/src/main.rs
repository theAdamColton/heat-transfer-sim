/**
 * Adam Colton 2021
 *
 * Runs the simulator on a variety of starting conditions
 */
mod entities;

use entities::*;

fn main() {
    let water_spec_heat = 4181.3f64;
    let steel_spec_heat = 466f64;

    let pipefluid = Fluid {
        mass: 4f64,
        specific_heat: water_spec_heat,
        temp: 25f64,
    };

    let tankfluid = Fluid {
        mass: 300f64,
        specific_heat: water_spec_heat,
        temp: 25f64,
    };

    let steelpipe = Pipe {
        length: 10f64,
        mass: 5f64,
        rad_int: 0.05f64,
        rad_ext: 0.07f64,
        temp: 1f64,
        specific_heat: 452f64,
        thermal_cond: 35f64,
        fluid: &pipefluid,
    };

    let tank = Tank {
        mass: 40f64,
        height: 3f64,
        radius: 1f64,
        specific_heat: 400f64,
        temp: 22f64,
        fluid: &tankfluid,
        pipe_in: &steelpipe,
        pipe_out: &steelpipe,
    };

    run_simulation(1e-6, 10f64, &steelpipe, &tank);
}

/**
 * Runs the simulation
 */
fn run_simulation(delta_t: f64, input_watts: f64, pipe: &Pipe, tank: &Tank) {

}
