#[allow(dead_code)]
pub fn soln(x: i32) -> bool {
    if x < 0 {
        return false;
    }
    let mut digits: Vec<i32> = Vec::new();
    let mut cx = x;
    while cx > 0 {
        digits.push(cx % 10);
        cx = cx / 10;
    }
    let sz = digits.len();
    for i in 0..(sz / 2) {
        if digits[i] != digits[sz-i-1] {
            return false;
        }
    }
    true
}