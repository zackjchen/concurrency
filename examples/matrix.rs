// #![allow(unused)]
// use std::{
//     fmt::{Debug, Display},
//     ops::{Add, AddAssign, Mul},
// };

// use anyhow::Result;

// struct Matrix<T> {
//     pub data: Vec<T>,
//     pub row: usize,
//     pub col: usize,
// }
// // [[1,2],[1,2],[1,2]]
// fn main() {}

// fn multiply<T>(a: Matrix<T>, b: Matrix<T>) -> Result<Matrix<T>>
// where
//     T: Mul<Output = T> + Add<Output = T> + AddAssign + Copy + Default,
// {
//     if a.col != b.row {
//         return Err(anyhow::anyhow!(
//             "Matrix A's column must be equal to Matrix B's row"
//         ));
//     }
//     // let mut data: Vec<T> = Vec::with_capacity(a.row * b.col);
//     let mut data = vec![T::default(); a.row * b.col];
//     for i in 0..a.row {
//         for j in 0..b.col {
//             for k in 0..a.col {
//                 data[i * b.col + j] += a.data[i * a.col + k] * b.data[k * b.col + j]
//             }
//         }
//     }
//     Ok(Matrix::new(data, a.row, b.col))
// }

// impl<T> Matrix<T> {
//     pub fn new(data: impl Into<Vec<T>>, row: usize, col: usize) -> Self {
//         let data = data.into();
//         Self { data, row, col }
//     }
// }

// impl<T: Debug> Display for Matrix<T> {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         write!(f, "[")?;
//         for i in 0..self.row {
//             write!(f, "[")?;
//             for j in 0..self.col {
//                 write!(f, "{:?} ", self.data[i * self.col + j])?;
//             }
//             if i == self.row - 1 {
//                 write!(f, "]")?;
//             } else {
//                 writeln!(f, "],")?;
//             }
//         }
//         write!(f, "]")?;
//         Ok(())
//     }
// }

// impl<T: Debug> Debug for Matrix<T> {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         write!(f, "row: {}, col: {}, {:?}", self.row, self.col, self.data)
//     }
// }

// #[cfg(test)]
// mod test {
//     use super::*;
//     #[test]
//     fn test_matrix_multiply() -> Result<()> {
//         let a = Matrix::new(vec![1, 2, 3, 4, 5, 6], 2, 3);
//         let b = Matrix::new(vec![1, 2, 3, 4, 5, 6], 3, 2);
//         let c = multiply(a, b).unwrap();
//         assert!(c.col == 2 && c.row == 2);
//         assert_eq!(c.data, vec![22, 28, 49, 64]);
//         Ok(())
//     }
// }
use anyhow::Result;
use concurrency::matrix::Matrix;
fn main() -> Result<()> {
    let a = Matrix::new(vec![1, 2, 3, 4, 5, 6], 2, 3);
    let b = Matrix::new(vec![1, 2, 3, 4, 5, 6], 3, 2);
    println!("a*b: {:?}", a * b);

    Ok(())
}
