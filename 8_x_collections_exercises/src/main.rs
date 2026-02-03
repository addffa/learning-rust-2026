fn main() {
    // 1. Given a list of integers, use a vector and return the median
    // (when sorted, the value in the middle position) and mode
    // (the value that occurs most often; a hash map will be helpful here) of the list.
    let mut nums = vec![1, 9, 5, 8, 3, 4, 6, 7, 2, 5, 3];
    let len = nums.len();
    nums.sort();
    let median = if len % 2 == 0 {
        (nums[len>>1] + nums[(len>>1)+1]) / 2
    } else {
        nums[len>>1]
    };
    println!("median: {median}");

    use std::collections::HashMap;
    let mut mode = HashMap::new();
    for &mut num in nums.iter_mut() {
        mode.entry(num).and_modify(|num| { *num += 1}).or_insert(1);
    }
    println!("mode: {mode:#?}");

    // Convert strings to Pig Latin. The first consonant of each word is moved to the end of
    // the word and ay is added, so first becomes irst-fay. Words that start with a vowel
    // have hay added to the end instead (apple becomes apple-hay).
    // Keep in mind the details about UTF-8 encoding!
    let mut word = String::from("apply");
    println!("word: {word}");
    let first_char = word.remove(0);
    let vowels = ['a', 'i', 'u', 'e', 'o'];
    let pig_latin = if vowels.contains(&first_char) {
        format!("{first_char}{word}-hay")
    } else {
        format!("{word}-{first_char}-ay")
    };
    println!("Pig Latin: {pig_latin}");

    // Using a hash map and vectors, create a text interface to allow a user to add
    // employee names to a department in a company; for example,
    // “Add Sally to Engineering” or “Add Amir to Sales.” Then, let the user retrieve
    // a list of all people in a department or all people in the company by department,
    // sorted alphabetically.
    let _commands = [
        String::from("Add Sally to Engineering"),
        String::from("Add Amir to Sales"),
    ];
    let commands = [
        (String::from("Sally"), String::from("Engineering")),
        (String::from("Amir"), String::from("Sales")),
    ];

    let mut deps_empl_map = HashMap::<String, Vec<String>>::new();
    for (name, dept) in commands {
        deps_empl_map.entry(dept).or_insert(Vec::<String>::new()).push(name);
    }
    println!("People in department Sales:");
    if let Some(sales_people) = &deps_empl_map.get("Sales") {
        println!("{sales_people:#?}");
    } else {
        println!("No people in this dept");
    }
    for (dept, employees) in deps_empl_map.iter_mut() {
        println!("Department: {dept}");
        employees.sort();
        println!("List of sorted employee: {employees:#?}");
    }
}
