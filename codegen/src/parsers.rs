use std::str;
    
pub fn shift_while<F>(i: &[u8], f: F) -> &[u8] where F: Fn(u8) -> bool {
    let mut ctr = 0;
    for c in i {
        if f(*c) {
            ctr += 1;
        }
        else {
            break;
        }
    }

    &i[ctr..]
}

pub fn take_while1<F>(i: &[u8], f: F) -> (&[u8], &str) where F: Fn(u8) -> bool {
    let mut ctr = 0;

    for c in i {
        if f(*c) || ctr == 0 {
            ctr += 1;
        }
        else {
            break;
        }
    }

    (&i[ctr..], str::from_utf8(&i[0..ctr]).unwrap())
}

pub fn take_first<F>(i: &[u8], f: F) -> (&[u8], u8) where F: Fn(u8) -> bool {
    let size = i.len();

    let mut ctr = 0;

    for c in i {
        if f(*c) || ctr == size - 1 {
            break;
        }
        else {
            ctr += 1;
        }
    }

    (&i[ctr..], i[ctr])
}

pub fn shift(i: &[u8], c: usize) -> &[u8] {
    match c {
        c if c >= i.len() => &[],
        _ => &i[c..]
    }
}