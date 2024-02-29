fn main() {
    for i in 0..25{
        println!("{}:\t I have {} bananas.", i, get_bananas(i));
    }

    for x in 0..=5{
        for y in 0..=5{
            let point = (x, y);
            let place = match point{
                (0, 0) => "origin",
                (_x, 0) => "x-axis",
                (0, _y) => "y-axis",
                _ => "plane",
            };
            println!("Point{:?} is at {:?}", point, place);
        }
    }

    for (pos, i) in (1..=10).enumerate(){
        println!("{0}x{0} = {1}", pos+1, i*i);
    }

    println!("\n\n\nSquares:\n=====================");
    get_squares(10000);
    println!("\n\n\nCubes:\n=====================");
    get_cubes(10000);



}



fn get_bananas(number:u32) -> String { //&'static str{
    match number {
        0 => "no".to_string(),
        _ if number % 6 == 0 => if number == 6 {format!("{:?} dozen of", number /6)} else {format!("{:?} dozens of", number /6)},
        1 | 2 => "one or two".to_string(),
        3..=7 => "a few".to_string(),
        _ if number % 2 == 0 => "even number of".to_string(),
        _ => "many".to_string(),
    }
}

// Implementing while loops

fn get_squares(limit:i32) {
    let mut num = 0;
    while num*num <= limit{
        println!("{0}x{0}\t= {1}", num, num*num);
        num = num + 1;
    }
}

// Implementing default loop

fn get_cubes(limit:i32){
    let mut num = 0;
    loop{
        println!("{0}x{0}x{0}\t= {1}", num, num*num*num);
        num = num + 1;
        if num*num*num > limit{
            break;
        }
    }
}