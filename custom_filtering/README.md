# Custom Filtering Function in Rust

This Rust project implements a custom filtering function that allows filtering elements from a given collection based on a specific condition. The goal is to provide a beginner-friendly solution that avoids using closures to simplify the understanding of the code.

## How to Use

1. Create a new Rust project by running the following command in the terminal:


2. Open the `main.rs` file in a text editor.

3. Define a struct called `FilterCondition` with a single field of the desired type for filtering.

4. Implement a method called `is_match` on the `FilterCondition` struct that takes a reference to an item of the same type as the filter condition and returns a boolean indicating whether the item matches the condition.

5. Define a function called `custom_filter` that takes a collection (e.g., a vector) and a reference to a `FilterCondition` object as arguments. The function iterates over the elements in the collection and returns a new collection containing only the elements that match the filter condition.

6. In the `main` function, create a collection (e.g., a vector) with some elements and initialize a `FilterCondition` object with the desired value.

7. Call the `custom_filter` function with the collection and the `FilterCondition` object, storing the result in a new variable.

8. Print the filtered result to the console.

9. Compile and run the program to test its functionality.


