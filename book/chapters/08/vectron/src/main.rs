fn main() {
    // These two statements are roughly equivalent.
    /*
    let mut v = Vec::new();
    v.push(2);
    v.push(1);
    v.push(-95);
    v.push(10485);
    // **/
    let v = vec![2, 1, -95, 10485];
    println!("Just got a vec: {:?}", v);
    {
        let mut v = vec![8, 6, 7, 5, 3, 0, 9];
        // get the third element...
        let num: &i32 = &v[2];
        println!("The third element is: {}", num);
        // worth noting .get(...) gets us an Option<&T>
        // So, `num` here is a also &i32 as in the above, just without an explicit `&` reference cast.
        match v.get(2) {
            Some(num) => println!("Elemento trés está: {}", num),
            None => println!("Looks like the list doesn't have a third element."),
        }
        for i in &v {
            println!("Element: {}", i);
        }
        for i in &mut v {
            *i += 50;
        }
        println!("Modified vec: {:?}", v);
    }
}
