use std::collections::HashSet;

pub fn soln(s: &String) -> i32 {
        if s.len() < 2 {
        return s.len() as i32;
    }
    let mut chars_record: HashSet<u8> = HashSet::new();
    let mut tail = 0;
    let (mut max_cnt, mut cnt) = (0, 0);
    let s_chars = s.as_bytes();
    for i in 0..(s.len()) {
        let c = s_chars[i];
        if chars_record.contains(&c) {
            if max_cnt < cnt {
                max_cnt = cnt;
            }
            while tail < i {
                let c_rm = s_chars[tail];
                chars_record.remove(&c_rm);
                cnt -= 1;
                tail += 1;
                if c_rm == c {
                    break;
                }
            }
        }
        cnt += 1;
        chars_record.insert(c);
    }
    if cnt > max_cnt {
        cnt
    } else {
        max_cnt
    }
} 