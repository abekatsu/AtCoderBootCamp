use proconio::input;

fn pass_qualify(
    passed_student: usize,
    international_student_position: usize,
    a: usize,
    b: usize,
    ch: char,
) -> bool {
    // remark idx begins with 0
    let retvalue: bool = match ch {
        'a' => {
            // 国内の学生は、現在予選の通過が確定した参加者がA+B人に満たなければ、
            // 予選を通過する
            if passed_student < (a + b) {
                true
            } else {
                false
            }
        }
        'b' => {
            // 国内の学生は、現在予選の通過が確定した参加者がA+B人に満たなければ、
            // 予選を通過する
            if passed_student < (a + b) && international_student_position < b {
                true
            } else {
                false
            }
        }
        'c' => false,
        _ => false,
    };

    return retvalue;
}

fn main() {
    input! {
        _n: usize,
        a: usize,
        b: usize,
        s: String,
    }
    let mut passed_student: usize = 0;
    let mut international_student_position: usize = 0;

    for ch in s.chars() {
        let result: bool = pass_qualify(passed_student, international_student_position, a, b, ch);
        if result {
            passed_student += 1;
            if ch == 'b' {
                international_student_position += 1;
            }
        }
        println!("{}", if result { "Yes" } else { "No" });
    }
}
