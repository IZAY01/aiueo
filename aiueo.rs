use std::env;


fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() == 1 {
        puts_full_table();
        return;
    }

    for query_str in &args[1..] {
        puts_query_item(query_str);
    }
}


fn puts_full_table() {
    for item in &TABLE {
        println!("{:?}", item);
    }    
}


fn puts_query_item(s: &String) {
    for i in 0..46 {
        if TABLE[i].contains(&s.as_str()) {
            if i < 5 {
                print!("{}{}  : ", KEY_FONT, s);
                println!("{}{}  {} {}", VALUE_FONT, TABLE[i][0], TABLE[i][1], TABLE[i][2]);
            } else {
                print!("{}{} : ",  KEY_FONT, s);
                println!("{}{} {} {}",  VALUE_FONT, TABLE[i][0], TABLE[i][1], TABLE[i][2]);
            }
            return;
        }
    }
}


const KEY_FONT: &str = "\x1b[31m";
const VALUE_FONT: &str = "\x1b[32m";
const RESET_LABEL: &str = "\x1b[0m";
const TABLE: [ [&str; 3]; 46] = 
[
    ["a" , "あ", "ア"], ["i" , "い", "イ"], ["u" , "う", "ウ"], ["e" , "え", "エ"], ["o" , "お", "オ"],
    ["ka", "か", "カ"], ["ki", "き", "キ"], ["ku", "く", "ク"], ["ke", "け", "ケ"], ["ko", "こ", "コ"],
    ["sa", "さ", "サ"], ["si", "し", "シ"], ["su", "す", "ス"], ["se", "せ", "セ"], ["so", "そ", "ソ"],
    ["ta", "た", "タ"], ["ti", "ち", "チ"], ["tu", "つ", "ツ"], ["te", "て", "テ"], ["to", "と", "ト"],
    ["na", "な", "ナ"], ["ni", "に", "ニ"], ["nu", "ぬ", "ヌ"], ["ne", "ね", "ネ"], ["no", "の", "ノ"],
    ["ha", "は", "ハ"], ["hi", "ひ", "ヒ"], ["hu", "ふ", "フ"], ["he", "へ", "ヘ"], ["ho", "ほ", "ホ"],
    ["ma", "ま", "マ"], ["mi", "み", "ミ"], ["mu", "む", "ム"], ["me", "め", "メ"], ["mo", "も", "モ"],
    ["ya", "や", "ヤ"],                     ["yu", "ゆ", "ユ"],                     ["yo", "よ", "ヨ"],
    ["ra", "ら", "ラ"], ["ri", "り", "リ"], ["ru", "る", "ル"], ["re", "れ", "レ"], ["ro", "ろ", "ロ"],
    ["wa", "わ", "ワ"],                                                             ["wo", "を", "ヲ"],
                                            ["n",  "ん", "ン"]
];
