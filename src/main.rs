use std::arch::aarch64::vaba_s8;
use std::char::from_u32_unchecked;
use log::log;

fn main() {
    println!("Hello, world!");
}

#[test]
fn hello_test(){
    println!("Hello Test 🙌");
}


#[test]
fn variable_test(){
    let name = "David";
    // name = "Jhon"; ⚠️ error cannot assign value
    println!("{}", name);

    let first_name = "Fajri";
    let last_name = "Diputra";
    println!("{} {}", first_name, last_name);
}

#[test]
fn mutable_test(){
    let mut x = 5;
    x = 3;
    println!("{}", x);
}

#[test]
fn static_type_test(){
    let mut name = "David";

    // name = 10; ⚠️ error cannot change data type
    println!("{}", name);
}

/*
    Shadowing
    create variable with same name
 */
#[test]
fn shadow_test(){
    let x = "five";
    println!("{}", x);

    let x = 1;
    println!("{}", x);
}

#[test]
fn explicit_test(){
    let age:i8 = 5;
    println!("{}", age);

    let var_float:f32 = 1.0;
    println!("{}", var_float);
}

/*
    Convert Number Data Type

    ✅ i16 to i8
    ❌ i16 to i64
 */

#[test]
fn number_convert(){
    let d:i32 = 100000000;
    let e:i8 = d as i8;
    println!("{}", e);
}

#[test]
fn numeric_operator_test(){
    let a = 10;
    let b = 20;
    let c = a + b;
    println!("{}", c);
}

#[test]
fn augmented_assignment(){
    let mut a = 10;
    print!("{}", a);

    a += 10;
    println!("{}", a);

    a -= 10;
    println!("{}", a);
}

#[test]
fn boolean_test(){
    let a = true;
    let b = false;
    println!("{} {}", b, a);
}


/*
    >
    <
    >=
    <=
    ==
 */
#[test]
fn comparaison_operator_test(){
    let result = 10 > 20;
    println!("{}", result);
}

#[test]
fn operator_boolean_test(){
    let absen = 70;
    let nilai_akhir = 288;

    let lulus = absen >= 75;
    let lulus_nilai_akhir = nilai_akhir >= 75;

    let lulus_final = lulus && lulus_nilai_akhir;
    print!("{}", lulus_final);
}


/*
    Tuple adalah tipe data kumpulan lebih dari satu tipe data
    Jumlah data di Tuple sudah final, artinya tidak bisa berkurang atau bertambah
    Jika kita membuat Tuple dengan total ada 3 data, maka tidak akan bisa diubah lagi jumlah data dan juga tipe data nya
    Untuk membuat Tuple, kita bisa gunakan () tanda kurung
 */
#[test]
fn tuple_test(){
    let data: (i32, f64, bool) = (500, 6.4, true);
    println!("{:?}", data);
}

/*
    Untuk mengakses tiap data di Tuple, kita bisa gunakan . (titik), lalu diikuti dengan nomor index (lokasi) data nya
    Index di Tuple dimulai dari nomor 0
 */
#[test]
fn tuples_test(){
    let data: (i32, f64, bool) = (500, 6.4, true);
    println!("{:?}", data);

    let a = data.0;
    let b = data.1;
    let c = data.2;
    println!("{} {} {}", a, b, c);
}

/*
    Destructuring Tuple

    Kadang ketika kita ingin mengakses seluruh data Tuple dan menyimpannya di Variable, akan sangat menyulitkan jika kita akses satu-satu menggunakan nomor index nya
    Tuple mendukung destructuring, yaitu membongkar isi Tuple, dan menyimpannya dalam variable
    Jika ada data di Tuple yang tidak kita butuhkan, kita bisa gunakan tanda _ (garis bawah) ketika melakukan destructuring Tuple

 */
#[test]
fn tuples_test2() {
    let data = (500, 6.4, true);
    println!("{:?}", data);

    let (a, b, c) = data;
    println!("{} {} {}", a, b, c);
}

/*
    Mutable Tuple

    Saat kita buat immutable Variable, secara otomatis Tuple yang sudah dibuat tidak bisa diubah lagi isi datanya
    Namun, jika kita membuat Variable dalam bentuk Mutable, maka data Tuple bisa kita ubah
    Untuk mengubah data Tuple, kita cukup gunakan nomor index dan = (sama dengan)
    Mirip seperti mengubah data variable biasanya
 */

#[test]
fn tuples_test3(){
    let mut data = (500, 6.4, true);
    println!("{:?}", data);

    let (a, b, c) = data;
    println!("{} {} {}", a, b, c);

    data.0 = 20;
    data.1 = 20.4;
    data.2 = false;
    // data.3 = 122; ⚠️ error, solution => use Vec, Shadowing or create new variable
    println!("{:?}", data);
}

/*
    Unit

    Unit adalah tuple tanpa nilai apapun, ditulisnya ()
    Hal ini mungkin kelihatan tidak ada gunanya
    Biasanya Unit ini digunakan untuk function-function yang tidak membutuhkan hasil data apapun
 */

fn unit() {
    println!("Hello, Unit 🤍!");
}

#[test]
fn unit_test(){
    let result = unit();
    println!("{:?}", result)
}

/*
    Array

    Array adalah tipe data yang berisi kumpulan data, sama seperti Tuple
    Yang membedakan Array dengan Tuple adalah, pada Array, kita hanya bisa menggunakan satu tipe data
    Untuk membuat Array, kita bisa gunakan [] tanda kurung kotak
 */

#[test]
fn array_test(){
    let array = [1,2,3,4,5];
    println!("{:?}", array);
}

/*
    Mengakses Array

    Untuk mengakses Array, sama seperti Tuple,
    kita perlu tentukan nomor Index yang akan diakses dimulai dari 0 (nol)
    Namun cara mengaksesnya berbeda, tidak menggunakan . (titik), melainkan menggunakan [index]
 */

#[test]
fn array_test2(){
    let array = [1,2,3,4,5];
    println!("{:?}", array);

    let a = array[0];
    let b = array[1];
    println!("{} {}", a, b);
}

/*
    Mutable Array

    Sama seperti Tuple, jika kita membuat variable Array yang immutable, secara otomatis data Array tidak bisa diubah
    Jika data array ingin bisa diubah, kita harus buat variable dalam bentuk mutable
    Kita bisa mengubah data Array dengan menggunakan [index] = diisi dengan nilai baru
 */

#[test]
fn array_test3(){
    let mut array = [1,2,3,4,5];
    println!("{:?}", array);

    let a = array[0];
    let b = array[1];
    println!("{} {}", a, b);

    array[0] = 10;
    array[1] = 20;
    println!("{:?}", array);
}

/*
    Panjang Array

    Hal yang membedakan dengan Tuple, kita bisa mendapatkan jumlah data di Array dengan menggunakan function len() milik Array
 */

#[test]
fn array_test4(){
    let mut array = [1,2,3,4,5];
    println!("{:?}", array);

    let a = array[0];
    let b = array[1];
    println!("{} {}", a, b);

    array[0] = 10;
    array[1] = 20;
    println!("{:?}", array);

    let length: usize = array.len();
    println!("length {}", length);
}

/*
    Two Dimensional Array

    Saat membuat Array, kita bisa menggunakan tipe apapun di dalam Array, bahkan jika didalamnya adalah tipe Array lagi
 */

#[test]
fn two_dimensional_array(){
    let matrix:[[i32;2];2] = [
        [1,2],
        [2,3],
    ];

    println!("{:?}", matrix);
    println!("{:?}", matrix[0][0]);
    println!("{:?}", matrix[0][1]);
    println!("{:?}", matrix[1][0]);
    println!("{:?}", matrix[1][1]);
}

/*
    Constant

    Constant adalah variable immutable menggunakan kata kunci const
    Yang membedakan const dan let adalah, const tidak memiliki mutable, selain itu nilai const harus dideklarasikan ketika kode program dibuat (bukan dijalankan), oleh karena itu nilai const tidak bisa hasil dari kalkulasi nilai lain yang belum jelas hasilnya
    Untuk membuat const, wajib disebutkan tipe datanya secara eksplisit
    Nama const di Rust harus huruf besar, dan biasanya pemisah kata menggunakan _ (garis bawah)
 */

const MAXIMUM: i32 = 100;

#[test]
fn constant_test(){
    const MINIMUM:i64 = 0;
    println!("{} {}", MINIMUM, MAXIMUM);
}


/*
    Variable Scope

    Dalam dunia programming, variable scope mendefinisikan area dimana variable bisa digunakan
    Variable bisa digunakan di dalam scope tempat definisi variable, dan juga di inner scope
    Namun variable tidak bisa digunakan di outer scope
 */
#[test]
fn var_scope_test(){
    let fajri = 1;

    {
        println!("inner fajri {}", fajri);
        let diputra = 4;
        println!("inner diputra {}", diputra);
    }

    // println!("outer diputra {}", diputra; ⚠️ error
}

/*
    Memory Management

    Garbage Collection adalah fitur yang banyak digunakan bahasa pemrograman
    untuk melakukan manajemen memory, seperti Java dan Golang
    Secara berkala Garbage Collection akan memantau data yang sudah
    tidak digunakan lagi di memory, dan menghapusnya secara otomatis
    Atau di bahasa pemrograman tanpa Garbage Collection,
    yang biasanya harus melakukan manajemen memory secara manual, seperti C/C++
    Tanpa Garbage Collection, kita harus mengalokasikan data secara manual di memory,
    begitu juga ketika sudah tidak butuh, kita harus menghapus data dari memory dari memory
    Rust memiliki pendekatan yang berbeda, Rust tidak menggunakan Garbage Collection,
    Rust juga tidak menggunakan Manual Memory Management
 */

/*
    Stack dan Heap

    Rust membagi data di memory dalam dua bagian, Stack dan Heap
    Stack adalah bagian dimana data disimpan dalam struktur data tumpukan, last in first out.
    Semua data di Stack harus yang fixed size (artinya ukuran data sudah pasti)
    Heap berbeda, heap seperti tempat untuk menyimpan data,
    dimana untuk menyimpan data di Heap kita akan melakukan request ke Heap,
    lalu di dalam Heap terdapat
    Memory Allocator yang bertugas untuk menemukan area kosong
    untuk menyimpan dan mengalokasikan data ke area tersebut.
    Setelah itu kita akan diberi pointer (penunjuk) ke lokasi dimana data itu berada di Heap.
    Pointer dari Heap berukuran fix sized, oleh karena itu pointer akan disimpan di Stack
 */

#[test]
fn stack_heap_test(){
    function_a();
    function_b();
}

fn function_b(){
    let a = 10;
    let b = String::from("Diputra");

    println!("{} {}", a, b);
}

fn function_a(){
    let a = 10;
    let b = String::from("Fajri");

    println!("{} {}", a, b);
}

/*
    Drop Function

    Saat variable keluar dari scope nya, yang artinya tidak bisa diakses lagi,
    secara otomatis Rust akan memanggil drop function
    Drop function adalah function untuk menghapus data,
    sehingga akan dibersihkan dari Heap
    Dan jika Rust function() sudah selesai dieksekusi,
    maka function() tersebut akan dihapus pula dari Stack Frame
    Oleh karena itu, Rust tidak membutuhkan Garbage Collection ataupun Manual Memory Management
 */

/*
    &str dan String

    Rust memiliki tipe data text yang fixed size, yaitu &str (string slice),
    dan yang bisa mengembang ukurannya, yaitu String
    &str karena ukurannya fixed size, jadi Rust akan menyimpannya di Stack,
    sedangkan String karena bisa mengembang, maka disimpan di Heap
 */

/*
    Immutable str

    Karena ukuran &str adalah fixed size, maka operasi &str adalah tipe data yang immutable,
    artinya isi data &str tidak bisa diubah
    Ketika kita buat variable mutable, dan mengubah data &str,
    sebenarnya yang dilakukan adalah mengganti isi variable, bukan mengubah isi dari &str
    &str memiliki banyak sekali method yang bisa digunakan untuk memanipulasi &str nya,
    namun akan menghasilkan nilai &str baru
    Namun perlu diperhatikan, beberapa method dari &str akan mengembalikan bentuk data String,
    bukan &str
    https://doc.rust-lang.org/std/primitive.str.html
 */

#[test]
fn string_test(){
    let name: &str = " Fajri Al Banjari ";
    let trim: &str = name.trim();
    println!("name {}", name);
    println!("trim {}", trim);
}

/*

    String

    String di Rust merupakan tipe data text UTF-8, dan bisa berkembang ukurannya
    Ketika kita buat dalam bentuk immutable variable,
    maka String tidak bisa berkembang, namun tetap disimpan di Heap
    Ketika kita buat dalam bentuk mutable variable, maka String bisa berkembang di Heap
    String juga memiliki method / function untuk memanipulasi data,
    namun perlu diperhatikan ada method yang digunakan untuk mengubah datanya sendiri,
    ada juga method yang digunakan untuk mengubah dalam bentuk data baru, tanpa memodifikasi data asli nya
    https://doc.rust-lang.org/std/string/struct.String.html
 */

#[test]
fn string_test2() {
    let mut name: String = String::from(" Fajri DIPUTRA");
    name.push_str(" Al Banjari");
    name.trim();
    println!("name : {}", name);

    let agus = name.replace("DIPUTRA", "");
    println!("agus : {}", agus);

}

/*
    Ownership

    Rust menggunakan Ownership untuk melakukan data management di Memory
    Ownership adalah salah satu fitur unik di Rust yang mungkin jarang ada di bahasa pemrograman lain
    Ownership wajib dimengerti, karena akan berdampak ke hampir semua fitur di Rust
    Ownership adalah fitur yang digunakan oleh Rust untuk menjadikan
    Rust menjadi bahasa pemrograman yang aman dalam mengelola data di memory,
    tanpa harus adanya fitur Garbage Collection atau Manual Memory Management
    Karena Ownership adalah konsep yang baru untuk kebanyakan programmer,
    maka kadang kita butuh waktu untuk memahaminya
 */

/*
    Ownership Rules

    Setiap value di Rust harus punya owner (variable pemilik value)
    Dalam satu waktu, hanya boleh ada satu owner
    Ketika owner keluar scope, value akan dihapus
 */

#[test]
fn ownership_test(){
    let a = 10;
    {
        let b = 20;
        println!("{}", b)
    }
    println!("{}", a)
}

/*
    Data Copy

    Sesuai aturan di Ownership Rules, setiap value harus dimiliki oleh satu owner pada satu waktu
    Ketika kita berinteraksi dengan data, maka data akan dimiliki hanya oleh satu owner
    Semua data yang bersifat fixed size (yang disimpan di Stack),
    ketika kita tambahkan ke variable berbeda (owner baru), maka hasilnya adalah data akan di copy,
    sehingga variable baru (owner baru) akan memiliki data hasil copy dari variable lama (owner lama)
    Oleh karena itu, tiap data akan selalu dimiliki oleh satu owner pada satu waktu
 */

#[test]
fn data_copy_test() {
    let a = 10; // Data store on Stack
    let b = a; // copy new data, old data not deleted

    println!("{} {}", a, b);
}

/*
    Ownership Movement

    Namun Data Copy tidak terjadi untuk tipe data yang disimpan di Heap
    Seperti aturan di Ownership, dalam satu waktu value hanya dimiliki satu owner
    Maka ketika kita coba buat variable baru (owner baru) dari variable lama (owner lama),
    maka yang terjadi bukanlah copy, melainkan transfer ownership dari owner lama ke
    owner baruSetelah proses transfer selesai, secara otomatis owner lama akan dianggap
    tidak valid lagi digunakan
 */

#[test]
fn ownership_test2() {
    let name1: String = String::from(" Fajri");
    println!("name1 : {}", name1);

    let name2: String = name1;
    println!("name2 : {}", name2);
    // println!("name1 : {}", name1); ⚠️ Error, cuz data store on Heap
}

/*
    Clone

    Sekarang kita tahu bahwa data di Stack akan di Copy sedangkan data di Heap
    akan dipindahkan ownership nya
    Lantas bagaimana jika kita juga ingin melakukan Copy untuk data di Heap?
    Maka kita harus melakukan Clone
    Clone adalah membuat data tiruan yang sama dari data aslinya
    String memiliki method clone() untuk melakukan ini
    Saat kita memanggil method clone() maka method tersebut akan meng-copy data
    String menjadi data String baru
    Semua tipe data yang disimpan di Heap di Rust memiliki method clone()
 */

#[test]
fn clone_test(){
    let name1 = String::from("Fajri");
    let name2 = name1.clone(); // Clone is like copy data, but on Heap memory

    println!("name1 : {}, name2 : {}", name1, name2);
}

/*
    If Expression

    Sama seperti bahasa pemrograman yang lain, Rust juga mendukung If Expression
    If expression digunakan untuk membuat percabangan kode sesuai dengan kondisi.
    Jika kondisi terpenuhi, maka blok kode If akan dieksekusi,
    jika kondisi tidak terpenuhi, klok kode If tidak akan dieksekusi
 */

#[test]
fn if_exps_test(){
    let condition = 7;

    if condition >= 7 {
        println!("Okay 👍🏻")
    }
}

/*
    Else Expression

    Ketika kondisi If tidak terpenuhi, kadang kita ingin melakukan sesuatu
    Kita bisa lakukan itu dengan Else Expression
    Blok di else akan dieksekusi jika kondisi If tidak terpenuhi
 */

/*
    Else If Expression

    Saat membuat If expression, kadang kita ingin membuat beberapa kondisi
    Untuk membuat beberapa kondisi, kita bisa gabungkan dengan Else If expression
 */

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

/*
    Let Statement

    If di Rust adalah sebuah expression, artinya bisa menghasilkan value dan
    bisa digunakan dengan Let statement untuk mengisi data di variable
    Ini sangat berguna sehingga kita tidak perlu memasukkan nilai ke variable
    terpisah dengan deklarasi variable nya
 */

#[test]
fn if_expression_test() {
    let value = 10;
    if value >= 8 {
        println!("Good")
    } else if value >= 6 {
        println!("Bad")
    } else if value >= 3 {
        println!("Lil Bad")
    } else {
        println!("Really Bad")
    };
}

/*
    Let Statement

    If di Rust adalah sebuah expression, artinya bisa menghasilkan value dan
    bisa digunakan dengan Let statement untuk mengisi data di variable
    Ini sangat berguna sehingga kita tidak perlu memasukkan nilai ke variable
    terpisah dengan deklarasi variable nya
 */

#[test]
fn let_test(){
    let value = 10;
    let result: &str ;

    if value >= 8 {
        result = "Good";
    } else if value >= 6 {
        result = "Bad Aje";
    } else if value >= 3 {
        result = "Bad";
    } else {
        result = "Very Bad";
    }

    println!("{}", result);
}

#[test]
fn if_let_statement(){
    let value:i8 = 3;
    let result: &str = if value >= 8 {
        "Good"
    } else if value >= 6 {
        "Lil Good"
    } else if value >= 3 {
        "Bad"
    } else { "Very Bad" };

    println!("{}", result);
}

/*
    Loop

    Setiap bahasa pemrograman biasanya memiliki fitur untuk
    melakukan perulangan
    Rust mendukung beberapa cara untuk melakukan perulangan,
    pertama kita akan bahas tentang Loop
    Loop merupakan perintah di Rust digunakan untuk melakukan
    perulangan terus-menerus, sampai kita memerintahkannya untuk berhenti
    Jika kita tidak memerintahkan untuk berhenti,
    maka Loop tidak akan pernah berhenti melakukan perulangan
 */

/*
    Break dan Continue

    Untuk menghentikan perulangan, kita bisa menggunakan perintah break
    Selain break, ada juga perintah continue,
    yang artinya menghentikan perulangan saat ini,
    dan langsung dilanjutkan ke perulangan berikutnya
 */

#[test]
fn loop_test(){
    let mut counter = 0;
    loop {
        counter += 1;
        if counter > 10 {
            break;
        } else if counter % 3 == 0 {
            continue
        }

        println!("{}", counter);
    }
}

/*
    Return Value di Loop

    Sama seperti If Expression, di Loop juga kita bisa mengembalikan nilai,
    sehingga bisa disimpan dalam variable dengan Let Expression
    Caranya kita bisa gunakan break lalu diikuti dengan nilai yang akan dikembalikan di Loop
 */

#[test]
fn loop_return_value(){
    let mut counter = 0;
    let result = loop {
        counter += 1;
        if counter > 10 {
            break counter * 2;
        }
    };
    println!("{}", result);
}

/*
    Loop Label

    Kadang kita sering membuat Loop didalam Loop,
    dan ketika ingin menghentikan Loop paling atas dari Loop yang ada di dalam,
    maka hal itu tidak bisa dilakukan
    Loop memiliki fitur Label, dimana kita bisa memberi nama pada Loop
    Keuntungannya memberi Label pada loop adalah,
    kita bisa menghentikan Loop yang ingin kita hentikan dengan cara menyebutkan nama Label nya
 */

#[test]
fn loop_label_test(){
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

/*
    While Loop

    While Loop adalah jenis perulangan dimana memiliki kondisi
    Jika kondisi masih terpenuhi, maka perulangan akan dilanjutkan
    Namun jika perulangan tidak terpenuhi, maka perulangan akan dihentikan
    While Loop mirip seperti Loop, bisa dihentikan menggunakan break dan continue
 */

#[test]
fn while_loop_test() {
    let mut number = 0;
    while number < 10 {
        if number % 2 == 0 {
            println!("{}", number);
        }
        number += 1;
    }
}

/*
    Iterasi Array

    Salah satu yang biasa kita lakukan ketika menggunakan Array adalah,
    melakukan pengambilan semua data di Array dari data pertama sampai data terakhir
    Biasanya, kita akan menggunakan While Loop,
    lalu membuat variable untuk mengakses index nya
 */

#[test]
fn array_iteration() {
    let array = ["A", "B", "C", "D"];
    let mut index = 0;

    while index < array.len() {
        println!("Value : {}", array[index]);
        index += 1;
    }
}

/*
    For Loop

    Rust menyediakan cara yang lebih mudah untuk melakukan pengambilan data dari Array menggunakan For Loop
 */

fn array_for_loop() {
    let array = ["A", "B", "C", "D"];
    for value in array {
        println!("Value {}", value);
    }
}

/*
    Range

    Rust memiliki tipe data bernama Range
    Range adalah jarak antara start dan end
    Range merupakan tipe data Collection seperti Array, sehingga bisa dilakikan
    pengulangan For Loop
    Data range akan dimulai daro start dan diakhiri sebelum emd (exclusive)
 */

#[test]
fn range() {
    let array = ["A", "B", "C", "D", "E"];

    let range = 0..5;
    println!("Start : {:?}", range.start);
    println!("End : {:?}", range.end);

    for i in range {
        println!("Value {}", array[i]);
    }
}

/*
    Range Inclusive

    Selain Range yang exclusive, Rust juga memiliki tipe data Range Inclusive
    Implementasinya berbeda dengan Range sebelumnya
    https://doc.rust-lang.org/std/ops/struct.RangeInclusive.html
 */

#[test]
fn range_inclusive() {
    let array = ["A", "B", "C", "D", "E"];
    let range = 0..=4;

    println!("Start : {:?}", range.start());
    println!("End : {:?}", range.end());

    for i in range {
        println!("Value {}", array[i]);
    }

}