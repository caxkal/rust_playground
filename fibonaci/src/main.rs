fn fibonaci_recursive( number: i32 ) -> i64
{
    if number == 0 || number == 1 
    {
        return 1;
    } else { 
        return fibonaci_recursive( number - 1 ) + fibonaci_recursive( number - 2 );
    }
}

fn fibonaci_loop( ) -> i64
{
    const num: usize = 119;
    let mut a = [1; num];
    let mut index: usize = 2;

    while index < num {
        let val1 = a[index-1];
        let val2 = a[index-2];
        a[index] = val1 + val2;
        index += 1;
    }

    return a[num - 1];
 }

fn fibonaci_optimised( num: usize ) -> i64
{
    let mut first = 1;
    let mut second = 1;

    let mut index: usize = 2;
    while index < num {
        let val1 = first;
        let val2 = second;
        second = val1 + val2;
        first = val2;
        index += 1;
    }

    return second;
 }

fn main() {
    let number = 10;
    let result = fibonaci_recursive(number);
    println!("Fibonaci for {} is {}", number, result);

    let num = 50;
    let result = fibonaci_optimised(num);
    println!("Fibonaci optimized for {} is {}", num, result);

    let result = fibonaci_loop();
    println!("Fibonaci loop for {} is {}", num, result);
}


