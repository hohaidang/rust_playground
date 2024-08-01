// use print_macro::PrintOnCreate;

// #[derive(PrintOnCreate, Default)]
// struct MyStruct {
//     field1: i32,
//     field2: String,
// }

// #[derive(PrintOnCreate, Default)]
// struct AnotherStruct {
//     field1: i32,
// }

// fn main() {
//     let _instance = MyStruct::new();
//     let _another_instance = AnotherStruct::new();
//     #[cfg(debug_assertions)]
//     {
//         println!("Debug build");
//     }
//     #[cfg(not(debug_assertions))]
//     {
//         println!("Release build");
//     }
// }
/* ____________________________ */

/* tuple struct*/
// pub struct Point(u32, u32);
// pub enum PointError {
//     ToaDoZero,
// }

// impl Point {
//     pub fn new(x: u32, y: u32) -> Result<Self, PointError> {
//         if x == 0 && y == 0 {
//             Err(PointError::ToaDoZero)
//         } else {
//             Ok(Self(x, y))
//         }
//     }

//     pub fn x(&self) -> u32 { //&self: co chuc nang define read-only function
//         self.0
//     }

//     pub fn y(&self) -> u32 {
//         self.1
//     }
//     // Self la kieu du lieu
//     // self la refer den bien (data)
//     // &self la read-only refer den bien
//     // &mut self la read-write refer den bien
// }

// fn main() {
//     let result: Result<Point, PointError> = Point::new(1, 2);
//     let point = match result {
//         Ok(point) => {
//             println!("Here is called");
//             point
//         },
//         Err(e) => {
//             print!("Error");
//             return;
//         }
//     };
//     println!("x = {}, y = {}", point.x(), point.y());
// }
/* ____________________________ */

// /* Compute hash code */
// use postgres::{Client, NoTls};
// use sha2::{Sha256, Digest};
// use std::fmt::Write;

// fn main() -> Result<(), Box<dyn std::error::Error>> {
//     // Connect to the PostgreSQL database
//     // let mut client = Client::connect("host=0.0.0.0 user=raptor password=Raptor dbname=raptor2_test", NoTls)?;
//     let mut client = Client::connect("host=/home/haidang/raptor2/db user=haidang password=Raptor dbname=raptor2 port=5433", NoTls)?;

//     // Retrieve all table names
//     let tables = client.query("SELECT table_name FROM information_schema.tables WHERE table_schema = 'public'", &[])?;

//     // To store the hash of the entire schema
//     let mut overall_hasher = Sha256::new();

//     for table in &tables {
//         let table_name: &str = table.get("table_name");
//         println!("Processing table: {}", table_name);

//         // Retrieve the schema of the table
//         let columns = client.query(
//             "SELECT column_name, data_type, is_nullable, column_default
//              FROM information_schema.columns
//              WHERE table_name = $1 AND table_schema = 'public'",
//             &[&table_name],
//         )?;

//         // To store the hash of the current table schema
//         let mut table_hasher = Sha256::new();

//         // Concatenate all column definitions into a single string
//         let mut schema_str = String::new();
//         for column in &columns {
//             let column_name: &str = column.get("column_name");
//             let data_type: &str = column.get("data_type");
//             let is_nullable: &str = column.get("is_nullable");
//             let column_default: Option<&str> = column.get("column_default");

//             write!(
//                 schema_str,
//                 "{} {} {} {:?};",
//                 column_name, data_type, is_nullable, column_default
//             )?;
//         }

//         // Update the hasher with the table schema
//         table_hasher.update(schema_str.as_bytes());

//         // Finalize the hash for the table
//         let table_hash = table_hasher.finalize();
//         println!("Hash for table {}: {:x}", table_name, table_hash);

//         // Update the overall hasher with the table hash
//         overall_hasher.update(table_hash);
//     }

//     // Finalize the hash for the entire schema
//     let overall_hash = overall_hasher.finalize();
//     println!("Overall schema hash: {:x}", overall_hash);

//     Ok(())
// }
/* ____________________________ */

// /* Type Hash */
// use type_hash::TypeHash;
// use std::fmt;
// #[derive(TypeHash)]
// pub enum Message {
//     LaunchMissles { destinations: String},
//     CancelMissles,
// }

// // #[derive(TypeHash)]
// // pub struct MyStruct {
// //     #[type_hash(foreign_type)]
// //     data: ArrayVec<[u16; 7]>
// // }

// // Implementing Debug trait for Message to print the enum
// impl fmt::Debug for Message {
//     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//         match self {
//             Message::LaunchMissles { destinations } => write!(f, "LaunchMissiles {{ destination: {} }}", destinations),
//             Message::CancelMissles => write!(f, "CancelMissiles"),
//         }
//     }
// }

// fn main() {
//     let hash = Message::type_hash();
//     println!("hash number {}", hash);
//     let dang = Message::LaunchMissles { destinations: String::from("Tokyo") };
//     println!("enum = {:?}", dang);
//     assert_eq!(hash, 11652809455620829461);
// }
/* ____________________________ */

/* Arc su dung trong multi-thread, de 1 bien co the duoc dung trong nhieu thread */
// use std::sync::Arc;
// use std::thread;

// fn main() {
//     // Create an Arc to share a value between threads
//     let value = Arc::new(42);

//     // Create a clone of the Arc for another thread
//     let value_clone = Arc::clone(&value);

//     // Spawn a new thread
//     let handle = thread::spawn(move || {
//         println!("Value from thread: {}", value_clone);
//         let i: i32 = 5;
//         println!("dang dep trai: {}", i);
//     });

//     // Access the value in the main thread
//     println!("Value from main: {}", value);

//     // Wait for the thread to finish
//     handle.join().unwrap();
// }
/* ____________________________ */

/* Tao test module for git */
/* Running: cargo test */
// #[allow(clippy::pedantic, unsafe_code)]
// pub mod my_module {
//     pub fn add(a: i32, b: i32) -> i32 {
//         a + b
//     }

//     pub fn subtract(a: i32, b: i32) -> i32 {
//         a - b
//     }

//     pub mod tests {
//         use super::*;
//         #[test]
//         fn test_add() {
//             assert_eq!(add(1, 2), 3);
//         }
//     }
// }

// fn main() {
//     println!("DangDepTrai");
// }

/* Lifetime parameter */
// 'a la lifetime annotation, thong bao cho compiler biet la return value se valid neu input x va y co cung lifetime 'a
// fn longer<'a>(x: &'a str, y: &'a str) -> &'a str {
//     if x.len() > y.len() {
//         x
//     } else {
//         y
//     }
// }

// fn main() {
//     let string1 = String::from("long string is long");
//     let result;
//     {
//         let string2 = String::from("xyz");
//         result = longer(string1.as_str(), string2.as_str());
//         // Build se bi loi, o truong hop nay string2 se bien mat khi ra khoi ngoac {}
//         // nen neu string2 dai hon string1, thi result se la string2, nhung ma vi khi ra khoi ngoac
//         // string2 da bien mat, nen result se khong co gia tri??, nen compiling se bao loi
//         // borrowed value does not live long enough
//     }
//     // string2 is dropped here
//     println!("The longer string is {}", result);
// }

/* PhantomData in rust */
/* PhantomData la marker (dau an) de gan vao 1 struct, muc tich trong viec gan lifetime vao trong 1 pointer
De bao voi compiler biet la ref cho 1 bien can phai gan voi 1 lifeTime nhat dich, de neu lifetime cua 1 ptr
bi over thi compiler se bao loi luc compiling time luon */
// use std::{marker::PhantomData, ptr::NonNull};
// struct MyStruct<T> {
//     ptr: NonNull<T>,
// }

// impl<T> MyStruct<T> {
//     fn new(reference: &T) -> MyStruct<T> {
//         MyStruct {
//             ptr: NonNull::from(reference),
//         }
//     }

//     fn get(&self) -> &T {
//         unsafe { self.ptr.as_ref() }
//     }
// }

// struct MyStructWithPhantom<'a, T> {
//     ptr: NonNull<T>,
//     _marker: PhantomData<&'a T>,
//     // ta gan _market vao PhantomData voi life time 'a, de thong bao cho compiler biet rang
//     // ptr luon phai gan lien voi lifetime 'a
//     // boi vi ptr khong bao gio co lifetime o trong do (giong nhu C) nen Rust define them 1 bien PhantomData
//     // de gan ptr vao lifetime
// }

// impl<'a, T> MyStructWithPhantom<'a, T> {
//     fn new(reference: &'a T) -> MyStructWithPhantom<'a, T> {
//         MyStructWithPhantom {
//             ptr: NonNull::from(reference),
//             _marker: PhantomData,
//         }
//     }

//     fn get(&self) -> &'a T {
//         unsafe { self.ptr.as_ref() }
//         // boi vi tra ve ref cho ptr nen phai de vao block unsafe{}, nham bao cho compiler biet rang
//         // developer se chiu trach nhiem ve viec quan ly ptr nay
//     }
// }

// fn main() {
//     let my_struct: MyStruct<i32>;
//     {
//         let i: i32 = 5;
//         my_struct = MyStruct::new(&i);
//     }
//     // my_struct hien tai dang lay reference cua i, nhung o day i se invalid, cho nen code nay se co risk.
//     // Do la ly do tai sao PhantomData ra doi. PhantomData giup thong bao cho compiler biet
//     // reference trong my_struct se phai ton tai trong cung 1 life time voi 'a trong PhantomData
//     println!("Print out my_struct value = {}", my_struct.get());

//     let my_struct_with_phantom;
//     {
//         let i: i32 = 33;
//         my_struct_with_phantom = MyStructWithPhantom::new(&i);
//         println!("Print out my_struct_with_phantom = {}", my_struct_with_phantom.get());
//     }
//     // khong the chay lenh println!() o day, vi bien i khong con valid nua, lifetime out
//     // PhantomData giup phat hien loi o compiling time
//     // println!("Print out my_struct_with_phantom = {}", my_struct_with_phantom.get());

// }

/* Primitive type in rust */
// Primitive thuong duoc dung de store tat ca cac primary data type(String, int, float,...) trong rust
// No duoc thich hop cho viec flexible trong code va generic programing
// Nhu trong vi du duoi day, chung ta co 3 loai data la username(string), age(int), email(string)
// Chung ta muon tuan tu hoa (serialize) 3 loai data nay thanh 1 mang string, theo kieu json file
// vd: { "username": "john_doe", "age": 30, "email": "john.doe@example.com" }
// use std::any::Any;

// pub enum Schema {
//     Struct(StructSchema),
//     Primitive(String),
// }

// pub struct StructSchema {
//     fields: Vec<(String, Schema)>,
//     //tuple cua Vector vs 2 phna tu la String va Schema
//     // String o day de chua ten du lieu, vd "username", "age"
//     // Schema(Schema nay neu la Primitive thi co the la bat cu data nao, int, string, float,...), no se la gia value cua du lieu
//     // vd age: thi value la 30, username thi value la "dang"
//     // muc dich cua Primitive duoc the hien o day khi co the tao ra nhieu dang data nhu age, username,... ma khong can phai define nhieu khi du lieu
// }

// pub struct UserProfile {
//     username: String,
//     age: i32,
//     email: String,
// }

// // data la du lieu dau vao, nhu ten la gi, tuoi bao nhieu
// // schema la kieu du lieu de serialize
// fn serialize(data: &dyn Any, schema: &Schema) -> String {
//     match schema {
//         Schema::Primitive(type_name) => {
//             match type_name.as_str() {
//                 "int" => format!("{}", data.downcast_ref::<i32>().unwrap()),
//                 "string" => format!("\"{}\"", data.downcast_ref::<String>().unwrap()),
//                 _ => panic!("Unsupported primitive type"),
//             }
//         },
//         Schema::Struct(struct_schema) => {
//             let mut serialized_fields = Vec::new();
//             let user_profile = data.downcast_ref::<UserProfile>().unwrap();
//             for (field_name, field_schema) in &struct_schema.fields {
//                 let field_value: &dyn Any = match field_name.as_str() {
//                     "username" => &user_profile.username,
//                     "age" => &user_profile.age,
//                     "email" => &user_profile.email,
//                     _ => panic!("Unknown field"),
//                 };
//                 serialized_fields.push(format!(
//                     "\"{}\": {}",
//                     field_name,
//                     serialize(field_value, field_schema)
//                 ));
//             }
//             format!("{{ {} }}", serialized_fields.join(", ")) // return string
//         },
//     }
// }

// fn main() {
//     // Define the schema for UserProfile
//     let user_profile_schema = Schema::Struct(StructSchema {
//         fields: vec![
//             (String::from("username"), Schema::Primitive(String::from("string"))),
//             (String::from("age"), Schema::Primitive(String::from("int"))),
//             (String::from("email"), Schema::Primitive(String::from("string"))),
//         ],
//     });

//     // Create a user profile instance
//     let user_profile = UserProfile {
//         username: String::from("john_doe"),
//         age: 30,
//         email: String::from("john.doe@example.com"),
//     };

//     // Serialize the user profile
//     let serialized_profile = serialize(&user_profile, &user_profile_schema);
//     println!("Serialized user profile: {}", serialized_profile);

//     // Here, you could save the serialized_profile to a file or send it over a network
// }

/* Vector in Rust va cach de assign nhanh 1 Vec<Column> o trong struct vao 1 bien */
// #[derive(Debug, Clone)]
// pub struct Column {
//     name: String,
//     data_type: String,
//     check: Option<String>,
// }

// /// Schema for a struct.
// #[derive(Debug, Clone)]
// pub struct StructSchema {
//     /// Columns
//     columns: Vec<Column>,
// }

// /// Only used in derive macro.
// #[derive(Debug, Clone)]
// pub enum Schema {
//     /// Composite type
//     Struct(StructSchema),
//     /// Primitive type
//     Primitive(String),
// }

// pub fn flatten(schemas: &[(String, StructSchema)]) -> StructSchema {
//     // Example implementation: return the first schema's StructSchema
//     schemas[0].1.clone()
// }

// fn main() {
//     let key_schema = StructSchema {
//         columns: vec![
//             Column {
//                 name: "id".to_string(),
//                 data_type: "int".to_string(),
//                 check: None,
//             },
//             Column {
//                 name: "name".to_string(),
//                 data_type: "string".to_string(),
//                 check: None,
//             },
//         ],
//     };

//     let schemas = [("key".to_string(), key_schema.clone())];

//     let StructSchema {
//         columns: primary_columns,
//         ..
//     } = flatten(&schemas);
//     // flatten tra ve StructSchema, roi sau do assign return (StructSchema.columns) vao trong variable name primary_columns
//     // hoac co the viet nhu secondary column duoi day
//     let result_schema: StructSchema = flatten(&schemas);
//     let secondary_columns = result_schema.columns;

//     println!("{:?}", primary_columns);
//     println!("{:?}", secondary_columns);

//     let data: Vec<i32> = vec![1, 2, 3];
//     println!("Dang vector = {:?}", data);
// }

// /* IntoIterator la mot method trong vector dung de loop qua tat ca cac phan tu */
// struct MyCollection {
//     items: Vec<i32>,
// }

// fn main() {
//     let my_collection = MyCollection {
//         items: vec![1, 2, 3, 4, 5],
//     };

//     // sau khi goi into_iter() o day thi my_collection.items se bi mat
//     for value in my_collection.items.into_iter() {
//         println!("{}", value);
//     }

//     // hoac co the viet 1 cach khac
//     // let mut iter = my_vector.into_iter();
//     // while let Some(value) = iter.next() {
//     //     println!("{}", value);
//     // }
// }

/* std::any::TypeId, la unique identifier cho moi Type */
// use std::any::{TypeId};

// struct MyStruct {
//     k: i32,
//     l: i32,
// }

// fn main() {
//     // 'static la 1 lifetime annotation dac biet, no la 1 longest possible lifetime, no duoc allocate o heap
//     // va chi bi deallocate khi program exits.
//     fn print_type_id<T: 'static>() {
//         println!("Type of T is: {:?}", TypeId::of::<T>());
//     }
//     let
//     print_type_id::<i32>();
//     print_type_id::<u32>();
//     print_type_id::<MyStruct>();
// }

/* Box, dyn, Any */
// Box cung tuong tu nhu unique_ptr() trong C++
// no la smart pointer de tao ra heap allocation. No dam bao rang 1 thoi diem chi co 1 owner duoc su dung

/* dyn trong Rust cung tuong tu nhu virtual function trong C++
trait trong Rust cung tuong nhu class trong C++
Trong C++ ta define 1 base class (nhu Animal), va 1 class khac (nhu Dog, Cat) inherit tu base class
Sau do trong class thu 2 co virtual function (speak). Khi Compiler chay
no se tuy thuoc vao bien la class (Dog, Cat) nao ma goi virtual function speak la "Wolf" hay "Meow" */
/* dyn trong rust cung tuong tu nhu vay */

trait Animal {
    fn speak(&self) {}
}
struct Dog;
impl Animal for Dog {
    fn speak(&self) {
        println! {"Woof"};
    }
}

struct Cat;
impl Animal for Cat {
    fn speak(&self) {
        println!("Meow");
    }
}

fn main() {
    let data = Box::new(5);
    println!("Box integer: {}", data);

    let animals: Vec<Box<dyn Animal>> = vec![Box::new(Dog), Box::new(Cat)];
    // dyn cho phep trong run time co the bien dich ra Animal la Dog hoac Cat de goi function tuong ung
    // giong nhu virtual function trong C++

    for animal in animals {
        animal.speak(); // khi goi den Dog thi se la Woof, Cat se la Meow
    }
}
