//depencies: rand
use std::io;

fn main() {
    let rand_numb = rand::random_range(1..100);
    let mut attemps: i32 = 0;
    println!("Я загадал число, попробуй угадать его");
    loop {
        let mut text = String::new();
        io::stdin().read_line(&mut text).unwrap();
        let numb: i32 = text.trim().parse().unwrap();
        if numb == rand_numb {
            println!("Вы угадали число за {} попыток", attemps);
            break;
        } else if numb > rand_numb {
            println!("Загаданное число меньше");
            attemps += 1;
        } else {
            println!("Загаданное число больше");
            attemps += 1;
        }
    }
}
