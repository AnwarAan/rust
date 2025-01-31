mod kedua;
mod pertama;

mod model {
    pub struct User {
        pub first_name: String,
        pub last_name: String,
        pub username: String,
        pub email: String,
        pub age: u8,
    }

    impl User {
        pub fn say_hello(&self, name: &str) {
            println!("Hello {}, my name is {}", name, self.first_name);
        }
    }
}

mod first {
    pub fn say_hello() {
        println!("Hello first");
    }
}

mod second {
    pub fn say_hello() {
        println!("Hello second");
    }
}

use std::{
    cell::{Ref, RefCell},
    cmp::Ordering,
    collections::{btree_map::Values, BTreeSet, HashMap, HashSet, LinkedList, VecDeque},
    fmt::format,
    result, str,
};

use first::say_hello as say_hello1;
use second::say_hello as say_hello2;

#[test]
fn test_user() {
    say_hello1();
    say_hello2();
    pertama::kedua::say_hello();
}

fn main() {
    let user = model::User {
        first_name: String::from("Muchamad"),
        last_name: String::from("Coirul"),
        username: String::from("mcnwr"),
        email: String::from("@mail.com"),
        age: 10,
    };
    user.say_hello("Banan");

    println!("Hello, world!");
}

#[test]
fn test_use() {
    first::say_hello();
    second::say_hello();

    say_hello1();
    say_hello2();
}

#[test]
fn hello_world() {
    println!("Hello, test");
}

#[test]
fn variable() {
    let name = "Hai";
    println!("{}", name);
}

#[test]
fn mutable() {
    let mut name = "Hai";
    println!("{}", name);
    name = "Hai, saya Mca";
    println!("{}", name);
}

#[test]
fn static_typing() {
    let name = "Hai";
    println!("{}", name);

    // let name=1;
}

#[test]
fn shadowing() {
    let name = "Hai";
    println!("{}", name);
    let name = 1;
    println!("{}", name);
}

#[test]
fn comment() {
    // println!("Hello, test");
}

#[test]
fn explicit() {
    let age: i128 = 32;
    println!("{}", age);
}

#[test]
fn number() {
    let age = 32.1;
    println!("{}", age);
}

#[test]
fn number_conversion() {
    let a: i8 = 100;
    let b: i16 = a as i16;
    let c = b as i32;
    let d = 1000000000;
    let e = d as i8;
    println!("{}", c);
    println!("{}", e);
}

#[test]
fn number_operator() {
    let a = 100;
    let b = 100;
    let c = a * b;
    println!("{}", c);
}

#[test]
fn assigment_operator() {
    let mut a = 100;
    a = a + 10;
    println!("{}", a);
}

#[test]
fn boolean() {
    let a = true;
    let b = false;

    println!("{} {}", a, b);
}

#[test]
fn comparison() {
    let a = 10;
    let b = 20;
    let c = a > b;
    let d = a < b;

    println!("{} {}", c, d);
}

#[test]
fn boolean_operator() {
    let a = true;
    let b = false;
    let c = a && b;
    let d = a || b;
    let e = !a;
    println!("{}, {}, {}", c, d, e);
}

#[test]
fn char() {
    let a = 'a';
    println!("{}", a);
}

#[test]
fn tuple() {
    let data = (10, 10.5, true);
    println!("{:?}", data);
    // let a = data.0;
    // let b = data.1;
    // let c = data.2;

    let (a, _, c) = data;

    println!("{}, {}", a, c);
}

fn unit() {
    println!("Hello")
}

#[test]
fn call_unit() {
    let res = unit();
    println!("{:?}", res);

    let test = ();
    println!("{:?}", test);
}

#[test]
fn array() {
    let array = [1, 2, 3, 4, 5];
    println!("{:?}", array);

    let a = array[0];
    let b = array[1];
    println!("{},{}", a, b);
    println!("{}", array.len());
}

#[test]
fn two_dimensional_array() {
    let matrix: [[i32; 3]; 2] = [[1, 2, 3], [4, 5, 6]];

    println!("{:?}", matrix);
    println!("{:?}", matrix[0]);
    println!("{:?}", matrix[0][0]);
    println!("{:?}", matrix[0][1]);
    println!("{:?}", matrix[0][2]);
    println!("{:?}", matrix[1]);
    println!("{:?}", matrix[1][0]);
    println!("{:?}", matrix[1][1]);
    println!("{:?}", matrix[1][2]);
}

const MAXIMUM: i32 = 100;

#[test]
fn constant() {
    const MINIMUM: i32 = 0;
    println!("{} {}", MINIMUM, MAXIMUM);
}

#[test]
fn variable_scope() {
    println!("{}", MAXIMUM);

    let user = 1;

    {
        println!("{}", user);
        let user2 = 2;
        println!("{}", user2);
    }

    // println!("{}", user2); //error
}

#[test]
fn stack_heap() {
    function_a();
    function_b();
}

fn function_a() {
    let a = 10;
    let b = String::from("user2");
    println!("{}, {}", a, b);
}

fn function_b() {
    let a = 10;
    let b = String::from("user");
    println!("{}, {}", a, b);
}

#[test]
fn string() {
    let name: &str = "   user user2 user3   ";
    let trim: &str = name.trim();

    println!("{}", name);
    println!("{}", trim);
}

#[test]
fn string_type() {
    let mut name: String = String::from("user user2");
    println!("{}", name);

    name.push_str(" user3");
    println!("{}", name);

    let budi = name.replace("user", "Budi");
    println!("{}", name);
    println!("{}", budi);
}

#[test]
fn ownership_rules() {
    let a = 10;

    {
        let b = 10;
        println!("{}", b);
    }

    println!("{}", a);
}

#[test]
fn data_copy() {
    let a = 10;
    let b = a;

    println!("{} {}", a, b);
}

#[test]
fn ownership_movement() {
    let name1: String = String::from("user");
    println!("{}", name1);

    let name2: String = name1; // ownerhsip pindah ke name2
    println!("{}", name2);
    // println!("{}", name1);
}

#[test]
fn clone() {
    let name1 = String::from("user");
    let name2 = name1.clone();

    println!("{} {}", name1, name2);
}

#[test]
fn if_expression() {
    let value = 10;
    let result: &str = if value >= 8 {
        "Good"
    } else if value >= 6 {
        "Not Bad"
    } else if value >= 3 {
        "Bad"
    } else {
        "Very Bad"
    };

    println!("{}", result);
}

#[test]
fn loop_expression() {
    let mut counter = 0;
    loop {
        counter += 1;

        if counter > 10 {
            break;
        } else if counter % 2 == 0 {
            continue;
        }

        println!("Counter : {}", counter);
    }
}

#[test]
fn loop_return_value() {
    let mut counter = 0;
    let result = loop {
        counter += 1;
        if counter > 10 {
            break counter * 2;
        }
    };

    println!("{}", result);
}

#[test]
fn loop_label() {
    let mut number = 1;
    'outer: loop {
        let mut i = 1;
        loop {
            if number > 10 {
                break 'outer;
            }

            println!("{} x {} = {}", number, i, number * i);
            i += 1;
            if i > 10 {
                break;
            }
        }
        number += 1;
    }
}

#[test]
fn while_loop() {
    let mut counter = 0;
    while counter <= 10 {
        if counter % 2 == 0 {
            println!("Counter : {}", counter);
        }

        counter += 1;
    }
}

#[test]
fn array_iteration() {
    let array: [&str; 5] = ["A", "B", "C", "D", "E"];
    let mut index = 0;

    while index < array.len() {
        println!("Value : {}", array[index]);
        index += 1;
    }
}

#[test]
fn array_iteration_for_loop() {
    let array: [&str; 5] = ["A", "B", "C", "D", "E"];

    for value in array {
        println!("Value : {}", value);
    }
}

#[test]
fn range() {
    let range = 0..5;
    println!("Start : {}", range.start);
    println!("End : {}", range.end);

    let array: [&str; 5] = ["A", "B", "C", "D", "E"];

    for i in range {
        println!("{}", array[i]);
    }
}

#[test]
fn range_inclusive() {
    let range = 0..=4;
    println!("Start : {}", range.start());
    println!("End : {}", range.end());

    let array: [&str; 5] = ["A", "B", "C", "D", "E"];

    for i in range {
        println!("{}", array[i]);
    }
}

fn say_hello() {
    println!("Hello");
}

#[test]
fn test_say_hello() {
    say_hello();
    say_hello();
    say_hello();
    say_hello();
}

fn say_goodbye(first_name: &str, last_name: &str) {
    println!("Goodbye {} {}", first_name, last_name);
}

#[test]
fn test_say_goodbye() {
    say_goodbye("user", "user3");
    say_goodbye("Budi", "Nugraha");
    say_goodbye("Joko", "Susilo");
}

fn factorial_loop(n: i32) -> i32 {
    if n < 1 {
        return 0;
    }

    let mut result = 1;
    for i in 1..=n {
        result *= i;
    }

    result
}

#[test]
fn test_factorial_loop() {
    let result: i32 = factorial_loop(5);
    println!("{}", result);

    let result: i32 = factorial_loop(-10);
    println!("{}", result);
}

fn print_text(value: String, times: u32) {
    if times == 0 {
        return;
    } else {
        println!("{}", value);
    }

    print_text(value, times - 1);
}

#[test]
fn test_print_text() {
    print_text(String::from("user"), 10);
}

fn factorial_recursive(n: u32) -> u32 {
    if n <= 1 {
        return 1;
    }

    n * factorial_recursive(n - 1)
}

#[test]
fn test_factorial_recursive() {
    let result = factorial_recursive(5);
    println!("{}", result);
}

fn print_number(number: i32) {
    println!("number {}", number);
}

fn hi(name: String) {
    println!("name {}", name);
}

#[test]
fn test_hi() {
    let number = 10;
    print_number(number); // print_number(10)
    println!("{}", number);

    let name = String::from("user");
    hi(name);
    // println!("{}", name);
}

fn full_name(first_name: &String, last_name: &String) -> String {
    format!("{} {}", first_name, last_name)
}

#[test]
fn test_full_name() {
    let first_name = String::from("muchamaad");
    let last_name = String::from("coirul");
    let name = full_name(&first_name, &last_name);

    println!("{}", first_name);
    println!("{}", last_name);
    println!("{}", name);
}

fn change_value(value: &mut String) {
    value.push_str("Test");
}

#[test]
fn test_change_value() {
    let mut value = String::from("Muchamad");
    let value_borrow = &mut value;

    change_value(value_borrow);
    change_value(value_borrow);
    change_value(value_borrow);
}

fn full_name_2(first_name: &String, last_name: &String) -> String {
    let full_name = format!("{} {}", first_name, last_name);
    return full_name;
}

#[test]
fn test_full_name_2() {
    let first_name = String::from("Muchamad");
    let last_name = String::from("Coirul");

    let name = full_name_2(&first_name, &last_name);
    println!("{}", name)
}

#[test]
fn slice_reference() {
    let array = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    let a = &array[..];
    println!("{:?}", a);

    let b = &array[..5];
    println!("{:?}", b);

    let c = &array[5..];
    println!("{:?}", c);

    println!("{:?}", array);
}

#[test]
fn string_clice() {
    let name = String::from("Muchamad Coirul");
    // let name = "Muchamad Coirul";
    let first_name = &name[0..3];
    println!("{:?}", first_name);
}

struct Person {
    first_name: String,
    last_name: String,
    age: u8,
}

impl Person {
    fn say_hello(&self, name: &str) {
        println!("Hello {}, my name is {}", name, self.first_name);
    }
}

fn print_person(person: &Person) {
    println!("{}", person.first_name);
    println!("{}", person.last_name);
    println!("{}", person.age);
}

#[test]
fn struc_fn() {
    let first_name = String::from("Muchamad");
    let person = Person {
        first_name,
        last_name: String::from("Coirul"),
        age: 10,
    };
    print_person(&person);

    let person2 = Person {
        first_name: person.first_name.clone(),
        last_name: person.last_name.clone(),
        ..person
    };
    print_person(&person2);
    println!("{}", person.first_name);
}

struct Geo(f64, f64);

#[test]
fn tuple_struct() {
    let geo = Geo(21.341515, 22.2415151);
    println!("Long: {}", geo.0);
    println!("Lat: {}", geo.1);
}

struct Nothing;

#[test]
fn struct_nothing() {
    let _n1 = Nothing;
    let _n2 = Nothing {};
}

#[test]
fn test_method() {
    let person = Person {
        first_name: String::from("Muchamad"),
        last_name: String::from("Coirul"),
        age: 20,
    };

    person.say_hello("user");
    println!("{}", person.first_name);
}
impl Geo {
    fn new(long: f64, lat: f64) -> Geo {
        Geo(long, lat)
    }
}
#[test]
fn test_method_new() {
    let geo = Geo::new(12.4198419, 13.14212414);
    println!("Long:{}", geo.0);
    println!("Lat{}", geo.1);
}

enum Level {
    Regular,
    Premiun,
    Platinum,
}

#[test]
fn test_enum() {
    let _level = Level::Premiun;
}

enum Payment {
    CreditCard(String),
    BankTransfer(String, String),
    EWallet(String, String),
}

impl Payment {
    fn pay(&self, amount: u32) {
        match self {
            Payment::CreditCard(number) => {
                println!("Paying with Credit Card {} amount {}", number, amount)
            }
            Payment::BankTransfer(bank, number) => {
                println!(
                    "Paying with Bank Transfer {} {} amount {}",
                    bank, number, amount
                )
            }
            Payment::EWallet(wallet, number) => {
                println!(
                    "Paying with EWallet {} {} amount {}",
                    wallet, number, amount
                )
            }
        }
    }
}

#[test]
fn test_payment() {
    let payment1 = Payment::CreditCard(String::from("BCA"));
    payment1.pay(100000);

    let payment2 = Payment::BankTransfer(String::from("BCA"), String::from("123456789"));
    payment2.pay(100000);

    let payment3 = Payment::EWallet(String::from("Gopay"), String::from("123456789"));
    payment3.pay(100000);
}

#[test]
fn test_enum_match() {
    let level = Level::Premiun;
    match level {
        Level::Regular => {
            println!("Regular")
        }
        Level::Premiun => {
            println!("Premiun")
        }
        Level::Platinum => {
            println!("Platinum")
        }
    }
}

#[test]
fn test_match_value() {
    let name = "Muchamad";
    match name {
        "Muchamad" => {
            print!("Hello Muchamad");
        }
        other => {
            print!("Hello Something Wrong");
        }
    }

    match name {
        "Muchamad" | "user" => {
            print!("Hal Bang");
        }
        other => {
            print!("Hello Something Wrong");
        }
    }
}

#[test]
fn test_range() {
    let value = 100;
    match value {
        75..=100 => {
            println!("A");
        }
        50..=75 => {
            println!("B");
        }
        25..=50 => {
            println!("C");
        }
        0..=25 => {
            println!("D");
        }
        other => {
            println!("Invalid Number {}", other)
        }
    }
}

#[test]
fn test_struct_pattern() {
    let point = Geo(0.0, 10.1);
    match point {
        Geo(long, 0.0) => {
            println!("Long: {}", long)
        }

        Geo(0.0, lat) => {
            println!("Lat: {}", lat)
        }
        Geo(long, lat) => {
            println!("Long: {}, Lat: {}", long, lat)
        }
    }

    let person = Person {
        first_name: String::from("Muchamad"),
        last_name: String::from("Coirul"),
        age: 10,
    };

    match person {
        Person {
            first_name,
            last_name,
            ..
        } => {
            println!("{}, {}", first_name, last_name);
        }
    }
}

#[test]
fn test_ignoring() {
    let point = Geo(10.0, 11.0);
    match point {
        Geo(long, _) => {
            println!("Long: {}", long)
        }
        _ => {
            println!("Ignore")
        }
    }
}

#[test]
fn test_match() {
    let value = 32;
    let res = match value {
        0 => "Nol",
        1 => "Satu",
        2 => "Dua",
        _ => "Invalid",
    };
    println!("{}", res)
}

type Age = u8;
type IndetifyNumber = String;

struct Customer {
    id: IndetifyNumber,
    name: String,
    age: Age,
}

#[test]
fn alias() {
    let customer = Customer {
        id: String::from("123456"),
        name: String::from("Muchamad"),
        age: 10,
    };
    println!(
        "ID:{}, Name: {}, Age:{}",
        customer.id, customer.name, customer.age
    )
}

trait CanSayHello {
    fn hello(&self) -> String {
        String::from("Hello")
    }
    fn say_hello(&self) -> String;
    fn say_hello_to(&self, name: &str) -> String;
}

trait CanSayGoodBye {
    fn say_good_bye(&self) -> String;
    fn say_good_bye_to(&self, name: &str) -> String;
}

impl CanSayGoodBye for Person {
    fn say_good_bye(&self) -> String {
        format!("Goodbye, my name is {}", self.first_name)
    }

    fn say_good_bye_to(&self, name: &str) -> String {
        format!("Goodbye {}, my name is {}", name, self.first_name)
    }
}

impl CanSayHello for Person {
    fn say_hello(&self) -> String {
        format!("Hello, my name is {}", self.first_name)
    }
    fn say_hello_to(&self, name: &str) -> String {
        format!("Good bye {}, my name is {}", name, self.first_name)
    }
}

fn say_hello_trait(value: &impl CanSayHello) {
    println!("{}", value.say_hello());
}

fn hello_and_good_bye(value: &(impl CanSayHello + CanSayGoodBye)) {
    println!("{}", value.say_hello());
    println!("{}", value.say_good_bye());
}

#[test]
fn test_trait() {
    let person = Person {
        first_name: String::from("Muchamad"),
        last_name: String::from("Muchamad"),
        age: 10,
    };

    say_hello_trait(&person);
    hello_and_good_bye(&person);

    let result = person.say_hello_to("May");
    println!("{}", result);

    let result = person.hello();
    println!("{}", result);

    println!("{}", person.say_good_bye());
    println!("{}", person.say_good_bye_to("Mei"));

    CanSayHello::say_hello(&person);
    Person::say_hello(&person, "Mei");
}

struct SimplePerson {
    name: String,
}

impl CanSayGoodBye for SimplePerson {
    fn say_good_bye(&self) -> String {
        format!("Goodbye, my name is {}", self.name)
    }

    fn say_good_bye_to(&self, name: &str) -> String {
        format!("Goodbye {}, my name is {}", name, self.name)
    }
}

fn create_person(name: String) -> impl CanSayGoodBye {
    SimplePerson { name }
}

#[test]
fn test_return_trait() {
    let person = create_person(String::from("Mca"));
    print!("{}", person.say_good_bye());
    print!("{}", person.say_good_bye_to("Mei"));
}

trait CanSay: CanSayHello + CanSayGoodBye {
    fn say(&self) {
        println!("{}", self.say_hello());
        println!("{}", self.say_good_bye());
    }
}

struct Point<T = i32> {
    x: T,
    y: T,
}

impl<T> Point<T> {
    fn get_x(&self) -> &T {
        &self.x
    }
    fn get_y(&self) -> &T {
        &self.y
    }
}

#[test]
fn test_generic_struct() {
    let integer = Point::<i32> { x: 1, y: 2 };
    let float = Point::<f32> { x: 1.0, y: 2.0 };
}

enum Value<T> {
    NONE,
    VALUE(T),
}

#[test]
fn test_generic_enum() {
    let value = Value::<i32>::VALUE(10);

    match value {
        Value::NONE => {
            println!("none")
        }
        Value::VALUE(value) => {
            println!("{}", value)
        }
    }
}

struct Hi<T = SimplePerson>
where
    T: CanSayGoodBye,
{
    value: T,
}

#[test]
fn test_generic_bound() {
    let hi = Hi::<SimplePerson> {
        value: SimplePerson {
            name: String::from("MCA"),
        },
    };
    println!("{}", hi.value.name)
}

fn min<T>(value1: T, value2: T) -> T
where
    T: PartialOrd,
{
    if value1 < value2 {
        value1
    } else {
        value2
    }
}

#[test]
fn generic_in_function() {
    let result = min(10, 20);
    println!("{}", result);

    let result = min(20.0, 10.);
    println!("{}", result);
}

#[test]
fn test_generic_method() {
    let point = Point { x: 10, y: 10 };
    println!("{}", point.get_x());
    println!("{}", point.get_x());
}

trait GetValue<T>
where
    T: PartialOrd,
{
    fn get_value(&self) -> &T;
}

impl<T> GetValue<T> for Point<T>
where
    T: PartialOrd,
{
    fn get_value(&self) -> &T {
        &self.x
    }
}

use core::{hash, ops::Add};
struct Apple {
    quantity: i32,
}

impl Add for Apple {
    type Output = Apple;

    fn add(self, rhs: Self) -> Self::Output {
        Apple {
            quantity: self.quantity + rhs.quantity,
        }
    }
}

#[test]
fn test_operator_add() {
    let apple1 = Apple { quantity: 10 };
    let apple2 = Apple { quantity: 10 };
    let apple3 = apple1 + apple2;

    println!("{}", apple3.quantity);
}

fn double(value: Option<i32>) -> Option<i32> {
    match value {
        None => None,
        Some(i) => Some(i * 2),
    }
}

#[test]
fn test_option() {
    let result = double(Some(10));
    println!("{:?}", result);

    let result = double(None);
    println!("{:?}", result);
}

impl PartialEq for Apple {
    fn eq(&self, other: &Self) -> bool {
        self.quantity == other.quantity
    }
}

impl PartialOrd for Apple {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.quantity.partial_cmp(&other.quantity)
    }
}

#[test]
fn test_comparing() {
    let apple1 = Apple { quantity: 10 };
    let apple2 = Apple { quantity: 10 };

    println!("Apple1 == Apple2 : {}", apple1 == apple2);
    println!("Apple1 < Apple2 : {}", apple1 < apple2);
    println!("Apple1 > Apple2 : {}", apple1 > apple2);
}

#[test]
fn test_string_manipulation() {
    let s = String::from("Muchamad Coirul Anwar");

    println!("{}", s.to_uppercase());
    println!("{}", s.to_lowercase());
    println!("{}", s.len());
    println!("{}", s.replace("Coirul", "Choirul"));
    println!("{}", s.starts_with("Much"));
    println!("{}", s.ends_with("war"));
    println!("{}", s.trim());
    println!("{}", &s[0..3]);
    println!("{:?}", s.get(0..3));
}

struct Category {
    id: String,
    name: String,
}

use std::fmt::{Debug, Formatter};

impl Debug for Category {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Category")
            .field("id", &self.id)
            .field("name", &self.name)
            .finish()
    }
}

#[test]
fn test_format() {
    let category = Category {
        name: String::from("Gadget"),
        id: String::from("GADGET"),
    };
    println!("{:?}", category);
}

#[test]
fn test_closure() {
    let sum: fn(i32, i32) -> i32 = |value1: i32, value2: i32| -> i32 { value1 + value2 };

    let result = sum(10, 10);
    println!("{}", result);
}

fn print_with_filter(value: String, filter: fn(String) -> String) {
    let result = filter(value);
    println!("{}", result);
}

#[test]
fn test_closuer_as_parameter() {
    let filter = |value: String| -> String { value.to_uppercase() };

    print_with_filter(String::from("MCA"), filter);
}

fn to_uppercase(value: String) -> String {
    value.to_uppercase()
}

#[test]
fn test_function_as_closure() {
    print_with_filter(String::from("MCA"), to_uppercase);
}

#[test]
fn test_closure_score() {
    let mut counter = 0;
    let mut increment = || {
        counter += 1;
        println!("Increment")
    };

    increment();
    increment();
    increment();

    println!("Counter {}", counter);
}

struct Counter {
    counter: i32,
}

impl Counter {
    fn increment(&mut self) {
        self.counter += 1;
        println!("Increment");
    }
}

#[test]
fn test_counter() {
    let mut counter = Counter { counter: 0 };
    counter.increment();
    counter.increment();
    counter.increment();
    counter.increment();
    counter.increment();

    println!("Counter {}", counter.counter);
}

#[test]
fn test_vector() {
    let array = ["Muchamad", "Coirul", "Anwar"];

    for value in array {
        println!("{}", value)
    }

    println!("{:?}", array);

    let mut names: Vec<String> = Vec::<String>::new();
    names.push(String::from("Muchamad"));
    names.push(String::from("Coirul"));
    names.push(String::from("Anwar"));

    for name in &names {
        println!("{}", name)
    }

    println!("{:?}", names);
    println!("{}", names[0]);
}

#[test]
fn test_vector_deque() {
    let mut names: VecDeque<String> = VecDeque::<String>::new();
    names.push_back(String::from("Muchamad"));
    names.push_back(String::from("Coirul"));
    names.push_back(String::from("Anwar"));

    for name in &names {
        println!("{}", name)
    }

    println!("{}", names[1]);
}

#[test]
fn test_linked_list() {
    let mut names = LinkedList::<String>::new();
    names.push_back(String::from("Muchamad"));
    names.push_back(String::from("Coirul"));
    names.push_back(String::from("Anwar"));

    for name in names {
        println!("{}", name)
    }
}

#[test]
fn test_has_map() {
    let mut map = HashMap::new();
    map.insert(String::from("name"), String::from("Muchamad"));
    map.insert(String::from("age"), String::from("20"));
    map.insert(String::from("gender"), String::from("male"));

    let name = map.get("name");
    let age = map.get("age");

    println!("Name {}", name.unwrap());
    println!("Age {}", age.unwrap());

    for entry in map {
        println!("{} : {}", entry.0, entry.1)
    }
}

#[test]
fn test_hash_set() {
    let mut set = HashSet::new();
    set.insert(String::from("Muchamad"));
    set.insert(String::from("Muchamad"));
    set.insert(String::from("Coirul"));
    set.insert(String::from("Coirul"));
    set.insert(String::from("Anwar"));
    set.insert(String::from("Anwar"));

    for value in set {
        println!("{}", value)
    }
}

#[test]
fn test_btree_set() {
    let mut set = BTreeSet::new();
    set.insert(String::from("Muchamad"));
    set.insert(String::from("Muchamad"));
    set.insert(String::from("Coirul"));
    set.insert(String::from("Coirul"));
    set.insert(String::from("Anwar"));
    set.insert(String::from("Anwar"));

    for value in set {
        println!("{}", value)
    }
}

#[test]
fn test_iterator() {
    let array = [1, 2, 3, 4, 5];
    let mut iterator = array.iter();

    while let Some(value) = iterator.next() {
        println!("{}", value)
    }

    for value in iterator {
        println!("{}", value)
    }
}

#[test]
fn test_iterator_method() {
    let vector = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    println!("{:?}", vector);

    let sum: i32 = vector.iter().sum();
    println!("{:?}", sum);

    let count = vector.iter().count();
    println!("{:?}", count);

    let double: Vec<i32> = vector.iter().map(|x| x * 2).collect();
    println!("{:?}", double);

    let odd: Vec<&i32> = vector.iter().filter(|x| *x % 2 != 0).collect();
    println!("{:?}", odd);
}

fn connect_database(host: Option<String>) {
    match host {
        None => {
            panic!("No cache host")
        }
        Some(host) => {
            println!("Connect to database {}", host)
        }
    }
}

#[test]
fn test_panic() {
    connect_database(Some(String::from("localhost")));
}

fn connect_cache(host: Option<String>) -> Result<String, String> {
    match host {
        None => Err("No cache host".to_string()),
        Some(host) => Ok(host),
    }
}

fn connect_email(host: Option<String>) -> Result<String, String> {
    match host {
        None => Err("No emaill host".to_string()),
        Some(host) => Ok(host),
    }
}

fn connect_application(host: Option<String>) -> Result<String, String> {
    connect_cache(host.clone())?;
    connect_email(host.clone())?;
    Ok("Connected to app".to_string())
}

#[test]
fn test_application_error() {
    let result = connect_application(None);
    match result {
        Ok(host) => {
            println!("Success Connect with message {}", host)
        }
        Err(err) => {
            println!("Success Connect with message {}", err)
        }
    }
}

#[test]
fn test_recoverable_error() {
    let cache = connect_cache(None);

    match cache {
        Ok(host) => {
            println!("Success Connect with message {}", host)
        }
        Err(err) => {
            println!("Success Connect with message {}", err)
        }
    }
}

#[test]
fn test_dangling_reference() {
    let r: &i32;
    {
        let x = 5;
    }
    r = &40;
    println!("{}", r);
}

fn longest<'a>(value1: &'a str, value2: &'a str) -> &'a str {
    if value1.len() > value2.len() {
        value1
    } else {
        value2
    }
}

#[test]
fn test_lifetime_annotation() {
    let value1 = "Muchamad";
    let value2 = "Coirul";
    let value3 = "Anwar";
}

#[test]
fn test_lifetime_annotation_danging() {
    let string1 = String::from("Muchamad");
    let string2 = String::from("Coirul");
    let result;

    {
        result = longest(string1.as_str(), string2.as_str());
    }

    println!("{}", result);
}

struct Student<'a, 'b> {
    name: &'a str,
    last_name: &'b str,
}

impl<'a, 'b> Student<'a, 'b> {
    fn longest_name(&self, student: &Student<'a, 'b>) -> &'a str {
        if self.name.len() > student.name.len() {
            self.name
        } else {
            student.name
        }
    }
}

fn longest_student_name<'a, 'b>(student1: &Student<'a, 'b>, student2: &Student<'a, 'b>) -> &'a str {
    if student1.name.len() > student2.name.len() {
        student1.name
    } else {
        student2.name
    }
}

#[test]
fn test_student() {
    let student1 = Student {
        name: "MCA",
        last_name: "Anwar",
    };

    println!("{}", student1.name);

    let student2 = Student {
        name: "Coirul",
        last_name: "mcnwr",
    };
    let result = longest_student_name(&student1, &student2);
    println!("{}", result);

    let result = student1.longest_name(&student2);
    println!("{}", result);
}

struct Teacher<'a, ID>
where
    ID: Ord,
{
    id: ID,
    name: &'a str,
}

#[test]
fn test_lifetime_annotation_generic() {
    let teacher = Teacher {
        id: 10,
        name: "MCA",
    };

    println!("{}", teacher.id);
    println!("{}", teacher.name);
}

#[derive(Debug, PartialEq, PartialOrd)]
struct Company {
    name: String,
    location: String,
    web: String,
}
#[test]
fn test_attribute_derive() {
    let company = Company {
        name: "Cohen Lobe".to_string(),
        location: "US".to_string(),
        web: "https://www.xxx".to_string(),
    };
    let company2 = Company {
        name: "Cohen Lobe".to_string(),
        location: "US".to_string(),
        web: "https://www.xxx".to_string(),
    };
    let company3 = Company {
        name: "Cohen Lobe".to_string(),
        location: "US".to_string(),
        web: "https://www.xxx".to_string(),
    };

    println!("{:?}", company);

    let result = company == company2;
    println!("{}", result);

    let result = company > company2;
    println!("{}", result);
}

fn display_number(value: i32) {
    println!("{}", value);
}

fn display_number_reference(value: &i32) {
    println!("{}", value);
}

#[test]
fn test_box() {
    let value = Box::new(10);
    println!("{}", value);
    display_number(*value);
    display_number_reference(&value);
}

#[derive(Debug)]
enum ProductCategory {
    Of(String, Box<ProductCategory>),
    End,
}

#[test]
fn test_box_enum() {
    let category = ProductCategory::Of(
        "Laptop".to_string(),
        Box::new(ProductCategory::Of(
            "Dell".to_string(),
            Box::new(ProductCategory::End),
        )),
    );
    println!("{:?}", category);
}

#[test]
fn test_deference() {
    let value1 = Box::new(10);
    let value2 = Box::new(20);

    let result = *value1 * *value2;
    println!("{}", result);
}

struct MyValue<T> {
    value: T,
}

use std::ops::Deref;

impl<T> Deref for MyValue<T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

#[test]
fn test_deference_struct() {
    let value = MyValue { value: 10 };
    let real_value = *value;
    println!("value {}", real_value);
}

fn say_hello_reference(name: &String) {
    println!("Hello {}", name);
}

#[test]
fn test_deref_reference() {
    let name = MyValue {
        value: "Mca".to_string(),
    };

    say_hello_reference(&name);
}

struct Book {
    title: String,
}

impl Drop for Book {
    fn drop(&mut self) {
        println!("Dropping Book {}", self.title)
    }
}

#[test]
fn test_drop() {
    let book = Book {
        title: "Rust Programming".to_string(),
    };
    println!("{}", book.title);
}
use std::rc::Rc;

enum Brand {
    Of(String, Rc<Brand>),
    End,
}

#[test]
fn test_multiple_owneship() {
    let apple = Rc::new(Brand::Of("Apple".to_string(), Rc::new(Brand::End)));
    println!("Apple Reference Count : {}", Rc::strong_count(&apple));

    let laptop = Rc::new(Brand::Of("Apple".to_string(), Rc::new(Brand::End)));
    println!("Apple Reference Count : {}", Rc::strong_count(&apple));

    {
        let hp = Rc::new(Brand::Of("Apple".to_string(), Rc::new(Brand::End)));
        println!("Apple Reference Count : {}", Rc::strong_count(&apple));
    }

    println!("Apple Reference Count : {}", Rc::strong_count(&apple));
}

#[derive(Debug)]
struct Seller {
    name: RefCell<String>,
    active: RefCell<bool>,
}

#[test]

fn test_ref_cell() {
    let seller = Seller {
        name: RefCell::new("Mca".to_string()),
        active: RefCell::new(true),
    };

    {
        let mut result = seller.name.borrow_mut();
        *result = "Anwar".to_string();
    }

    println!("{:?}", seller);
}

static APPLICATION: &str = "My App";

#[test]
fn test_static() {
    println!("{}", APPLICATION);
}

static mut COUNTER: u32 = 0;

unsafe fn increment() {
    COUNTER += 1;
}

#[test]
fn test_unsafe() {
    unsafe {
        increment();
        COUNTER += 1;
        println!("Counter : {}", COUNTER);
    }
}

macro_rules! hi {
    () => {
        println!("Hi Macro!")
    };
    ($name: expr) => {
        println!("Hi {}!", $name);
    };
}

#[test]
fn test_macro() {
    hi!();
    hi!("MCA");
    hi! {
        "ANWAR"
    };

    let name = "mcnwr";
    hi!(name);
}

macro_rules! iterate {
    ($array:expr) => {
        for i in $array{
            println!("{}",i);
        }
    };
    ($($item: expr),*)=>{
        $(
            println!("{}",$item);
        )*
    }
}

#[test]
fn test_macro_iterate() {
    iterate!([1, 2, 3, 4, 5, 6, 7, 8, 9, 10]);
    iterate!(1, 2, 3, 4, 5, 6, 7, 8, 9, 10);
}
