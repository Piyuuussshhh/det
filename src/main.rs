use clap::Parser;

#[derive(Parser)]
struct CLI {
    /// Order of the matrix.
    order: isize,
    /// Matrix.
    matrix: String,
}

struct Matrix {
    order: isize,
    mat: Vec<Vec<isize>>,
}

impl Matrix {
    fn new(order: isize, m: String) -> Self {
        let rows: Vec<&str> = m.split(',').collect();

        let mut res: Vec<Vec<isize>> = Vec::new();

        for &row in rows.iter() {
            let temp: Vec<&str> = row.split(' ').collect();
            res.push(temp.into_iter().map(|n| n.parse::<isize>().expect("Invalid input")).collect());
        }

        for row in res.iter() {
            println!("{:?}", row);
        }

        Matrix {
            order,
            mat: res,
        }
    }

    fn det(&self) -> isize {
        if self.order == 2 {
            let mat = &self.mat;
            return mat[0][0]*mat[1][1] - mat[0][1]*mat[1][0];
        }

        let first_row = &self.mat[0];
        let mut res: isize = 0;
        for (idx, &elem) in first_row.into_iter().enumerate() {
            let mut det_mat = self.mat.clone();
            det_mat.remove(0);
            for row in det_mat.iter_mut() {
                row.remove(idx);
            }

            res += (idx as isize % 2 * 2 - 1) * elem * (Matrix {
                order: self.order - 1,
                mat: det_mat,
            }).det();
        }

        res
        // todo!()
    }
}

fn main() {
    let args = CLI::parse();

    println!("Determinant: {}", Matrix::new(args.order, args.matrix).det());
}