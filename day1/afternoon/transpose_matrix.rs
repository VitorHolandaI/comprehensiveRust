fn transpose(matrix: [[i32; 3]; 3]) -> [[i32; 3]; 3] {
    let mut matrix_clone = matrix.clone();
    println!("Clone:");
    for row in &matrix_clone {
        println!("{:?}", row);
    }
    for index in 0..3 {
        for index2 in 0..3 {
            matrix_clone[index2][index] = matrix[index][index2];
        }
    }
    matrix_clone
}

fn main() {
    let matrix = [
        [101, 102, 103], // <-- the comment makes rustfmt add a newline
        [201, 202, 203],
        [301, 302, 303],
    ];

    println!("Original:");
    for row in &matrix {
        println!("{:?}", row);
    }

    let transposed = transpose(matrix);

    println!("\nTransposed:");
    for row in &transposed {
        println!("{:?}", row);
    }
}
