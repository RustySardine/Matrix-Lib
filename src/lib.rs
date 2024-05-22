use std::borrow::BorrowMut;

use std::io::{Error, ErrorKind};
use std::ops::{Add, AddAssign, Sub};
use std::process::Output;

struct Matrix<T> {
    rows: usize,
    cols: usize,
    data: Vec<T>,
}
impl<T> Matrix<T>
where
    T: Add + Copy,
    Vec<T>: FromIterator<<T as Add>::Output>,
{
    fn add_matrix(&mut self, other: &Matrix<T>) -> Result<(), Error> {
        if self.rows != other.rows || self.cols != other.rows {
            return Err(Error::new(
                ErrorKind::InvalidInput,
                "Incompatitble Matrices Sizes",
            ));
        };
        self.data = self
            .data
            .iter()
            .zip(other.data.iter())
            .map(|(&x, &y)| x + y)
            .collect();
        Ok(())
    }
}
fn add_matrices<T: Add + Copy>(matrix1: &Matrix<T>, matrix2: &Matrix<T>) -> Result<Matrix<T>, Error>
where
    Vec<T>: FromIterator<<T as Add>::Output>,
{
    if matrix1.rows != matrix2.rows || matrix1.cols != matrix2.cols {
        return Err(Error::new(
            ErrorKind::InvalidInput,
            "Incompatitble Matrices Sizes",
        ));
    }
    let rows = &matrix1.rows;
    let cols = &matrix1.cols;
    let data: Vec<T> = matrix1
        .data
        .iter()
        .zip(matrix2.data.iter())
        .map(|(&x, &y)| x + y)
        .collect();
    Ok(Matrix::<T> {
        rows: *rows,
        cols: *cols,
        data,
    })
}

fn sub_matrices<T: Sub + Copy>(matrix1: &Matrix<T>, matrix2: &Matrix<T>) -> Result<Matrix<T>, Error>
where
    Vec<T>: FromIterator<<T as Sub>::Output>,
{
    if matrix1.rows != matrix2.rows || matrix1.cols != matrix2.cols {
        return Err(Error::new(
            ErrorKind::InvalidInput,
            "Incompatitble Matrices Sizes",
        ));
    }
    let rows = &matrix1.rows;
    let cols = &matrix1.cols;
    let data: Vec<T> = matrix1
        .data
        .iter()
        .zip(matrix2.data.iter())
        .map(|(&x, &y)| x - y)
        .collect();
    Ok(Matrix::<T> {
        rows: *rows,
        cols: *cols,
        data,
    })
}
impl<T: Copy + Default + Add<Output = T>> Add for Matrix<T> {
    type Output = Matrix<T>;

    fn add(self, other: Matrix<T>) -> Matrix<T> {
        assert!(self.rows == other.rows && self.cols == other.cols);
        add_matrices(&self, &other).unwrap()
    }
}
impl<T: Copy + Default + Sub<Output = T>> Sub for Matrix<T> {
    type Output = Matrix<T>;

    fn sub(self, other: Matrix<T>) -> Matrix<T> {
        assert!(self.rows == other.rows && self.cols == other.cols);
        sub_matrices(&self, &other).unwrap()
    }
}

impl<T: Copy + Add + Default + AddAssign> AddAssign for Matrix<T>
where
    Vec<T>: FromIterator<<T as Add>::Output>,
{
    fn add_assign(&mut self, other: Matrix<T>) {
        self.add_matrix(&other).unwrap();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_add_functionality() {
        let matrix1: Matrix<i32> = Matrix {
            rows: 2,
            cols: 2,
            data: vec![2, 3, 54, 5],
        };
        let matrix2: Matrix<i32> = Matrix {
            rows: 2,
            cols: 2,
            data: vec![4, 3, 1, 6],
        };
        let matrix3 = matrix1 + matrix2;
        assert_eq!(matrix3.data, vec![6, 6, 55, 11]);
        println!("yo");
    }
}
