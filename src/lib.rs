// fn suffix_sarray(sarr: Vec<u8>) ->Vec<usize>{
//
// }
//
use std::cmp::Ordering;

fn idx(arr: &[usize], i: usize) -> usize {
    if i < arr.len() {
        arr[i]
    } else if i >= arr.len() && i < arr.len() + 2 {
        0
    } else {
        panic!(format!("bad access len {} i {}", arr.len(), i))
    }
}

fn sort_by(arr: &[usize], sarr: &mut [usize], offset: usize) {
    let mut sorted = vec![0usize; sarr.len()];
    let mut count = [0usize; 256];
    for e in sarr.iter() {
        count[idx(arr, *e + offset)] += 1;
    }
    for i in 1..count.len() {
        count[i] = count[i - 1] + count[i];
    }
    for v in sarr.iter().rev() {
        let idx = &mut count[idx(arr, *v + offset)];
        *idx -= 1;
        sorted[*idx as usize] = *v;
    }
    sarr.copy_from_slice(&sorted[..]);
}

fn radix_sort_first_3(arr: &[usize]) -> Vec<usize> {
    let mut sa: Vec<usize> = (0..arr.len()).collect();

    for offset in (0..3).rev() {
        sort_by(arr, &mut sa[..], offset);
    }
    sa
}

fn assign_lexicographer(arr: &[usize], sa: &[usize]) -> Vec<usize> {
    let mut rank = 0usize; // map each value to rank
    let gettuple = |i| (idx(arr, i), idx(arr, i + 1), idx(arr, i + 2));
    let mut ranks = vec![0usize; sa.len()];
    let mut lastvisited = gettuple(sa[0]);
    for i in 0..sa.len() {
        let tmp = gettuple(sa[i]);
        if lastvisited != tmp {
            rank += 1;
            lastvisited = tmp;
        }
        ranks[i] = rank;
    }

    let mut ret = vec![0usize; sa.len()];
    for (i, &v) in sa.iter().enumerate() {
        ret[v] = ranks[i];
    }
    ret
}

fn unique_lexname(sa: &[usize]) -> bool {
    // sa has been assigned with lexicographic names
    match sa.iter().max() {
        Some(m) if m + 1 == sa.len() => true,
        Some(_) => false,
        None => true,
    }
}

pub fn suffix_array(arr: &[usize]) -> Vec<usize> {
    let sort3 = radix_sort_first_3(arr);
    let mut sa = assign_lexicographer(arr, &sort3);
    if unique_lexname(&sa) {
        return sort3;
    }
    let mut pos1 = 0;
    let mut pos2 = pos1 + (arr.len() - 2) / 3 + 1;
    let pos0 = pos2 + (arr.len() - 3) / 3 + 1;
    let mut sa12 = vec![0usize; pos0];
    for (i, &v) in sa.iter().enumerate() {
        match i % 3 {
            1 => {
                sa12[pos1] = v;
                pos1 += 1;
            }
            2 => {
                sa12[pos2] = v;
                pos2 += 1;
            }
            _ => (),
        }
    }
    // compute sa0
    let sa12 = suffix_array(&sa12);
    let mut sa12r = vec![0usize; sa12.len()];
    for (i, &v) in sa12.iter().enumerate() {
        sa12r[v] = i + 1;
    }

    let mut sa0 = vec![0usize; arr.len() - sa12.len()];
    let lastpos = sa0.len() - 1;
    sa0[lastpos] = arr.len() / 3 * 3;

    let mut sa0i = 0;
    for (_, &v) in sa12.iter().enumerate() {
        if v < pos1 {
            sa0[sa0i] = v * 3;
            sa0i += 1;
        }
    }
    sort_by(arr, &mut sa0, 0);
    // merge
    let mut p12 = 0;
    let mut p0 = 0;
    let mut p = 0;

    while p12 < sa12.len() && p0 < sa0.len() {
        let relation;
        let p12sfx = sa12[p12];
        let p0sfx = sa0[p0];
        if p12sfx < pos1 {
            // mod 3 == 1
            let a = (idx(arr, p12sfx * 3 + 1), idx(&sa12r, p12sfx + pos1));
            let b = &(idx(arr, p0sfx), idx(&sa12r, p0sfx / 3));
            relation = a.cmp(b);
        } else {
            //mod % 3 == 2
            let a = (
                idx(arr, (p12sfx - pos1) * 3 + 2),
                idx(arr, (p12sfx - pos1) * 3 + 3),
                idx(&sa12r, p12sfx - pos1 + 1),
            );
            let b = &(
                idx(arr, p0sfx),
                idx(arr, p0sfx + 1),
                idx(&sa12r, p0sfx / 3 + pos1),
            );
            relation = a.cmp(b);
        }
        match relation {
            Ordering::Less => {
                if p12sfx < pos1 {
                    sa[p] = p12sfx * 3 + 1;
                } else {
                    sa[p] = (p12sfx - pos1) * 3 + 2;
                }
                p12 += 1;
            }
            Ordering::Greater => {
                sa[p] = p0sfx;
                p0 += 1;
            }
            _ => panic!(format!("should not be here {}", p12sfx)),
        }
        p += 1;
    }
    while p12 < sa12.len() {
        let p12sfx = sa12[p12];
        if p12sfx < pos1 {
            sa[p] = p12sfx * 3 + 1;
        } else {
            sa[p] = (p12sfx - pos1) * 3 + 2;
        }
        p12 += 1;
        p += 1;
    }
    while p0 < sa0.len() {
        sa[p] = sa0[p0];
        p0 += 1;
        p += 1;
    }

    sa
}

pub fn suffix_array_str(a: &str) -> Vec<usize> {
    let mut a = a
        .as_bytes()
        .iter()
        .map(|i| *i as usize)
        .collect::<Vec<usize>>();
    a.push(0);
    suffix_array(&a)
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::Rng;
    use std::str;

    fn naive_suffix_array(arr:&str) -> Vec<usize>{
        let arr = arr.as_bytes();
        let mut s_arr = (0..arr.len()+1).collect::<Vec<usize>>();
        let compare = |a:&usize, b:&usize|arr[*a..arr.len()].cmp(&arr[*b..arr.len()]);
        s_arr.sort_by(compare);
        return s_arr;
    }

    #[test]
    fn test_suffix_array_str() {
        let expect = vec![11, 10, 7, 4, 1, 0, 9, 8, 6, 3, 5, 2];
        assert_eq!(expect, suffix_array_str("mississippi"));

        let mut rng = rand::thread_rng();
        assert_eq!(vec![6, 5, 3, 1, 0, 4, 2], suffix_array_str("banana"));

        let mut testrnd = |len|{
            let s = (0..len).map(|_|rng.gen::<u8>() % 128).collect::<Vec<u8>>();
            let s = str::from_utf8(&s).unwrap();
            assert_eq!(naive_suffix_array(s), suffix_array_str(s));
        };
        testrnd(50);

    }
}
