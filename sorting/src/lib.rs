mod b_rand;

use std::fmt::Debug;

// O(n^2)
pub fn bubble_sort<T: PartialOrd + Debug>(v: &mut [T]) {
    for i in 0..v.len() {
        let mut sorted = true;
        for j in 0..v.len() - i - 1 {
            if v[j] > v[j + 1] {
                v.swap(j, j + 1);
                sorted = false;
            }
        }
        println!("{:?}", v);
        if sorted {
            return;
        }
    }
}

// sort the left half + sort the right half: O(n*log(n))
// merge the sorted halves together: O(n)
// totally: O(n*log(n))
pub fn merge_sort<T: PartialOrd + Debug>(mut v: Vec<T>) -> Vec<T> {
    println!("MS:{:?}", v);

    if v.len() <= 1 {
        return v;
    }

    let mut res = Vec::with_capacity(v.len());

    // sort
    let b = v.split_off(v.len() >> 1);

    let a = merge_sort(v);
    let b = merge_sort(b);

    // merge
    let mut a_it = a.into_iter();
    let mut b_it = b.into_iter();
    let mut a_peak = a_it.next();
    let mut b_peak = b_it.next();


    loop {
        match a_peak {
            Some(ref a_val) => match b_peak {
                Some(ref b_val) => {
                    if b_val < a_val {
                        res.push(b_peak.take().unwrap());
                        b_peak = b_it.next();
                    } else {
                        res.push(a_peak.take().unwrap());
                        a_peak = a_it.next();
                    }
                }
                None => {
                    res.push(a_peak.take().unwrap());
                    res.extend(a_it);
                    return res;
                }
            }
            None => {
                if let Some(b_val) = b_peak {
                    res.push(b_val);
                }
                res.extend(b_it);
                return res;
            }
        }
    }
}

pub fn pivot<T: PartialOrd>(v: &mut [T]) -> usize {
    let mut p = b_rand::rand(v.len());
    v.swap(p, 0);
    p = 0;
    for i in 1..v.len() {
        if v[i] < v[p] {
            v.swap(p + 1, i);
            v.swap(p, p + 1);
            p += 1;
        }
    }
    p
}

pub fn quick_sort<T: PartialOrd + Debug>(v: &mut [T]) {
    if v.len() <= 1 {
        return;
    }
    let p = pivot(v);
    print!("{:?}", v);

    let (a, b) = v.split_at_mut(p);
    quick_sort(a);
    quick_sort(&mut b[1..]);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bubble_sort() {
        let mut v = vec![4, 6, 1, 8, 11, 13, 3];
        bubble_sort(&mut v);
        assert_eq!(v, vec![1, 3, 4, 6, 8, 11, 13]);
    }

    #[test]
    fn test_merge_sort() {
        let v = vec![4, 6, 1, 8, 11, 13, 3];
        let v = merge_sort(v);
        assert_eq!(v, vec![1, 3, 4, 6, 8, 11, 13]);
    }

    #[test]
    fn test_quick_sort() {
        let mut v = vec![4, 6, 1, 8, 11, 13, 3];
        quick_sort(&mut v);
        assert_eq!(v, vec![1, 3, 4, 6, 8, 11, 13]);
    }
}
