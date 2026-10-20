fn main() {
    christmas();
}

fn christmas(){
    let order = ["first","second","third","fourth","fifth","sixth","seventh","eighth","ninth","tenth","eleventh","twelfth"];
    let gifts = ["A partridge in a pear tree","Two turtle doves","Three French hens","Four calling birds","Five golden rings","Six geese a-laying","Seven swans a-swimming","Eight maids a-milking","Nine ladies dancing","Ten lords a-leaping","Eleven pipers piping","Twelve drummers drumming"];

    for days in 0..12{
        let index = order[days];

        println!("On the {index} day of Christmas my true love gave to me");

        for i in (0..=days).rev(){
            let index2 = gifts[i];
            if i == 0 && days > 0{
                println!("And a partridge in a pear tree");
            }else {
                println!("{index2}");
            }
        }
        println!();
    }
}
