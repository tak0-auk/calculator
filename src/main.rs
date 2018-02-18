use std::io;

const OB: &'static str = "(";
const CB: &'static str = ")";
const PLUS: &'static str = "+";

fn main() {

    let mut input = String::new();

    let _ = io::stdin().read_line(&mut input);
    let chars: Vec<char> = input.replace(" ", "").trim().chars().collect();
    let mut token:Vec<String> = Vec::new();
    let mut num: Vec<i64> = Vec::new();

    let mut number: String = String::new();
    let mut flg = false;
    let mut ob_cnt = 0;
    let mut cb_cnt = 0;
    for s in chars {
        match s.to_string().as_ref() {
            OB => ob_cnt += 1,
            CB => cb_cnt += 1,
            PLUS => {
                let i = &number.parse::<i64>().unwrap();
                num.push(i.clone());
                number.clear();
                token.push(s.to_string());
                },
            _ => concat_str(&mut number, s.to_string().as_ref())
        };
    }

    let i = &number.parse::<i64>().unwrap().clone();
    num.push(i.clone());

    if ob_cnt != cb_cnt {
        panic!("not ");
    }

    let mut res = 0;
    for t in token {
        println!("{}", t);
    }
    for i in num {
        res += i;
        println!("i = {} res = {}", i, res)
    }
    print!("{}", res);



}
fn push_p(pushed: &mut Vec<i64>, p: i64) {
    pushed.push(p);
}

fn concat_str(left: &mut String, right: &str) {
    left.push_str(right);
}