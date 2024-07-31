# Avian 3D minimum reproduction

## How to test

Run the project normally and press the A/D keys to rotate the cube to observe the behavior in `0.1.0`. Then, in `Cargo.toml`, uncomment the line to enable `0.1.1`, and recompile the project. Note how the cube now is restricted from turning.

## Specifics of this situation

There are two requirements for this behavior to occur:

1. The entity must have a `Transform` with some rotation other than `Y` is up. In this example, `Z` is considered up.
2. There must be `LockedAxes`. In my case, every rotation but around the `Z` axis is locked.
