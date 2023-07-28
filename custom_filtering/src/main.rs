
struct FilterCondition<T> {
    condition: T,
}

impl<T: PartialEq> FilterCondition<T> {
    fn is_match(&self, item: &T) -> bool {
        &self.condition == item
    }
}


fn custom_filter<T>(collection: Vec<T>, condition: &FilterCondition<T>) -> Vec<T>
where
    T: PartialEq + Clone,
{
    let mut filtered_collection = Vec::new();
    for item in collection {
        if condition.is_match(&item) {
            filtered_collection.push(item.clone());
        }
    }
    filtered_collection
}


fn main() {
    // Create a collection (e.g., a vector) with some elements
    let original_collection = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];

    // Initialize a FilterCondition object with the desired value
    let filter_condition = FilterCondition { condition: 5 };

    // Call the custom_filter function with the collection and FilterCondition object
    let filtered_collection = custom_filter(original_collection, &filter_condition);

    // Print the filtered result to the console
    println!("Filtered collection: {:?}", filtered_collection);
}

