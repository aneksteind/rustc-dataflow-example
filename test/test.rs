fn test() {
    let mut x = 2;
    let y = &mut x;
    if *y < 10 {
        *y = *y + 1;
    }
    *y = 3;
}