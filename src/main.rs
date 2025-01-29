fn main() {
    let x = 5;

    let x = x + 1;

    {
        let x = x * 2;
        println!("The value of x in the inner scope is : {x}");
    }

    println!("The value of x is : {x}");

    // shadowing help when we want to change types of the same data, For example say our program asks a user to show how many spaces they want between some text by inputting space characters, and then we want to store that input as a number

    let spaces = "   ";
    let spaces = spaces.len();
}
