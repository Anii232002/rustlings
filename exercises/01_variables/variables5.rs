fn main() {
    let number = "T-H-R-E-E"; // Don't change this line
    println!("Spell a number: {number}");

    // TODO: Fix the compiler error by changing the line below without renaming the variable.
    let number = 3;
    //variable shadowing by agin saying let number = 3 , again introduces a new variable
    println!("Number plus two is: {}", number + 2);
}
