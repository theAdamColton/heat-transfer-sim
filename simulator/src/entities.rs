// Adam Colton 2021
//! Defines structs to hold thermodynamic entities to be simulated.

use std::f64::consts::PI;
const WATER_SPECIFIC_HEAT: f64 = 4181.3f64; // J / kg K
const WATER_DIFFUSIVITY: f64 = 1.4e-7; // m^2 / second
const WATER_CONDUCTIVITY: f64 = 0.5918; // W / m * Kelvin

/// A thermodynamic entity that contains Water
pub trait Entity {
    /// Updates this entity's tempurature over a timestep delta_t
    fn step(&mut self, delta_t: f64);

    /// Adds water to this entity
    ///
    /// Sets the new water to the new temperature
    /// considering the mass of added water and its temperature
    fn add_water(&mut self, added_water: &Water) {
        let curr_water = self.get_water();
        let new_temp = (curr_water.mass * curr_water.temp + added_water.mass * added_water.temp)
            / (curr_water.mass + added_water.mass);
        let new_water = Water {
            mass: added_water.mass + curr_water.mass,
            temp: new_temp,
        };
        self.set_water(new_water);
    }

    /// Takes water from this entity
    ///
    /// Returns Water with mass equal subtracted mass,
    /// and temerature the average of this entity's water
    fn take_water(&mut self, mass: f64) -> Water {
        let mut water: Water = self.get_water();
        let mass_to_take: f64;
        // Cannot take more water than is in this entity
        if mass > water.mass {
            mass_to_take = water.mass;
        } else {
            mass_to_take = mass;
        }

        water.mass -= mass_to_take;
        self.set_water(water);

        Water {
            mass: mass_to_take,
            temp: water.temp,
        }
    }

    fn get_water(&self) -> Water;

    fn set_water(&mut self, water: Water);
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
    /// The fluid that is contained in this pipe
    pub water: Water,
}

/**
 * Calculate the heat transfer rate (W) at a specific point in time
 * between a Pipe and a constant temperature fluid, then applies the
 * heat change to the pipe temperature and the water temperature.
 */
impl Entity for Pipe {
    fn step(&mut self, delta_t: f64) {
        // The heat transfer coefficient of the pipe interior wall
        let h_int: f64 = self.thermal_cond / (self.rad_ext - self.rad_int);
        // The heat transfer coefficient of the pipe exterior wall and the
        // surrounding material
        let h_ext: f64 = 2f64;
        let heat_rate = 2f64 * PI * self.length * (self.water.temp - self.temp)
            / (1f64 / (h_int * self.rad_int)
                + (1f64 / self.thermal_cond) * (self.rad_ext / self.rad_int).ln()
                + 1f64 / (h_ext * self.rad_ext));

        let delta_q = heat_rate * delta_t;

        self.temp += calculate_dtemp(delta_q, self.mass, self.specific_heat);
        self.water.temp += calculate_dtemp(delta_q, self.water.mass, WATER_SPECIFIC_HEAT);
    }

    fn get_water(&self) -> Water {
        self.water
    }
    fn set_water(&mut self, water: Water) {
        self.water = water
    }
}

/// Represents an amount of Water at a specific point in time
/// temperature are calculated from the flow_rate.
/// All fluid is assumed to exhibit turbulent flow.
#[derive(Copy, Clone)]
pub struct Water {
    /// Current temperature in Kelvin
    pub temp: f64,
    /// Mass of this unit of water in kg
    pub mass: f64,
}

/// Represents a liquid tank
/// For simplicity, the internal liquid temperature
/// is only modelled as the average temperature of all of the liquid
/// in the tank
pub struct Tank {
    /// Mass of dry tank in kg
    pub mass: f64,
    pub height: f64,
    pub radius: f64,
    /// Specific heat of dry tank J / kg * K
    pub specific_heat: f64,
    /// Avgr temp of the dry tank K
    pub temp: f64,
    /// Represents the fluid that is contained in this pipe
    pub water: Water,
}

impl Entity for Tank {
    /**
     * Calculates the change in temperature to the tank, and to the water in the tank based on the
     * current heat_transfer_coefficient. This implementation only facors in the heat loss from
     * convection to the sides of the cylinder. A better way to do this would add the heat loss
     * from the bottom and top of the cylinder, which would need to be calculated individually.
     */
    fn step(&mut self, delta_t: f64) {
        // Calculates the rayleigh number
        let r_a = get_rayleigh(self.temp, self.height, &self.water);
        // Calculates the Prandtl number
        // https://www.tec-science.com/mechanics/gases-and-liquids/prandtl-number/
        let p_r = water_viscosity(self.water.temp) / WATER_DIFFUSIVITY;
        // Calculates the N_u Nusselt number
        // Using equation for vertical plate convection
        let n_u = 0.825
            + 0.381 * r_a.powf(1f64 / 6f64)
                / (1f64 + (0.492 / p_r).powf(9f64 / 16f64)).powf(8f64 / 27f64);
        // Calculates the heat transfer coefficient h from the Nusselt number
        let h = n_u * WATER_CONDUCTIVITY / self.height;

        // Since the heat_transfer_coefficient only applies to vertical plate convection
        // I only take the surface_area of the side of the tank.
        let surface_area = 2f64 * PI * self.radius * self.height;

        let heat_rate = h * surface_area * (self.water.temp - self.temp);

        // Calculates the (sub) instantaneous heat change
        let delta_q = heat_rate * delta_t;

        //println!("r_a {}, p_r {}, n_u {}, h {}, heat_rate: {}, delta_q:{}",r_a, p_r, n_u, h, heat_rate, delta_q);

        // Applies the change to the temperatures
        self.temp += calculate_dtemp(delta_q, self.mass, self.specific_heat);

        self.water.temp += calculate_dtemp(delta_q, self.water.mass, WATER_SPECIFIC_HEAT);
    }

    fn get_water(&self) -> Water {
        self.water
    }
    fn set_water(&mut self, water: Water) {
        self.water = water
    }
}

pub struct Panel {
    pub mass: f64,
    /// Area in m^2
    pub area: f64,
    pub temp: f64,
    /// Input heat into the system in watts
    pub input_w: f64,
    pub water: Water,
    pub specific_heat: f64,
    /// The heat transfer coeffient for this panel;
    /// This can differ significantly based on
    /// heat exchanger design. (W/ m^2 K)
    pub heat_transfer_coefficient: f64,
}

impl Entity for Panel {
    fn step(&mut self, delta_t: f64) {
        /*** This implementation performs a very simple calculation. This is because this method
         * assumes that the heat_transfer_coefficient is constant as given.
         */
        let heat_rate = self.heat_transfer_coefficient * self.area * (self.water.temp - self.temp);
        let delta_q = heat_rate * delta_t + self.input_w * delta_t;

        self.temp += calculate_dtemp(delta_q, self.mass, self.specific_heat);
        self.water.temp += calculate_dtemp(delta_q, self.water.mass, WATER_SPECIFIC_HEAT);
    }

    fn get_water(&self) -> Water {
        self.water
    }
    fn set_water(&mut self, water: Water) {
        self.water = water
    }
}

/// t_s is the temperature of the solid object,
/// t_inf is the temperature of the fluid
/// The thermal expansion coeff is assumed to be 1.
/// TODO Equation
fn get_rayleigh(t_s: f64, length: f64, water: &Water) -> f64 {
    let viscosity = water_viscosity(water.temp);
    9.81 * (t_s - water.temp).abs() * length.powi(3) / (viscosity * WATER_DIFFUSIVITY)
}

/// Viscosity as a funct of temp of water
/// This calculates the dynamic viscosity.
/// For my purposes, I assume that the dynamic viscosity is
/// the same as the kinematic viscosity.
/// https://en.wikipedia.org/wiki/Viscosity#Water
fn water_viscosity(temp: f64) -> f64 {
    0.02939 * (507.88 / (temp - 149.3)).exp()
}

/// Calculates the change in temperature based on the input energy in Joules, mass, and
/// specific_heat
/// temp = q (J) / (m (kg) * c (J/ kg * K) )
pub fn calculate_dtemp(delta_q: f64, mass: f64, specific_heat: f64) -> f64 {
    delta_q / (mass * specific_heat)
}
