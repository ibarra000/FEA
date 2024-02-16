use std::f64::EPSILON;

/// Calculates the stiffness matrix for a truss element.
///
/// This function takes in the elastic modulus, cross-sectional area,
/// length, and orientation angle of a truss element and calculates
/// the corresponding stiffness matrix.
///
/// # Arguments
///
/// * `e_mod` - Elastic modulus of the material.
/// * `area` - Cross-sectional area of the truss element.
/// * `length` - Length of the truss element.
/// * `angle` - Orientation angle of the truss element (in degrees).
///
/// # Returns
///
/// A 4x4 matrix representing the stiffness matrix of the truss element.
///
/// # Examples
///
/// ```
/// let e_mod = 120.0;
/// let area = 500.0;
/// let length = 2000.0;
/// let angle = 233.13; // Assuming angle is given in degrees
/// let stiffness_matrix = truss_matrix(e_mod, area, length, angle);
/// ```
fn truss_matrix(e_mod: f64, area: f64, length: f64, angle: f64) -> Vec<Vec<f64>> {
    // Convert angle to radians for trigonometric calculations
    let angle_rad = angle.to_radians();

    // Trigonometric values to simplify matrix elements
    let cos_sq = angle_rad.cos().powi(2);
    let sin_sq = angle_rad.sin().powi(2);
    let cos_sin = angle_rad.cos() * angle_rad.sin();

    // Initial stiffness matrix for a truss element
    let mut stiffness_matrix: Vec<Vec<f64>> = vec![
        vec![cos_sq, cos_sin, -cos_sq, -cos_sin],
        vec![cos_sin, sin_sq, -cos_sin, -sin_sq],
        vec![-cos_sq, -cos_sin, cos_sq, cos_sin],
        vec![-cos_sin, -sin_sq, cos_sin, sin_sq],
    ];

    // Calculate spring constant based on material properties and geometry
    let spring_const = e_mod * area / length;

    // Scale and round small values in the stiffness matrix
    for row in stiffness_matrix.iter_mut() {
        for val in row.iter_mut() {
            *val = if val.abs() < EPSILON {
                0.0
            } else {
                *val * spring_const
            };
        }
    }

    stiffness_matrix
}

/// Prints a matrix with dashes between elements for better visualization.
///
/// This function takes a matrix and prints it with dashes between
/// elements for better readability. It also determines the maximum
/// width of values for proper formatting.
///
/// # Arguments
///
/// * `matrix` - The matrix to be printed.
/// * `units` - The units to be displayed along with each element.
fn print_matrix_with_dashes(matrix: Vec<Vec<f64>>, units: &str) {
    // Determine the maximum width of the values in the matrix
    let max_width = matrix.iter().flatten().fold(0, |max, val| {
        let width = format!("{:.1}", val).len();
        max.max(width)
    });

    // Create a dash line string with the desired length
    let dash_line = format!(" {}", "-".repeat((max_width + units.len() + 6) * matrix[0].len() - 1));

    // Print the truss matrix header with the dash line
    println!("\nTruss Matrix:\n{}", dash_line);

    // Iterate over each row in the matrix
    for row in matrix.iter() {
        // Create a formatted string for the current row
        let line = format!(
            "| {} |",
            row.iter()
                .map(|val| format!("{:>width$.1} ({})", val, units, width = max_width))
                .collect::<Vec<_>>()
                .join(" | ")
        );

        // Print the formatted row and a dash line below it
        println!("{}\n{}", line, dash_line);
    }

    // Add an extra newline for better readability
    println!();
}


/// Main function to demonstrate truss_matrix and print_matrix_with_dashes.
fn main() {
    let e_mod = 120.0;
    let area = 500.0;
    let length = 2000.0;
    let angle = 233.13; // Assuming angle is given in degrees, convert to radians
    let result_units = "kN/mm";

    // Calculate and print the truss matrix
    let result = truss_matrix(e_mod, area, length, angle);
    print_matrix_with_dashes(result, result_units);
}
