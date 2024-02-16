# FEA Code
## Welcome

### Overview
This code serves as a tool to streamline the computations for ME4508, specifically aiding in the setup of matrices used in Finite Element Analysis (FEA). The primary functionality is centered around calculating the truss matrix, a crucial step in structural analysis.

### Truss Matrix Calculation
The `truss_matrix` function computes the stiffness matrix for a truss element based on provided parameters such as Young's Modulus (`e_mod`), cross-sectional area (`area`), length (`length`), and the angle (`angle`) of the truss element. The resulting matrix is adjusted by applying scaling factors derived from these input parameters.

#### Parameters:
- `e_mod`: Young's Modulus of the material
- `area`: Cross-sectional area of the truss element
- `length`: Length of the truss element
- `angle`: Angle of the truss element in degrees

#### Return:
- A 4x4 matrix representing the truss stiffness matrix

### Matrix Visualization
The `print_matrix_with_dashes` function facilitates the visual representation of matrices with proper formatting. It prints the truss matrix with aligned values and dash lines for better readability.

#### Parameters:
- `matrix`: The matrix to be printed
- `units`: Units associated with the values in the matrix

### Example Usage (Main Function)
The `main` function demonstrates the practical use of the `truss_matrix` and `print_matrix_with_dashes` functions. It initializes parameters such as Young's Modulus, cross-sectional area, length, and angle, calculates the truss matrix, and then visually presents the matrix with appropriate formatting.

Feel free to modify and integrate this code into your ME4508 projects to simplify the FEA matrix setup process. If you have any questions or suggestions, please reach out.

**Note:** Make sure to convert angles to radians before passing them to the `truss_matrix` function, as it expects angles in radians.