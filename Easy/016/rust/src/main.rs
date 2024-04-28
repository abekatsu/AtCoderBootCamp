use proconio::input;

struct User {
    id: usize,
    num: usize
}

fn main() {
    input! {
        n: usize,
        a: [usize; n]
    }

    let mut users = Vec::<User>::with_capacity(n);

    for (idx, el) in a.iter().enumerate() {
        users.push(User {
            id: idx + 1,
            num: *el
        });
    }

    let user_cmp = |l: &User, r: &User| l.num.cmp(&r.num);
    users.sort_by(user_cmp);
    println!("{}", users.iter().map(|(_, user)| user.id.to_string() ).collect::<Vec<_>>().join(" "));
}
