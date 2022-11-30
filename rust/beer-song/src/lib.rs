pub fn verse(n: u32) -> String {
    String::from(format!("{}\n{}\n", first(n), second(n)))
}

pub fn sing(start: u32, end: u32) -> String {
    let mut verses = vec![];
    for num in (end..=start).rev() {
        verses.push(verse(num));
    }
    verses.join("\n")
}

fn first(num: u32) -> String {
    match num {
        2..=99 => String::from(format!("{} bottles of beer on the wall, {} bottles of beer.", num, num)),
        1 => String::from("1 bottle of beer on the wall, 1 bottle of beer."),
        0 => String::from("No more bottles of beer on the wall, no more bottles of beer."),
        _ => panic!(),
    }
}

fn second(num: u32) -> String {
    match num {
        3..=99 => String::from(format!("Take one down and pass it around, {} bottles of beer on the wall.", num-1)),
        2 => String::from("Take one down and pass it around, 1 bottle of beer on the wall."),
        1 => String::from("Take it down and pass it around, no more bottles of beer on the wall."),
        0 => String::from("Go to the store and buy some more, 99 bottles of beer on the wall."),
        _ => panic!(),
    }
}
