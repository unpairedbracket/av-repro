# Avian 3D minimum reproduction

## How to test

Run the project normally and press the A/D keys to rotate the cube to observe the behavior in `0.1.0`. Then, in `Cargo.toml`, uncomment the line to enable `0.1.1`, and recompile the project. Note how the cube now is restricted from turning.

## Specifics of this situation

There are two requirements for this behavior to occur:

1. The entity must have a `Transform` with some rotation other than `Y` is up. In this example, `Z` is considered up.
2. There must be `LockedAxes`. In my case, every rotation but around the `Z` axis is locked.

## 0.1.0

https://github.com/user-attachments/assets/94f25534-6091-4d9f-9725-209d0d522688

## 0.1.1

https://github.com/user-attachments/assets/d3fd548e-cff5-4a4a-830d-1501ccf2f750

