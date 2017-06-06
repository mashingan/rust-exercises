fn main() {

    let vowels = vec!['a', 'e', 'i', 'o', 'u'];
    let s = String::from("first");
    let fay = match s.chars().nth(0) {
        Some(c) => {
            //println!("the c is {}", c);
            isin(c, &vowels)
        },
        None => false,
    };
    //println!("fay is {}", fay);

    println!("{} tobe {}", s, fayed(&s, fay));
    let s = String::from("apple");
    let fay = isin(s.chars().nth(0).unwrap(), &vowels);
    println!("{} tobe {}", s, fayed(&s, fay));
    dofayed("saple");
}

fn isin(val: char, vec: &Vec<char>) -> bool {
    for c in vec {
        if val == *c {
            return false;
        }
    }
    true
}

fn fayed(thestring: &str, isfay: bool) -> String {
    let mut res = String::new();
    let fchar = thestring.chars().nth(0).unwrap_or_default();
    let mut pos: u32 = 0;
    for c in thestring.chars() {
        pos += 1;
        if isfay && pos == 1 {
            continue;
        }
        res.push(c);
    }
    if isfay {
        res.push('-');
        res.push(fchar);
        res.push_str("ay");
    } else {
        res.push_str("-hay");
    }
    res
}

fn dofayed(string: &str) {
    let vowels = vec!['a', 'e', 'i', 'o', 'u'];
    let fay = isin(string.chars().nth(0).unwrap(), &vowels);
    println!("{} tobe {}", string, fayed(string, fay));
}
