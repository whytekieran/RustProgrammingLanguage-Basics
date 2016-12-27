//THIS PROGRAM HAS NO PARTICULAR PURPOSE OTHER THAN TO SHOW THE BASICS OF THE RUST PROGRAMMING LANGUAGE
//THE RUST PROGRAMMING LANGUAGE IS A SYSTEMS PROGRAMMING LANGUAGE WHICH HAS SIMILARITIES WITH 
//THE C PROGRAMMING LANGUAGE. RUST IS MAINLY AN EXPRESSION BASED LANGUAGE AND HAS FEW STATEMENTS IN IT. 
//THE DIFFERENCE BEING THAT EXPRESSIONS RETURN A VALUE AND STATEMENTS TO DO. THIS PROGRAM SHOWS IVE COVERED THE BASICS OF THE LANGUAGE

//use std::net::TcpStream;

//To perform io operations we need the io module from the std (standard) library.
use std::io;
//use std::io::prelude::*;	//Needed for example two of reading user input

fn main() {
    
//VARIABLES - Section about basic variable types and declarations in the Rust programming language
//Variables are declared using the 'let' keyword. If this keyword is not used it is an assignment not a declaration.
//In the Rust programming language you MUST assign a value to your variable when declaring it. 
let num1 = 5;			//Integer
let num2 = 10.5;		//Float
let result = false;		//Boolean
let name = "Kieran";	//String
let letter = 'a';		//Char

//Variables can also be declared like so:
//This is called pattern matching and is quite useful in rust
let (x, y) = (1, 2); //Here x = 1 and y = 2
println!("X = {0} Y= {1}", x, y);

//If you wish it is possible to also assign the data type to the variable like so
//The Rust documentation has more on primitive types: https://doc.rust-lang.org/std/
let num3: i32 = 34; 			//Int32
let result2: bool = true;		//Boolean	

//When we create variables like above they are automatically created as constants, hence are immutable (cant be changed).
//so reassigning a variable above like as follows: num1 = 100; Is not possible. 
//To create a variable which we can reassign we use the 'mut' keyword
let mut num4 = 777; //This variable can be reassigned. 

//Outputting the values
println!("Here are the variables, num1 is {0}, num2 is {1}, result is {2}, name is {3} and letter is {4}", num1, num2, result, name, letter);

//Outputting variables with assigned types to show they work perfectly fine as well
println!("Here are the variables declared with specified type, num3 is {0} and result2 is {1}", num3, result2);

//We must use the muttable variable once before it is changed. Otherwise there was no point assigning it this value in the first
//place. We will also get a warning message saying "not read" if we do not use the variable before we change it.
println!("The mutable variable num4 has a value of {0}", num4); 

//Changing value then outputting the new value
num4 = 776;
println!("The value of num4 has been changed and is now {0}", num4);

//Variables in Rust have scope just like many other programming languages, for example:
let num5: i32 = 17;
    {
        let num6: i32 = 3;
        println!("The value of num5 is {} and value of num6 is {}", num5, num6);//This will work because both are visible
    }
//println!("The value of x is {} and value of y is {}", num5, num6); // This won't work, num6 is not visible from the inner block.

//SELECTION - Any programming language needs a way of making selections and Rust if no different. Like many other languages this is
//accomplished by the use of if/else statements. Here are some examples.
let num6 = 5;

if num6 == 5 {
    println!("num6 is five!");
}

if num6 == 5 {
    println!("num6 is five!");
} else if x == 6 {
    println!("num6 is six!");
} else {
    println!("num6 is not five or six :(");
}

//We can also do this
let num7 = if num6 == 5 { 10 } else { 15 }; // y: i32 - if num6 = 5 then num7 = 10 else num7 = 15
println!("num7 is {0}", num7);

//LOOPS - Programming languages need a way of iterating of data, this is done using a concept called a loop. The Rust programming
//language is the same and has different kinds of loops. Some are similar to other languages, others are not.

//The simplest type of loop which can be used in Rust is the infinite loop. This will just loop forever unless stopped by using
//the 'break' keyword. The 'continue' keyword also exists. This type of loop can be useful in certain circumstances.
loop {
    println!(" Would Loop forever! ....but going to stop it now with a break");

    if true{
    	break;
    }
}

//Like many languages, Rust also has a while loop, which loops like this:
let mut counter = 0;
while counter != 5 {
	println!("Counter will stop before 5. Counter is now {0}", counter);
	counter += 1; //Unlike other languages, there is no ++ or -- operation. We increment using the method shown here.
} 

//The Rust programming language also has a for loop. Although the for loop in rust tends to look different from languages like Java,
//C# and C. Below is an example, x will start at 0 and loop until we reach 9....not 10!
for x in 0..10 {
    println!("{0}", x); 
}

//Can also count how many times you loop using the enumerate() function. Below is an example which tracks the index of the current
//iteration and the current value we are iterating over.
for (index, value) in (5..10).enumerate() {
    println!("index = {0} and value = {1}", index, value);
}

//ARRAYS - The Rust programming language has arrays, they are just declared slightly differently. Arrays size is known at compile time.
//Here are some basic examples of array declarations and use of them.

// Fixed-size array which can be changed (mut) and has five elements of type i32 (int32)
let mut array1: [i32; 5] = [1, 2, 3, 4, 5];
let array3 = [1,2,3,4,5,6,7,8];	//Can also do this way but i prefer to be specific and give the type.
println!("Element 5 in array3 is {0}", array3[4]);
let element3 = array1[2];							//Accessing an element from the array
println!("Value of element3 in array1 is {0}", element3);		//Printing it out

array1[0] = 33;										//Assign new value to element one, can only do this because array has 'mut'
println!("Value of element1 array1 is {0}", array1[0]);	//Printing it out
println!("The length of array1 is {0}", array1.len());	//Printing out the length of an array

// All elements can be initialized to the same value. Here we have an array of 10 elements all of type i32 and initialized to 0.
let mut array2: [i32; 10] = [0; 10];

//Looping and giving each index in the array a value using a while loop.
let mut count = 0;
let mut values = 1;
while count != 10 {
	array2[count] = values;

	values += 1;
	count += 1;
}

println!("Value of index 4 in array2 is {0}", array2[4]); //printing out one of the assigned values

//Then accessing each value with a for loop. Notice we use the syntax '&array2' in the loop. This means we are getting a reference
//to array2 (borrowinG). Leaving out the '&' symbol means we are taking ownership of array2 and we would get errors. We can also 
//do the following 'for value in &mut array2'. This a mutable reference to array2.
let mut index = 0;
//array2 itself must be a mutable reference to change its elements in the loop. If we simply referenced array2 like '&array2' we 
//would not be able to change the value as it would be immutable
for value in &mut array2 {	
	println!("The value of index {0} in array2 is {1}", index, value);
	*value = 6;	//adding the * symbol makes value a muttable reference to an element of array2.
	index += 1;
}

//Best way allows index and element
for (i, elem) in array2.iter_mut().enumerate() {

	//i is of usize because it refers to array index and all index are usize. can use 'as i32' to convert it. Also 'elem' needs to
	//be a mutable reference. Adding the * symbol allows this. The iter_mut() function belongs to the slice module and the enumerate()
	//function belongs to the iter module.
	*elem = i as i32; 
    println!("Index is {0} and element is {1}", i, elem); 
}

//2D ARRAYS - Rust has 2D arrays. Here are some examples of how we would create, iterate and access them.
let mut darray1 = [[0i32; 4]; 5];		//We can decalre types like: 0i32 which is like saying elements are '0' and i32 type.
let mut darray2 = [[0i32; 4]; 5];		//The type isnt nessesary though, here we declare the exact same 2D array without it.	

darray1[1][1] = 6;						//Assigning a vlue to index of 2D array
darray2[0][3] = 3;						//Just assigning some values to the other 2D array before looping over it.
darray2[2][2] = 7;
darray2[2][1] = 31;
//darray2 = acceptdarray(&mut darray2);		//Functions that accepts and returns a 2D array. Defined below main() changes index 0,0
println!("Element 0,0 after function {:?}", darray2[0][0]);		//Output the new value
println!("2d {:?}", darray1[1][1]);		//Outputting the value of that index.

//Passing reference to a 2D array.
//let darray_ref: &[[i32; 4]] = &darray1;  //You can the reference to the array like whats shown here
//println!(" referenced array element 0,1 is {:?}", darray_ref[0][1]);

//Looping over a 2D array
for row in darray2.iter(){
	for &col in row.iter(){

		println!("No joy this isnt the value 7!!");
		if col == 7{
			println!("Found a value of 7!!");
		}
	}
}

//SLICES - Like in the Go programming language slices also exist in Rust. They are useful for certain tasks. One of those being that
//we can pass a part of an array into a slice. Slices are efficient as they do not make a copy of the array section assigned to it.
//Instead they reference that part of the array. You must use iter() to loop over slices

// Arrays can be automatically borrowed as slices
println!("Borrowing the whole array as a slice");
let slice1 = analyze_slice(&array1);
println!("Element 1 of Slice 1 has the value {}", slice1[0]);

// Slices can point to a section of an array
println!("Borrowing a section of the array as a slice"); //Index 1-4 including 1 but not 4.
let slice2 = analyze_slice(&array2[1 .. 4]);	//The [..] is slicing sytax and creates a view into the section of the array we want
println!("Element 2 of Slice 2 has the value {}", slice2[1]);

//VECTORS - Vectors are very similar to arrays the difference being that they are dynamic or growable in size. This makes them
//similar to Lists which you see in other languages like Java. Vectors always allocate their data on the heap. For addition examples
//view the documentation https://doc.rust-lang.org/std/vec/struct.Vec.html where you can find different Vector functions. Like slices
//you must user iter() to loop over vectors
//Vectors are LIFO, You can create them with the vec! macro like so:
let mut vector1 = vec![1, 2, 3, 4, 5]; 
//or we can declare the type of the vector during the declaration like so, both will work fine.:
let mut vector2: Vec<i32> = vec![1, 2, 3, 4, 5, 6, 7, 8];
//Adding a new value to a vector
vector1.push(7);
vector2.push(9);
println!("Vector1 is {:?}", vector1);
println!("Vector2 is {:?}", vector2);
println!("Vector1 length is {0}", vector1.len());
println!("Element 6 in vector2 is {0}", vector2[5]);
//Get and remove the last element from the vector. This is 9 and will be returned like 'Some(9)'. Some belongs to the enum type 
//Option. Option has to values, Some(someValue) and None(noValue). 
println!("Popping last element inserted into vector2. That element is {:?}", vector2.pop());
//Quick example of using the Option enum type and an associated function, more info here:https://doc.rust-lang.org/std/option/enum.Option.html
let x: Option<u32> = Some(2);	//Variable x of type Option<UnsignedInt> that has 'Some' value. (which is two)
assert_eq!(x.is_some(), true);	//Check if x is a value...which it is. So this is true. If not program would terminate

//TUPLES - In Rust programming a Tuple is like an array. The difference being that a Tuple can hold multiple data types. Tuples are
//useful in certain circumstances, for example we may want to return more than one thing from a function. 
let tuple1 = (1, "hello", 4.5, true);							//Creating a basic tuple
let (a,b,c,d) = tuple1; //We can destructure a tuple and access al variables in it in a single expression like so: 
println!("a: {:?}", a);	//Printing out the values
println!("b: {:?}", b);
println!("c: {:?}", c);
println!("d: {:?}", d);


println!("The second value in tuple1 is: {0}", tuple1.1);		//Printing out one of its values, tuple indexing
// Tuples can also be tuple members
let tuple_of_tuples = ((1, 2, 3), (4, -1), -2);
let inner_tuple = tuple_of_tuples.1;			//Then assign one of the inner tuples to another
println!("Inner tuple element2 is: {0}", inner_tuple.1);			//and index the the inner tuple

//Here we create a vector of tuples containing two elements and iterate over the vector Vector declaration Vec
let mut v = Vec::<(i32, f32)>::with_capacity(2);
v.push((1, 2.5));
v.push((2, 7.0));
for &(a, b) in v.iter() {
    println!("a: {}  b: {}", a, b);
}

//Here we have an example of using a function the accepts and also returns a tuple. Allowing us to return more than one thing from
//a function. The function simply reverse the elements of the tuple
let pair = (1, true);
println!("pair is {:?}", pair);
println!("the reversed pair is {:?}", reverse(pair));

//STRUCTS - There are three types of struct in Rust. Tuples are one of them. We can have something called a 'unit struct' and lastly
//we have the same type of struct that we use in the C programming language.

// A unit struct is fieldless and are useful when used in generics
struct Nil;

// A tuple struct
struct Pair(i32, f32);

// A struct with two fields, this is the type of struct we use in the C language
struct Point {
    x: f32,
    y: f32,
}

// Structs can be reused as fields of another struct (nested structs)
#[allow(dead_code)]
struct Rectangle {
    p1: Point,
    p2: Point,
}

// Instantiate a unit struct
let _nil = Nil;

// Instantiate a tuple struct
let pair = Pair(1, 0.1);
println!("{:?}", pair.0);	//Access a value

// Instantiate a `Point` struct
let point = Point { x: 0.3, y: 0.4 };
// Access the fields of the point
println!("point coordinates: ({}, {})", point.x, point.y);

// Destructure the point using a `let` binding
//let Point { x: my_x, y: my_y } = point;

//Instantiate the rectangle struct
let _rectangle = Rectangle {
        p1: Point { x: 0.5, y: 0.8 },
        p2: Point { x: 0.2, y: 0.7}
};
println!("Rectangle P2, Xcord is: {:?}", _rectangle.p2.x);//Accessing value of nested struct

//STRINGS IN RUST - Rust has two main types of strings: &str and String. 
//First lets mention &str. These are called ‘string slices’. A string slice has a fixed size, and cannot be mutated.
//"Hello there." is a string literal and its type is &'static str. A string literal is a string slice that is 
//statically allocated, meaning that it’s saved inside our compiled program, and exists for the entire duration it runs
let greeting = "Hello there."; // greeting: &'static str
println!("{:?}", greeting);

//String literals can span multiple lines, this will be printed with the \n as part of it
let s1 = "foo
    bar";
println!("{:?}", s1);

//Adding a / will trim the spaces and newlines
let s2 = "foo\
    bar";
println!("{:?}", s2);

//Next we have the type String which is growable and can be mutated. Strings are commonly created by converting 
//from a string slice using the to_string() method shown below. 
let mut s3 = "Hello".to_string(); // mut s: String
println!("{}", s3);

//Now we can change the string, here we add additional text to it.
s3.push_str(", world.");
println!("{}", s3);

//Strings will coerce into &str with an &:
let s4 = "Hello".to_string();
takes_slice(&s4);	//This method accepts an &str type. By adding '&' to our string type we convert it to &str.

//This coercion does not happen for functions that accept one of &str’s traits instead of &str. For example, 
//TcpStream::connect has a parameter of type ToSocketAddrs. A &str is okay but a String must be explicitly converted using &*
//Here is an example:
//TcpStream::connect("192.168.0.1:3000"); // &str parameter
//let addr_string = "192.168.0.1:3000".to_string();
//TcpStream::connect(&*addr_string); // convert addr_string to &str

//We can loop over the charaters or bytes of a string like so:
for letter in s3.as_bytes() {
    print!("{}, ", letter);
}

for letter in s3.chars() {
    print!("{}, ", letter);
}

//OWNERSHIP

//REFERENCES AND BORROWING

//MACROS

//TRAITS

//MATCH

//USE OF CUSTOM FUNCTIONS CREATED IN FUNCTIONS SECTION BELOW THIS main() FUNCTION
let sum = add_numbers(num1, num3);
println!("Sum is {0}", sum);
print_sum(num1, num3);
let returned_num = return_number(10);
println!("Returned number is {0}", returned_num);

//READING INPUT AND SHOWING OUTPUT - Reading user input whether its from a GUI or simply from the console is important to have index
//any programming language and Rust is no different. There are different ways we can read user input. This section will show some
//of these different ways. First we need to import the io module from the standard library with the 'use' keyword. This is shown
//above the main() method. Here are some examples of reading user input.

//Example 1 - Asks the user to enter there name then prints back a hello message
let mut input1 = String::new();

println!("Please enter your name: ");
io::stdin().read_line(&mut input1)
    .ok()
    .expect("Couldn't read line");    

println!("hello {}", input1);


//Example 2 - Simply keeps accepting user input in an infinite loop, could be modified for other purposes.
/*let stdin = io::stdin();
    
for line in stdin.lock().lines() {
    println!("{}", line.unwrap());
}*/

//Example 3 - Gets user input then checks if input is an integer or not. Could be applied to check for other data types not just int.
//std::io::stdin() returns BufferedReader<StdReader>, and BufferedReader implements Buffer trait. This means that you can 
//call read_line() method on it:

println!("Please enter an integer: ");
let mut input_text = String::new();
io::stdin()
    .read_line(&mut input_text)
    .expect("failed to read from stdin");

let trimmed = input_text.trim();
match trimmed.parse::<u32>() {
    Ok(i) => println!("Your integer input is: {0}", i),
    Err(..) => println!("This was not an integer: {0}", trimmed)
};


//FORMATTING OUTPUT

}//End main() function

//FUNCTIONS - Section about writing functions using the Rust programming language
//Functions are declared using the 'fn' keyword in Rust. Unlike variables, when we create a function that accepts an argument
//we must declare what type of value that is. Here are some examples.

//This function adds two numbers and returns a value. We outline the return type as an int32 by using the syntax -> i32
//The last line of a function always determines what is returned. This line is NOT ended with a semi-colon as can be seen below.
fn add_numbers(x: i32, y: i32) -> i32{
	x + y
}

//We do not have to return a value if we dont want to. The function below does the same thing as the previous one except
//we simply print out the sum instead of returning it. In this case we dont need to define a return type and the last line
//of the function does end with a semi-colon.
fn print_sum(x: i32, y: i32) {
    println!("Sum is: {}", x + y);
}

//It is possible to return a value before the last line of a function. This is done using the 'returns' keyword. 
fn return_number(x: i32) -> i32 {
    return x;

    // we never run this code as it would never be reached
    //x + 1
}

//You can use the 'return' keyword on the last line of a function, like in Java. However in Rust this is considered bad practice

//There is another type of function in Rust called a 'diverging function'. Esentially this is a function that never will return.
//In the example below the syntax shows we write the return type like -> !. This shows that we have a diverging function. The
//panic!() keyword will cause the current thread of execution to crash with the specified message, hence when we get to this
//line the program crashes and this function never actually returns. Useful in this type of circumstance.
/*fn never_returns() -> !{
	panic!("The program has crashed");
}*/

//Its also possible to allow a variable to point to a function and then use that variable in the same way. Here is an example:
/*fn plus_one(i: i32) -> i32 {
    i + 1
}*/

//variable f now points to the function plus_one(). This type of variable declaration also declares its type as a function that 
//accepts and returns an i32
//let f: fn(i32) -> i32 = plus_one;

//here we dont declare the type but works just fine
//let f = plus_one;

//We can then use the variable f to call the function like so:
//let six = f(5);

//Function used in the slice section
fn analyze_slice(slice: &[i32]) -> &[i32] {
    println!("first element of the slice: {}", slice[0]);
    println!("the slice has {} elements", slice.len());
    slice 			//Return the newly created slice.
}

//Function used in the Tuple section
// Tuples can be used as function arguments and as return values
fn reverse(pair: (i32, bool)) -> (bool, i32) {
    // `let` can be used to bind the members of a tuple to variables
    let (integer, boolean) = pair;

    (boolean, integer)
}

//String to &str
fn takes_slice(slice: &str) {
    println!("Got: {}", slice);
}

//Accepts a 2D array with length of 4. Gives an index a value and returns the array
fn acceptdarray(x: &mut [[i32; 4]]) -> &[[i32; 4]] {  
    x[0][0] = 42;
    x
}