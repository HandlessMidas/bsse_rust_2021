#[derive(Debug, Copy, Clone, PartialEq)]
enum Comparison {
    Equal, // список `a` равен списку `b`
    Sublist, // список `a` является подсписком `b`
    Superlist, // список `b` является подсписком `a`
    Other, // списки не равны и не являются подсписками друг друга
}

fn prefix_function<T: PartialEq>(a: &[T]) -> Vec<usize> {
    let mut p = Vec::new();
    p.resize(a.len(), 0);
    for i in 1..a.len() {
        let mut cur = p[i - 1];
        while cur > 0 && a[cur] != a[i] {
            cur = p[cur - 1];
        }
        if a[cur] == a[i] {
            cur += 1;
        }
        p[i] = cur;
    }
    p
}

fn is_sublist<T: PartialEq>(a: &[T], b: &[T]) -> bool {
    if a.len() > b.len() {
        return false;
    }
    let p = prefix_function(a);
    let mut i: usize = 0;
    for x in b {
        while i > 0 && *x != a[i] {
            i = p[i - 1];
        }
        if *x == a[i] {
            i += 1;
        }
        if i == a.len() {
            return true
        }
    }
    false
}

fn compare<T: PartialEq>(a: &[T], b: &[T]) -> Comparison {
    match (is_sublist(a, b), is_sublist(b, a)) {
       (true, true) => Comparison::Equal,
       (true, false) => Comparison::Sublist,
       (false, true) => Comparison::Superlist,
       (false, false) => Comparison::Other,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_data() -> Vec<(Vec<u32>, Vec<u32>)> {
        let mut data = Vec::new();
        data.push((vec![1, 2, 3], vec![1, 2, 3]));
        data.push((vec![1, 2, 3, 4], vec![1, 2, 3]));
        data.push((vec![4, 1, 2, 3], vec![1, 2, 3]));
        data.push((vec![5, 6, 7], vec![1, 2, 3, 5, 6, 7, 8]));
        data.push((vec![5, 6, 7], vec![1, 2, 3]));
        data.push((vec![1, 2, 4, 3], vec![1, 2, 3]));
        data
    }

    #[test]
    fn test_transform() {
        let data = test_data();

        let res: Vec<Comparison> = test_data().iter().map(|(a, b)| { compare(a, b) }).collect();

        assert_eq!(res[0], Comparison::Equal);
        assert_eq!(res[1], Comparison::Superlist);
        assert_eq!(res[2], Comparison::Superlist);
        assert_eq!(res[3], Comparison::Sublist);
        assert_eq!(res[4], Comparison::Other);
        assert_eq!(res[5], Comparison::Other);
    }
} 
