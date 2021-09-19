/**
 * Adam Colton 2021
 *
 * Runs the simulator on a variety of starting conditions
 */
mod entities;

use entities::*;

fn main() {
    let steel_spec_heat = 466f64;
    let steel_thermal_cond = 35f64;

    let pipewater = Water {
        mass: 4f64,
        temp: 25f64,
    };

    let tankfluid = Water {
        mass: 300f64,
        temp: 25f64,
    };

    let mut steelpipe = Pipe {
        length: 10f64,
        mass: 5f64,
        rad_int: 0.05f64,
        rad_ext: 0.07f64,
        temp: 25f64,
        specific_heat: 452f64,
        thermal_cond: 35f64,
        water: pipewater,
    };

    let mut steelpipe_out = Pipe {
        length: 10f64,
        mass: 5f64,
        rad_int: 0.05f64,
        rad_ext: 0.07f64,
        temp: 25f64,
        specific_heat: steel_spec_heat,
        thermal_cond: steel_thermal_cond,
        water: pipewater.clone(),
    };

    let mut tank = Tank {
        mass: 40f64,
        height: 3f64,
        radius: 1f64,
        specific_heat: 400f64,
        temp: 22f64,
        water: tankfluid,
    };

    run_simulation(1e-6, 10f64, 0.5f64, &tank);
}

/**
 * Runs the simulation
 */
fn run_simulation(delta_t: f64, input_watts: f64, pumprate: f64, tank: &Tank) {}
