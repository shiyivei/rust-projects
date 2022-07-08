fn main() {
    let mut space = "    ";
    let space = space.len();

    let num = 57u8;
    let count = 1_000;

    println!("{},{},{}", space, num, count);

    //整型默认是 i32。isize 和 usize 的主要应用场景是用作某些集合的索引

    let mut array = [1, 2, 3, 4, 5];
    array[0] = 10;

    let nums = [3; 5];

    println!("{:?},{:?}", array, nums);

    for i in (1..4).rev() {
        println!("{:?}", i)
    }

    let slice = String::from("shiyivei"); //

    let s = slice.as_bytes();

    for (i, &item) in s.iter().enumerate() {
        //slice上的迭代器
        println!("{},{:?}", i, item) //返回的是字符编码
    }

    let scale = 2;
    let debug = dbg!(scale * 2); //神奇，它会打印出整个表达式

    println!("{:?}", debug);

    panic!("crush and run")
}
