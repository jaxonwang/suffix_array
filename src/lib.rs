// fn suffix_array(arr: Vec<u8>) ->Vec<usize>{
//
// }

fn sort_lexicographer(arr: &mut [u8]) {
    let mut sorted = vec![0u8; arr.len()];
    let mut count = [0u8; 256];
    for e in arr.iter() {
        count[*e as usize] += 1;
    }
    for i in 1..count.len() {
        count[i] = count[i - 1] + count[i];
    }
    for v in arr.iter().rev() {
        let mut index = &mut count[*v as usize];
        *index -= 1;
        sorted[*index as usize] = *v;
    }

    let mut rank = 0u8;
    let mut lastvisited = sorted[0];
    for i in 0..sorted.len(){
        if lastvisited != sorted[i] {
            rank += 1;
            lastvisited = sorted[i];
        }
        sorted[i] = rank;
    }
    arr.copy_from_slice(&sorted[..]);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sort_lexicographer() {
        let mut a = vec![1, 5, 6, 11, 13, 13, 14,8, 9, 0, 5, 5, 3, 2, 4, 7, 3, 5];
        sort_lexicographer(&mut a);
        println!("{:?}", a);
    }
}
