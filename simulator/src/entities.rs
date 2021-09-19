// Adam Colton 2021
//! Defines structs to hold thermodynamic entities to be simulated.

use std::f64::consts::PI;

pub trait Entity {
    fn step(&mut self, delta_t: f64, delta_w: f64);
}

/// Represents a pipe at a specific point in time
pub struct Pipe {
    /// The current temperature of the Pipe surrounding, in Kelvin
    pub temp: f64,
    /// The mass in Kg
    pub mass: f64,
    /// The interior radius in m
    pub rad_int: f64,
    /// Exterior radius in m
    pub rad_ext: f64,
    /// Length of pipe in m
    pub length: f64,
    /// The specific_heat in J / K
    pub specific_heat: f64,
    /// Thermal conductivity
    /// This is in units of W/ (m*K)
    pub thermal_cond: f64,
}

/// Represents an amount of Fluid at a specific point in time
/// Fluid is not modeled by units of mass. Instead, changes in
/// temperature are calculated from the flow_rate.
/// All fluid is assumed to exhibit turbulent flow.
pub struct Fluid {
    /// Current temperature in Kelvin
    pub temp: f64,
    /// The specific_heat in J / kg * K
    pub specific_heat: f64,
    /// Mass of this unit of water in kg
    pub mass: f64,
}

/// Calculates the temperature change over a time unit delta_t
/// temp = q (J) / (m (kg) * c (J/ kg * K) )
pub fn calculate_temp(mass: f64, specific_heat: f64, delta_q: f64) -> f64 {
    delta_q / (mass * specific_heat)
}

/// Calculate the heat transfer rate (W) at a specific point in time
/// between a Pipe and a constant temperature fluid.
/// TODO Equation in write up
pub fn calculate_pipe_heat_rate(pipe: &Pipe, fluid: &Fluid) -> f64 {
    // The heat transfer coefficient of the pipe interior wall
    let h_int: f64 = pipe.thermal_cond / (pipe.rad_ext - pipe.rad_int);
    // The heat transfer coefficient of the pipe exterior wall and the
    // surrounding material
    let h_ext: f64 = 2f64;
    2f64 * PI * pipe.length * (fluid.temp - pipe.temp)
        / (1f64 / (h_int * pipe.rad_int)
            + (1f64 / pipe.thermal_cond) * (pipe.rad_ext / pipe.rad_int).ln()
            + 1f64 / (h_ext * pipe.rad_ext))
}

/// Represents a liquid tank
/// For simplicity, the internal liquid temperature
/// is only modelled as the average temperature of all of the liquid
/// in the tank
pub struct Tank {
    /// Mass of dry tank in kg
    mass: f64,
    height: f64,
    radius: f64,
    /// Specific heat of dry tank J / kg * K
    specific_heat: f64,
    /// Mass of contained liquid in kg
    liq_mass: f64,
    /// Avgr temp of all of the liquid in the tank K
    liq_avgr_temp: f64,
    /// Specific heat of the liquid
    liq_specific_heat: f64,
    /// Avgr temp of the dry tank K
    temp: f64,
}

/// TODO Equation writeup
/// Calculates the heat transfer rate at a specific point in time
/// Returns the heat transfer rate in watts 
/// https://www.engineersedge.com/heat_transfer/cylinder_heat_transfer_buried_in_medium_13835.htm
pub fn calculate_tank_heat_rate(tank : &Tank) -> f64 {
    // Conduction shape factor
    let s = 2f64 * PI * tank.height / (2f64 * tank.height / tank.radius).ln();
    s * tank.liq_specific_heat * (tank.temp - tank.liq_avgr_temp)
}
