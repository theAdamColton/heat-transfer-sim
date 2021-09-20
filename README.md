Adam Colton 2021
================

# Heat Transfer Simulator

## How to build and run
* cargo cli tools required
* Run ```cargo run```, from the simulator directory

## How to edit starting conditions
You may want to edit starting conditions such as the heat_transfer_coefficient for the solar panel, or the input energy to the solar panel. This is done by modifying the struct declarations for the different entities, in the main method of ```main.rs```. The units for each parameter are given as doc comments in the ```entities.rs``` file.

If you want to change the runtime of the simulation, or the console output behavior, these options are passed as parameters to the run_simulation function.

## Explanation for equations
Some well known thermodynamic equations are not explained in the comments. The complicated formulas for the Nusselt number are defined in the HeatTransferBooklet.pdf. For the tank, I chose to only calculate the convection heat loss between the water and the tank walls, modelling the tank as a horizontal plate. For the pipes, I used the internal flow equations for a constant heat_transfer_coefficient between the pipe and the surrounding mileau. For the panel, I assumed a generic heat_transfer_coefficient typical for heat converters, which allowed me to use a very simple equation to describe the energy tranfer at each step.

## Explanation of results
One item my simulation needs work on is the effect of the pumprate on the heat transfer between the entities and the water. Conceptually, a faster pumprate would mean that more heat could be transferred from the pipe/panel into the water. My equations were not able to exhibit this, as changing the input flow rate had little effect on the temperature change over time. 
