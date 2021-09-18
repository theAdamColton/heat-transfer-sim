// Adam Colton 2021
//! Defines structs to hold thermodynamic entities to be simulated.

use std::f64::consts::PI;
use f64::ln;

pub trait Entity {
    fn step(&mut self, delta_t : f64, delta_w : f64);
}


/// Represents a pipe at a specific point in time
pub struct Pipe {
    /// The current temperature of this entire Pipe, in Kelvin
    temp: f64,
    /// The mass in Kg
    mass: f64,
    /// The interior radius in m
    rad_int: f64,
    /// Exterior radius in m
    rad_ext: f64,
    /// Length of pipe in m
    length: f64,
    /// The specific_heat in J / K
    specific_heat: f64,
    /// Thermal conductivity
    /// This is in units of W/ (m*K)
    thermal_cond: f64,
}

/// Represents an amount of Fluid at a specific point in time
/// Fluid is not modeled by units of mass. Instead, changes in
/// temperature are calculated from the flow_rate.
/// All fluid is assumed to exhibit turbulent flow.
pub struct Fluid {
    /// Current temperature in Kelvin
    temp: f64,
    /// The specific_heat in J / kg * K
    specific_heat: f64,
    /// Mass of this unit of water in kg
    mass: f64,
}


/// Calculates the temperature change over a time unit delta_t
/// temp = q (J) / (m (kg) * c (J/ kg * K) )
fn calculate_temp(mass: f64, specific_heat: f64, delta_q: f64) -> f64 {
    delta_q / ( mass * specific_heat )
}


/// Calculate the power transfer (W) at a specific point in time
/// between a Pipe and a constant temperature fluid.
///
/// TODO Equation in write up
fn calculate_pipe_heat_rate(pipe : &Pipe, fluid : &Fluid) -> f64 {
    // The heat transfer coefficient of the pipe interior wall
    let h_int : f64 = pipe.thermal_cond / (pipe.rad_ext - pipe.rad_int);
    // The heat transfer coefficient of the pipe exterior wall
    // I'm assuming that nothing else exists in this scenerio except for the closed system
    let h_ext : f64 = 0f64;
    2f64 * PI * pipe.length * (fluid.temp - pipe.temp) / 
        (1f64 / (h_int * pipe.rad_int) + (1f64 / pipe.thermal_cond) * ln(pipe.rad_ext / pipe.rad_int) + 1f64 / (h_ext * pipe.rad_ext))
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn demonstrate() {
        let steelpipe = Pipe {
            length : 10f64,
            mass : 5f64,
            rad_int : 0.05f64,
            rad_ext : 0.07f64,
            temp : 30f64,
            specific_heat : 510f64,
            thermal_cond : 35f64,
        };
    }
}
