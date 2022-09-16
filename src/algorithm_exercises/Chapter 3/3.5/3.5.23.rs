#![allow(non_snake_case)]
#![allow(non_camel_case_types)]

// p522

// 3.5.23 - Sparse matrices. 
// Develop an API and an implementation for sparse 2D matrices. Support matrix addition and matrix multiplication. Include constructors for row and column vectors.

mod utils {pub mod LinearProbingHashTable;}
use utils::LinearProbingHashTable::LinearProbingHashTable as HashST;

pub struct SparseVector {
    st: HashST<usize, f32>
}

impl SparseVector {
    pub fn new(size: usize) -> Self {
        SparseVector{st: HashST::new(size)}
    }

    pub fn size(&self) -> usize {self.st.size()}

    pub fn put(&mut self, index: usize, number: f32) {self.st.put(index, number)}

    pub fn delete(&mut self, index: usize) {self.st.delete(index)}

    pub fn get(&self, index: &usize) -> f32 {
        if !self.st.contains(index) {return 0.0}
        else {return self.st.get(index).unwrap()}
    }

    // Find dot product of stored SparseVector, by another vector
    pub fn dot(&self, vector: Vec<&f32>) -> f32 {
        let mut sum = 0.0;

        for index in self.st.keys() {
            sum += vector[*index as usize] * self.get(index)
        }

        sum
    }

    // Term-by-term sum
    // Handle case where entry becomes 0
    pub fn sum(&self, vector: SparseVector) -> SparseVector {
        // Create sum vector
        let size1 = self.size();
        let size2 = vector.size();
        let mut sum_vector: SparseVector;

        if size1 > size2 {sum_vector = SparseVector::new(size1)}
        else {sum_vector = SparseVector::new(size2)}

        // Get all indexes to iterate over
        let mut index_set = self.st.keys();

        for index in vector.st.keys() {
            if !index_set.contains(&index) {index_set.push(index)}
        }

        index_set.sort();

        // Iterate over each index

        for index in index_set {
            let index_clone = index.clone();
            let sum = self.get(index) + vector.get(index);
            if sum != 0.0 {sum_vector.put(index_clone, sum);}
        }

        sum_vector
    }
}

pub struct SparseMatrix {
    numRows: usize,
    numColumns: usize,
    rows: Vec<SparseVector>,
    columns: Vec<SparseVector>,
}

impl SparseMatrix {
    pub fn new(numRows: usize, numColumns: usize) -> Self {
        let mut rows = Vec::new();
        let mut columns = Vec::new();
        
        for _ in 0..numRows {rows.push(SparseVector::new(numColumns))}
        for _ in 0..numColumns {columns.push(SparseVector::new(numRows))}

        SparseMatrix {
            numRows: numRows,
            numColumns: numColumns,
            rows: rows,
            columns: columns
        }
    }

    pub fn get(&self, i: usize, j: usize) -> Result<f32, &'static str> {
        if i >= self.numColumns || j >= self.numRows {return Err("invalid i and/or j value")}
        Ok(self.columns[i].get(&j))
    }

    pub fn put(&mut self, i: usize, j: usize, value: f32) -> Result<bool, &'static str> {
        if i >= self.numColumns || j >= self.numRows {return Err("invalid i and/or j values")}

        if value == 0.0 {
            self.delete(i, j);
            return Ok(true)
        }

        self.columns[i].put(j, value);
        self.rows[j].put(i, value);
        return Ok(true)
    }

    pub fn delete(&mut self, i: usize, j: usize) -> Result<bool, &'static str> {
        if i >= self.numColumns || j >= self.numRows {return Err("invalid i and/or j values")}

        self.columns[i].delete(j);
        self.rows[j].delete(i);
        return Ok(true)
    }

    pub fn sum(&self, matrix: &SparseMatrix) -> Result<SparseMatrix, &'static str> {
        if matrix.numRows != self.numRows || matrix.numColumns != self.numColumns {return Err("Matrices must match in height and width")}
        
        let mut sum = SparseMatrix::new(self.numRows, self.numColumns);

        for i in 0..self.numColumns {
            for j in 0..self.numRows {
                let element1 = self.get(i, j).unwrap();
                let element2 = matrix.get(i, j).unwrap();
                sum.put(i, j, element1 + element2);
            }
        }

        Ok(sum)
    }

    pub fn dot(&self, matrix: &SparseMatrix) -> Result<SparseMatrix, &'static str> {
        if self.numColumns != matrix.numRows {return Err("Matrix width != provided matrix height")}
        let mut dot = SparseMatrix::new(self.numRows, matrix.numColumns);

        for i in 0..matrix.numColumns {
            for j in 0..self.numRows {
                let mut element = 0.0;
                for k in 0..self.numColumns {
                    element += self.get(k, j).unwrap() * matrix.get(i, k).unwrap();
                }
                dot.put(i, j, element);
            }
        }

        Ok(dot)
    }

}

fn main() {
    let mut matrixA = SparseMatrix::new(2, 2);
    let mut matrixB = SparseMatrix::new(2, 2);
    let mut matrixC = SparseMatrix::new(2, 3);
    let mut matrixD = SparseMatrix::new(2, 3);

    //Matrix A
    // 1 0
    // 7 2

    matrixA.put(0, 0, 1.0);
    matrixA.put(0, 1, 7.0);
    matrixA.put(1, 1, 2.0);

    //Matrix B
    // -4 -5
    //  2  1

    matrixB.put(0, 0, -4.0);
    matrixB.put(0, 1, 2.0);
    matrixB.put(1, 0, -5.0);
    matrixB.put(1, 1, 1.0);

    //Matrix C
    // 3 4 2
    // 1 0 3

    matrixC.put(0, 0, 3.0);
    matrixC.put(0, 1, 1.0);
    matrixC.put(1, 0, 4.0);
    matrixC.put(2, 0, 2.0);
    matrixC.put(2, 1, 3.0);

    //Matrix D
    // 0 0 0
    // 0 2 0

    matrixD.put(1, 1, 2.0);

    //Matrix A + Matrix B
    // -3 -5
    //  9  3

    let sum1 = matrixA.sum(&matrixB).unwrap();
    assert!(sum1.get(0, 0) == Ok(-3.0));
    assert!(sum1.get(0, 1) == Ok(9.0));
    assert!(sum1.get(1, 0) == Ok(-5.0));
    assert!(sum1.get(1, 1) == Ok(3.0));

    //Matrix A x Matrix C
    //  3  4  2
    // 23 28 20

    let dot1 = matrixA.dot(&matrixC).unwrap();
    assert!(dot1.get(0, 0) == Ok(3.0));
    assert!(dot1.get(0, 1) == Ok(23.0));
    assert!(dot1.get(1, 0) == Ok(4.0));
    assert!(dot1.get(1, 1) == Ok(28.0));
    assert!(dot1.get(2, 0) == Ok(2.0));
    assert!(dot1.get(2, 1) == Ok(20.0));

    //Matrix B x Matrix C
    // -17 -16 -23
    //   7   8   7

    let dot2 = matrixB.dot(&matrixC).unwrap();
    assert!(dot2.get(0, 0) == Ok(-17.0));
    assert!(dot2.get(0, 1) == Ok(7.0));
    assert!(dot2.get(1, 0) == Ok(-16.0));
    assert!(dot2.get(1, 1) == Ok(8.0));
    assert!(dot2.get(2, 0) == Ok(-23.0));
    assert!(dot2.get(2, 1) == Ok(7.0));

    //Matrix C + Matrix D
    // 3 4 2
    // 1 2 3

    let sum2 = matrixC.sum(&matrixD).unwrap();
    assert!(sum2.get(0, 0) == Ok(3.0));
    assert!(sum2.get(0, 1) == Ok(1.0));
    assert!(sum2.get(1, 0) == Ok(4.0));
    assert!(sum2.get(1, 1) == Ok(2.0));
    assert!(sum2.get(2, 0) == Ok(2.0));
    // assert!(sum2.get(2, 1) == Ok(3.0));

    //Matrix A x Matrix D
    // 0 0 0
    // 0 4 0

    let dot3 = matrixA.dot(&matrixD).unwrap();
    assert!(dot3.get(0, 0) == Ok(0.0));
    assert!(dot3.get(0, 1) == Ok(0.0));
    assert!(dot3.get(1, 0) == Ok(0.0));
    assert!(dot3.get(1, 1) == Ok(4.0));
    assert!(dot3.get(2, 0) == Ok(0.0));
    assert!(dot3.get(2, 1) == Ok(0.0));

    //Matrix B x Matrix D
    // 0 -10 0
    // 0   2 0

    let dot4 = matrixB.dot(&matrixD).unwrap();
    assert!(dot4.get(0, 0) == Ok(0.0));
    assert!(dot4.get(0, 1) == Ok(0.0));
    assert!(dot4.get(1, 0) == Ok(-10.0));
    assert!(dot4.get(1, 1) == Ok(2.0));
    assert!(dot4.get(2, 0) == Ok(0.0));
    assert!(dot4.get(2, 1) == Ok(0.0));
}