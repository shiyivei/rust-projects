# 1 Hello world 

## 1.1 可执行文件

```
rustc mian.rs //编译
./main 运行二进制文件
```

## 1.2 注释

普通注释：

//

/**/

文档注释

/// 为接下来的项生成帮助文档

//！为注释所属的项生成帮助文档

{} 在输出中充当占位符

## 1.3 格式化输出

由std::fmt中的fmt宏处理

```
fn main() {
    println!("{} days", 31i32); //居然可以加后缀
    println!("{0},this is {1}. {1},this is {0}", "Alice", "Bob"); //要打印的值是有索引的
    println!("{subject}", subject = "the lazy dog"); //可以使用命名参数
    println!("{} of {:b}", 1, 16); //十进制转二进制
}
```

## 1.4 Debug

```
所有 std 库类型都天生可以使用 {:?} 来打印：
```

```
#[derive(Debug)] //所有类型加上debug都能自动为其实现Debug trait，fmt::Display 则需要手动实现
struct DebugPrintable(String); //单元组结构体，牛
#[derive(Debug)]
struct Deep(DebugPrintable); //还可以嵌套打印
fn main() {
    let de = Deep(DebugPrintable(String::from("shiyivei")));
    println!("{:?}", de);
    println!("{0:?}", Deep(DebugPrintable(String::from("shiyivei")))); //可以直接传值打印
    println!("{0:#?}", Deep(DebugPrintable(String::from("yivei")))); //要想变得更美，用#,它就会给你打印成结构体
}
```

### 1.5 Display

```
use std::fmt;

#[derive(Debug)]
struct Structure(i32); //单元组结构体，牛

impl fmt::Display for Structure {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

fn main() {
    println!("{}", Structure(10)); //只会打印某个值
    println!("{:#?}", Structure(20)); //会打印整个结构体
}
```

### 1.6 测试实例：List

```
？操作符
```

```
write!(f, "{}", value)?; //如果有错误就返回错误，没有错误就继续执行
```

```
use std::fmt;

#[derive(Debug)]
struct List(Vec<i32>); //单元组结构体，牛

impl fmt::Display for List {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let vec = &self.0;
        write!(f, "[")?;

        for (count, v) in vec.iter().enumerate() {
            if count != 0 {
                write!(f, ", ")?;
            }
            write!(f, "{}", v)?;
        }
        write!(f, "]")
    }
}

fn main() {
    let v = List(vec![1, 2, 3]); //有时候为了美观需要为类型实现display trait
    println!("{}", v); //只会打印某个值
    println!("{:#?}", List(vec![1, 2, 3])); //会打印整个结构体
}
```

# 2 原生类型

标量类型有两个特殊

```
char（字符）：单个 Unicode 字符，如 'a'，'α' 和 '∞'（每个都是 4 字节）
```

```
单元类型（unit type）：()。其唯一可能的值就是 () 这个空元组
```

符合类型

```
数组（array）：如 [1, 2, 3]
元组（tuple）：如 (1, true)
```

## 2.1 字面量和运算符

```
前缀 0x、0o、0b，数字可以用十六进制、八进制或二进制记法表示
```

```
在数值字面量中插入下划线，比如：1_000 等同于 1000，0.000_001 等同于 0.000001
```

可以在字面量后面加上类型告诉编译器字面量的类型：如10i32

```
println!("1 - 2 = {}", 1i32 - 2); //类型决定了它的操作范围
```

```
 println!("1 * 2^5 = {}", 1u32 << 5); //位运算
```

## 2.2 元组

元组可以当函数参数和返回值

```
fn reverse(pair: (i32, bool)) -> (bool, i32) {
    let (integer, boolean) = pair;
    (boolean, integer) //最后一个语句不要加 ;
}

let (a, b) = reverse((12, true));
println!("{},{}", a, b);

let tuple = reverse((12, true));
println!("{:?}", tuple);
```

```
struct Matrix(f32, f32, f32, f32); //结构体元组，元组也可以套娃，可以解构
```

## 2.3 数组和切片

和Go语言中的含义一样

```
let xs: [i32; 5] = [1, 2, 3, 4, 5]; 数组
```

```
fn analyze_slice(slice: &[i32]) {
    println!("first element of the slice: {}", slice[0]);
    println!("the slice has {} elements", slice.len());
}
```

