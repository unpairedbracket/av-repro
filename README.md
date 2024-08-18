# Avian 3D minimum reproduction

## How to test

Run the project normally and press the A/D keys to rotate the cube to observe the behavior. In `Cargo.toml`, switch between published versions `0.1.0`, `0.1.1` and `0.1.2`, and my proposed fix. Note that applied torques do not affect the cube in any of the three published versions.

## Specifics of this situation

There is only one requirement for this behavior to occur:

1. There must be `LockedAxes` with any of the rotational axes locked.

Then, any `ExternalTorque` used on the entity will have no effect
