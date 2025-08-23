fn main() {
    // Vector
    let mut v: Vec<i32> = Vec::new();
    
    // This is another form to create a vector 
    // with their respective Macro
    let _v_2 = vec![1, 2, 3];

    // Also you can add values to vector with push method.
    v.push(5);
    v.push(6);
    v.push(7);
    v.push(8);

    // Reading elements from a Vector
    
    let v_3 = vec![1, 2, 3, 4, 5];

    let third: &i32 = &v_3[2];
    println!("The third element is {third}");

    let third: Option <&i32> = v_3.get(2);
    //let third: Option <&i32> = None;
    match third {
        Some(third) => println!("The third element is {third}"),
        None => println!("There is no third element!.")
    }


    //let does_not_exist = &v[100]; // This code finish in panic! 
    let does_not_exist = v.get(100); // This just return a None
    
    //let first = &v[0];
    //
    //v.push(6);
    //
    //println!("The first element is: {first}")
    //
    //In the last example the borrowin 
    //not allow reference a index value not mutable!.

    let mut v = vec![100, 32, 57];
    for i in &v {
        println!("{i}");
    }

    for i in &mut v{
        *i += 50;
        println!("{i}")
    }

    // You can store multiple data types
    // with the help of a enum inside the vector
    enum SpreadSheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }
    
    let row = vec![
        SpreadSheetCell::Int(3),
        SpreadSheetCell::Text(String::from("blue")),
        SpreadSheetCell::Float(10.12),
    ];


    // Dropping vector Elements
    
    let mut v = vec![1, 2, 3, 4];

    v.pop();

    println!("Look: {v:?}");

}
