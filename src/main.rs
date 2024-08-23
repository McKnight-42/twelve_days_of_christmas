fn main() {
    // create to array's of the days
    let days = [
        "first", "second", "third", "fourth", "fifth", "sixth", "seventh", "eighth", "ninth",
        "tenth", "eleventh", "twelfth",
    ];

    let gifts = [
        "a Partridge in a Pear Tree",
        "two Turtle Doves",
        "three French Hens",
        "four Calling Birds",
        "five Golden Rings",
        "six Geese a Laying",
        "seven Swans a Swimming",
        "eight Maids a Milking",
        "nine Ladies Dancing",
        "ten Lords a Leaping",
        "eleven Pipers Piping",
        "twelve Drummers Drumming",
    ];

    // Loop through the days from first to 12
    for day in 0..12 {
        // print that days gift
        println!(
            "On the {} day of Christmas, my true love gave to me:",
            days[day]
        );
        //  if day came from earlier set reverse loop and append and to statement
        for gift in (0..=day).rev() {
            if day > 0 && gift == 0 {
                print!("and ");
            }
            println!("{}", gifts[gift]);
        }
        println!();
    }
}
