use clap::Parser;

#[derive(Parser)]
struct CLI {
    /// Order of the matrix.
    order: usize,
    /// Matrix.
    matrix: String,
}

struct Matrix {
    order: usize,
    mat: Vec<Vec<isize>>,
}

impl Matrix {
    fn new(order: usize, m: String) -> Self {
        // Split the input string into its corresponding rows.
        let rows: Vec<&str> = m.split(',').collect();

        let mut res: Vec<Vec<isize>> = Vec::new();

        // Split each row into elements, and push the row after converting to isize.
        for &row in rows.iter() {
            let temp: Vec<&str> = row.split(' ').collect();
            res.push(temp.into_iter().map(|n| n.parse::<isize>().expect("Invalid input in the matrix, not a number")).collect());
        }

        // Print the matrix in human-readable form.
        for row in res.iter() {
            println!("{:?}", row);
        }

        // If input order does not match the input matrix's order, exit.
        if order != res.len() {
            panic!("Matrix does not have the order specified!");
        }

        Matrix {
            order,
            mat: res,
        }
    }

    /// Calculates determinant of given matrix.
    fn det(&self) -> isize {
        // If order is, calculate and return determinant.
        if self.order == 2 {
            let mat = &self.mat;
            return mat[0][0]*mat[1][1] - mat[0][1]*mat[1][0];
        }

        // For simplicity, we only use the first row to calculate the determinant.
        // We first calculate the minor of an element in the row.
        // Then we calculate its determinant recursively.
        // After that we multiply it with the element and its associated sign
        // Finally, we add the cofactors.
        let first_row = &self.mat[0];
        let mut res: isize = 0;
        for (idx, &elem) in first_row.into_iter().enumerate() {
            // Closure to calculate minor.
            let minor = || -> Vec<Vec<isize>> {
                let mut det_mat = self.mat.clone();
                det_mat.remove(0);
                for row in det_mat.iter_mut() {
                    row.remove(idx);
                }
                det_mat
            };

            // det +=       (-1)^(i + j)      * elem * det(minor)
            res += (idx as isize % 2 * 2 - 1) * elem * (Matrix {
                order: self.order - 1,
                mat: minor(),
            }).det();
        }

        res
    }
}

fn main() {
    let args = CLI::parse();

    println!("Determinant: {}", Matrix::new(args.order, args.matrix).det());
}