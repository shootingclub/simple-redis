use std::fmt;
use std::ops::{Add, AddAssign, Mul};

struct Matrix<T> {
    data: Vec<T>,
    rows: usize,
    cols: usize,
}

impl<T> Matrix<T>
where
    T: fmt::Debug,
{
    fn new(data: impl Into<Vec<T>>, rows: usize, cols: usize) -> Self {
        Matrix {
            data: data.into(),
            rows,
            cols,
        }
    }
}

impl<T> fmt::Display for Matrix<T>
where
    T: fmt::Debug,
{
    // display a 2*3 as {1 2 3 , 4 5 6}
    // display b 3*2 as {1 2 ,3 4, 5 6}
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for i in 0..self.rows {
            for j in 0..self.cols {
                write!(f, "{:?} ", self.data[i * self.cols + j])?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

impl<T> fmt::Debug for Matrix<T>
where
    T: fmt::Debug,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Matrix {{ data: {:?}, rows: {}, cols: {} }}",
            self.data, self.rows, self.cols
        )
    }
}

fn multiply<T>(a: &Matrix<T>, b: &Matrix<T>) -> anyhow::Result<Matrix<T>>
where
    T: Default + Copy + Mul<Output = T> + Add<Output = T> + AddAssign,
{
    if a.cols != b.rows {
        return Err(anyhow::anyhow!("Matrix dimensions do not match"));
    }
    let mut data: Vec<T> = vec![T::default(); a.rows * b.cols];
    //遍历a的每一行
    for i in 0..a.rows {
        //遍历b的每一列
        for j in 0..b.cols {
            for k in 0..a.cols {
                data[i * b.cols + j] += a.data[i * a.cols + k] * b.data[k * b.cols + j];
            }
        }
    }
    Ok(Matrix {
        data,
        rows: a.rows,
        cols: b.cols,
    })
}

fn main() -> anyhow::Result<()> {
    let a = Matrix::new(vec![1, 2, 3, 4, 5, 6], 2, 3);
    let b = Matrix::new(vec![1, 2, 3, 4, 5, 6], 3, 2);
    println!("left: \n{}", a);
    println!("right: \n{}", b);
    let c = multiply(&a, &b)?;
    println!("left * right: \n{}", c);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_multiply() {
        let a = Matrix::new(vec![1, 2, 3, 4, 5, 6], 2, 3);
        let b = Matrix::new(vec![1, 2, 3, 4, 5, 6], 3, 2);
        println!("left: \n{}", a);
        println!("right: \n{}", b);
        let c = multiply(&a, &b).unwrap();
        println!("left * right: \n{}", c);
    }
}
