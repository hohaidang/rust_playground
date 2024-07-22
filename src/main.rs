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
use std::{marker::PhantomData, ptr::NonNull};
struct MyStruct<T> {
    ptr: NonNull<T>,
}

impl<T> MyStruct<T> {
    fn new(reference: &T) -> MyStruct<T> {
        MyStruct {
            ptr: NonNull::from(reference),
        }
    }

    fn get(&self) -> &T {
        unsafe { self.ptr.as_ref() }
    }
}

struct MyStructWithPhantom<'a, T> {
    ptr: NonNull<T>,
    _marker: PhantomData<&'a T>, 
    // ta gan _market vao PhantomData voi life time 'a, de thong bao cho compiler biet rang
    // ptr luon phai gan lien voi lifetime 'a
    // boi vi ptr khong bao gio co lifetime o trong do (giong nhu C) nen Rust define them 1 bien PhantomData
    // de gan ptr vao lifetime
}

impl<'a, T> MyStructWithPhantom<'a, T> {
    fn new(reference: &'a T) -> MyStructWithPhantom<'a, T> {
        MyStructWithPhantom {
            ptr: NonNull::from(reference),
            _marker: PhantomData,
        }
    }

    fn get(&self) -> &'a T {
        unsafe { self.ptr.as_ref() }
        // boi vi tra ve ref cho ptr nen phai de vao block unsafe{}, nham bao cho compiler biet rang
        // developer se chiu trach nhiem ve viec quan ly ptr nay
    }
}


fn main() {
    let my_struct: MyStruct<i32>;
    {
        let i: i32 = 5;
        my_struct = MyStruct::new(&i);
    }
    // my_struct hien tai dang lay reference cua i, nhung o day i se invalid, cho nen code nay se co risk.
    // Do la ly do tai sao PhantomData ra doi. PhantomData giup thong bao cho compiler biet
    // reference trong my_struct se phai ton tai trong cung 1 life time voi 'a trong PhantomData
    println!("Print out my_struct value = {}", my_struct.get());
    

    let my_struct_with_phantom;
    {
        let i: i32 = 33;
        my_struct_with_phantom = MyStructWithPhantom::new(&i);
        println!("Print out my_struct_with_phantom = {}", my_struct_with_phantom.get());
    }
    // khong the chay lenh println!() o day, vi bien i khong con valid nua, lifetime out
    // PhantomData giup phat hien loi o compiling time
    // println!("Print out my_struct_with_phantom = {}", my_struct_with_phantom.get());

}