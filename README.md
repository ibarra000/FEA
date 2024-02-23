# Truss FEA Matrix Code
## Welcome

### Overview
This code serves as a tool for streamlining computations in ME4508, specifically aiding in the setup of matrices used in Finite Element Analysis (FEA). The primary functionality is centered around calculating the truss matrix, a crucial step in structural analysis.

### Truss Matrix Calculation
The `TrussStructure` struct now owns its members, which are instances of the `TrussMember` struct. The truss matrix calculation is facilitated by the `global_stiffness_matrix` method, which computes the stiffness matrix for the truss structure based on its owned members and associated parameters.

#### Return:
- A (total_nodes * 2) x (total_nodes * 2) matrix representing the truss stiffness matrix


### Example Usage (Main Function)
The `main` function demonstrates the practical use of the `TrussStructure` and associated methods. It initializes parameters such as Young's Modulus, cross-sectional area, length, and angle, creates instances of `TrussMember`, constructs a `TrussStructure` with owned members, calculates the truss matrix, and then visually presents the matrix with appropriate formatting.

Feel free to modify and integrate this code into your ME4508 projects to simplify the FEA matrix setup process. If you have any questions or suggestions, please reach out.
