use rand::{thread_rng, Rng};

pub struct Matrix {
    pub rows: usize,
    pub cols: usize,
    pub data: Vec<Vec<f64>>
}

impl Matrix {
    pub fn zeros(rows: usize, cols: usize) -> Matrix {
        Matrix {
            rows,
            cols,
            data: vec![vec![0.0; cols]; rows]
        }
    }

    pub fn value(rows: usize, cols: usize, value: f64) -> Matrix {
        Matrix {
            rows,
            cols,
            data: vec![vec![value; cols]; rows]
        }
    }

    pub fn random(rows: usize, cols: usize) -> Matrix {
        let mut rng = thread_rng();

        let mut res = Matrix::zeros(rows, cols);

        for i in 0..rows {
            for j in 0..cols {
                res.data[i][j] = rng.gen::<f64>() * 2.0 - 1.0;
            }
        }

        res
    }

    pub fn from(data: Vec<Vec<f64>> ) -> Matrix {
        Matrix {
            rows: data.len(),
            cols: data[0].len(),
            data,
        }
    }

    pub fn stiffness_dbl(modulus: f64, area: f64, length: f64, angle1: f64, angle2:f64) -> Matrix {
        let spring_constant = modulus * area / length;
        Matrix::from(vec![
        vec![spring_constant * angle1.cos().powi(2), spring_constant * angle1.cos() * angle1.sin(), -spring_constant * angle1.cos() * angle2.cos(), -spring_constant * angle1.cos() * angle2.sin()],
        vec![spring_constant * angle1.cos() * angle1.sin(), spring_constant * angle1.sin().powi(2), -spring_constant * angle2.cos() * angle1.sin(), -spring_constant * angle2.sin() * angle1.sin()],
        vec![-spring_constant * angle1.cos() * angle2.cos(), -spring_constant * angle2.cos() * angle1.sin(), spring_constant * angle2.cos().powi(2), spring_constant * angle2.cos() * angle2.sin()],
        vec![-spring_constant * angle1.cos() * angle2.sin(), -spring_constant * angle2.sin() * angle1.sin(), spring_constant * angle2.cos() * angle2.sin(), spring_constant * angle2.sin().powi(2)],
        ])
    }

    pub fn stiffness_single(modulus: f64, area: f64, length: f64, angle: f64) -> Matrix {
        let spring_constant = modulus * area / length;
        Matrix::from(vec![
        vec![spring_constant * angle.cos().powi(2), spring_constant * angle.cos() * angle.sin(), -spring_constant * angle.cos().powi(2), -spring_constant * angle.cos() * angle.sin()],
        vec![spring_constant * angle.cos() * angle.sin(), spring_constant * angle.sin().powi(2), -spring_constant * angle.cos() * angle.sin(), -spring_constant * angle.sin().powi(2)],
        vec![-spring_constant * angle.cos().powi(2), -spring_constant * angle.cos() * angle.sin(), spring_constant * angle.cos().powi(2), spring_constant * angle.cos() * angle.sin()],
        vec![-spring_constant * angle.cos() * angle.sin(), -spring_constant * angle.sin().powi(2), spring_constant * angle.cos() * angle.sin(), spring_constant * angle.sin().powi(2)],
        ])
    }


    pub fn multiply(&mut self, other: &Matrix) -> Matrix {
        if self.cols != other.rows {
            panic!("Attempted to multiply by matrix of incorrect dimensions.");
        }

        let mut res = Matrix::zeros(self.rows, other.cols);
        
        for i in 0..self.rows {
            for j in 0..other.cols {
                let mut sum = 0.0;
                for k in 0..self.cols {
                    sum += self.data[i][k] * other.data[k][j];
                }
                
                res.data[i][j] = sum;
            }
        }

        res
    }

    pub fn add(&mut self, other: &Matrix) -> Matrix {
        if self.rows != other.rows || self.cols != other.cols {
            panic!("Attempted to add matrix of incorrect dimensions.")
        }

        let mut res = Matrix::zeros(self.rows, self.cols);

        for i in 0..self.rows {
            for j in 0..self.cols {
                res.data[i][j] = self.data[i][j] + other.data[i][j];
            }
        }

        res
    }

    pub fn dot_product(&mut self, other: &Matrix) -> Matrix {
        if self.rows != other.rows || self.cols != other.cols {
            panic!("Attempted to perform dot product on matrix of incorrect dimensions.")
        }

        let mut res = Matrix::zeros(self.rows, self.cols);

        for i in 0..self.rows {
            for j in 0..self.cols {
                res.data[i][j] = self.data[i][j] * other.data[i][j];
            }
        }

        res
    }

    pub fn subtract(&mut self, other: &Matrix) -> Matrix {
        if self.rows != other.rows || self.cols != other.cols {
            panic!("Attempted to subtract matrix of incorrect dimensions.")
        }

        let mut res = Matrix::zeros(self.rows, self.cols);

        for i in 0..self.rows {
            for j in 0..self.cols {
                res.data[i][j] = self.data[i][j] - other.data[i][j];
            }
        }

        res
    }

    pub fn map(&mut self, function: &dyn Fn(f64) -> f64) -> Matrix {
        Matrix::from(
            (self.data)
            .clone()
            .into_iter()
            .map(|row| row.into_iter().map(|value| function(value)).collect())
            .collect()
        )
    }

    pub fn transpose(&mut self) -> Matrix {
        let mut res = Matrix::zeros(self.cols, self.rows);

        for i in 0..self.rows{
            for j in 0..self.cols {
                res.data[j][i] = self.data[i][j];
            }
        }

        res
    }

    pub fn to_string(&self) -> String {
        let max_width = self.data.iter().flatten().fold(0, |max, val| {
            let width = format!("{:.3}", val).len();
            max.max(width)
        });

        let lines: Vec<String> = self.data.iter().map(|row| {
            format!(
                "| {} |",
                row.iter()
                    .map(|val| format!("{:>width$.3} ", val, width = max_width))
                    .collect::<Vec<_>>()
                    .join(" | ")
            )
        }).collect();

        lines.join("\n")
    }

    pub fn print(&self) {
        println!("{}\n",self.to_string());
    }
}