// SHYANJMC
// copyleft GPLv3 - https://www.gnu.org/licenses/gpl-3.0.en.html
//
// Source Code Rust tutorial
//////////////////////////////////////////////////////////////////////////
// To compile do; rustc [file].rs

//////////////////////////////////////////////////////////////////////////
// Cargo is the project manager of Rust
//
// To create a project do; cargo new [project_name]
// Then you can edit the source code of your program under; src/main.rs
//
// To build do; cargo build
// Then you can run the program under; target/debug/
// To build and then run; cargo run
//
// To build the program with optimizations do; cargo build --release
//
// /////
//
// The dependencies and information of this program can be found in;
// [program_folder]/Cargo.toml
//
// Under "[dependencies]" statement with syntax:
// [lib_name] = "[version]"
// if [version] have "^" at the beggining, maens;
// "any version that has a public API compatible with version 0.3.14."
//
// For example to add rand;
//
// [dependencies]
// rand = "0.4.0"
// /////
// When a project with a specific Cargo.toml dependencies is builded, Cargo create a file to block
// all dependencies until you upgrade; Cargo.lock
// /////
// To upgrade all dependencies in Cargo.toml with the latest versions do; cargo update
// and then Cargo will lock the dependencies with new versions in Cargo.lock

/////////////////////////////////////////////////////////////////////////

// Each word that is used by the language in functions, options and etc is called "keyword".
// Standard library with input output trait.
use std::io;

// Rand libraries.
// "Rng" trait defines methods for generate random number.
// This is not from standard library, come from an external package. See Cargo.toml file to see
// which external packages are required by this program.
use rand::Rng;

// String compare library
// in specific the "Ordering" function to use only with strings.
use std::cmp::Ordering;

// String library
use std::str;

// Network Library
use std::net::{IpAddr, Ipv4Addr, Ipv6Addr};

// Function "main". The entry point of the program.
// The program execute this at the start and only execute the rest of
// functions/program if they are called from here.
fn main() {
    // Print some in screen.
    println!("Hello.");
    println!("This is a Source Code tutorial developed by Joaquin (aka; ShyanJMC ) Crespo.\nRemember
see and study the code to understand what does the program.\nI recommend start from 'main'
function.");

    // The next are custom functions declared with "fn" keyword.
    // A function is a piece of code which do things, only when they are called.
    // The function is called with her name and "arguments" between "( )".
    // An argument is an option that modify the normal execution of the program.
    // In programming the arguments are called "parameters".
    // But in the practice are the same in the theory.
    // For example; if we call "println!()" the same will print an screen a
    // newline ("\n" in C), but if we specify some argument (for example;
	// "Hello." as above) will print the argument instead of a new line.
    // That is an argument because modify the normal function of a function or
	// program to do different things.


    // Call "ingress" function.
    ingress();

    // Call "randomnumber" function.
    randomnumber();

    // Take in memory this difference;
    // 1- When the syntax is; A::B indicates that the argument "B" is a
	// associated function of "A". So "B" is shared by all things of the same
	// type as "A", this is more clear if you think some like Class.
    //    But keep in mind that class don't exist in rust.
    // 2- When the syntax is; A.B indicates that the return of A is passed to B.
	// So B's value depends of A.

    // Call compare_function
    compare_function();

    // Call Loop function
    loopfn();

    // Call Input validation
    inputvalidation();

    // Call constants validation
    constants();

    // Call shadowing function
    shadowing();

    // Call vtypes function. In reference to "variable's types".
    vtypes();

    // Call nop function. In reference to "numeric operations".
    nop();

    // Call bop. In reference to "boolean operation".
    bop();

    // Call cvar. In reference to "char variable".
    cvar();

    // Call ttuple.
    ttuple();

    // Call arrvar.
    arrvar();

    // Call function_parameters with a parameter.
    let _parameter = 345;
    function_parameters(_parameter);
    // From this point as _parameter's value was moved into
	// function_parameters, so should no valir from this point
    // But as _parameter is interger, its "Copy" and is valid after.
    // The values that are "Copy" when are passed as parameters are;
    // 1- Integers
    // 2- Bool
    // 3- Floating
    // 4- Char
    // 5- Tuples, when they don't have "string" type.
    // So if you try to use a string variable after move it to a function, you
	// will get a compilation error, because was moved to function
    //  and its out of scope.

    // Call statements_expressions with a parameter.
    statements_expressions(30);

    // Call return_values function and store the return.
    let _return_value1 = return_values();
    println!("The return of 'return_values()' function is: {}",_return_value1);

    // Call conditionals function.
    conditionals(32000);

    // Call conditional_loop function.
    conditional_loop();

    // Call for_loop function.
    for_loop();

    // Rust have her security on the called; ownership.
    // While languages like Java or Python use a garbage collector and in others
	// , like C, must be the programmer who allocate and release the memory.
    // But in Rust, the programming language use something like a garbage
	// collector nextgen.
    // Because Rust is a systems programming languages, there are some
	// somethings you must know;
    // - In systems programming, there are two things for manage the main
	// memory (RAM); stack and heap, both are structured in different ways.
    // -- The stack store data from down to up. This means that the second data
	// will be stored on the first, and the third on the second, and so on.
    // When is requirements delete some data, the first data in be deleted is
    // the last one that was put.
    // So if you have 5 data, the last in be added on stack top will be the 5th
	// (fifth), and the first data in be deleted will be the 5th (fifth).
    // Add data is called; push (verb; pushing).
    // Delete data is called; pop (verb; popping).
    //
    // ---------------------> Stack top
    // ^ 5th Fifth value
    // | 4th Fourth value
    // | 3th Third value
    // | 2th Second value
    // | 1th First value
    // --------------------> Stack floor
    //
    // All data must be know in size, because all data have reserved the space in RAM for work.

    // When the data size is unknown or when the variable size is also unknown, will be stored in
	// the heap.
    // When the data will be stored into the heap, the OS find a new memory block with enough space
	// for that data. When that data is
    // allocated, the system return a pointer to it (for example, pointer to; 900000AA).
    // As the data size is unknown, when the program runs put that variable into memory with NULL
	// value (	NOT zero, NULL) and when the data size
    // is know, allocate it into the heap with the right size.

    // When one is allocating data into the stack is not considering an allocate process.
    // As the pointer is a data with know size, it can be stored into the stack. But take this
	// consideration, if you use the value stack of the pointer
    // you will get the address but if you use the value of pointer in heap, you will get the value
	// allocated in pointer's address.

    // For example;
    // let welcome_msg: &str = "Welcome to Rust SourceCodeTutorial!"
    // That variable will be allocated into the stack, why? Because we know the size of that string;
	// 35 bytes (one character per byte) from the
    // start of compile process, so the system can allocate a specific data size into the stack.
    // If you need help to calculate string size; https://mothereff.in/byte-counter
    // Instead of;

    // let mut _variabletocompare = String::new();
    // io::stdin().read_line(&mut _variabletocompare)
    //            .expect("Error to read line.");

    // In which we do not know the size value that will be ingress by user.

    // Keeping track of what parts of code are using what data on the heap,minimizing the amount of
	// duplicate data on the heap,
    // and cleaning up unused data on the heap so you don’t run out of space are all problems that
	// ownership addresses.
    // Ownership Rules:
    // 1- Each value in Rust must have a variable that it's called; "owner".
    // 2- There can only be one owner per value.
    // 3- When the owner go out of scope, the value is dropped.

    // The scope do reference to this;
    // fn main(){
    //      let string: &str = "Hello World";
    // }
    //
    // There the owner is the variable; "string".
    // The scope is between the keys "{ }" of "main" function. Anything outside main's keys is out
	// of scope.
    // When the function start, all values are pushed into stack (because the scope start), and when
	// the function ends the same (the function) and
    // his values are out of scope, so the values are popped out the stack and cleaned.

    // Call strings_formats function
    strings_formats();

    // When a function ends, Rust automatically call the function "drop" will clean the function's
	// memory
    // Call memory_scenarios function
    memory_scenarios();

    // Now we will see some scenarios about how to copy values for stack and heap locations.
    // Call heap_clone function.
    heap_clone();

    // How to assign a function's return to a variable.
    // Call variable_return functions.
    variable_return();
    variable_return2();

    // Here there are the instructions how to return multiple returns
    // Call multiple_returns function.
    // We pass "HelloWorld" as a string.
    multiple_returns("HelloWorld".to_string());

    // Ok, now we know that if we pass a parameter to a function, the value is moved into scope of
	// that function being unavailable when finish.
    // But if we don't want take the ownership of that value, we must pass the value by reference.
    // with it, the function will be go to data's value in RAM instead of take ownership.
    let _var_by_reference = String::from("This is a value to be passed by reference.");
    // The "&" indicate, that the value is passed by reference.
    use_of_reference( &_var_by_reference);
    // So, because was not moved, just specified by reference _var_by_reference is still available
	// to use.

    // Take in consideration this; when you don't use "&" you must return the value to keep it
	// available,
    // but when you pass by reference as the function didn't take the ownership you don't need
	// return it.
    // So if you pass a value by reference, as the function don't have the ownership you will can
	// not edit it.
    // Unless we use; mutable references.

    let mut _var_by_reference2 = String::from("This is the second value to be by reference.");
    // You must specify that is mutable.
    use_of_mutable_reference( &mut _var_by_reference2 );
    // But there are one restriction;
    // - You can only use a mutable variable at once, so if the same mutable variable is used by two
	// another will fail.
    // The benefit of having this restriction is that Rust can prevent data races at compile time.
    // A data race is similar to a race condition and happens when these three behaviors occur:
    // 1- Two or more pointers access the same data at the same time.
    // 2- At least one of the pointers is being used to write to the data.
    // 3- There’s no mechanism being used to synchronize access to the data.
    // Also you can get a mutable variable doing reference to immutable variable, but not reverse.

    // Dangling References:
    // If you know C, you know also that there are situations in which when you use pointers you can
	// do security issues and many type of issues.
    // Not all, but the most pointer's issues happen when the pointer do reference to a memory
	// location but the data is no longer available.
    // To avoid this; Rust invalid pointers to that location when the data is invalid.
    //
    // But be careful with &String, when you use it as return. Because you will be returning a
	// pointer into a value of function's scope, so the return
    // will be invalid. So in that case the solution is return the value directly, without a
	// reference.

    // Reference's type; Slice
    // This is one data type which have not ownership.
    // Slice do reference to contiguous sequence of elements in a collection rather than the whole
	// collection.
    // Call first_word function, transforming "Hello World" as string and passing it as reference.
    first_word( &"Hello World".to_string() );

    // Now we have another type of strings; String slice.
    // A string slice is a part of mayor string.
    string_slice();

    // But, that is not the only slice;
    numeric_slice();

    // Now we have new data; Structs
    function_struct();

    // Methods
    function_methods();

    // Enumns
    function_enums();

    // Math function
    function_match( "Hi".to_string() );

    // Iflet statement
    function_iflet();

    // Packages and Crates
    packages_and_crates();

    // After packages_and_crates see the modules.
    // Search "MODULE_START"

    // Collections
    structures_as_collections();

    // Errors
    hanling_errors();

    // Generics values
    generics(&15);
}





// Function "ingress".
fn ingress(){
    // The "let" statement create a variable.
    // To avoid mistakes with internal words, put "_" at the beggining of the variable name (is the standard recomendation).
    let _message = "Type some to see the standard input and store.";

    // Print in the standard output the argument. The "!" at the end of function means that it is a
    // macro.
    println!("{}",_message);

    // By default all variables in Rust are in-mutables, with "mut" any variable will be mutable.
    // The "new" statement create a new empty string and convert the same to string type (String::), by default
    // "String" encode all in UTF-8.
    // The "A::B" indicates that the argument "B" is a associated function of "A" (some like
    // pipeline).
    let mut _input = String::new();

    // With this we use the "stdin" function to pass to "read_line" for store it in "input"
    // variable.
    // The function of "read_line" is take from standard input and store it in a mutable variable.
    // In the argument section we are using "&mut" because even when the "input" variable is
    // mutable, we need be sure about that change.
    // As in C the "&" indicate a memory section (reference), so is not need copy the variable to use the
    // value. References are in-mutable by default.
    // The "." statement indicate "newline" to break long lines
    io::stdin().read_line(&mut _input)

        // The "expect" statement use the return of "read_line"; io::Result and then comparate with
        // the argument.
        // "Result" is a enum type, contain "Ok" or "Err", each case the enum returned have
        // information about it. If the "Result" is "Err" "expect" will crash the program and will
        // display the argument. Commonly if "read_line" returns "Err" it means an error on the OS.
        // But if the return is "Ok" will show it, so you can use if you need.
        .expect("Failed to read line");


    // Print the result
    // The "{}" indicates that in it place will be the value of a variable.
    println!("You typped; {}", _input);
}

fn randomnumber(){
    // Print message.
    println!("This functon will generate random numbers.");

    // Create the variable "number" to store.
    // "rand" indicate the top function class.
    // "thread_rng" indicate the thread of rand. This is for paralelism.
    // "gen_range" is the random number generator, with limits from 1 to (without include) 101.
    let _number = rand::thread_rng().gen_range(1,101);

    // Print number.
    println!("Random number; {}", _number);
}

fn compare_function(){
    // Print message
    println!("This part of the program will comparate a new random number with you type number");
    println!("Insert number to compare with stored in the program.");

    // Variable mutable creation as string.
    // Even being a number, must be created as string because you don't know if will type 50 or 43212356544321233 or wherever
    let mut _variabletocompare = String::new();

    // Read line from stdin and assign to variable "_variabletocompare" with mutable option.
    io::stdin().read_line(&mut _variabletocompare)
                .expect("Error to read line.");

    let _rightnumber=rand::thread_rng().gen_range(1,101);

    // Now you can see that "_rightnumber" is a numeric variable and "_variabletocompare" is a string variable.
    // So you can not compare mismatch type variables, you need first translate string variable to numeric variable.
    // In specific a 32 bits unsigned number variable.
    // The skill to transform one variable type to another is called; shadowing
    // "XXX.trim" will delete any spece at the beginning and at the end of the string, also with "\n".
    // "XXX.parse" will parse string into numbers, because of that is requeriments specifie the type; u32

    let _variabletocompare: u32 = _variabletocompare.trim().parse()
        .expect("Insert a type number");

    // Compare two variables that must be numeric type.
    // "match" is the equivalent to "if A" in another lenguajes.
    // "cmp" will compare [A] (the first argument before dot) with [B] (the cmp's argument. Must be a variable (with &A) not a value).
    // Like anothers functions "Ordering" is a enum that returns the errors with information inside, so use "expect" to check
    // the error returned is a very good option.
    // The comparation must be between exactly the same type variables.

    match _variabletocompare.cmp(&_rightnumber){
            // Call ordening function and check if is more less than value.
            Ordering::Less => println!("Is more less than the right value."),
            // Call and check if is more Greater.
            Ordering::Greater => println!("Is more greather than the right value."),
            // Call and check if is equal.
            Ordering::Equal => println!("Congrulations!."),
    }
}

fn loopfn(){
    // Inside the keys everything will repetar, again and again for ever.
    loop {
        println!("This is a loop, hehe");
        // Break the loop.
        break;
    }
}

fn constants(){
    // This is an constant. The constant's name must be in upercase.
    // The difference between an constant and a not mutable variable, is this last (a variable) can be redefined to a
    // mutable, and a constant don't matter anything can not be edited.
    // u32 = The 32-bit unsigned integer type.
    const _CONSTANT_VARIABLE: u32 = 55000;
    println!("{}", _CONSTANT_VARIABLE);

    const _CONSTANT_STRING: &str = "Hello, this is a constante string";
}

fn inputvalidation(){
    // Variable crated as a string.
    let mut _variable = String::new();
    // Print message.
    println!("Insert number for input validation: ");
    // Read line, assign the value to "variable" and if there errors show the message after ".expect".
    io::stdin().read_line(&mut _variable)
        .expect("Failed to read line");
    // Compare between self to check thats was assigned right
    let _variable: u32 = match _variable.trim().parse(){
        // "num" is a collection of numeric types and traits.
        // So check if the return of ".parse" is an "num".
        // if everything is "ok" will execute "println!".
        Ok(num) => num,
        // if there are issues, will execute "continue". But the execution can be anything.
        // Can be used "continue" for the case that is a loop in function to continue in error case.
        Err(_) => 1, // If you don't want do anything when there are an error you can use "continue".
    };
}

fn shadowing(){
    // First, we create a variable.
    let _variable1 = "Hello ";
    // Now we redefine the variable with new values.
    // We use ".to_owned" to concanate the old value ("Hi ") with the new ("World") to create; "Hello World".
    let _variable1 = _variable1.to_owned() + "World";
    // Print the value.
    println!("Shadowed variable1 value:{}",_variable1);
    // This is called "shadowing" but for this is not neccesary use the old value, just
    // recreate.

    // "X.len()" take the  lenght of X variable and store the same in variable2. Must be u32 because len return numeric type.
    // The X variable must be str (string).
    let _variable2 = _variable1.len();
}

fn vtypes(){
    // integer 8bits signed variable.
    let _variable1: i8 = 1;
    // integrer 16bits unsigned variable.
    let _variable2: u16 = 2;
    // And you can understand the point. These are the complete list for int's types values:
    // Length   Signed  Unsigned    sLimit                                                      uLimit
    // 8-bit    i8      u8          -128 a 127                                                  0-255
    // 16-bit   i16     u16         -32768 to 32767                                             0 to 65535
    // 32-bit   i32     u32         -2147483648 to 2147483647                                   0 to 4294967295]
    // 64-bit   i64     u64         -9,223,372,036,854,775,808 to 9,223,372,036,854,775,807     0 to 18446744073709551615
    // 128-bit  i128    u128        −2^(127) to 2^(127)-1                                       0 to 2^(128)-1
    // arch     isize   usize
    //  Signed or unsigned means if the value can be negative or not.
    //  If the value is unsigned the same (the value) can only be positive.
    //
    // Number-literals      Example
    // Decimal              98_222
    // Hex                  0xff
    // Octal0               o77
    // Binary               0b1111_0000
    // Byte (u8 only)       b'A'
    //
    //  You MUST have in consideration;
    //      if you change the original type's size from (for example) from u8 to u32,
    //      you will get "integer overflow" because when a variable is created, the size of the
    //      variable is reserved and if is redefined with a more langer than the original, will over use it.
    //
    //  If in a release (the build with "--release" argument) you do an integer overflow, the next value to the maxium of the
    // variable will be wrapped to the minimum value of the variable's range.
    // For example;
    //      In a variable u8 the range is from 0 to 255. So if you go to 256 (one more the limit), the 256 number will be wrapped
    //      to 0 (zero) and 257 to 1. This is to avoid buffer overflow, one of most common vulnerabilities.
    //
    //  Floating variables = variables with decimal points, because of that many times the floating variabels are called it, the name do
    // reference at coma "," .
    //
    // Floating-Variable        Architecture
    // f32                      32 bits.
    // f64                      64 bits.
    //
    // Floating-point numbers are represented according to the IEEE-754 standard.
    // The f32 type is a single-precision float, and f64 has double precision.
}

fn nop(){
    // Numeric operations.
    //
    // Addition positive.
    let _add: u64 = 64 + 64;
    // Addition negative or also called substraction.
    let _add2: i64 = 12 - 65;
    // Multiplication positive.
    let _mult: u64 = 15 * 30;
    // Multiplication negative.
    let _mult2: i64 = -15 * 32;
    // Division.
    let _div: u64 = 15 / 3;
    // Floating Division.
    let _div2: f64 = 15.3 / 3.4;
    // Remainder operation.
    //  With sintax [A] % [B], indicate if [A] is divisible by [B].
    let _rem: i64 = 43 % 5;
}

fn bop(){
    // Bool assign.
    let _bool1 = true;
    let _bool2 = false;
    // Bool assign, way 2. Explicit way.
    let _bool3: bool = true;
    let _bool4: bool = false;
}

fn cvar(){
    // Char variables.
    let _a = 'z';
    let _b = 'H';
    // Rust also support emoji and icons in char.
    // It's because Rust support Unicode Scalar Value.
    // This means that Rust sopport a lot of icons and emojis, not just ASCII.
    // For complete glossary of Unicode see; https://unicode.org/glossary/
}

fn ttuple(){
    // A tuple is a group of numbers. Each field indicate the variable type.
    let _tvar: (i32,f64,u64) = (-64,3.55,3321);

    // Ok, put special attention to this. This is called destructuring.
    //  First, we create a tupple and assign values to it. Then we create 3 variables and
    // each value of the tuple will be assigned to the variable with the same position
    // (first to first, second to second, and more).
    let _tvar2 = (30, 40, -125);
    let (_a, _b, _c) = _tvar2;
    // With this, "a" have 30, "b" have 40, and "c" -125.

    // Also can be specified the exact index of tuple (tvar2).
    // The first index is always zero (position).
    let _index = _tvar2.0;
    let _index2 = _tvar2.1;
    // With this, "index" have 30 and "index2" 40.
}

fn arrvar(){
    // Array variable. Unlike tuples, all values must be the same type.
    let _a = [1, 2, 3, 4, 5, 6, 7, 8, 9];
    let _b = ['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'j'];
    let _c = ["First","Second","Thirty","Fourty"];
    // The first parameter indicate the type and the second is the times that type is used.
    // In this case, the type is i64 and the times used is 5, so the 5 tuple's values must be i64.
    let _d: [i64; 5] = [32764, 32765, 32766, 32767, 32768];
    // The first parameter indicate the value, and the second, indicate the times that repeat.
    let _e = [3;10];
    // So, that is the same that; let e = [3, 3, 3, 3, 3, 3, 3, 3, 3, 3]
    // For access to specific values of arrays, you must indicate the array number. In this example we use the array "d".
    let _f = _d[0];
    // So "f" have the value; 32764.
}

fn function_parameters(_parameterx: i64){
    // This function use two new things:
    //  1- The Rust function's name nomenclature standard
    //  2- Parameters.
    //   The nomenclature standard specifie that the function's name must be complete words in lowercase,
    //  separated by underscore (_).
    //   The "x: i64" parameter. In this case we have the specification that when this function is called, we must include
    //  a data that must be this type ( i64, integer of 64bits if you dont remember). This data will be used for the function.
    //  For example, here we use the parameter to print the value in screen.
    println!("The value of parameter 'parameterx' is; {}",_parameterx);
    // The parameters must be separated by a coma.

}


fn statements_expressions(_parameter_2: i32){
    // statements are instructions that perform actions and do not return a value.
    // expressions evaluate the resulting value.
    // For example, "let" is an statement and "println!" also.

    // This are an statements for example;
    let _var1: u64;
    println!("Statement message.");

    // This are expressions. The first is because evalue the value of "6" to correspond to "var2" type and assign it to variable.
    // The thrid also that is it because a number evaluates to the value 6 and return the value to "var3".
    // Because of that the expression "var3_1 + 6" dont have semi colon (;). Because if have semi colon will be converted
    // directly to statment, and a statement do not return a value.
    let _var2: i32 = _parameter_2;
    let _var3 = {
        let _var3_1 = 1;
        _var3_1 + 6
    };

}

fn return_values() -> u64{
    // The new added is " -> u64" in which the "->" indicate that will return a value to fathers function.
    let _var1 = 6;
    _var1
    // With this last expression the return value will be _var1, without a expression for return a value, cargo will give error.
}

fn conditionals(_parameter_3: i64){
    // Strings Constants. Are expressions that returns the string to the constants variables, and "const" are statements that take
    //  the return value and assign the same to memory space.
    const _MINOR10: &str = "(parameter minor to 10)";
    const _DIVISIBLE10: &str = "(parameter divisible by 10)";
    const _DIVISIBLE5: &str = "(parameter divisible by 5)";

    // Check if the value is less minor to 10.
    // The "if" and "else" are expressions that returns the statements "println!" if the conditionals are True.
    if _parameter_3 < 10 {
        println!("Condition {} is true.", _MINOR10);
    }
    else {
        println!("Condition {} s false.",_MINOR10);
    }
    if (_parameter_3 % 10) == 0 {
        // Check if _parameter_3 is disible by 10 and the rest is equal to zero.
        println!("Condition {} is true.",_DIVISIBLE10);
    }
    // if you need add more conditions you must use; else if
    else if (_parameter_3 % 5) == 0 {
        // Check if _parameter_3 is disible by 5 and the rest is equal to zero.
        println!("Condition {} is true.",_DIVISIBLE5);
    }
    else {
        println!("Conditon {} or for {} is false.",_DIVISIBLE10, _DIVISIBLE5);
    }
    //
    // If the condition is true return value to assign it to variable.
    let _variable_bool: bool = true;
    //      if variable's value is true proceed, if not go to else{}
    let _variable_for_return = if _variable_bool {
                    // Will return 5 to variable_for_return
                    _parameter_3
    }
    // must include else{};
    else {
    10
    };
}

fn conditional_loop(){
    let mut _counter: i32 = 0;
    // "If" inside "loop".
    let _result = loop {
        _counter += 1;
        println!("Counter variable value; {}",_counter);
        if (_counter >= 10) {
            break;
            }
    };
    // "While" condition. Check if the variable is "true", if is execute the statements and expressions inside, if not call a "break".
    let mut _variable_true: bool = true;
    let mut _counter2: i32 = 0;
    // The condition to be true to execute can be also somethings like;
    //      [A] < [B]
    //      Can be anyone operation that return a "true" or "false".
    while _variable_true {
        _counter2 += 1;
        if _counter2 == 100 {
            _variable_true = false;
            println!("Counter2 variable value: {}", _counter2);
        }
    }

    // Another example of while
    let _tupple = [2,4,6,8,10,12];
    let mut _counter3 = 0;

    while _counter3 < 6 {
        println!("Index value {}, correspond to tupple value {}",_counter3,_tupple[_counter3]);
        _counter3 += 1;
    }
    // But there are a problem with that code, we could cause the program to panic if theindex length is incorrect.
    // It’s also slow, because the compiler adds runtime codeto perform the conditional check on every element on
    // every iteration throughthe loop.
}

fn for_loop(){
    let _tupple_a = [1,2,3,4,5,6,7,8,9,10];

    // Note; "iteration" means go to each value, assign it to another variable and operate once with the value and continue with the next.
    // This is the statement "for", which create a new variable ("element") and assign each iteration of tupple, then operate.
    // ".iter()" function will iterate in each value of variable.
    for element in _tupple_a.iter() {
        println!("Variable 'element' have _tuppleA's value; {}",element);
    }
}

fn strings_formats(){
    // First, we create a new variable. Using the string type "String" (yes, the same name).
    // The new here is that "String" will indicate that new variable must be allocated into the heap instead of stack.
    // The "::" indicate that function "from()" will be executed into the same namespace than "String" and the value returned by it,
    // will be available to "String" function.
    // So, "from()" will parse data to "String" formatting the same.
    // NOTE: If you specifie the variable type with &str you will get error, because string create a struct string, not just a string.
    let mut _variable1 = String::from("Hello");

    // "push_str()" will concatenate the parameter into the variable.
    _variable1.push_str(", world!\nLinux Rules!!! :D");

    println!("String concatenated; {}",_variable1);

    // With the mutable variable and using "String" function, we will put the data into the heap even if we harcode all values that will
    // shape the final string.

    // When this ends (the function ends), the scope of those variables also, so the memory that they use will be release.
    // So, one good practice is divide as most you can the functions of the software to limit the access to memory and the effectiveness about clean memory.
}

fn memory_scenarios(){
    // Here we have an specific situation in which the memory management is different.
    // Here we create a variable with numeric value, then will create a copy of it (the variable's value) in memory (stack) to assign to a new one.
    let _x: u8 = 5;
    let _y: u8 = _x;
    // But here, we create a variable of string type to assign her value to the heap, but the new variable will no get a copy of the first, instead that,
    // will use the value (using a pointer to value) and specifying the length of value (the size in bytes).
    let _var1 = String::from("Welcome.");
    let _var2 = _var1;
    // So you need very carefully, because if you change var2 thinking that var1's value is safe, not, you will be changing also var1's original value.
    // So, What happen if (theoretically)  var1's scope ends and Rust call "drop" function when var1's value is under use also by var2? That two drop memory
    // operation will be invalid and will cause many issues for var2, moving that scenario to a security issue.
    // Because of that Rust do this; when var2 is created and have the same value as var1 (because both point to same RAM's value) , Rust will invalid var1, so
    // this last (var1) will be no longer available. This is know as; var1 was moved into var2.
}

fn heap_clone(){
    // As was explained in "memory_scenarios", in non-numeric values Rust point two or more variables at the same RAM's value, just be valid the last
    // variable that point it.
    // But, if we want clone the RAM's value instead of point to the same that first variable we need do this;
    let _string1 = String::from("Hello World.");
    let _string2 = _string1.clone();
    // With the use of "clone()" you do a data deep copy of string1's heap and that new data is available to string2, pointing to a new heap.
}

// Remember, the "-> X" indicate the return's type.
fn variable_return() -> u32 {
     // Put some simple example.
     // As you know if the last line don't have semicolon (;) means that it is the return.
    let var1 = 500;
    var1
    // Return to father function a unsigned 32 bits type variable.
}

fn variable_return2() -> String {
     // Put some simple example.
     // As you know if the last line don't have semicolon (;) means that it is the return.
    let var1 = String::from("Hello. String type return.");
    var1
    // Return to father function a string type variable.
}

// As you can check here, the "s" variable
// Don't forget; usize => unsigned numeric size
fn multiple_returns(s: String ) -> (String, usize) {
    // Take the length of "s" variable and store the return in length.
    let length = s.len();
    // We return "s", the same parameter passed to this function and the size and the variable that have the size.
    // The order is how is typed.
    (s, length)
    // You can use this to return errors and the error's code.
}

// As was specified more down, the "&" indicate; passed by reference.
fn use_of_reference( s: &String) {
    // Is not a requirement use () in "if", but I prefer it because is more clean for the developer.
    if (s == "This is a value to by passed by reference." ){
        println!("Is equal.");
        }
        else {
        println!("Is not equal.");
        }
}

fn use_of_mutable_reference ( s: &mut String) {
    // We concatenate a string to "s" variable.
    s.push_str(" and this is the concatenated value to original value.  ");
    // Now, what happen if we want clear the string variable? Very simple
    s.clear();
    // That will make the string as; "" which is equal to say NULL, which is different to say empty.
}

fn first_word(s: &String) -> usize {
    // Transform "s" string to bytes storing into "vbytes" because for iterate is more easy than use ASCII and maybe have issues with the language.
    let vbytes = s.as_bytes();
    // For loop.
    // Take vbytes variable, use "iter()" function to return each value in a collection, that iter()'s return pass to "enumerate()" function
    //  storing the word in "i" var and the index value in "&item" (because is a value to a position in another variable must be used by reference).
    //  So we can conclude that "enumerate()" function returns a tuple; first item and second position.
    // For each iteration check if the position is equal to space but specifying in equal byte (b' ').
    for (i, &item) in vbytes.iter().enumerate() {
        if item == b' ' {
            return i;
            }
    }
    s.len()
}

fn string_slice(){
    // NOTE; ALL RANGES MUST BE IN UTF-8

    // Lets create an empty string.
    let mut _vstring = String::from("hello world");
    // Now we will use a string slice.
    // As I said in main function, a string slice is a string portion of mayor string.
    // Is so simple as indicate the arrange bits, the syntax is;
    // [bit_start..bit_end]
    let _hello = &_vstring[0..5];
    let _world = &_vstring[6..11];
    // The first position is always zero.
    // "hello" start in position zero and ends in position four (4), so, why ends in position five (5)?
    // because that additional byte indicate a space (if no byte is indicated will set a space).
    // "world" start in byte number six (6) after the space in byte number five. But "world" word ends in byte number 10, as
    // was specified zero in "hello" we are not doing reference to number of bytes, we are doing reference to position number (which storage 1 byte).
    // So position number 11 indicate EOL (end of line), the end of the string.
    //
    // Also if you want specifie the end of specific string
    let _end_string = _vstring.len();
    let _string = &_vstring[0.._end_string];
    // In that case "_end_string" storage the leght of "_vstring" and then we specify the end of string slice with that variable's value.


    // If you want to start from first position (number zero) you can do;
    let _hi = &_vstring[0..2];
    // or
    let _hi = &_vstring[..2];
    // Both means the same; start from beginning, the position zero.

    // Also if you want start from beggining to end
    let _v2string = &_vstring[..];
    // or
    let _v2string = &_vstring[0..];

    // Using this and the learned in "first_word" you can specifie also what do you want return.
    // As Rust don't leave us return two different things in different ways, I will write in a comment:
    // return &_string
    // or
    // &_v2string[..4]

    // As a string slice can be the whole string (if you specifie it) is better put on the parameters use
    // strings slices instead of "String".
    // "Ok, understand, so how I do that?"
    // Simple; with &str  ¯\_(ツ)_/¯ becuase &str do reference at memory's position of known string size, not whole string.


}

fn numeric_slice(){
    // Suppose that we have this;
    let _vtuple = [1,2,3,4,5,6,7,8,9];
    // We can use the slice to specify the positions;
    let _vslice = &_vtuple[0..3];
    // So vslice take the values from; 1 to 4 (remember, the first position is zero).

}

fn function_struct(){
    // Structs is an array of data, which one have multiple different data type inside.
    // There, we have an instance called "UserExample" defining a new type with the same name.
    struct UserExample {
        // The syntax is;
        // [key]: [value]
        username: String,
        email: String,
        age: String,
        civil: bool,
    };
    // NOTE: Do not specify types as reference yet, because when you do it in a struct, you must also add lifetime. Which will be discussed more later.
    // Here we wait the user input and after store we use to init the new type.
    // Remember, we create a mutable variable string type.
    let mut _vinput = String::new();
    println!("Intert username");
    // As associated for "io" we use; the stdin function to use the capabilities of standard input and read a line for storage that input into "_vinput"
    io::stdin().read_line( &mut _vinput )
        // In case of error show this message.
        .expect("Fail to read line");

    let mut _vage = String::new();
    io::stdin().read_line( &mut _vage )
        .expect("Fail to read line");
    // Now for not write a lot of repetitions we init the struct with that _vinput's value.
    // The init must be trough a variable.
    // The type is the self struct.
    // Even if you don't complete the struct, you must specify all fields.
    // Because of that, complete your variables and then assign them to the respective struct.

    // Here how to do from a variable or with string hardened.
    let mut _user1 = UserExample {
        username: String::from( _vinput),
        email: String::from("nobody@nobody.com"),
        age: _vage,
        civil: true,
    };

    // But you want use an existing struct
    _user1.civil = false;

    // With that you can create functions that use parameters to create and complete structs.
    // Also you can return a struct, just must have the same struct name that the inside, in this
    // case; UserExample
    // Also if you have this types;
    //
    // username: username,
    //
    // You can replace it for just;
    //
    // username,
    // That is because rustc (the Rust compiler) will assign into the type the variable with the same name.

    // There are some called; linked structs
    // For example; in the below struct there are references to user1 variable struct, so linked values depend of user1's values.
    let mut _user2 = UserExample {
        // Here the field "username" from "user1" instance will be used as value.
        username: _user1.username,
        // The same for "email".
        email: _user1.email,
        // But, if you have a lot of subtypes and if you want specify that they must have values from "X" instance, you can do;
        .._user1
        // So, the subtypes "age" and "civil" will take values from user1 instance.
    };

    // We can also can create tuples as structures, in which each case field don't have linked a name just the argument type.
    // As the others structs; each struct have a new type which is the struct's name.
    // (x,y,z)
    struct Position3DSelected(bool,bool,bool);
    struct Position3D(i64,i64,i64);

    let _init1 = Position3D(3,5,15);
    let _init1_position = Position3DSelected(true,true,true);

    // Is very important know that in structs tuples you can not link between them, even if subtypes are the same.

    // Unit-Like structs
    // Are full empty structs
    struct UnitLikeStruct();

    // Now there are some special point, what happen if you want set the debug information into the struct ?
    // This is useful when you want show some struct with println!, but remember delete it when you do a release
    #[derive(Debug)]
    struct Rectangle {
        width: u32,
        height: u32,
    }

    let _dvar = Rectangle { width: 30, height: 50 };
    // Now we can use the debug mode of that structure to see the code.
    // But println! will not work unless we set see the debug mode.
    println!("Showing the debug struct; {:?}", _dvar);
    // So, as you can see the; {:?} enable debug mode in println
    // But the output will not be easy to read if is huge, so we can add # to format the output automatically
    println!("Showing the debug struct, now with auto format; {:#?}", _dvar);
}

fn function_methods(){
    // The methods are similar to functions in which they are declared with "fn" keyboard and then their name, can have parameters
    // and have also a return.
    // But the main difference is that; methods are defined in the context of struct and the first parameter is always "self" doing reference to the instance of the struct.
    // In another words;
    // You create the "A" struct, then you implement the "B" method which when is called will modify (or not) "A" to work specifically with it and when finish the values will still be available.
    // The first argument (and the main) to use methods is the self function of Rust. If you create an struct inside a function ( remember the function of ownership ) when finish, the memory which include the struct will be cleaned.
    // And if you want use the struct outside the function is not like C, you will must do a method to access specifically to it, so when the method finish and outside struct will be still available to check the value.
    //
    // The struct must be outside the function.
    struct InfoStatus {
        logged: bool,
        last_time: u64,
    }

    // Now the method's syntax is;
    // impl [Struct_Name] {
    //      fn [func_name] ( &mut self ) -> [return_type_to_variable] {
    //              self.[Struct's_variable] ;
    //      }
    // The "self" argument to reference to "[Struct_Name]"
    // And "impl" do reference to "implementation" linking the struct to that function.
    // You must set "&mut self" if you want change the struct's data, if not use "&self" instead.

    impl InfoStatus {
        fn status_update ( &mut self ) -> String {
                // And because you can use methods with variables, you can return values.
                let str1 = String::from("Status updated");
                str1
                }

        // A method not limit the number of functions that have whiting.
        // You can store a lot of functions inside one implementation or you can create a lot of implementations with the same name and different functions.
        // There are not reason for create a lot of "impl" blocks, but is permitted.
    }

    // Now we pass parameters to the "implementation" for operate with the struct.
    // First, Rust send the the parameters to the struct and then the function is called to operate.
    // Now "var1" do reference to the struct, so if you want print it, must be with debug mode, or print some field specific. The same for operate it.
    let mut _var1 = InfoStatus { logged: true, last_time: 300 };

    println!("Var1 InfoStatus logged value: {}", _var1.logged);
    println!("Var1 InfoStatus method function return: {}", _var1.status_update());
    // Also when invoke a function you can pass variables to it, as any function.
    // According to your Rust version is plausible that "var1" must be mutable.

}

fn function_enums(){
    // Enums are a new type of data.
    // You set a "start" value and with a enum you can enumerate all variants of it.
    // The syntax is alike implementation.
    enum IpAddress {
        V4,
        V6
        }
    // With that we created a enum called "IpAddress" (is a new custom type) with two sub-types; "V4" and "V6"

    // We can create new variables with the new type "IpAddress" with specific subtypes.
    let _nvar1 = IpAddress::V4;
    let _nvar2 = IpAddress::V6;

    // We can nest the new type to structs
    struct NStruct1 {
        vtype: IpAddress,
        // Must be a string because include dots: 127.0.0.0
        address: String,
    };

    // Now we create a new instance of the "nstruct1" struct and each type of IpAddress type.
    let _vinstance1 = NStruct1 {
                        vtype : IpAddress::V4,
                        address : String::from("127.0.0.1"),
                        };
    let _vinstance2 = NStruct1 {
                        vtype : IpAddress::V6,
                        address : String::from("::1"),
                        };

    // But, is not necessary create a struct to detail the basic type which is associated.
    // We can specify directly the basic type for each field, between "( )".
    enum NIpAddress {
        V4(String),
        V6(String),
        };

    // And the variables with that new type keep this;
    let _vinstance3 = NIpAddress::V4(String::from("127.0.0.1"));
    let _vinstance4 = NIpAddress::V6(String::from("::1"));

    // Even, one advantage of enums to struct is each variant/field can have different basic types.

    enum V2IpAddress {
        V4(u8,u8,u8,u8),
        V6(String),
        };

    let _vinstance5 = V2IpAddress::V4(127,0,0,1);
    let _vinstance6 = V2IpAddress::V6(String::from("::1"));

    // But if we don't want use our types for storage IP addresses. We can use the standard libraries and functions to store and operate with IPs.
    // Remember that the functions and respective structs only will be available if you specifies use the respective library.
    // use std::net::{IpAddr, Ipv4Addr, Ipv6Addr};
    let _localhost_v4 = IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1));
    let _localhost_v6 = IpAddr::V6(Ipv6Addr::new(0, 0, 0, 0, 0, 0, 0, 1));

    // Taked from the Rust learning webpage: https://doc.rust-lang.org/book/
    // " This code illustrates that you can put any kind of data inside an enum variant:
    // strings, numeric types, or structs, for example. You can even include another
    // enum! Also, standard library types are often not much more complicated than
    // what you might come up with.
    // Note that even though the standard library contains a definition for IpAddr ,
    // we can still create and use our own definition without conflict because we
    // haven’t brought the standard library’s definition into our scope. "

    // Another type of enum is "Option".
    // This is very useful when you are in a scenario in which you need that the value could be something or nothing.
    // This functionality can prevent bugs that are very common in another languages.
    // This is because Rust don't include the type "Null" that have another languages. Even Tony Hoare in 2009 said that
    // the inclusion of "Null" type was because was very easy to implement but that implementation caused a lot of crashes, errors
    // and vulnerabilities around more than forty years.
    // And this is because sometimes developers try to use Null values as not-null values.

    // "Option" has two sub-types; "Some" and "None".
    // Don't worry, none dev understand this at the first, because Rust use "Some" when inserts data
    // or exist data and "None" when the data don't exist.
    // Let me show you;
    // First we create a new struct:
    struct People {
        first_name: String,
        second_name: Option<String>, // can be any type of data.
        last_name: String,
    };
    // First we specified that the "firstName" field is a string, but as no everybody have a second
    // name we specify it as an option in which can exist or not but if it is, must be a string.
    // Now we init the struct with a new variable:
    let _john_wick = People {
        first_name: String::from("John"),
        // Here John don't have a second name, so we specify to Rust that value don't have any type
        // of data.
        second_name: None,
        last_name: String::from("Wick"),
    };
    // Ultra secret, the real name:
    let _john_wick_real = People {
        first_name: String::from("Jardani"),
        // Here we specify the second name of John, so we indicate to Rust compiler that in this
        // case exist a data.
        second_name: Some(String::from("Awesome")),
        last_name: String::from("Jovonovich"),
        };
    // So if you think that "Option<TYPE>" indicate that can exist data of type "TYPE" in that
    // variable and "Some" indicate that exist and "None" no, you are right.

    // But there are some important things that you need to know:
    // 1- If you specify "Option<TYPE>" the value will have type as; Option<TYPE> not as "TYPE".
    // 2- If you want operate with the value with "Option" type, you first need convert to "TYPE".
}

fn function_match( _x: String ){
    // The match is a control flow operator which allow to compare a data against a series of
    // patterns and then execute code based in it. Is like the next digievolution of "if".
    // But one important thing about "match" is that when data fits into a pattern will execute
    // that pattern.
    // Yes, we used before but here will be explain in more detail.

    // match _variable {
    //      [Value_to_match] => [Value_to_return_or_thing_to_execute],
    //      [Value_to_match] => [Value_to_return_or_thing_to_execute],
    // }
    //
    // There, for example, are to "arms" but can be any as you want.
    // But also includes in values to match somethings like:
    // true : To evaluate if _variable is boolean with "true" value.
    // false : To evaluate if _variable is boolean with "false" value.
    // A | B | C : To evaluate if _variable match with A or B or C (you can include the number of
    //      "or" that you want.
    // A..B : To evaluate if _variable match with values from A to B.
    // A..=B : To evaluate if _variable match with values from A to B inclusive.
    // Some(x) : To evaluate if _variable is "option" type and match with "x".
    // None : To evaluate if _variable is nothing (see function_enums for remember it).
    // Complex conditions like:
    //      Some(x) if x < 10
    //      Some(x) if { _variable.set(_variable.get()+1);false}
    // In resume, any expression can be inside "match".
    // See; https://doc.rust-lang.org/reference/expressions.html
    // So, what is the difference to use "if"?
    // If you use "if" you must return a boolean, if you use "match" you can return anything.
    //
    // If you want execute more than one line in the match's arm, you must put between curly
    // brackets; {}
    // [Value_to_match] => {
    //          [execute_line_1];
    //          [execute_line_n];
    //          }
    // For example;
    let _var_example: String = String::from("Hi");
    let _var_compare: String = String::from("Value2");
    match _var_example {
        _var_compare => println!("Match!."),
    }
    // Also we can compare to Option<T>
    let _var_example2: Option<u8> = Some(10);
    match _var_example2 {
        None => None,
        // Here "i" take the Some's value of parameter in this case; 10
        Some(i) => Some(i+1),
    };
    // Remember that both values "None" and "Some" are mandatory.
    // If you are Linux user, you must know the wildcard for zsh and bash; *
    // Well, Rust have the same for match; _ , yes _
    // Take in consideration you can not compare more than one, why? Simple, ownership my friend. So we create another.
    let _var_example2: String = String::from("Hi");
    let _var_compare2: String = String::from("Value2");
    match _var_example2 {
        _var_compare2 => println!("Match!."),
        // Just remember put " _ " at the end of match, because if you put before another arm will not check it.
        _ => println!("Don't match with _var_compare."),
    }

}

fn function_iflet(){
    // Página 149
    // iflet allow to create a variable and then comparate it to another value
    // For example if can use this;
    let some_u8_value = Some(0u8);
    match some_u8_value {
        Some(3) => println!("three"),
        _ => (),
    }
    // Or we can use this, more light;
    if let Some(3) = some_u8_value {
        println!("three");
    }

}

fn packages_and_crates(){
    // Here are many very very important theory parts:

    // Crate:
    // A "create" is a library or binary (like librand that we used for generate
	// random numbers). A library es a Rust program  which don't generate a
	// binary program, instead it, only provide functions to your program. That
	// means if you want open and modify a file in your SSD/HDD, you must
	// use a library that provide you the enough functions for that. A
	// library can not be compiled into a binary (and
	// executable) program by it-self, why? Because don't provide a "main()"
	// function and have another file extension. From Rust Source: "The
	// create root is a source file that the Rust compiler starts from and
	// makes up the root module of your create". So in this example the
	// create root is; src/main.rs
	// Keeping the example of rand library, as we used before, the
	// functionality of the create is accessible trough the create's name; rand.

    // Package: Is one or more creates that provide a set of functionality. A
	// package contains a Cargo.toml file that describes how to build those
	// creates. From Rust Source: "A package must contain zero or one
	// library creates, and no more. It can contain as many binary creates
	// as you'd like, but it must contain as least one create (either library
	// or binary)." So, if you build a library for (for example) provide
	// enhanced functions like libcurl (for C) the // final lib can only by one
	// (the "creates"), but can contain as many depdencies as you need.
	// "Cargo" is the package management of Rust. When you have a huge program
	// is better for everyone use a management for keep the changes,
	// dependencies and another things. When you do "cargo new [proyect_name]"
	// Cargo will create the requirements for create a new package with
	// "Cargo.toml" inside and [proyect_name]/src/ will be your "main.rs"
	// create root file (don't change the name or Cargo will not know what file
	// pass to rustc to compile).
	// [project_name]/src/main.rs	-> Cargo and rustc will create a binary.
	// [project_name]/src/lib.rs	-> Cargo and rustc will create a library.
	// If you have both, Cargo will provide it to rustc and compile both with
	// the same project's name.

	// If you want check if there are issues in your code without compile it;
	// "cargo check"
	// When you execute "cargo build" the binary/library file will be allocated
	// in [project_name]/src/bin directory.
	// Because this function only have comments is equal an empty function.
}

////////////////////////////////////////////////////////////////////////////////
////////////////////////////MODULE_START////////////////////////////////////////
// For this I must put outside function, if not will not work properly:
	// Module:
	// Is an organization about a create (bin or lib) into groups.
	// That groups can hold data which can be accessed from outside or inside.

	// For create a new library with Cargo:
	// cargo new --lib [lib_name]
	// And then use as main; src/lib.rs
	// So how we create a module in the new lib?
	//
	//	mod [module_name]{
	//		functions_which_use_to_work();
	//	}
	//  Inside one module, you can get as many sub-modules as you want.
	//
	mod module_1 {
		pub mod module_2 {
			pub fn function_print() {
				println!("Even you can create your functions");
			}
		}
	}
	// By default Rust do private all modules.
	// Items in parent module can't use items in child modules. But child
	// modules can use items in parent modules. Because of that we must put
	// "pub" before module_2, if not public_function_2 will can not use
	// function_print. Also for function_print. Remember making the module
	// public doesn't make its contents public. The privacy rules apply also to
	// structs, enums, functions, methods and modules.

	// My recommendation is you put all your modules inside your library and use
	// from it. In that case, if you use the module_1 will use also the module_2
	//, because is the only inside one.
	// Also in the libraries you can specify what function can be used by the
	// others programs who use your lib (functions with not "pub fn" will can
	// used inside the lib's functions):
	pub fn public_function() {
		println!("This public function only prints in screen.");
	}
	// I put every this inside this because is more easy for you learn. But
	// don't have any sense put modules inside the main binary. All must go
	// inside the library.
	// If you want use modules inside your functions you must import first
	// with; "use [lib_name;]". And then;
	pub fn public_function_2(){
		// You can specify by absolute path to function inside lib:
		crate::module_1::module_2::function_print();
		// Or you can use the relative path, be careful with the positions:
		module_1::module_2::function_print();
		}

	// Even you can define the use of outside functions, with "super::"
	fn outside_funct() {}
	mod module_x {
			fn function() {
				super::outside_funct();
				}
		}

	// Ok. We know how to use module's functions. But, How if we want use impl?
	// Easy, as before, Rust do private by default. So even if the struct is
	// public, we must set what variables are too.
	mod build_house {
		pub struct HouseMeasurements {
			pub house_floor_large: u16,
			pub house_floor_width: u16,
			house_radius: u16,
		}

		impl HouseMeasurements {
			// Remember return the struct's same type.
			pub fn house_measurements_fn(wid: u16, lar: u16) -> HouseMeasurements {
				HouseMeasurements {
					house_floor_large: lar,
					house_floor_width: wid,
					house_radius: lar * wid,
				}
			}
			// If you are confused why house_measurements_fn can use house_radius even if is not
			// public, member that a child item can read a upper item because is created in the
			// same context but a parent item can not access to child items.

		}

		// Do you prefer field by field? Just do this outside the module
		// let mut struct_x = build_house::HouseMeasurements::house_measurements_fn(64,15);
		// println!("{}",struct_x.house_radius);

	}

		// Now we understand the use of modules we can explain something that we used before but
		// never explain; "use" keyword.
		// The "use" keyword bring into scope some modules. For example:
		mod be_something{
			pub mod tobe_or_not_tobe{
				pub fn something(){
					println!("Something, yes literally");
				}
			}
		}

		// I will put the ways you can use something function so you can see the "use" propose:
		// Way 1:
		pub fn see_the_function() {
		crate::be_something::tobe_or_not_tobe::something();
			}

		// Way 2:
		use crate::be_something::tobe_or_not_tobe;
		pub fn see_the_function2() {
			tobe_or_not_tobe::something();
			// This way and way3 you import into program's scope a module's function with "use".
			}
		// Way 3:
		use crate::be_something::tobe_or_not_tobe::something;
		pub fn see_the_function3(){
			something();
			// The Way 2 is better than this because you can see the parent module, with this way
			// not.
		}

		// That way also can be used into return's types, if its a type of course.
		// You can not import into scope two modules or functions with the same name, even if come
		// from different parent modules. You need import with different name.
		use crate::be_something::tobe_or_not_tobe as the_same_tobe_or_not_tobe;
		pub fn see_the_function4(){
			the_same_tobe_or_not_tobe::something;
		}

		// When we bring a name into scope with the use keyword, the name available in the new
		// scope is private. To enable the code that calls our code to refer to that name as if it
		// had been defined in that code’s scope, we can combine pub and use. This technique is
		// called re-exporting because we’re bringing an item into scope but also making that item
		// available for others to bring into their scope.
		// Re-exporting is useful when the internal structure of your code is different from how
		// programmers calling your code would think about the domain. With pub use, we can write
		// our code with one structure but expose a different structure. Doing so makes our library
		// well organized for programmers working on the library and programmers calling the
		// library.

		// Now we will see something of order:
		// use std::cmp::Ordering;
		// use std::io;

		// But is better use:
		// use std::{cmp::Ordering, io};

		// Another example:
		// use std::io;
		// use std::io::Write;

		// or better:
		// use std::io::{self, Write};

		// or if you want import into scope all modules about one, you can use the global operator:
		use std::collections::*;
		// That will bring into scope all submodules of collections's module.

		// Now we will know how to split functions into different files.
		// For that go to; src/lib_source_code.rs
		// Now we import into scope the public (must be, if not will only for internal use into the
		// same file) modules:
		mod lib_source_code;
		use crate::lib_source_code::module1;
		// As you can see the "lib_source_code.rs" is trated like a module when you must specify
		// the same name into the "use". This can be also applied for directories in which those
        // are trated like modules.
		// Now we can use the function inside module1:
		fn external1(){
			module1::separated_function1();
		}

///////////////////////////////////////////////////////////////////////////////////////////////////

fn structures_as_collections(){
    // Rust's standard library include many usefull data structures known as collections. A
    // collection can contain multiple values instead of one value as another data types.
    // The mainly difference between another values types is collections are stored in the heap of
    // the RAM memory, so can grow or shrik at real time when the program is running.

    // There are many differents types of collections:
    // -vectors: This collection type allow to store many variable number of values next to each
    // other. This is very usefull when you don't know how many values will have something and want
    // to be sure that you will not go into a bad scenario (in specific if you want use this for
    // bigdata). The index of each element start from zero (0).
    // -strings: As we mencionated before, this is a string of UTF-8 alphanumeric characters.
    // -hash map: This is new. This allow associate a value with a particular key/tag.

    // This is how you can create a vector.
    // let mut variable: Vec<T> = Vec::new() where T is the variable's type.

    let mut vvariable: Vec<i32> = Vec::new();

    // Or also you can create a vector with vec! macro.
    // Rust will interpret the code for configure the type.

    let mut vvariable2 = vec![1,2,3];

    // For update the existing macro adding items:
    vvariable.push(4); // Add 4 in the vector as first value.
    vvariable.push(8); // Add 8 after 4 as second value.

    // For remove the last existing value and return the last available.
    vvariable.pop();

    // Dropp vector for release memory.
    // As you know when you finish the key's code, the scope is out and the data is cleaned.
    // Just put your vector into keys and then finish scope.
    {
    let mut vvariable3: Vec<i8> = Vec::new();
    vvariable3.push(1);
    vvariable3.push(2);
    // Do another things that you want.
    }

    // There are many ways in which you can get information from vector's position:
    // Way1 using another variable:

    let secondvalue: i32 = vvariable[1]; // The second value of "v". Remember; the first index is zero.
    println!("Second Value of vvariable vector: {}",secondvalue);

    // Way 2
    // Without intermediary for use in another comparations.
    // This is the best way if you want compare values for vectors.
    match vvariable.get(1) { // The second value.
        Some(secondvalue) => println!("Match. With second value: {} {}", secondvalue, vvariable[1]),
        None => println!("Don't match. Wich is very rare, check the code."),
    }

    // Way 2.1, directly to variable
    let firstvalue = vvariable.get(0); // Remember. The index start from zero.

    // Iterating over values
    for i in vvariable {
        // Remember; i variable store each value of vvariable and then iterate them.
        println!("vvariable items; {}", i);
    }

    // Even if the variable is muteable, you can iterate over them changing values, remember that
    // the vector variable must be mutable, if not you can add; &mut before the name here:
    for mut i2 in vvariable2 {
        i2 += 15; // It's the same than put; i2 = i2 + 15. Because of that must be mutable.
        println!("vvariable items with 15 add; {}", i2);
    }

    // If you remember, we said that a vector can store any value inside. So is reasonable use a
    // vector with a enum.
    enum Evariable1 {
        Var1(i8),
        Var2(f64),
        Var3(String),
    }
    let data = vec![
        Evariable1::Var1(3),
        Evariable1::Var2(15.64),
        Evariable1::Var3(String::from("Hello World")),
    ];


    // Before we touched the strings but now we will go more deeply.
    // Just a note by ShyanJMC:
    // Take in consideration that Rust can controll embebeed hardware with low level so many things
    // in the strings are trated with low-level controll in mind, between languages like C and
    // others like Python.
    // As you see before, there are two types of strings in rust; str which is in the core of Rust
    // and String wich is in the Rust standar library.
    // You can see an string (str or String) like a collection of UTF-8 bytes in memory. Because of
    // is very needed for minimal usage, kown how to create a String we told you at start but in
    // this part of the live source code, we will go more deeply in this terms to understand
    // better the functionality of this.
    // Because Rust must know how to manage the string's memory, is because you must use
    // ".to_string()" function if you want add UTF-8 bytes to existing before string variable.
    let mut s: String = String::new();
    s = "Hello ".to_string();
    s = String::from("World.");
    // If you remember; .push_str() for add strings into variables, you probably know that with
    // .push() you can add only one single character.
    s.push('.');
    // Or even you can add strings between them:
    let s2 = String::from("Linux Rules!");
    let s3 = s + &s2; // With addition the second parameter must always do by reference/borrowing.

    // In other languges you can index the value of a string's character;
    // C:
    //      int string = c[0];
    // In Rust you can not do that in anyway. Why? Because if do it, Rust should return you the byte
    // value of the character in that position, not the character per se. Also when you use index
    // to characters, the language must start from the zero position to the indicated, so its
    // consume time and sometimes the number index is wrong.
    // There are tree ways in which the languge store the data:
    // - Byte; This the most common way, and the lowest. Is the way in which all cpus store data.
    // - Unicode: This is the way in wich Rust store char (one single character) data.
    // - Grapheme cluster: This is most abstract way, because this way is which humans read.
    // As I said; is a very bad idea use index in strings, but if you still want use it the only
    // way in which you can use is by byte.
    // &c[X .. Y]
    // In which "c" is the string variable, and X the byte init and Y the byte end.
    // So be very carefull which range use, because if you select a wrong range, you will get a
    // very bad error at runtime.
    // If you need split a string into each character you can use; .chars()
    for a in s3.chars() {
        println!("{}", a);
        // So, will print each char of "s" into new lines.
    }
    // If you need split by byes; .bytes()
    for b in s3.bytes() {
        println!("{}",b);
    }

    // Hash maps:
    // The hash are like a score map; [KEY_NAME]: [VALUE]. Simple and more efficient than use
    // variables, in performance and security terms.
    // First, we import into score the HashMap module.
    use std::collections::HashMap;
    // Create the new type.
    let mut scores = HashMap::new();
    // Insert keys and values.
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);
    // Take in consideration that;
    // - The key must be a String, wich is owned by the hash map.
    // - The value must be in i32, so you can not set any type of value and is not owned by the
    // hash map.

    // What about is we have configured a HashMap with a hundred of keys and we need known if
    // someone match?
    // Take a look:
    // We started first a string with the key to match:
    let team_color = String::from("Red");
    // Now we use the started HashMap to match (scores variable is the HashMap from upper lines):
    let vmatch = scores.get(&team_color);
    // The .get() function will provide the value for the key "team_color" (and their value).
    // The two possible returns are "None" is don't match with a value, or "Some<t>"
    match vmatch {
        None => println!("The team doesn't match with existing one."),
        _ => for (key,value) in &scores {
                // As the HashMaps store the data next each other you can print value in pairs
                println!("{}: {}", key, value);
        },
    };
    // Rust only allow that each key is pointing to only one value at the same time.
    // So, is you want update a value, you must overwrite the old.
    // The overwrite can be done with ".insert", yes insert the value doesn't matter if have value
    // or not, while the key be the same.
    scores.insert(String::from("Blue"), 30); // So, this overwrite the older value; 10.
    let mut blue_team_score = 100;
    scores.insert(String::from("Blue"), blue_team_score); // This overwrite the last value; 30.

    // But, what happen if you want only add if not exist a previus value? With ".entry()" function,
    // wich will return the value of key, we will pass the return to ".or_insert()" which if
    // doesnt' exist a value, will insert the parameter.
    // Remember; scores variable is our HashMap variable.
    scores.entry(String::from("Blue")).or_insert(1200);
    // That will pass the "score" HashMap to "entry" function to check if exist a value to "Blue"
    // key passing it to "or_insert", if doesn't "or_insert" will insert 1200 value to it.

    // Now we will see how to split by whitespaces and use HashMap for store the count of words.

    let text = "hello world wonderful world";

    let mut map = HashMap::new();

    for word in text.split_whitespace() {
        // ".split_whitespace()" will split words using as separator whitespaces.
        let count = map.entry(word).or_insert(0);
        // Then, each word stored in the iteration (in the variable word at start from "for") will
        // be checked if have value
        *count += 1;
    }

    println!("{:?}", map);
}

fn hanling_errors() {
    // HashMap uses a “cryptographically strong”1 hashing function that can provide resistance to
    // Denial of Service (DoS) attacks. This is not the fastest hashing algorithm available, but
    // the trade-off for better security that comes with the drop in performance is worth it.
    // If you profile your code and find that the default hash function is too slow for your
    // purposes, you can switch to another function by specifying a different hasher.
    // A hasher is a type that implements the BuildHasher trait.

    // Rust groups errors in two; recoverable and unrecoverable error. For the first, for example
    // when a file is not found, is reasonable report it to the user and try again. For the second
    // there are two basic options; or the program will show a internal bug or will crash.
    // We saw the first with "Result<T, E>" and now we will see the second with; panic! (remember;
    // the ! at the end indicate it is a macro. Also remember that Result<T,E> is a return of
    // standard library, not a function you can use).

    // When you execute the panic! macro, Rust will stop execution, clean the memory and then quit.
    // The process of clean the memory for each function is called; unwinding.
    // Is secure use the panic! macro because clean the stack of memory when exit. But if you need
    // that the binary must be small as you can, you can avoid the memory clean but be carefull
    // because that remain memory must be clean by the OS. To avoid the automatic clean you can
    // use; abort.
    // Go to Cargo.toml in the parent folder to see how to create alias, in this case for use panic
    // as an alias "abort".

    // Now after editing Cargo.toml you will see that when you call panic! will use abort.
    // I highly reccomend see this part of error handles in the rust webpage, because there are so
    // many examples that is more easy, and with more quality, see there;
    // https://doc.rust-lang.org/book/ch09-01-unrecoverable-errors-with-panic.html
    //

    // Now we will use Result<T,E> to handle between recoverable and unrecoverable errors.
    // T is the result type to return if was right and E the result type to return if fail with
    // some bug.
    // For this we will use files. Yes, it's time to operate with files in your OS.

    use std::fs::File; // "std" for standard library, "fs" of file system and "File".

    // First, before open a file we need a variable to store the data to point to file, and then
    // run the command to open it.

    let vfile = File::open("test1.txt"); // If we don't spcify the absolute path, will be used
    // or checked that name in your parent folder, in which you execute this file.
    // Now we will compare to ensure that is valid using this time panic! if not:
    // We use the same var to not use more memory.
    // vfile = match vfile {
    //    Ok(file) => file,
    //    Err(error) => panic!("Failed to open file. Check the same. Error; {}", error),
    // };

    // If we want to handle different type of issues, we must use a second match for many type of
    // issues:
    
    // Here we must include the crate that support matching errors:
    use std::io::ErrorKind; // Remember "io" means "imput/otput".

   // We could use match as many times we can, but I will show you a more optimized way.
   // Is easy lost in this, so don't be fraid and re read as many times you need and understand
   // each line.
   // After the order to open a file, we unwrap the return of "open" operation to operate many
   // times. The return of all after is under "error".
   let vfile2 = File::open("test2.txt").unwrap_or_else(|error| {
       // Still inside the "unwrap_or_else" we evaluate if the error match with "NotFound".
       if error.kind() == ErrorKind::NotFound {
           // If Match we create it. And evaluate again using as temporal variable "error".
           File::create("hello.txt").unwrap_or_else(|error| {
               // If there are some type of error we make an inmediate panic.
               panic!("Problem creating the file: {:?}", error);
           })
        // Else if the error don't match with "ErrorKind::NotFound"
        } else {
            panic!("Problem opening the file: {:?}", error);
        }
    });
   // By default when you use the call ".unwrap" you are doing a corroboration.
   // Unwrap will check the result immediately, if the result is ok will return the value ( Ok() ),
   // if there are some error ( Err(error) ) will return panic!.
   // Is the same function that we used some lines upper, but with the messages of panic!
   // specified.
   // Additionally, the "unwrap_or_else" provide the feature to use inside the "if" and "else".
   // As you now the "unwrap" function is very similar to another that we used before; "expect". We
   // used it since the first function. The main difference between one and another is the second
   // ("expect") allow us to specify the panic! message.
   
    let f = File::open("hello.txt").expect("Failed to open hello.txt");

    // Propagating the error: when you call a function and that function fails, is better not end
    // the program there. Instead of that, you can return the value to the code that called him and
    // then decide what to do.
    // Becuase of this, is mandatory add to the function's header as return; Result<String, io::Error>
    
    // There are an excelent paper to read about this, in specific "Propagating Errors" of Rust's
    // documentation:
    // https://doc.rust-lang.org/book/ch09-02-recoverable-errors-with-result.html#propagating-errors
    
    // When your code performs operations on values, your code should verify the values are valid 
    // first and panic if the values aren’t valid. This is mostly for safety reasons: 
    // attempting to operate on invalid data can expose your code to vulnerabilities. 
    // This is the main reason the standard library will call panic! if you attempt an out-of-bounds 
    // memory access: trying to access memory that doesn’t belong to the current data structure 
    // is a common security problem. 
    // Functions often have contracts: their behavior is only guaranteed if the inputs meet 
    // particular requirements. Panicking when the contract is violated makes sense because a contract 
    // violation always indicates a caller-side bug and it’s not a kind of error you want the calling 
    // code to have to explicitly handle. In fact, there’s no reasonable way for calling code to 
    // recover; the calling programmers need to fix the code. Contracts for a function, especially 
    // when a violation will cause a panic, should be explained in the API documentation for the function.
}


// Now we will re check how to write specifics and generics function's headers:
// - As you know, if you don't write anything in arguments path, the function don't allow
// arguments.

// - If you specify a variable name, you must specify after the type;           _x: String

// - You can specify a generic type (for anyone; can be string, integer, or anything) for use at
// the same type for many variables;                                            fn [NAME]<T>([NAME]: &[T]) -> &T
// So it indicate that; the function [NAME] have a generic type "T", then the parameters is the
// same generic type and it is returned when the function finished.

// - If you use it in structs, you must indicate after the name. For example;     struct [NAME]<T> { }
// Remember that if you specify one generic type, you can use as many arguments as you want but
// all must be the same type. If you can specify more than one generic type, you can do it
// separating it with "," .
// Maybe you already know, but if you didn't see you I explain you; Option, match and Result
// have generic types, so you can operate without matter the parameter's type:
//       enum Result<T, E> {
//           Ok(T),
//           Err(E),
//       }
// But what if we want implement a impl ? Easy, the same syntax:

struct Point<T> {
    x: T,
    y: T,
}

// If you specify some specific type as i32 is the same way, replacing T for specific type instead
// of generic.
impl<T> Point<T> {
        fn x(&self) -> &T {
           &self.x
        }
}

// As was mentioned before you can specify a lot of generics type separeting by ","
struct Point2<X,UYX>{
    x: X,
    y: UYX,
}

impl<X,UYX> Point2<X,UYX>{
    fn fnc1_0<Z,YUXZ>(self, other: Point2<X,UYX>) -> Point2<X,UYX> {
        Point2 {
            x: self.x,
            y: other.y,
        }
    }
}


fn generics<T>( _x: &T) -> Point<&T> {

// And as was specified in the header, the return must be T type.
let varx = Point{ x: _x, y: _x};
varx

}

// Recomended  lecture;
// https://doc.rust-lang.org/book/ch10-01-syntax.html#performance-of-code-using-generics

fn traits(){
    // A trait tells to Rust's compiler about functionality a particular type has and can share
    // with other types. 
    // Traits are very similar to a feature oftern called interfaces in other languajes. 
    // A type’s behavior consists of the methods we can call on unkown type (self). Different types 
    // share the same behavior if we can call the same methods on all of those types. 
    // Trait definitions are a way to group method signatures together to define a set of 
    // behaviors necessary to accomplish some purpose.
    
    // A trait can have multiple methods in its body: the method signatures are listed one per 
    // line and each line ends in a semicolon.

    // Syntax:
    // pub trait [NAME] {
    //      fn [FUNCTION](&self) -> [RETURN_TYPE];
    // }
    //
    // In another words; the propose of traits is execute functions when you passtrough values.
    // [variable].[trait]       or      [trait0].[trait]
    // So supose that your trait is "trait". When you pass the data of "variable" or "trait0" to
    // "trait" will execute specific methods that you indicated before.
    // As you probably already imaginae, is better if you put your Trait into a module.

    // You need take under consideration this;
    // - A trait is defined with the function will execute when you call it.
    // - It trait is then used in a implementation for a specific type (all methods must be
    // implemented in that specific type). So "self" will be considered as that type.
    //
    // But wait, you said "unkown type (self)" before. Yes, remember "different types share the
    // same behavior" so you use specifics methods of the Trait for specific types (you will not
    // use String method for a u32, right? I hope so).

    pub trait TraitPrime {
        fn prime_string(&self) -> std::str::Chars;
        fn prime_u32(&self) -> &u32;
        fn prime_i32(&self);
    } 
    // Here we need stop to clarify first: you can or not specifie the complete method. Before
    // I didn't, so when I implement that Trait in a specific type I need put the complete option:

    impl TraitPrime for String {
        // TraitPrime is implemeted for type "string". And then I speficie which function will use:
        fn prime_string(&self) -> std::str::Chars {
            self.chars()
        }
        // Remember all methods must be implemented into the type.
        fn prime_u32(&self) -> &u32 {
            let v: &u32 = &(0 + 1);
            // Remember, is refferenced with "&".
            v
        }

        fn prime_i32(&self) {
            println!("Done. Used; {}", self); 
        }
    }
    // If we specified the complete function when we created the trait is not necessary do it in
    // the implementation.

    let vtrait: std::str::Chars = TraitPrime::prime_string(&(String::from("hello world")));
    // As you can see above, we used the trait for "prime_string" method/function with "string" as
    // input (as is in; impl TraitPrime for String ) and we specified the type of "vtrait" variable which the return
    // specified when we declareted the method in "pub trait TraitPrime".
    // But what make special? Easy, as you specify the type implementation, you can use the same
    // Trait for as many types you want putting as many "impl TraitPrime for XXX" you considered.
    // The adventage of this is doesn't matter the "self" type, Rust will redirrect to the specific
    // implementation.
    // Of course you can use Traits in as many ways you want (be carefull that are used in right
    // way). This include the "self" variable, you can operate
    // Just remember; if you want do an implementation inside a Struct the type must be the
    // Struct's name.
    // One restriction to note with trait implementations is that we can implement a trait on a type 
    // only if either the trait or the type is local to our crate.
    // But we can’t implement external traits on external types.
    // This restriction is part of a property of programs called coherence, and more specifically
    // the orphan rule, so named because the parent type is not present. This rule ensures that
    // other people’s code can’t break your code and vice versa. Without the rule, two crates could
    // implement the same trait for the same type, and Rust wouldn’t know which implementation to
    // use.
    // And even something better; include more than one trait.

    pub fn notify2(item: &(impl TraitPrime + TraitPrime)) {
    }
    pub fn notify3<T: TraitPrime + TraitPrime>(item: &T) {
    }

    // When you specify one more trait for arguments, Rust will use the trait which correspond to
    // the argument's type.
    // In this case as specified use "T" generic type with traits "PartialOrd" and "copy" which
    // first is for u32 type and second for char type.
    // When we call "largest" we can use char or u32 types without matter the situation. Rust will
    // detect the argument's type and will detect if the type is the right for which implement it
    
    fn largest<T: PartialOrd + Copy>(list: &[T]) -> T {
        let mut largest = list[0];
        for &item in list {
            if item > largest {
                largest = item;
            }
        }
        largest
    }

    fn call_multi_trait() {
        let number_list = vec![34, 50, 25, 100, 65];
        let result = largest(&number_list);
        println!("The largest number is {}", result);

        let char_list = vec!['y', 'm', 'a', 'q'];
        let result = largest(&char_list);
        println!("The largest char is {}", result);
    }


    // Even we can return a trait.
    // Re see the function "largest", Do you see how the return is the generic type "T"?
    // As before we speciied.


}
