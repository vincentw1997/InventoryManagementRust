use std::fs::File;
use std::fmt::{Display, Formatter};
use std::fmt::Result as OtherResult; 
use std::io::{self, Write, Read, BufReader};
use chrono::{DateTime, TimeZone, NaiveDateTime, Utc};
use cli_table::{format::Justify, print_stdout, Cell, Row, Style, Table, WithTitle};
use serde::{Serialize, Deserialize};

#[derive(Table, Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Student {
    ID : String, //can be immatrikulationsnummer 
    fullname: String, 
    email_address: String,
}

impl Display for Student {
    fn fmt(&self, f: &mut Formatter<'_>) -> OtherResult {
        write!(f, "{} {} {}", 
                self.ID, 
                self.fullname,
                self.email_address,)
            }
        }

#[derive(Table, Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Item{
    item_name: String,
    category_name: String,
    serial_number: i64, //can be the IMEI of the item so 15 numbers
    item_id: String,
    price: f32, //
    location: String,
    conditions: String,
    remarks: String,
    item_status: String, //new, used but Ok, broken
    date_added: DateTime<Utc>,
    borrowed_until: DateTime<Utc>,

}

impl Display for Item {
        fn fmt(&self, f: &mut Formatter<'_>) -> OtherResult {
            write!(f, "{} {} {} {} {} {} {} {} {} {} {}", 
                    self.item_name, 
                    self.category_name,
                    self.serial_number,
                    self.item_id,
                    self.price,
                    self.location,
                    self.conditions,
                    self.remarks,
                    self.item_status,
                    self.date_added,
                    self.borrowed_until)
    }
}

struct Category{
    category_name: String,
    category_id: String,

}

#[derive(Table, Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LendReturnOrder{
    action_id: String,
    action_name: String, // lend or return
    current_date: DateTime<Utc>,
    student_name: String,
    item_id: String,
    lend_date_start: DateTime<Utc>,
    lend_date_end: DateTime<Utc>,
    return_date: DateTime<Utc>,
    condition: String,
    remarks: String, 
}

impl Display for LendReturnOrder {
    fn fmt(&self, f: &mut Formatter<'_>) -> OtherResult {
        write!(f, "{} {} {} {} {} {} {} {} {} {}", 
                self.action_id, 
                self.action_name,
                self.current_date,
                self.student_name,
                self.item_id,
                self.lend_date_end,
                self.lend_date_start,
                self.return_date,
                self.condition,
                self.remarks,
                )
}
}

// struct ReturnOrder{
//     action_id: String,
//     current_date: SystemTime,
//     student_name: String,  
//     item_id: String,
//     lending_date_start: DateTime<Utc>,
//     lending_date_end: DateTime<Utc>,
//     return_date: SystemTime,
//     condition: String,
//     remarks: String,

// }

struct InventoryList {
    item_id: String,
    item_name: String,
    item_quantity: i32,
    status_available: i32,
    status_borrowed: i32,
}


fn item_generator(
    item_name: String,
    category_name: String,
    serial_number: i64, //can be the IMEI of the item so 15 numbers
    item_id: String,
    price: f32, //
    location: String,
    conditions: String,
    remarks: String,
    item_status: String, //new, used but Ok, broken
    date_added: DateTime<Utc>,
    borrowed_until: DateTime<Utc>,
) -> Item {
    Item {item_name: item_name, category_name: category_name, serial_number: serial_number, item_id: item_id, price: price, location: location,conditions: conditions,remarks:remarks, item_status: item_status, date_added: date_added,borrowed_until: borrowed_until}
}


fn student_generator(
    ID : String, 
    fullname: String, 
    email_address: String,
) -> Student {
    Student {ID: ID, fullname : fullname, email_address: email_address}
}

fn order_generator(
    action_id: String,
    action_name: String, // lend or return
    current_date: DateTime<Utc>,
    student_name: String,
    item_id: String,
    lend_date_start: DateTime<Utc>,
    lend_date_end: DateTime<Utc>,
    return_date: DateTime<Utc>,
    condition: String,
    remarks: String, 
) -> LendReturnOrder {
    LendReturnOrder {
        action_id: action_id,
        action_name: action_name,
        current_date: current_date,
        student_name: student_name, 
        item_id: item_id, 
        lend_date_start: lend_date_start,
        lend_date_end: lend_date_end,
        return_date: return_date,
        condition: condition,
        remarks: remarks,  
    }
}

fn main() {

    let mut item_list: Vec<Item> = Vec::new();
    let mut student_list: Vec<Student> = Vec::new();
    let mut order_list: Vec<LendReturnOrder> = Vec::new();

    // Load the JSON for Item list
    let json_list_path:&str = "./data/item.json";
    let json_student_path:&str = "./data/student.json";
    let json_order_path:&str = "./data/order.json";


    
    let file_item = File::open(json_list_path)
    .expect("file should open read only");

    let json_item: serde_json::Value = serde_json::from_reader(file_item)
    .expect("file should be proper JSON");
    
    // println!("{:?}", json_item.to_string());

    //item_list.push(json_item);

    //let student = json.get("student")
    //.expect("file should have Name key");
    //println!("Student {:?}", student);
    
    loop {
    println!("Laboratory Inventory Management");
    println!("-----------------------");
    println!("1. Item");
    println!("2. List of Students");
    println!("3. Lend / Return item");
    // println!("4. Return item");
    println!("4. Exit Program");
    println!("Choose the corresponding numbers:");
    println!("\n");

    let mut home_input = String::new();
    let stdin = io::stdin();
    let mut counter = 0;
    let now = Utc::now();
    let mut loop_counter = 0;
    

         
    while counter < 1 {
        
        stdin.read_line(&mut home_input).unwrap();
        let mut trimmed = home_input.trim();

        if trimmed.parse::<i8>() != Ok(1) && trimmed.parse::<i8>() != Ok(2) && trimmed.parse::<i8>() != Ok(3) && trimmed.parse::<i8>() != Ok(4) {
            println!("wrong input, please re enter the number");
            home_input.clear();
            continue;
        }

        else if trimmed.parse::<i8>() == Ok(1) {
            println!("You choose the {}. Item menu", trimmed);
            println!("\n");
            println!("1. Add New Item");
            println!("2. Add New Category");
            println!("3. Master of Item");
            println!("4. Return to Home Menu"); 
            println!("\n");
            
            let mut item_input = String::new();
            let mut counter_item = 0;

            while counter_item < 1{
                
                stdin.read_line(&mut item_input).unwrap();
                let mut trimmed_item = item_input.trim();
                

                let mut item_1 = Item {
                    item_name: "Monitor".to_string(),
                    category_name: "Hardware".to_string(),
                    serial_number: 123456789101112, //can be the IMEI of the item so 15 numbers
                    item_id: "blank".to_string(),
                    price: 200.00, //
                    location: "A".to_string(),
                    conditions: "Good".to_string(),
                    remarks: "".to_string(),
                    item_status: "OK".to_string(), //True means available, false means borrowed
                    date_added: now,
                    borrowed_until: now,
                };
                //item_list.push(item_1);
                
                if trimmed_item.parse::<i8>() != Ok(1) && trimmed_item.parse::<i8>() != Ok(2) && trimmed_item.parse::<i8>() != Ok(3) && trimmed_item.parse::<i8>() != Ok(4) {
                    println!("wrong input, please re enter the number");
                    item_input.clear();
                    continue;

                }

                else if trimmed_item.parse::<i8>() == Ok(1){
                    println!("You choose {}. Add New Item.", trimmed_item);
                    println!("\n");
                    let mut repeat = String::new();
                    let mut repeater = 0;

                    while repeater < 1 {
                        
                        let mut new_item = String::new();
                        let mut b = String::new();
                        let mut c_1:i64 = 000000000000;
                        let mut c = String::new();
                        let mut d = String::new();
                        let mut e_1: f32 = 0.00;
                        let mut e = String::new();
                        let mut f = String::new();
                        let mut g = String::new();
                        let mut h = String::new();
                        let mut i = String::new();
                        let mut j = now;
                        let mut k = now;

                        println!("Please enter the name of the item:");
                        let n = stdin.read_line(&mut new_item);
                        println!("\n");

                        println!("Please enter the category name:");
                        let n_1 = stdin.read_line(&mut b);
                        println!("\n");

                        println!("Please enter the serial number:");
                        let n_2 = stdin.read_line(&mut c);
                        println!("\n");

                        println!("Please enter the item ID:");
                        let n_3 = stdin.read_line(&mut d);
                        println!("\n");

                        println!("Please enter the price:");
                        let n_4 = stdin.read_line(&mut e);
                        println!("\n");

                        println!("Please enter the locations:");
                        let n_5 = stdin.read_line(&mut f);
                        println!("\n");

                        println!("Please enter the conditions:");
                        let n_6 = stdin.read_line(&mut g);
                        println!("\n");

                        println!("Please enter the remarks:");
                        let n_7 = stdin.read_line(&mut h);
                        println!("\n");

                        println!("Please enter the item status:");
                        let n_8 = stdin.read_line(&mut i);
                        println!("\n");

                        c_1 = c.trim().parse::<i64>().unwrap(); //error if input other things outside numbers
                        e_1 = e.trim().parse::<f32>().unwrap(); //error if input other things outside numbers
                        let mut a = item_generator(new_item, b, c_1, d, e_1, f, g, h, i, j, k,);
                        print!("Summary of the item added:
                        Item name: {} 
                        Category: {} 
                        Serial number: {} \n
                        item ID: {}
                        Price: {} \n 
                        Location: {} 
                        Conditions: {} 
                        Remarks: {} 
                        Item Status: {} 
                        Date added: {} 
                        Borrowed until: {} \n", a.item_name, a.category_name, a.serial_number, a.item_id, a.price, a.location, a.conditions, a.remarks, a.item_status, a.date_added, a.borrowed_until);
                        item_list.push(a);

                        let json_list_add = serde_json::to_string(&item_list).unwrap();
                        println!("{:?}", json_list_add);

                        std::fs::write(
                            json_list_path,
                            serde_json::to_string_pretty(&json_list_add).unwrap(),
                        ).unwrap();
                        
                        println!("Length: {}", item_list.len());

                        for (count, v) in item_list.iter().enumerate(){
                            //println!("{:?}", count);
                            //println!("{}", v);
                            println!("{}", item_list[count].category_name)
                        }
                        
                        //TODO; already can call the respective item list based on the index
                        //TODO; still there is a problem when exiting the program the item_list is not carried over. // solve by moving the creation of the vector outside the loop
                        

                        println!("\n Please press 1 if you want to add another item. \n Please press 2 if you want to go back to Item menu.");
                        let mut repeat_ans = stdin.read_line(&mut repeat).unwrap();

                        match repeat.trim().parse::<i8>() {
                            Ok(1) => {repeat.clear()},
                            Ok(2) => {repeater = 1},
                            _ => {repeater = 1;
                                println!("invalid response")}
                        }
                        
                                   
                    }
                    counter_item +=1;
                
                }

                else if trimmed_item.parse::<i8>() == Ok(2) {
                    println!("You choose {}. Add New Category.", trimmed_item);
                    println!("\n");

                    //only for category
                    let mut category = Vec::new();

                    for (count, v) in item_list.iter().enumerate(){
                        //println!("{:?}", count);
                        //println!("{}", v);
                        let mut a =  &item_list[count].category_name ;
                        category.push(a)                    
                    };



                    for (count, z) in category.iter().enumerate(){
                        // println!("{:?}", count);
                        // println!("{}", z);
                        let table = vec![
                            vec![z.cell()],
                        ]
                        .table()
                        .title(vec![
                            "Category of Item".cell().bold(true),
                        ])
                        .bold(true);

                        assert!(print_stdout(table).is_ok());

                    };

                    //TODO; add new function to only add the category
                    counter_item += 1;
                }
                else if trimmed_item.parse::<i8>() == Ok(3) {
                    println!("You choose {}. Master of Item.", trimmed_item);
                    println!("Master of item");
                    println!("--------------");
                    println!("1. Search for item name");
                    println!("2. Add item");
                    println!("3. Return to previous menu");

                    // not what I intended
                    // let table = vec![
                    //     vec![item_1.item_name.cell()],
                    //     vec!["empty".cell()],
                    // ]
                    // .table()
                    // .title(vec![
                    //     "Name of Item".cell().bold(true),
                    // ])
                    // .bold(true);

                    // assert!(print_stdout(table).is_ok());
                    
                    // printing all the item that is already saved in the memory
                    print!("{}", print_stdout(item_list.with_title()).is_ok());
                
                    
                    let mut moi_input = String::new();
                    stdin.read_line(&mut moi_input).unwrap();
                    let trimmed_moi = moi_input.trim();

                    if trimmed_moi.parse::<i8>() == Ok(1){
                        println!("You choose {}. You search for item name", trimmed_moi);
                        println!("\n");
                        println!("Please enter item name for search purposes.");
                        let mut search_engine = String::new();
                        let search_engine_responds = stdin.read_line(&mut search_engine);

                        let mut item_name_searching = Vec::new();
                        
                        for (count, v) in item_list.iter().enumerate(){
                            let mut a =  &item_list[count].item_name ;
                            item_name_searching.push(a)                    
                        };
                        
                        if item_name_searching.contains(&&search_engine) {
                            let index = item_name_searching.iter().position(|&r| r == &search_engine).unwrap();
                            println!("Index in item list is: {}", index)
                        }
                        counter_item += 1;
                    }
                    else if trimmed_moi.parse::<i8>() == Ok(2){
                        println!("You choose {}. Add Item", trimmed_moi);
                        println!("\n");
                        // moi_input.clear();
                        item_input = String::from("1");
                        println!("press Enter to continue...");
                        continue;
                    }
                    else if trimmed_moi.parse::<i8>() == Ok(3){
                        println!("You choose {}. Return to previous menu", trimmed_moi);
                        println!("\n");
                        counter_item += 1;
                    }
                    
                    
                } 
                else if trimmed_item.parse::<i8>() == Ok(4){
                    println!("You choose {}. Return to Home Menu", trimmed_item);
                    counter_item += 1;
                }
            }
            counter += 1;
        }
        
        else if trimmed.parse::<i8>() == Ok(2) {

            assert!(print_stdout(student_list.with_title()).is_ok());


            let mut student_counter = 0;
            let mut student_input = String::new();

            while student_counter < 1 { 
                println!("You choose {}. List of students menu", trimmed);
                println!("\n");
                println!("Choose the corresponding number for action:");
                println!("1. Add new student");
                println!("2. Search for student");
                println!("3. Return to previous menu.");
                println!("\n");

                let mut student_ans = stdin.read_line(&mut student_input).unwrap();
                let mut add_s_counter = 0;
                
                match student_input.trim().parse::<i8>() {
                    Ok(1) => {add_s_counter = 1},
                    Ok(2) => {add_s_counter = 2},
                    Ok(3) => {
                        println!("You choose {}. return to Home Menu", student_input);
                        add_s_counter = 0;
                        counter = 0;
                        student_counter = 1;
                    },

                    _ => {student_input.clear();
                        println!("invalid response")}
                }
                
                while add_s_counter == 1 {
                    let mut s_ID = String::new();
                    let mut s_name = String::new();
                    let mut s_email = String::new();

                    println!("Please enter the matrikulation number:");
                    let s = stdin.read_line(&mut s_ID);
                    println!("\n");

                    println!("Please enter the full name of the student:");
                    println!("\n");
                    let s_1 = stdin.read_line(&mut s_name);
                    println!("\n");

                    println!("Please enter the email:");
                    let s_2 = stdin.read_line(&mut s_email);
                    println!("\n");

                    let s_a = student_generator(s_ID , s_name, s_email);
                    print!("Summary of the item added:
                            ID: {} 
                            Fullname: {} 
                            email address: {}", s_a.ID, s_a.fullname, s_a.email_address);
                            
                    student_list.push(s_a);

                    let json_student_add = serde_json::to_string(&student_list).unwrap();
                        println!("{:?}", json_student_add);

                        std::fs::write(
                            json_student_path,
                            serde_json::to_string_pretty(&json_student_add).unwrap(),
                        ).unwrap();
                    
                    //println!("{}", student_list[0]);
                    assert!(print_stdout(student_list.with_title()).is_ok());


                    println!("\n Please press 1 if you want to add another student. \n Please press 2 if you want to go back to previous menu.");
                    let mut add_s_input = String::new();
                    let mut add_s_ans = stdin.read_line(&mut add_s_input).unwrap();
                    
                    match add_s_input.trim().parse::<i8>() {
                        Ok(1) => {add_s_input.clear()},
                        Ok(2) => {add_s_counter = 0;
                        student_input.clear()},
                        _ => {add_s_input.clear(); println!("This is an invalid response");}
                    }

                }
                while add_s_counter == 2 {
                    println!("You choose {}. You search for student name", add_s_counter);
                    println!("\n");
                    println!("Please enter student name for search purposes.");
                    let mut search_student= String::new();
                    let search_student_responds = stdin.read_line(&mut search_student);

                    let mut student_name_searching = Vec::new();
                    
                    for (count, v) in student_list.iter().enumerate(){
                        let mut a =  &student_list[count].fullname ;
                        student_name_searching.push(a)                    
                    };
                    
                    if student_name_searching.contains(&&search_student) {
                        let index = student_name_searching.iter().position(|&r| r == &search_student).unwrap();
                        println!("Index in student list is: {}", index)
                    }
                    student_counter = 0;
                    add_s_counter = 0;
                    student_input.clear();
                    }
              }
            counter += 1;
        } 
    
        else if trimmed.parse::<i8>() == Ok(3) {

            let mut lending_counter = 0;
            let mut lending_input = String::new();
            
            while lending_counter < 1 {
                println!("You choose {}. Lend / Return Item menu", trimmed);
                println!("\n");
                println!("Choose the corresponding number for action:");
                println!("1. Lend Item");
                println!("2. Return Item");
                println!("3. See All Lend Item.");
                println!("4. Return to previous menu.");
                println!("\n");

                let mut lend_ans = stdin.read_line(&mut lending_input).unwrap();
                let mut add_l_counter = 0;
                
                match lending_input.trim().parse::<i8>() {
                    Ok(1) => {add_l_counter = 1},
                    Ok(2) => {add_l_counter = 2},
                    Ok(3) => {add_l_counter = 3}, 
                    Ok(4) => {
                        println!("You choose {}. return to Previous Menu", lending_input.trim());
                        add_l_counter = 0;
                        counter = 0;
                        lending_counter = 1;
                    },

                    _ => {lending_input.clear();
                        println!("invalid response")}
                }

                while add_l_counter == 1 { //go to the lending page
                    // let gen = SequenceGenerator::default();
                    // let id = gen.next_id();
                    let mut l_ID = String::new();
                    let mut status = "lend";
                    let mut c_date = now;
                    let mut l_name = String::new();
                    let mut l_item = String::new();
                    let mut l_lend_start = now;
                    let mut l_lend_end = now;
                    let mut l_return = now;
                    let mut l_condition = String::new();
                    let mut l_remarks = String::new();

                    println!("You choose {}. Lend Item", lending_input.trim());

                    println!("Please enter the order ID:");
                    let l = stdin.read_line(&mut l_ID);
                    println!("\n");

                    println!("Please enter the Borrower name:");
                    let l_1 = stdin.read_line(&mut l_name);
                    println!("\n");

                    println!("Please enter the item ID borrowed:");
                    let l_2 = stdin.read_line(&mut l_item);
                    println!("\n");

                    println!("Please enter the condition of the item:");
                    let l_3 = stdin.read_line(&mut l_condition);
                    println!("\n");

                    println!("Please add any remarks:");
                    let l_4 = stdin.read_line(&mut l_remarks);
                    println!("\n");


                    let l_a = order_generator(l_ID , status.to_string(), c_date, l_name, l_item, l_lend_start, l_lend_end, l_return, l_condition, l_remarks);
                    print!("Summary of the item added:
                            ID: {} 
                            Status: {}
                            Current date: {}
                            Borrower: {}
                            Item ID: {}
                            Lend Date Start: {} 
                            Lend Date End: {}
                            Return Date: {}
                            Condition: {}
                            Remark: {}
                            ", l_a.action_id, l_a.action_name, l_a.current_date, l_a.student_name, l_a.item_id, l_a.lend_date_start, l_a.lend_date_end, l_a. return_date, l_a.condition, l_a.remarks);
                            
                    order_list.push(l_a);

                    assert!(print_stdout(order_list.with_title()).is_ok());

                    let json_order_add = serde_json::to_string(&order_list).unwrap();
                        println!("{:?}", json_order_add);

                        std::fs::write(
                            json_order_path,
                            serde_json::to_string_pretty(&json_order_add).unwrap(),
                        ).unwrap();

                    println!("\n Please press 1 if you want to add another lend order. \n Please press 2 if you want to go back to previous menu.");
                    let mut add_l_input = String::new();
                    let mut add_l_ans = stdin.read_line(&mut add_l_input).unwrap();
                    
                    match add_l_input.trim().parse::<i8>() {
                        Ok(1) => {add_l_input.clear()},
                        Ok(2) => {add_l_counter = 0;
                        lending_input.clear()},
                        _ => {add_l_input.clear(); println!("This is an invalid response");}
                    }

                }

                while add_l_counter == 2 { // go to return page
                    let mut l_ID = String::new();
                    let mut status = "return";
                    let mut c_date = now;
                    let mut l_name = String::new();
                    let mut l_item = String::new();
                    let mut l_lend_start = now;
                    let mut l_lend_end = now;
                    let mut l_return = now;
                    let mut l_condition = String::new();
                    let mut l_remarks = String::new();

                    println!("You choose {}. Return Item", lending_input.trim());

                    println!("Please enter the order ID:");
                    let l = stdin.read_line(&mut l_ID);
                    println!("\n");

                    println!("Please enter the Borrower name:");
                    let l_1 = stdin.read_line(&mut l_name);
                    println!("\n");

                    println!("Please enter the item ID borrowed:");
                    let l_2 = stdin.read_line(&mut l_item);
                    println!("\n");

                    println!("Please enter the condition of the item:");
                    let l_3 = stdin.read_line(&mut l_condition);
                    println!("\n");

                    println!("Please add any remarks:");
                    let l_4 = stdin.read_line(&mut l_remarks);
                    println!("\n");


                    let l_a = order_generator(l_ID , status.to_string(), c_date, l_name, l_item, l_lend_start, l_lend_end, l_return, l_condition, l_remarks);
                    print!("Summary of the item added:
                            ID: {} 
                            Status: {}
                            Current date: {}
                            Borrower: {}
                            Item ID: {}
                            Lend Date Start: {} 
                            Lend Date End: {}
                            Return Date: {}
                            Condition: {}
                            Remark: {}
                            ", l_a.action_id, l_a.action_name, l_a.current_date, l_a.student_name, l_a.item_id, l_a.lend_date_start, l_a.lend_date_end, l_a. return_date, l_a.condition, l_a.remarks);
                            
                    order_list.push(l_a);

                    assert!(print_stdout(order_list.with_title()).is_ok());

                    println!("\n Please press 1 if you want to add another return order. \n Please press 2 if you want to go back to previous menu.");
                    let mut add_l_input = String::new();
                    let mut add_l_ans = stdin.read_line(&mut add_l_input).unwrap();
                    
                    match add_l_input.trim().parse::<i8>() {
                        Ok(1) => {add_l_input.clear()},
                        Ok(2) => {add_l_counter = 0;
                        lending_input.clear()},
                        _ => {add_l_input.clear(); println!("This is an invalid response");}
                    }

                }

                while add_l_counter == 3 {
                    println!("You choose {}. See All Lend / Return Item.", lending_input.trim());
                    assert!(print_stdout(order_list.with_title()).is_ok());

                    println!("Press 1. To return to previous menu");
                    let mut see_l_input = String::new();
                    let mut see_l_ans = stdin.read_line(&mut see_l_input).unwrap(); 
                    
                    match see_l_input.trim().parse::<i8>(){
                        Ok(1) => {add_l_counter = 0; lending_input.clear()},
                        _ => {see_l_input.clear(); println!("Wrong input, please enter 1.")}
                    }
                }
            }



            counter += 1;
        }

        // else if trimmed.parse::<i8>() == Ok(4) {
        //     println!("You choose {}. Return Item menu", trimmed);
        //     counter += 1;
        // }
        
        else if trimmed.parse::<i8>() == Ok(4) {
            println!("Exitting program");
            loop_counter += 1;
            break;
        }
        

    
    }
    if loop_counter == 1 {
        break;
    }
    }

}