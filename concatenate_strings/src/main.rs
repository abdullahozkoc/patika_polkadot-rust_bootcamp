fn main() {
    

    let string1 = String::from("Abdullah");
    let string2 = String::from(" Özkoç");

    let concatenated_string = concatenate_strings(&string1, &string2);

    println!("{}",concatenated_string);

}


fn concatenate_strings( string1:&str,string2:&str) ->String {
    let mut result = String::from(string1);
    result.push_str(string2);
    return result ;
}