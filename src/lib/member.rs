use crate::lib::matrix::Matrix;
pub struct TrussMember {
    matrix: Matrix,
    nodes: Vec<usize>
}

impl TrussMember {
    pub fn create_single(modulus: f64, area: f64, length: f64, angle: f64, nodes: Vec<usize>) -> TrussMember {
        TrussMember {
            matrix: Matrix::stiffness_single(modulus, area, length, angle.to_radians()),
            nodes,
        }
    }

    pub fn create_double(modulus: f64, area: f64, length: f64, angle1: f64, angle2: f64, nodes: Vec<usize>) -> TrussMember {
        TrussMember {
            matrix: Matrix::stiffness_dbl(modulus, area, length, angle1.to_radians(), angle2.to_radians()),
            nodes,
        }
    }

    pub fn expand_to_global(&self, total_nodes: usize) -> Matrix {
        let mut expanded: Matrix = Matrix::zeros(total_nodes * 2, total_nodes * 2);
        let mut x = 0;
        let mut y = 0;
        for i in 0..total_nodes*2 {
            for j in 0..total_nodes*2 {
                if self.nodes.contains(&(i / 2 + 1)) && self.nodes.contains(&(j / 2 + 1)) {
                    expanded.data[i][j] = self.matrix.data[x][y];
                    if y < self.matrix.data.len() {
                        y += 1;
                    }
                    if y + 1 > self.matrix.data[0].len() {
                        x += 1;
                        y = 0;
                    }
                }
            }
        }
        expanded
    }

    pub fn print(&self) {
        println!("{}\n", self.matrix.to_string())
    }
    
}