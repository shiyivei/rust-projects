fn main() {
    println!("1 - 2 = {}", 1i32 - 2); //类型决定了它的操作范围
    println!("not true {}", !true); //类型决定了它的操作范围
    println!("1 * 2^5 = {}", 1u32 << 5); //位运算

    let tuple = reverse((12, true));
    println!("{:?}", tuple);
}

fn reverse(pair: (i32, bool)) -> (bool, i32) {
    let (integer, boolean) = pair;
    (boolean, integer) //最后一个语句不要加 ;
}
