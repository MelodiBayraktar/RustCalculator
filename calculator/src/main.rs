use std::io::stdin;
fn main() {
    loop{
        let mut input_num1 : String = String::new();
        let mut input_num2 : String = String::new();
        let mut input_operation : String = String::new();

        println!("Please enter the first number");
        stdin().read_line(&mut input_num1).unwrap();
        let  num1 = input_num1.trim().parse::<f64>().unwrap();

        println!("Please enter the second number");
        stdin().read_line(&mut input_num2).unwrap();
        let num2 = input_num2.trim().parse::<f64>().unwrap();

        println!("Please select the number of operation you would like
        1- Add
        2- Subtract
        3- Multiply
        4-Divide
        
        5-Finish");

        stdin().read_line(&mut input_operation).unwrap();
        let operation = input_operation.trim();

        let operator ;

        match operation {
            "1" => {
                operator = Operation::Add(num1, num2);
            },
            "2" => {
                operator= Operation::Subtract(num1, num2);
            },
            "3" => {
                operator= Operation::Multiply(num1, num2);
            },
            "4" => {
                operator= Operation::Divide(num1, num2);
            },
            "5" => {
                break;
            },
            _ => {
                println!("You have entered a wrong input");
                continue;
            }

        }
        let result: f64 = Calculate(operator);
        println!("Your result is : {result}");

    }
}
enum Operation{
    Add(f64,f64),
    Subtract(f64,f64),
    Multiply(f64,f64),
    Divide(f64,f64),
}
fn Calculate(operation : Operation)->f64{
    match operation {
        Operation::Add(number1, number2) => {
           return number1 + number2;
        }
        Operation::Subtract(number1, number2) => {
            return number1 - number2;
        }
        Operation::Multiply(number1, number2) => {
            return number1 * number2;
        }
        Operation::Divide(number1, number2) => {
            return number1 / number2;
        }
    }
}
