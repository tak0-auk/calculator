use std::io;

const OB: &'static str = "(";
const CB: &'static str = ")";
const PLUS: &'static str = "+";
const MINUS: &'static str = "-";
const AS: &'static str = "*";
const SR: &'static str = "/";

fn main() {

    let mut input = String::new();

    let _ = io::stdin().read_line(&mut input);
    let chars: Vec<char> = input.replace(" ", "").trim().chars().collect();
    let mut token:Vec<String> = Vec::new();
    let mut num: Vec<i64> = Vec::new();
    let mut number: String = String::new();
    let mut ob_cnt = 0;
    let mut cb_cnt = 0;
    for s in chars {
        match s.to_string().as_ref() {
            OB | CB | PLUS | MINUS | AS| SR => {
                if number.len() > 0 {
                token.push(number.clone());
                number.clear();
                }
                token.push(s.to_string());
                },
            _ => concat_str(&mut number, s.to_string().as_ref())
        };
    }

    token.push(number.clone());

    // let i = &number.parse::<i64>().unwrap().clone();

    if ob_cnt != cb_cnt {
        panic!("not ");
    }

    let mut res = 0;
    let mut left = 0;
    let mut right = 0;
    for t in token {
        println!("{}", t);
        match t.as_str() {
            OB => {},
            PLUS => {},
            _ => {
                //  t.parse::<i64>().unwrap();
            },
        }
    }
    for i in num {
        res += i;
        println!("i = {} res = {}", i, res)
    }
    print!("{}", add(10, -5));



}
// fn push_p(pushed: &mut Vec<i64>, p: i64) {
//     pushed.push(p);
// }

fn add(left: i64, right: i64) -> i64 {
    left + right
}

fn concat_str(left: &mut String, right: &str) {
    left.push_str(right);
}