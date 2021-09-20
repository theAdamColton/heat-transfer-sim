/**
 * Adam Colton 2021
 *
 * Runs the simulator on a set of starting conditions
 */
mod entities;

use entities::*;


fn main() {
    let steel_spec_heat = 466.0;
    let steel_thermal_cond = 35.0;

    let pipewater = Water {
        mass: 4.0,
        temp: 293.0,
    };

    let tankfluid = Water {
        mass: 300.0,
        temp: 293.0,
    };

    let mut steelpipe = Pipe {
        length: 10.0,
        mass: 5.0,
        rad_int: 0.05,
        rad_ext: 0.07,
        temp: 293.0,
        specific_heat: 452.0,
        h_ext: 200.0,
        thermal_cond: steel_thermal_cond,
        water: pipewater,
    };

    let mut steelpipe_out = Pipe {
        length: 10.0,
        mass: 5.0,
        rad_int: 0.05,
        rad_ext: 0.07,
        temp: 293.0,
        h_ext: 200.0,
        specific_heat: steel_spec_heat,
        thermal_cond: steel_thermal_cond,
        water: pipewater.clone(),
    };

    let mut tank = Tank {
        mass: 40.0,
        height: 3.0,
        radius: 1.0,
        specific_heat: steel_spec_heat,
        temp: 299.0,
        water: tankfluid,
    };

    let mut panel = Panel {
        area: 10.0,
        mass: 250.0,
        heat_transfer_coefficient: 30.0,
        input_w: 9900.0,
        specific_heat: steel_spec_heat,
        temp: 293.0,
        water: pipewater.clone(),
    };


    run_simulation(
        1e-4,
        5.0,
        10000.0,
        10000.0,
        &mut tank,
        &mut steelpipe,
        &mut steelpipe_out,
        &mut panel,
    );
}

/**
 * Runs the simulation
 *
 * delta_t is the time step, should be small for higher accuracy.
 * pumprate is the kg / s that the simulator will transfer between entities.
 * final_t defines how many seconds to run the simulator for.
 * print_every_t defines how often to print Entity temperature info.
 */
fn run_simulation(
    delta_t: f64,
    pumprate: f64,
    final_t: f64,
    print_every_t: f64,
    tank: &mut Tank,
    pipein: &mut Pipe,
    pipeout: &mut Pipe,
    panel: &mut Panel,
) {
    let mut i = 0;
    while delta_t * (i as f64) <= final_t {
        let curr_t = i as f64 * delta_t;

        if curr_t % print_every_t == 0.0 {
            println!("-----Itr {} t={}-----", i, curr_t);
            println!("Tank: {}K\n\twater: {}Kg {}K\nPipeout: {}K\n\twater: {} Kg {}K\npanel: {}K\n\twater: {}Kg {}K\npipein: {}K\n\twater: {}Kg {}K", tank.temp, tank.water.mass, tank.water.temp, pipeout.temp, pipeout.water.mass, pipeout.water.temp, panel.temp, panel.water.mass, panel.water.temp, pipein.temp, pipein.water.mass, pipein.water.temp);
        }
        // Computes the step for each entity
        tank.step(delta_t);
        pipeout.step(delta_t);
        panel.step(delta_t);
        pipein.step(delta_t);

        let pumpstep = pumprate * delta_t;

        // Takes water from each entities and adds it to the next one
        pipeout.add_water(&tank.take_water(pumpstep));
        panel.add_water(&pipeout.take_water(pumpstep));
        pipein.add_water(&panel.take_water(pumpstep));
        tank.add_water(&pipein.take_water(pumpstep));

        i += 1;
    }
}
