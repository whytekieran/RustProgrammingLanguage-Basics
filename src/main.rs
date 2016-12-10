//THIS PROGRAM HAS NO PARTICULAR PURPOSE OTHER THAN TO SHOW THE BASICS OF THE RUST PROGRAMMING LANGUAGE
//THE RUST PROGRAMMING LANGUAGE IS A SYSTEMS PROGRAMMING LANGUAGE WHICH HAS SIMILARITIES WITH 
//THE C PROGRAMMING LANGUAGE. RUST IS MAINLY AN EXPRESSION BASED LANGUAGE AND HAS FEW STATEMENTS IN IT. 
//THE DIFFERENCE BEING THAT EXPRESSIONS RETURN A VALUE AND STATEMENTS TO DO

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

//READING INPUT AND SHOWING OUTPUT

//ARRAYS

//VECTORS

//STRUCTS

//USE OF CUSTOM FUNCTIONS CREATED IN FUNCTIONS SECTION BELOW THIS main() FUNCTION
let sum = add_numbers(num1, num3);
println!("Sum is {0}", sum);
print_sum(num1, num3);
let returned_num = return_number(10);
println!("Returned number is {0}", returned_num);

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