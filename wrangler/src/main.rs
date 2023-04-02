mod dataframe;

use dataframe::DataFrame;

fn main() {
    // Read the data from a CSV file
    let data = DataFrame::read_csv("data.csv").unwrap();
    println!("Original data:\n{:?}", data);

    // Select a column and print it
    let column_b = data.select("b").unwrap();
    println!("Column 'b':\n{:?}", column_b);

    // Select multiple columns and print them
    let columns_ac = data.select_many(&["a", "c"]).unwrap();
    println!("Columns 'a' and 'c':\n{:?}", columns_ac);

    // Compute the mean and print it
    let mean = data.mean();
    println!("Mean:\n{:?}", mean);

    // Write the modified data to a new CSV file
    data.write_csv("modified_data.csv").unwrap();
}
