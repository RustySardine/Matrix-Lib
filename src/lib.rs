use std::io::{Error, ErrorKind};
use std::ops::{Add, AddAssign, Mul, Sub, SubAssign};

pub struct Matrix<T> {
    rows: usize,
    cols: usize,
    data: Vec<T>,
}

impl<T: Copy> Matrix<T> {
    #[must_use]
    pub fn get(&self, row: usize, col: usize) -> T {
        let point = (self.cols * (row - 1)) + col;
        self.data[point - 1]
    }
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

impl<T> Matrix<T>
where
    T: Sub + Copy,
    Vec<T>: FromIterator<<T as Sub>::Output>,
{
    fn sub_matrix(&mut self, other: &Matrix<T>) -> Result<(), Error> {
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
            .map(|(&x, &y)| x - y)
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

fn dot_matrices<T: Mul + Copy>(matrix1: &Matrix<T>, matrix2: &Matrix<T>) -> Result<Matrix<T>, Error>
where
    Vec<T>: FromIterator<<T as Mul>::Output>,
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
        .map(|(&x, &y)| x * y)
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
impl<T: Copy + Default + Mul<Output = T>> Mul for Matrix<T> {
    type Output = Matrix<T>;
    fn mul(self, other: Matrix<T>) -> Matrix<T> {
        assert!(self.rows == other.rows && self.cols == other.cols);
        dot_matrices(&self, &other).unwrap()
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
impl<T: Copy + Sub + Default + SubAssign> SubAssign for Matrix<T>
where
    Vec<T>: FromIterator<<T as Sub>::Output>,
{
    fn sub_assign(&mut self, other: Matrix<T>) {
        self.sub_matrix(&other).unwrap();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_get() {
        let matrix1: Matrix<i32> = Matrix {
            rows: 3,
            cols: 3,
            data: vec![4, 6, 1, 7, 3, 5, 7, 1, 2],
        };
        assert_eq!(matrix1.get(2, 1), 7)
    }
    #[test]
    fn test_std_add_trait() {
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
    #[test]
    fn test_addassign() {
        let mut matrix1: Matrix<i32> = Matrix {
            rows: 2,
            cols: 2,
            data: vec![2, 3, 54, 5],
        };
        let matrix2: Matrix<i32> = Matrix {
            rows: 2,
            cols: 2,
            data: vec![4, 3, 1, 6],
        };
        matrix1 += matrix2;
        assert_eq!(matrix1.data, vec![6, 6, 55, 11]);
    }
    #[test]
    fn test_std_sub_trait() {
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
        let matrix3 = matrix1 - matrix2;
        assert_eq!(matrix3.data, vec![-2, 0, 53, -1]);
    }
    #[test]
    fn test_subassign() {
        let mut matrix1: Matrix<i32> = Matrix {
            rows: 2,
            cols: 2,
            data: vec![2, 3, 54, 5],
        };
        let matrix2: Matrix<i32> = Matrix {
            rows: 2,
            cols: 2,
            data: vec![4, 3, 1, 6],
        };
        matrix1 -= matrix2;
        assert_eq!(matrix1.data, vec![-2, 0, 53, -1]);
    }
    #[test]
    fn test_dot_product() {
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
        let matrix3 = matrix1 * matrix2;
        assert_eq!(matrix3.data, vec![8, 9, 54, 30]);
    }
}
