# 1 Hello world 

## 1.1 可执行文件

```
rustc main.rs //编译
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

复合类型

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
let xs: [i32; 5] = [1, 2, 3, 4, 5]; 数组，存储在栈上
```

```
fn analyze_slice(slice: &[i32]) { //引用指的是给栈上的数据再创建一个指针，存放在栈上，引用停止时，它指向的栈上数据并不会被丢弃，只会丢弃引用本身这个指针，创建一个引用的行为就是借用，借用可以更改可变值，但是同一时间只能有一个可变借用，引用变量也可以是变得或者不可变的
    println!("first element of the slice: {}", slice[0]);
    println!("the slice has {} elements", slice.len());
}
```

# 3 自定义类型

rust有两种自定义类型：结构体和枚举

## 3.1 结构体

元组结构体：实际上就是具名元组

单元结构体：不带字段，在泛型中很有用

常规结构体

```
#[derive(Debug)]
struct Uint(i32);
#[derive(Debug)]
struct Pair(i32, i32);
#[derive(Debug)]
#[allow(dead_code)] // 该属性用于隐藏对未使用代码的警告。
struct Point {
    x: i32,
    y: i32,
}

fn main() {
    let _uint = Uint(-9);
    let pair = Pair(1, 2);
    let point = Point { x: 3, y: 4 };

    println!("{:#?},{:?},{:?}", _uint, pair, point);
}
```

## 3.2 枚举

**枚举**

枚举操作方式和结构体操作方式相同，但是实例化一个枚举只需要实现枚举中某一个实例即可，而结构体都要实现

```
enum WebEvent {
    PageLoad,//单元结构体
    PageUnload,
    KeyPress(char),//元组结构体
    Paste(String),
    Click { x: i64, y: i64 },
}

fn inspect(event: WebEvent) {
    match event { //枚举常和match结合使用
        WebEvent::PageLoad => println!("page load"),
        WebEvent::PageUnload => println!("page unload"),
        WebEvent::KeyPress(c) => println!(" pressed {}", c),
        WebEvent::Paste(s) => println!("pasted \"{}\"", s),
        WebEvent::Click { x, y } => println!("click at x={},y={}", x, y),
    }
}

fn main() {
    let load = WebEvent::PageLoad;
    let un_load = WebEvent::PageUnload;

    let press = WebEvent::KeyPress('x');
    let paste = WebEvent::Paste("my_text".to_owned());
    let click = WebEvent::Click { x: 20, y: 80 };
    inspect(load);
    inspect(un_load);
    inspect(press);
    inspect(paste);
    inspect(click);
}
```

**类型别名**

```
enum ThisEnumHasALongName {
    Add,
    Subject,
}

type Name = ThisEnumHasALongName;//重给起一个名字；let x = Name::Add;，最常见的就是在impl块中使用self
```

### 3.2.1 使用use

使用use就可以不使用变量/函数的完整路径了。use可以用在任何地方，代码文件开头或者函数体内部

### 3.2.2 C风格用法

```
#[derive(Debug)]
#[allow(dead_code)]
//隐式辨别值，跟智能的一样，Rust的灵活性确实非常的富有表现能力
enum Number {
    Zero,
    One,
    Two,
}

//显式辨别值
#[allow(dead_code)]
enum Color {
    Red = 0xff0000,
    Green = 0x00ff00,
    Blue = 0x0000ff,
}

fn main() {
    println!("zero is {}", Number::Zero as i32);
    println!("zero is {}", Number::Two as i32);
    println!("Red is {}", Color::Red as i32);
    println!("Green is {}", Color::Green as i32);
}
```

### 3.3.3 测试实例：链表

```
use List::*;
enum List {
    Cons(u32, Box<List>),
    Nil,
}

impl List {
    fn new() -> List {
        Nil
    }
    fn prepend(self, elem: u32) -> List {
        Cons(elem, Box::new(self))
    }

    #[allow(dead_code)]
    fn len(&self) -> u32 {
        match *self {
            Cons(_head, ref tail) => 1 + tail.len(),
            Nil => 0,
        }
    }

    fn stringify(&self) -> String {
        match *self {
            Cons(head, ref tail) => { //这个ref相当于给等号右边的值加&
                format!("{},{}", head, tail.stringify())
            }
            Nil => {
                format!("Nil")
            }
        }
    }
}

fn main() {
    let mut list = List::new();
    list = list.prepend(1);
    list = list.prepend(2);
    list = list.prepend(3);

    println!("{}", list.stringify())
}
```

## 3.3 常量

```
const TEST: i32 = 10;
static TESTAGAIN: &'static str = "Rust";
```

# 4 变量绑定

Rust通过静态类型来保证类型安全

```
let uint = ();
println!("{:?}", uint);
```

## 4.1 可变变量

加上 mut

## 4.2 作用域和遮蔽

## 4.3 先声明再绑定

暂不建议

## 4.4 冻结

可变变量被不变变量绑定时，在未出不变变量的作用域之前，该值变量会被冻结

# 5 类型系统

## 5.1 类型转换

Rust 不提供原生类型之间的隐式类型转换，使用as关键字进行显式类型转换

```
fn main() {
    let decimal = 49.4;
    let integer = decimal as u8;

    let chare = integer as char;
    println!("{},{},{}", decimal, integer, chare);
    println!("{}", 1000 as u16)
}
```

## 5.2 字面量

字面量默认使用i32和f64，但是可以在其后增加类型如

```
let num = 20i64
```

## 5.3 类型推断

编译很聪明，大多数情况下不需要写类型声明

## 5.4 别名

可以给任何一个类型起别名，而不仅仅限于自定义类型

```
type accountId = u32;
type IoResult<T> = Result<T, IoError>;
```

# 6 类型转换

Rust使用trait来解决类型转换的问题，一般用到的是From和Into两个trait。但是也可能用到其他trait，尤其是关于String的转换

## 6.1 From 和 Into

From

```
use std::convert::From;

#[derive(Debug)]
struct Num {
    value: i32,
}

impl From<i32> for Num { //自已实现一个from
    fn from(item: i32) -> Self {
        Num { value: item }
    }
}
```

Into就是把From倒过来

```
use std::convert::From;

#[derive(Debug)]
struct Num {
    value: i32,
}

impl From<i32> for Num {
    fn from(item: i32) -> Self {
        Num { value: item }
    }
}

fn main() {
    let int = 5;
    let num: Num = int.into();
    println!("{:?}", num);
}
```

## 6.2 `TryFrom` 和 `TryInto` trait 

`TryFrom` 和 `TryInto` trait 用于易出错的转换

```
use std::convert::TryFrom;
use std::convert::TryInto;

#[derive(Debug, PartialEq)]
struct EventNum(i32);

impl TryFrom<i32> for EventNum {
    type Error = ();

    fn try_from(value: i32) -> Result<Self, Self::Error> {
        if value % 2 == 0 {
            Ok(EventNum(value))
        } else {
            Err(())
        }
    }
}

fn main() {
    assert_eq!(EventNum::try_from(8), Ok(EventNum(8)));
    assert_eq!(EventNum::try_from(5), Err(()));

    let result: Result<EventNum, ()> = 8i32.try_into();
    assert_eq!(result, Ok(EventNum(8)))
}
```

## 6.3 ToString 和 FromStr

### 6.3.1 其他类型转字符串

要把任何类型转为String，请先实现Display trait，还多了一个打印的特性

这个和之前我们在打印那一节实现是一样的

```
use std::fmt;
use std::string::ToString;

struct Circle {
    radius: i32,
}

impl fmt::Display for Circle {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Circle of radius {}", self.radius)
    }
}

fn main() {
    let circle = Circle { radius: 10 };
    println!("{}", circle.radius.to_string());
}
```

### 6.3.2 字符串转数字

使用parse

```
let parse: u32 = "5".parse().unwrap();
let turbo_parsed = "10".parse::<u32>().unwrap(); //涡轮鱼法，好奇怪的名字
println!("{}，{}", parse + 10, turbo_parsed + 21);
```

### 6.3.3 把字符串转为其他类型

只需要为目标类型（自定义的）实现 [`FromStr`](https://rustwiki.org/zh-CN/std/str/trait.FromStr.html) trait 即可，标准库中为无数类型实现了FromStr trait

```
let f = "23.8";
let my_f = f.parse::<f32>().unwrap();
println!("{}", my_f);
```

# 7 表达式

带分号的都是表达式

# 8 流程控制

## 8.1 if/else

每个条件后面都跟一个{}代码块

## 8.2 loop 循环

```
fn main() {
    let mut count = 0u32;
    println!("let us count until infinity");

    loop {
        count += 1;
        if count == 3 {
            println!("three");
            continue;
        }

        if count == 6 {
            println!("ok, that's enough");
            break;
        }
    }
}
```

### 8.2.1 嵌套循环和标签

```
fn main() {
    'outer: loop {
        println!("entry the outer loop");
        'inner: loop {
            println!("entry the inner loop");
            // break 'outer;
            break;
        }
        println!("this point will never be reached");
        break;
    }
    println!("exit the outer loop")
}
```

### 8.2.2 从loop循环中返回

```
fn main() {
    let mut count = 0;
    let result = loop {
        count += 1;
        if count == 10 {
            break count * 2;
        }
    };
    assert_eq!(result, 20) //在常规代码中，也可以使用断言语句
}
```

## 8.3 while 循环

## 8.4 for循环

**一种循环结构的是for..in**

```
    fn main() {
    for i in 0..101 { //要包含右端点的可以选择 0..=101
        if i % 15 == 0 {
            println!("15")
        } else if i % 5 == 0 {
            println!("5")
        } else if i % 3 == 0 {
            println!("3")
        } else {
            println!("i")
        }
    }
}
```

**for和迭代器**

iter 借用

```
fn main() {
    let names = vec!["frank", "bob", "ferries"];
    for name in names.iter() { //借用集合中的元素，遍历完原集合可以继续使用
        match name {
            &"ferries" => println!("{} is good at rust", name),
            _ => println!("hello {}", name),
        }
    }
}
```

Into_iter 消耗

```
fn main() {
    let names = vec!["frank", "bob", "ferries"];
    for name in names.into_iter() {
        //消耗集合中的元素，遍历完原集合可以继续使用
        match name {
            "ferries" => println!("{} is good at rust", name),
            _ => println!("hello {}", name),
        }
    }
}
```

Iter_mut 就地改变

```
fn main() {
    let mut names = vec!["frank", "bob", "ferries"];
    for name in names.iter_mut() {
        //改变集合中的元素
        *name = match name {
            &mut "ferries" => "hello ferries", //直接更改，这一点比go语言要方便，go语言只有一种方式，就是通过索引
            _ => "hello",
        }
    }
    println!("{:?}", names)
}
```

## 8.5 match 匹配

与c语言中的switch相似

这个没什么可以说的，分支必须覆盖所有可能

```
fn main() {
    let boolean = true;

    let library = match boolean {
        true => 0,
        false => 1,
    };
}
```

### 8.5.1解构

##### 元组

```
fn main() {
    let triple = (1, 1, 2);

    println!("{:?}", triple);

    match triple {
        (0, y, z) => println!("{},{},{}", 0, y, z), //解构两个
        (1, x, ..) => println!("{},{}", 1, x),      //只解构第二个
        _ => println!("others"),
    }
}
```

##### 枚举

```
fn main() {
    let color = Color::RGB(127, 40, 17); //注意枚举实例的构造

    match color {
        Color::Red => println!("The color is red"),
        Color::Blue => println!("The color is blue"),
        Color::Green => println!("The color is green"),
        Color::RGB(r, g, b) => println!("Red {},Green {},Blue {}", r, g, b),
        Color::HSV(r, g, b) => println!("Red {},Green {},Blue {}", r, g, b),
        Color::CMY(r, g, b) => println!("Red {},Green {},Blue {}", r, g, b),
        Color::HSL(r, g, b) => println!("Red {},Green {},Blue {}", r, g, b),
        Color::CMYK(r, g, b, k) => println!("Red {},Green {},Blue {}", r, g, b),
    }
}
```

**指针和引用**

对于指针来说，解构和解引用要分开

解引用使用： *

解构使用：&、ref 和 mut

在等号左边创建引用一

```
let ref x = 3;
let ref mut y = "non";
```

在右边创建引用

```
let refe = &4;
```

```
fn main() {
    let refe = &4;

    match refe {
        &val => println!("the value is {:?}", val), //val这个名字随意命名
    }
    match *refe {
        val => println!("the value is {:?}", val), //两种方式
    }

    let ref x = 3; //获得引用
    let mut y = 7;
    match x {
        ref x => println!("{}", x),
    }

    match y {
        ref mut m => {
            //获得了引用
            *m += 10;
            println!("{}", m) //解应用后才能更改值
        }
    }
}
```

##### **结构体**

```
fn main() {
    let foo = Foo { x: (1, 2), y: 3 }; //先实例化，再解构

    let Foo { x: (x, y), y: z } = foo; //解构和赋值有点像，只不过在=左边，并且值用变量名称代替，右边是实例

    println!("{},{},{},{}", x, y, z, foo.y); //这样就可以一个一个打印出来了,当然如果不解构，也是可以打印的
}
```

### 8.5.2 卫语句

就是在match分支语句值后面加上条件语句

```
fn main() {
    let pair = (2, -2);

    match pair {
        (x, y) if x + y == 0 => println!("a = -b"),
        _ => println!("a != -b"),
    }
}
```

### 8.5.3 绑定

使用@

```
fn main() {
    match age() {
        0 => println!("not born"),
        n @ 1..=12 => println!("child"), //间接访问变量比如函数体内部的值，使用@重新绑定
        n @ 13..=19 => println!("young man"),
        _ => println!("old man"),
    }
}

fn age() -> u32 {
    15
}
```

可以使用绑定来解构 enum变体

```
fn main() {
    match some_number() {
        Some(n @ 42) => println!("The answer is {}", n),
        _ => println!("old man"),
    }
}

fn some_number() -> Option<u32> {
    Some(42)
}
```

## 8.6 If let

只取想要的值，其他都用else 分支来处理

```
fn main() {
    let number = Some(7);
    let letter: Option<i32> = None;
    let emotion: Option<i32> = None;

    if let Some(i) = letter {
        println!("{}", i)
    } else {
        println!("nothing match")
    }
}
```

**匹配枚举**

```
enum Foo {Bar}

fn main() {
    let a = Foo::Bar;

    // 变量匹配 Foo::Bar
    if let Foo::Bar = a {
    // ^-- 这就是编译时发现的错误。使用 `if let` 来替换它。
        println!("a is foobar");
    }
}

```

## 8.7 while let

if let 和 while let都可以把match 分支改的好看

```
fn main() {
    let mut optional = Some(9);//枚举Oprion的一个字段居然可以拿出来单独用
    loop {
        match optional {
            Some(i) => {
                if i > 9 {
                    optional = None
                } else {
                    optional = Some(i + 1);
                    println!("{}", i)
                }
            }
            _ => break,
        }
    }
}
```

```
 while let Some(i) = optional {
        if i > 10 {
            println!("Now the code is more beautiful");
            optional = None
        } else {
            optional = Some(i + 1);
            println!("{}", i)
        }
    }
```

# 9 函数

使用return 可以提前返回值，默认情况下返回最后一个表达式

没有返回值，实际上返回可一个()

## 9.1 方法

```
struct Pair(Box<i32>, Box<i32>);

impl Pair {
    fn destory(&self) {
        let Pair(first, second) = self;
        println!("Distory pair: {},{}", first, second); //离开作用域后释放
    }
}

fn main() {
    let pair = Pair(Box::new(1), Box::new(2));
    pair.destory();
}
```

## 9.2 闭包

闭包能够捕获周围作用域中的变量，它是一个函数

```
|val| val + x //一个可以捕获x变量的闭包
```

特点

使用 || 代替 () 包含参数

单个表达式可以不用函数体定界大括号

可以捕获外部环境变量

```
fn  function      (i: i32) -> i32 { i + 1 } //函数
let closure_one = |i: i32| -> i32 { i + 1 }; //闭包
```

```
println!("func {}", function(21));
println!("closure {}", closure_one(21)) //传参的时候一摸一样
```

### 9.2.1 捕获

即可移动又可以借用，但优先通过引用来捕获变量

```
fn main() {
    use std::mem;

    let color = String::from("Green");
    let print = || println!("color {}", color); //闭包借用，借用后归还
    
    print(); //调用

    let re_borrow = &color; //仍然可以被借用
    print();
    let _move = color; //也可以被move,栈上数据拷贝
   
}
```

可变引用

```
let mut count = 0;
    let mut inc = || {
        //闭包也要是可变的
        count += 1;
        println!("count: {}", count); //可变引用
    };
    inc();
    inc();
```

消耗型借用

```
let movable = Box::new(3);
  let consume = || {
    println!("movable: {:?}", movable);
    mem::drop(movable)
	};

consume();
```

加上move 会强制让闭包获得变量的所有权

```
 let heck = vec![1, 3, 4, 5];

    let contains = move |x| heck.contains(x);
    println!("{}", contains(&4));
    println!("{}", contains(&3));
```

### 9.2.2 作为输入参数

三个trait：Fn\FnMut\FnOnce

```
fn main() {
    use std::mem;
    let greeting = String::from("hello");

    let mut farewell = "goodbye".to_owned();

    let diary = || {
        //捕获了两个参数，一个可变一个不可变，

        println!("I said {}", greeting);
        farewell.push_str("!!!");
        println!("then I screamed {}", farewell);
        println!("Now I can sleep,zzz");

        mem::drop(farewell)
    };
    apply(diary);
    let double = |x| 2 * x; //一些操作逻辑在定义闭包时才实现
    println!("value is {}", apply_to_3(double));
}

//定义两个函数，参数类型是闭包，当然闭包要实现了指定的trait
fn apply<F>(f: F)
where
    //where 子句跟在返回值类型的后面，但是在函数体外
    F: FnOnce(), //闭包获取参数所有权
{
    //函数体
    f();
}

fn apply_to_3<F>(f: F) -> i32
where
    F: Fn(i32) -> i32, //有返回值的话要填参数类型
{
    f(3)
}
```

### 9.2.3 匿名类型

当闭包被定义后，编译器会创建一个匿名结构体存放闭包捕获的变量，同时为它实现了三个trait中的一种

### 9.2.4 输入函数

如果一个函数的参数是闭包，那任何满足了该trait的函数也是合法的参数

```
fn function() {//作为参数的函数
    println!("I am a function")
}
let closure = || println!("I am a closure1");//作为参数的闭包

fn call_me<F: Fn()>(f: F) { //调用者
    f()
}
//调用，函数默认实现了Fn，闭包也是好像。并且是根据其行为的
call_me(closure);
call_me(function);
```

### 9.2.5 输出参数

Rust目前不支持泛型返回，支持前面三种有效特征的闭包作为返回值，并且需要move 参数

```
fn create_fn() -> impl Fn() {
    let text = "Fn".to_owned();

    move || println!("this is a {}", text)
}
fn create_fnMut() -> impl FnMut() {
    let text = "FnMut".to_owned();

    move || println!("this is a {}", text)
}
fn create_fnOnce() -> impl FnOnce() {
    let text = "FnOnce".to_owned();

    move || println!("this is a {}", text)
}

fn main() {
    let fn_palin = create_fn();
    let mut fn_mut = create_fnMut();
    let fn_once = create_fnOnce();

    fn_palin();
    fn_mut();
    fn_once();
}
```

### 9.2.6 两个函数find和any

。。。

## 9.3 高阶函数

```
fn main() {
    let upper = 100000;

    //函数式编程，把一堆条件写成了链式函数
    let sum_of_squared_odd_numbers: u32 = (0..)
        .map(|n| n * n)
        .take_while(|&n| n < upper)
        .filter(|&n| is_odd(n))
        .fold(0, |sum, i| sum + i);

    println!("functional style {}", sum_of_squared_odd_numbers);
}
fn is_odd(n: u32) -> bool {
    n % 2 == 1
}
```

Option和迭代器都实现了不少高阶函数

## 9.4 发散函数

发散函数绝不返回，使用！来标记绝不返回

```
fn foo() -> ! {
    panic!("this call never returns")
}
```

```
fn some_fn() {
    () //照常返回
}
```

```
fn main() {
    fn some_odd_numbers(up_to: u32) -> u32 {
        //在main函数内部写一个具名函数
        let mut acc = 0;

        for i in 0..up_to {
            let addition: u32 = match i % 2 == 1 { //注意这种写法
                true => i,
                false => continue,
            };
            acc += addition;
        }
        acc
    }
    println!("{}", some_odd_numbers(9))
}
```

# 10 模块

## 10.1 可见性

```
pub(crate) fn public_function_in_crate() { //pub crate 使得函数只有在当前crate中可见
        println!("called `my_mod::public_function_in_crate()");
}
```

## 10.2 结构体的可见性

结构体的字段也是一个可见性的层次

## 10.3 use

将路径绑定到新名字

```
use deeply::nested::function as other_function;
```

## 10.4 super 和 self

self表示当前模块，super表示模块外

## 10.5 文件分层

就是按模块分层，这个无需多言

# 11 crate

crate是rust编译的独立单元，可以被编译为二进制执行文件或者库文件。可以通过rustc的选项 -- crate-type重载？？？

## 11.1 库

创建库

## 11.2 使用库

```
extern crate xxx;

use xxx::yyy::zzz;
另外还要加依赖
```

# 12 Cargo

## 12.1 依赖

cargo来管理依赖，一切都变得简单

## 12.2 约定规范

其他二进制问价放在src bin目录下

## 12.3 测试

不多讲了

## 12.4 构建脚本

啥是脚本？？？

# 13 属性

属性是应用于某些模块、crate 或项的元数据（metadata）。这元数据可以用来：

- [条件编译代码](https://rustwiki.org/zh-CN/rust-by-example/attribute/cfg.html)
- [设置 crate 名称、版本和类型（二进制文件或库）](https://rustwiki.org/zh-CN/rust-by-example/attribute/crate.html)
- 禁用 [lint](https://en.wikipedia.org/wiki/Lint_(software)) （警告）
- 启用编译器的特性（宏、全局导入（glob import）等）
- 链接到一个非 Rust 语言的库
- 标记函数作为单元测试
- 标记函数作为基准测试的某个部分

属性可以接受参数，有不同的语法形式：

- `#[attribute = "value"]`
- `#[attribute(key = "value")]`
- `#[attribute(value)]`

属性可以多个值，它们可以分开到多行中：

```rust
#[attribute(value, value2)]

#[attribute(value, value2, value3,
            value4, value5)]
```

## 13.1 死代码

```
#[allow(dead_code)] //注意在实际过程中应该将死代码清除
```

## 13.2 crate

用cargo 就行了

## 13.3 cfg

条件编译可能通过两种不同的操作符实现：

- `cfg` 属性：在属性位置中使用 `#[cfg(...)]`
- `cfg!` 宏：在布尔表达式中使用 `cfg!(...)`

### 13.3.1 自定义条件

有部分条件如 `target_os` 是由 `rustc` 隐式地提供的，但是自定义条件必须使用 `--cfg` 标记来传给 `rustc`

```
#[cfg(some_condition)]
fn conditional_function() {
    println!("condition met!")
}

fn main() {
    conditional_function();
}
```

# 14 泛型

泛型结构体

```
struct SingleGen<T>(T);
```

## 14.1 函数

```
fn generic<T>(_s: SGen<T>) {} //参数类型为泛型的函数
```

## 14.2 实现

```
#[derive(Debug)]
struct GenVal<T> { //定义写一次T
    gen_val: T,
}

impl<T> GenVal<T> { //实现写两次
    fn value(&self) -> &T {
        &self.gen_val
    }
}

fn main() {
    let x = GenVal { gen_val: 45 };
    println!("{:?}", x);
}
```

## 14.3 trait 

trait也可以是泛型的

```
struct Empty;
struct Null;

trait DoubleDrop<T> {
    fn double_drop(self, _: T); //定义类型里面的方法
}

impl<T, U> DoubleDrop<T> for U {
    //为trait实现trait
    fn double_drop(self, _: T) {}
}

fn main() {
    let empty = Empty;
    let null = Null;
    empty.double_drop(null); //这个操作会释放其本身和传入的参数类型
}
```

## 14.4 约束

虽然是泛型，但约束是要求必须要实现的trait

//函数中的约束

```
fn print<T: Display>(t: T) {
    println!("{}", t)
}
```

结构体中的泛型约束

```
struct S<T: Display>(T);
```

```
fn print_type<T: Debug>(t: T) {
    //函数的泛型参数必须实现Debug
    println!("{:?}", t);
}

#[derive(Debug)] //通过宏为类型实现Debug trait
struct Retangle {
    length: u32,
    width: u32,
}
```

调用

```
let retangle = Retangle {
        length: 32,
        width: 46,
    };
    print_type(retangle)
```

### 14.4.1 空约束

trait不含任何功能，如Eq和Org，但是仍然可以作为约束

## 14.5 多重约束

用 + 连接

```
fn compare_types<T: Debug, U: Debug>(t: &T, u: &U) {
    println!("t: `{:?}", t);
    println!("u: `{:?}", u);
}
```

## 14.6 where子句

为了更易读，为Option枚举中的类型实现trait

```
trait PrintInOption {   
    fn print_in_option(self);
}

// 这里需要一个 `where` 从句，否则就要表达成 `T: Debug`（这样意思就变了），
// 或者改用另一种间接的方法。
impl<T> PrintInOption for T where
    Option<T>: Debug {
    // 我们要将 `Option<T>: Debug` 作为约束，因为那是要打印的内容。
    // 否则我们会给出错误的约束。
    fn print_in_option(self) {
        println!("{:?}", Some(self));
    }
}
```

## 14.7 new type的用法

为不同种类的数据分别定义新的类型

## 14.8 关联项目 

是trait类型的扩展，与多种类型的项有关，可以在trait内部定义

有点像where子句是为了抽象的

## 14.9 虚类型参数

出现于静态检查时的参数 有点没太懂

# 15 作用域规则

## 15.1 RALL 

资源获取即初始化，即任何对象在离开作用域的时候，就会调用析构函数释放资源

```
fn main() {
    let int_val = Box::new(4); //在堆上分配内存
    println!("{}", int_val);

    //定义一个作用域
    {
        let int_val1 = Box::new(5i32);
    } //内存释放

    for i in 0..=10 {
        //在数字区间遍历
        println!("{:?}", create());
    } //内存释放
      //int_val 内存释放
}

fn create() -> u32 {
    let int_val = Box::new(32u32);
    println!("int_val {}", int_val);
    *int_val
}
```

使用valgrind对内存泄漏进行检查

Rust中析构函数是通过Drop trait提供的，并且是自动调用的

```
struct Top;

impl Drop for Top {
    fn drop(&mut self) {
        println!("Top is being dropped")
    }
}
fn main() {
    let x = Top;//我们为这个类型实现了Drop trait
    println!("Made a TopDrop")//
    //在此会调用Drop trait
}
```

```
Made a TopDrop
Top is being dropped
```

## 15.2 所有权和移动

### 15.2.1 可变性

还就是堆栈数据的所有权问题，可以通过Box::new（valuetype）把数据强行分配在堆上

```
let x = Box::new("x");
let y = x;
println!("{}", y);
// println!("{}",x) x引用的数据所有权已经转移
```

可以通过移动所有权来改变变量的可变性

```
fn main() {
    let immutable_value = Box::new(35i32);
    println!("{}", immutable_value);

    let mut can_change_value = immutable_value;

    *can_change_value = 25i32;
    println!("{}", can_change_value)
}
```

### 15.2.2 部分移动

```
fn main() {
    #[derive(Debug)]
    struct Person {
        name: String,
        age: u32,
    }

    let person = Person {
        name: String::from("John"),
        age: 27,
    };

    let Person { name, ref age } = person; //这里的name是解构出来的变量,而不是上面的结构体字段名称

    println!("{:?}", name);
    println!("{:?}", age);

    // println!("{:?}",person) //不能再整体使用，因为部分字段age 被移动
}
```

## 15.3 借用

借用是不会获取所有权的，但移动会（值传递）借用：&T，就是引用，引用是值不会被销毁，借用检查器会检查这一点

```
fn main() {
    let value1 = Box::new(45i32);
    let value2 = 35i32;

    eat_value(value1); //值传递
    reserve_value(&value2); //借用
}

fn eat_value(value: Box<i32>) {
    println!("the {} was destroyed", value)
}
fn reserve_value(value: &i32) {
    println!("the {} is still alive", value)
}
```

### 15.3.1 可变性

借用的可变性和之前的意思是一样的，无非就是引用和移动，Box新建返回的是栈上的指针，堆上的数据

```
struct Book {
    title: &'static str,
    auther: &'static str,
    year: u32,
}

fn borrow_book(book: &Book) {
    println!(
        "value was borrow {},{},{}",
        book.title, book.auther, book.year
    )
}

fn new_edition(book: &mut Book) {
    book.year = 2022;
    println!("year was changed {}", book.year)
}

fn main() {
    let book = Book {
        //一个实例
        title: "cat",
        auther: "shiyivei",
        year: 2001,
    };
    borrow_book(&book); //借用不可变变量
    let mut mut_book = book; //一个可变实例，移动
    borrow_book(&mut mut_book); //借用可变变量
    new_edition(&mut mut_book);
}
```

### 15.3.2 别名使用

不变借用就像读一样，可以多次。在可变借用之后，还可以继续借用

### 15.3.3 ref 模式

使用let 进行模式匹配和解时，ref‘可用来创建结构体/元组的字段的引用，前面已经展示过了。它是在左边对新变量的引用

## 15.4 生命周期

### 15.4.1 显示标注

借用检查器来检查变量的生命周期，保证所有引用都是有效，借用需要在变量丢弃之前使用完毕

和闭包类似，使用生命周期需要使用泛型，先声明，一般都是从函数外到函数内

### 15.4.2 函数

带生命周期函数签名的限制

1. 任何引用都需标注好生命周期
2. 任何被返回的引用都必须具有和某个输入量相同的生命周期或者静态类型

```
fn add_one<'a>(x: &'a mut i32) {
    *x += 1;
    println!("{}", x)
}

fn sum<'a, 'b>(x: &'a mut i32, y: &'a mut i32) {
    println!("{}", *x + *y) //引用要解引用了才能计算
}

fn pass<'a, 'b>(x: &'a mut i32, _: &'b i32) -> &'a i32 {
    x
}
fn main() {
    let (mut a, mut b) = (1i32, 2i32);

    add_one(&mut a);

    sum(&mut a, &mut b);
    let x = pass(&mut a, &mut b);
}
```

核心的两句话，使用者不能比被使用者活的长；使用前要先定在参数之前定义，不管是返回值还是参数

### 15.4.3 方法

方法里面是和函数一样的写法

```
struct Owner(i32);

impl Owner {
    fn add_one<'a>(&'a mut self) {
        self.0 += 1; //要写成+=，不要只写为+
    }
    fn print<'a>(&'a mut self) {
        println!("{}", self.0);
    }
}
fn main() {
    let mut owner = Owner(1);
    owner.add_one();

    owner.print();
    println!("{}", owner.0)
}
```

### 15.4.4 结构体

```
#[derive(Debug)]
struct Owner<'a> {
    x: &'a i32,
    y: &'a i32,
}

#[derive(Debug)]
enum Either<'a> {
    Num(i32),     //枚举里面是两个元组结构体
    Ref(&'a i32), //枚举里面是两个元组结构体
}

fn main() {
    let tuple = (5, 6); //给元组命名
    let (x, y) = (3, 4); //直接解构

    let double = Owner { x: &x, y: &y };
    let en1 = Either::Num(y); //注意枚举实例化的方式
    let en2 = Either::Ref(&x);

    println!("{:?}", double);
    println!("{:?}", en1);
    println!("{:?}", en2);
}
```

### 15.4.5 trait

```
#[derive(Debug)]
struct Borrowed<'a> {
    x: &'a i32,
}

impl<'a> Default for Borrowed<'a> { //在impl之后，Borrow之前
    fn default() -> Self {
        Self { x: &99 }
    }
}

fn main() {
    let borrow: Borrowed = Borrowed::default();
    println!("{:?}", borrow)
}
```

### 15.4.6 约束

可以把生命周期看作是类泛型特性，在实际过程中某个类型参数需要实现特定trait，也需要满足生命周期要求 'a 和 T地位一样

```
use std::fmt::Debug; //引入的traits或者其他类型要尽量在rs文件开头

#[derive(Debug)]
struct Borrowed<'a, T: 'a> {
    x: &'a T,
}

fn print<T>(t: T)
where
    T: Debug,
{
    println!("{:?}", t)
}

fn bound_print<'a, T>(t: &'a T)
//注意生命周期在标注中的写法，和函数中的约束是一样的
where
    T: Debug + 'a,
{
    println!("{:?}", t)
}

fn main() {
    let x = 32;
    let t = Borrowed { x: &x };
    print(t);
    bound_print(&x);
}
```

### 15.4.7 强制转换

感觉上并没有强制转换，只是一种合理的写法罢了

```
fn multiply<'a>(first: &'a i32, second: &'a i32) -> i32 {
    //两个参数生命周期都被强制转为‘a
    first * second
}

fn choose_first<'a: 'b, 'b>(first: &'a i32, _: &'b i32) -> &'b i32 {
    //手动声明以b‘为准
    first
}
fn main() {
    let a = 23i32;
    {
        let b = 32;
        let res = multiply(&a, &b);
        choose_first(&a, &b);
        println!("res {}", res);
        println!("choose {}", choose_first(&a, &b))
    }
}
```

### 15.4.8 static 

static 是最长的，存在于整个程序运行期间， ‘static 可以被转为一个更短的生命周期。两种方式，他们都把数据保存在可执行文件的只读内存区

```
使用 static 声明来产生常量（constant）
产生一个拥有 &'static str 类型的 string 字面量
```

```
static NUM: i32 = 99;

fn func<'a>(value: &'a i32) -> &'a i32 {
    &NUM
}

fn main() {
    {
        let static_string = "I'm in read_only memory";
        println!("{}", static_string)
    }
    {
        let short_life_value = 19; //定义一个生命周期比较短的变量

        func(&short_life_value);
        println!("{:?}", func(&short_life_value));//使用它也是可以的
    }
    println!("{}", NUM)
}
```

### 15.4.9 省略

有些生命周期模式过于常用所以编译器会隐式添加它，以增强代码的可读性。如

```
fn example1(t: &i32) -> &i32 {
    t
}
fn example2<'a>(t: &'a i32) -> &i32 {
    t
}
```

# 16 Trait

Trait是类型的方法集

```
//先定义一个变量类型
struct Sheep {
    naked: bool,
    name: &'static str,
}
//定义trait，方法集，为类型定义方法集合，主要是确定参数和返回值的类型和属性（trait 生命周期等）
trait Animal {
    fn new(name: &'static str) -> Self; //trait中的方法不用在方法名称前面声明生命周期标签，在实现的时候声明就可以了
    fn name(&self) -> &'static str;
    fn noise(&self) -> &'static str;
    fn talk(&self) {
        println!("{} sas {}", self.name(), self.noise())
    }
}
//最后还可以声明一些其他的方法在trait方法集外
impl Sheep {
    fn is_naked(&self) -> bool {
        self.naked
    }
    fn shear(&mut self) {
        if self.naked {
            println!("{} is already sheard", self.name())
        } else {
            println!("{} gets a shortcut", self.name());
            self.naked = true
        }
    }
}
//为类型实现实现trait，这一步就是确定方法的方法体
impl Animal for Sheep {
    fn new(name: &'static str) -> Sheep {
        Sheep {
            name: name,
            naked: false,
        }
    }
    fn name(&self) -> &'static str {
        self.name
    }
    fn noise(&self) -> &'static str {
        if self.naked {
            "naked"
        } else {
            "no naked"
        }
    }
    fn talk(&self) {
        println!("{} {}", self.name(), self.noise())
    }
}

fn main() {
    let mut dolly: Sheep = Animal::new("Dolly");
    dolly.talk();
    dolly.shear();
    dolly.talk();
}
```

## 16.1 Derive

`#[derive]` [属性](https://rustwiki.org/zh-CN/rust-by-example/attribute.html)，编译器能够提供某些 trait 的基本实现,其实就是为类型在trait中实现方法，这也是rust语言最强的地方之一

下面是可以自动派生的 trait：

- 比较 trait: [`Eq`](https://rustwiki.org/zh-CN/std/cmp/trait.Eq.html), [`PartialEq`](https://rustwiki.org/zh-CN/std/cmp/trait.PartialEq.html), [`Ord`](https://rustwiki.org/zh-CN/std/cmp/trait.Ord.html), [`PartialOrd`](https://rustwiki.org/zh-CN/std/cmp/trait.PartialOrd.html)
- [`Clone`](https://rustwiki.org/zh-CN/std/clone/trait.Clone.html), 用来从 `&T` 创建副本 `T`。
- [`Copy`](https://rustwiki.org/zh-CN/core/marker/trait.Copy.html)，使类型具有 “复制语义”（copy semantics）而非 “移动语义”（move semantics）。
- [`Hash`](https://rustwiki.org/zh-CN/std/hash/trait.Hash.html)，从 `&T` 计算哈希值（hash）。
- [`Default`](https://rustwiki.org/zh-CN/std/default/trait.Default.html), 创建数据类型的一个空实例。
- [`Debug`](https://rustwiki.org/zh-CN/std/fmt/trait.Debug.html)，使用 `{:?}` formatter 来格式化一个值

```
#[derive(PartialEq, PartialOrd, Debug)] //牛啊，以前一直不懂
struct Comparable(i32);

fn main() {
    let c1 = Comparable(26);
    let c2 = Comparable(30);

    let res = if c1 == c2 {
        println!("c1 = c2")
    } else {
        println!("c1 != c2")
    };
    println!("res is {:?}", res)
}
```

## 16.2 使用dyn返回trait

Rust需要知道返回类型需要多少空间，这意味着，所有的函数都必须返回一个具体类型。记下来就行，如果返回类型大小不确定

```
struct Sheep {}
struct Cow {}

trait Animal {
    // 实例方法签名
    fn noise(&self) -> &'static str;
}

// 实现 `Sheep` 的 `Animal` trait。
impl Animal for Sheep {
    fn noise(&self) -> &'static str {
        "baaaaah!"
    }
}

// 实现 `Cow` 的 `Animal` trait。
impl Animal for Cow {
    fn noise(&self) -> &'static str {
        "moooooo!"
    }
}

// 返回一些实现 Animal 的结构体，但是在编译时我们不知道哪个结构体。
fn random_animal(random_number: f64) -> Box<dyn Animal> {
    if random_number < 0.5 {
        Box::new(Sheep {})
    } else {
        Box::new(Cow {})
    }
}

fn main() {
    let random_number = 0.234;
    let animal = random_animal(random_number);
    println!("You've randomly chosen an animal, and it says {}", animal.noise());
}
```

## 16.3 运算符重载

啥意思，就是让运算符有更多的行为，比如，+ 表示求和，也可以表示把字符串连接起来，我们甚至可以把+ 定义为-

```
use std::ops;
struct Foo;
struct Bar;

#[derive(Debug)]
struct FooBar;

#[derive(Debug)]
struct BarFoo;

//分别实现了trait

impl ops::Add<Bar> for Foo {
    type Output = FooBar; //重命名

    fn add(self, _a: Bar) -> FooBar {
        println!("> Foo.add(Bar) was called");
        FooBar
    }
}

impl ops::Add<Foo> for Bar {
    type Output = BarFoo; //重命名

    fn add(self, _a: Foo) -> BarFoo {
        println!("> Bar.add(Foo) was called");
        BarFoo
    }
}

fn main() {
    println!("{:?}", Foo + Bar);
    println!("{:?}", Bar + Foo);
}
```

## 16.4 Drop

只有drop一个方法，主要用于释放资源

`Box`，`Vec`，`String`，`File`，以及 `Process` 是一些实现了 `Drop` trait 来释放 资源的类型。`Drop` trait 也可以为任何自定义数据类型手动实现

```
struct Droppable {
    name: &'static str,
}
impl Drop for Droppable {
    fn drop(&mut self) {
        println!("> Dropping {}", self.name);
    }
}

fn main() {
    let name = "choose";
    let drop = Droppable { name: &name };
    //函数结束时自动实现，控制台会打印信息
}
```

## 16.5 Iterator

用来对集合类型的数组实现迭代器

```
struct Fibonacci {
    curr: u32,
    next: u32,
}

impl Iterator for Fibonacci {
    type Item = u32;

    fn next(&mut self) -> Option<u32> {
        //枚举作为返回值类型时，只需要在枚举中放入值即可
        let new_next = self.curr + self.next;
        self.curr = self.next;
        self.next = new_next;

        Some(self.curr)
    }
}

fn fibonacci() -> Fibonacci {
    Fibonacci { curr: 1, next: 1 }
}

fn main() {
    let mut sequence = 0..5; //序列是迭代器的实例
    println!("{:?}", sequence.next());
    println!("{:?}", sequence.next());
    println!("{:?}", sequence.next());
    println!("{:?}", sequence.next());
    println!("{:?}", sequence.next());

    for i in 0..3 {
        println!("i is {}", i);
    }

    for i in fibonacci().take(4) {
        //使用take可以把值从枚举中拿出来,提取前n项，从iterator
        println!("the before 4 is {}", i);
    }

    for i in fibonacci().skip(4).take(4) {
        //相当于跳过了前四项，重新开始数了4项
        println!("removed {}", i);
    }

    let array = [1u32, 3, 3, 7];

    for i in array.iter() {
        println!("the element in array is {}", i)//迭代打印数组，牛皮
    }
}
```

## 16.6 impl trait

如果函数返回实现了MyTrait的类型，可以将其返回类型编写为 -> impl MyTrait。 这一节不是很懂

```
use std::iter;
use std::vec::IntoIter;

fn combine_vecs_explicit_return_type(
    v: Vec<i32>,
    u: Vec<i32>,
) -> iter::Cycle<iter::Chain<IntoIter<i32>, IntoIter<i32>>> {
    //使用结构体组合了两个i32
    v.into_iter().chain(u.into_iter()).cycle()
}

//直接返回一个迭代器
fn combine_vecs(v: Vec<i32>, u: Vec<i32>) -> impl Iterator<Item = i32> {
    v.into_iter().chain(u.into_iter()).cycle()
}

fn main() {
    let v1 = vec![1, 2, 3];
    let v2 = vec![4, 5];

    let mut v3 = combine_vecs(v1, v2); //传参

    assert_eq!(Some(1), v3.next());
    assert_eq!(Some(2), v3.next());
    assert_eq!(Some(3), v3.next());
    assert_eq!(Some(4), v3.next());
    assert_eq!(Some(5), v3.next());

    println!("All down")
}
```

某些rust类型无法写出，如每个闭包都有自己未命名的类型.在使用impl Trait语法之前，必须在堆上分配才能返回闭包。但是可以像下面这样静态的完成所有操作

```
fn make_adder_function(y: i32) -> impl Fn(i32) -> i32 {
    //返回再返回？不是很懂
    let closure = move |x: i32| x + y;
    closure
}

//使用map或者filter 闭包的迭代器，这可以使得使用map或filter更容易

fn double_positives<'a>(numbers: &'a Vec<i32>) -> impl Iterator<Item = i32> + 'a {
    numbers.iter().filter(|x| x > &&0).map(|x| x * 2)
}
```

## 16.7 Clone

通常我们使用Clone trait的clone方法

```
#[derive(Debug, Clone, Copy)]
struct Nil;

#[derive(Debug, Clone)]
struct Pair(Box<i32>, Box<i32>);

fn main() {
    let nil = Nil;
    let copied_nil = nil; //空值转移了相当于没转移

    println!("{:?}", nil);
    println!("{:?}", copied_nil);

    let pair = Pair(Box::new(1), Box::new(2));
    println!("{:?}", pair);

    let moved_pair = pair;
    // println!("{:?}",pair); 所有权转移，不可以再用
    println!("{:?}", moved_pair);

    let clone_moved_pair = moved_pair.clone();//堆栈上的数据都赋值了一份
    println!("{:?}", clone_moved_pair);

    drop(moved_pair);

    println!("{:?}",clone_moved_pair)
}
```

## 16.8 父Trait

Rust没有继承，但是可以将一个trait定义为另一个trait的超集，如：

```
trait Person {
    fn name(&self) -> String;
}

//Person是Student的父trait
trait Student: Person {
    fn university(&self) -> String;
}

trait Programmer {
    fn fav_language(&self) -> String;
}

trait ComScistudent: Programmer + Student {
    fn git_username(&self) -> String;
}

fn comp_sci_student_greeting(student: &dyn ComScistudent) -> String {
    format!(
        "my name is {} and i attended {}. My favorite language is {}. My git username is {}",
        student.name(),
        student.university(),
        student.fav_language(),
        student.git_username()
    )
}

fn main() {}
```

## 16.9 消除重叠trait

```
trait UserNameWeight {
    fn get(&self) -> String;
}

trait AgeWeight {
    fn get(&self) -> u8;
}

struct Form {
    username: String,
    age: u8,
}
impl UserNameWeight for Form {
    fn get(&self) -> String {
        self.username.clone()
    }
}

impl AgeWeight for Form {
    fn get(&self) -> u8 {
        self.age
    }
}

fn main() {
    let form = Form {
        username: "shiyivei".to_owned(),
        age: 27,
    };

    let username = <Form as UserNameWeight>::get(&form); //不是很懂
    assert_eq!("shiyivei".to_owned(), username);
    let age = <Form as AgeWeight>::get(&form);
    assert_eq!(27, age)
}
```

# 17 使用macro_rules ！创建宏

宏不产生调用而是展开源码，并和程序的其余部分一起被编译。rust的宏和C语言不同，它会展开称为抽象的语法树，而不是像字符串预处理那样直接替换成代码，这样就不会产生无法预料的优先权错误

宏通过macro_rules来创建

宏的优势：不用重复写代码，领域专用语言，可变接口，如println！

## 17.1 语法

### 17.1.1 指示符

宏参数使用 $作为前缀，使用指示符来注明类型

```
macro_rules! say_hello {
    //() 代表不接受参数
    () => {
        println!("Hello!") //展开成为此内容
    };
}

macro_rules! create_function {
    ($func_name:ident) => {
        //ident 会被替换
        fn $func_name() {
            println!("You called {:?}()", $func_name());
            stringify!($func_name);
        }
    };
}

create_function!(foo);
create_function!(bar);

macro_rules! print_result {
    ($expression:expr) => {
        //stringify! 会把对象转换为字符串
        println!("{:?} = {:?}", stringify!($expression), $expression);
    };
}

fn main() {
    say_hello!();
    foo();
    bar();

    print_result!(1u32 + 1);
    ({
        let x = 1u32;

        x * x + 2 * x - 1;
    });
}
```

其他指示符

```
block
expr 表达式
ident 用于变量名或者函数名
item 
pat 模式
path 
stmt 语句
tt 标记树 token tree
ty 类型type
```

### 17.1.2 重载

宏可以重载，从而接受不同的参数组合。在这方面，macro_rules! 的作用类似于匹配（match）的代码块

```
macro_rules! test {
    ($left:expr; and $right:expr) => {
        println!(
            "{:?} and {:?} is {:?}",
            stringify!($left),
            stringify!($right),
            $left && $right
        )
    };
    ($left:expr; or $right:expr) => {
        println!(
            "{:?} and {:?} is {:?}",
            stringify!($left),
            stringify!($right),
            $left || $right
        )
    };
}

fn main() {
    test!(1i32 + 1 == 2i32; and 2i32 * 2 == 4i32);
    test!(true;or false)
}
```

### 17.1.3 重复

宏在参数列表中使用 + 和 * 代表参数出现的次数

```
macro_rules! find_min {
    ($x:expr) => (
        $x
    );

    ($x:expr,$($y:expr),+) => (
        std::cmp::min($x,find_min!($($y),+))
    );
}

fn main() {
    println!("{}", find_min!(1u32));
    println!("{}", find_min!(1u32 + 2, 2u32));
    println!("{}", find_min!(5u32, 2u32 * 3, 4u32));
    println!("{}", find_min!(5u32, 2u32 * 3, 100u32, 20u32, 30u32));//不是很懂
} 
```

## 17.2 不用重复写代码

通过提取函数或侧集的公共部分，就不同重复造轮子了

```
macro_rules! find_min {
    ($x:expr) => (
        $x
    );

    ($x:expr,$($y:expr),+) => (
        std::cmp::min($x,find_min!($($y),+))
    );
}

fn main() {
    println!("{}", find_min!(1u32));
    println!("{}", find_min!(1u32 + 2, 2u32));
    println!("{}", find_min!(5u32, 2u32 * 3, 4u32));
    println!("{}", find_min!(5u32, 2u32 * 3, 100u32, 20u32, 30u32));
}
```

## 17.3 DSL（领域专用语言）

```
macro_rules! calculate { //做了个计算器
    (eval $e:expr) => {{
        {
            let val :usize = $e;
            println!("{} = {}",stringify!{$e},val);
        }
    }};
}

fn main() {
    calculate!(eval 1 +2 );

    calculate!(eval(1 + 2) * (3 / 4))
}
```

## 17.4 可变参数接口

比如说println 就可，再如：

```
macro_rules! calculate {
    // 单个 `eval` 的模式
    (eval $e:expr) => {{
        {
            let val: usize = $e; // Force types to be integers
            println!("{} = {}", stringify!{$e}, val);
        }
    }};

    // 递归地拆解多重的 `eval`
    (eval $e:expr, $(eval $es:expr),+) => {{
        calculate! { eval $e }
        calculate! { $(eval $es),+ }
    }};
}

fn main() {
    calculate! { // 妈妈快看，可变参数的 `calculate!`！
        eval 1 + 2,
        eval 3 + 4,
        eval (2 * 3) + 1
    }
}
```

# 18 错误处理

panic用来测试、不可恢复的错误

`Option` 类型是为了值是可选的、或者缺少值并不是错误的情况准备的

当错误有可能发生，且应当由调用者处理时，使用 `Result`

## 18.1 panic

```
fn give_princess(gift: &str) {
    if gift == "snake" {
        panic!("Aaaaaaa!")
    }
    println!("I love {}s", gift);
}

fn main() {
    give_princess("snake");
    give_princess("teddy bear")
}
```

## 18.2 Option和Unwrap

```
fn give_princess(gift: Option<&str>) {
    match gift {
        Some("snake") => println!("Yuck! I'm throwing that snake in a fire"),
        Some(inner) => println!("{}?,how nice", inner),
        None => println!("no gift? oh well"),
    }
}

fn give_gift(gift: Option<&str>) {
    let inside = gift.unwrap();
    if inside == "snake" {
        panic!("Aaaa")
    }

    println!("I love {}s !!!", inside)
}

fn main() {
    let food = Some("chicken");
    let snake = Some("snake");
    let void = None;
    give_princess(food);
    give_princess(snake);
    give_princess(void);

    let bird = Some("bird");
    let nothing = None;

    give_gift(bird);
    give_gift(nothing);
}
```

### 18.2.1 使用？解开Option

```
#[derive(Clone, Copy)]
struct PhoneNumber {
    area_code: Option<u8>,
    number: u32,
}

#[derive(Clone, Copy)]
struct Job {
    phone_number: Option<PhoneNumber>,
}

struct Person {
    job: Option<Job>,
}

impl Person {
    fn work_phone_area_code(&self) -> Option<u8> {
        self.job?.phone_number?.area_code
    }
}

fn main() {
    let p = Person {
        job: Some(Job {
            phone_number: Some(PhoneNumber {
                area_code: Some(61),
                number: 15278,
            }),
        }),
    };
    assert_eq!(p.work_phone_area_code(), Some(61))
}
```

### 18.2.2 组合算子：map

这个点可以再搞一搞

```
#[derive(Debug)]
enum Food {
    Apple,
    Carrot,
    Potato,
}

#[derive(Debug)]
struct Chopped(Food);
#[derive(Debug)]
struct Peeled(Food);
#[derive(Debug)]
struct Cooked(Food);

fn peel(food: Option<Food>) -> Option<Peeled> {
    match food {
        Some(food) => Some(Peeled(food)),
        None => None,
    }
}

fn chop(peeled: Option<Peeled>) -> Option<Chopped> {
    match peeled {
        Some(Peeled(food)) => Some(Chopped(food)),
        None => None,
    }
}

fn cook(chopped: Option<Chopped>) -> Option<Cooked> {
    chopped.map(|Chopped(food)| Cooked(food))
}

fn process(food: Option<Food>) -> Option<Cooked> {
    food.map(|f| Peeled(f))
        .map(|Peeled(f)| Chopped(f))
        .map(|Chopped(f)| Cooked(f))
}

fn eat(food: Option<Cooked>) {
    match food {
        Some(food) => println!("Mmm,I love {:?}", food),
        None => println!("Oh no! It wasn't edible"),
    }
}

fn main() {
    let apple = Some(Food::Apple);
    let carrot = Some(Food::Carrot);
    let potato = None;

    let cooked_apple = cook(chop(peel(apple)));
    let cooked_carrot = cook(chop(peel(carrot)));

    let cooked_potato = process(potato);

    eat(cooked_apple);
    eat(cooked_carrot);
    eat(cooked_potato);
}
```

### 18.2.3 组合算子：and_then

```
#![allow(dead_code)]

#[derive(Debug)]
enum Food {
    A,
    B,
    C,
}
#[derive(Debug)]
enum Day {
    M,
    T,
    W,
}

fn x(food: Food) -> Option<Food> {
    match food {
        Food::A => None,
        _ => Some(food),
    }
}
fn y(food: Food) -> Option<Food> {
    match food {
        Food::B => None,
        _ => Some(food),
    }
}

fn z(food: Food) -> Option<Food> {
    match x(food) {
        None => None,

        Some(food) => match y(food) {
            None => None,
            Some(food) => Some(food),
        },
    }
}

//使用add_then简化
fn u(food: Food) -> Option<Food> {
    x(food).and_then(y)
}

fn v(food: Food, day: Day) {
    match x(food) {
        Some(food) => println!("{:?},{:?}", food, day),
        None => println!("no"),
    }
}

fn main() {
    let (a, b, c) = (Food::A, Food::B, Food::C);

    v(a, Day::M);
    v(b, Day::T);
    v(c, Day::W);
}	
```

## 18.3 Result

Result用来描述可能的错误而不是可能的不存在

成功找到T元素 Ok(T),没找到返回Err(E)

它的方法，unwrap（），也和Option一样有很多组合算子

使用parse()方法可以返回类型

```
fn multiply(first_num_str: &str, second_num_str: &str) -> i32 {
    let first_number = first_num_str.parse::<i32>().unwrap();
    let second_number = second_num_str.parse::<i32>().unwrap//使用unwrap解包并产生panic

    first_number * second_number
}

fn main() {
    let twenty = multiply("10", "2");
    println!("{}", twenty);

    let tt = multiply("t", "2");
    println!("{}", tt)//解析错误
}
```

### 18.3.1 Result 的map

关于错误类型还要多多实战才行

```
use std::num::ParseIntError;

fn multiply(first_num_str: &str, second_num_str: &str) -> Result<i32, ParseIntError> {
    match first_num_str.parse::<i32>() {
        Ok(first_number) => match second_num_str.parse::<i32>() {
            Ok(second_number) => Ok(first_number * second_number),
            Err(e) => Err(e),
        },
        Err(e) => Err(e),
    }
}

fn print(result: Result<i32, ParseIntError>) {
    match result {
        Ok(n) => println!("{}", n),
        Err(e) => println!("{}", e),
    }
}

fn main() {
    let twenty = multiply("10", "2");
    print(twenty);

    let tt = multiply("t", "2");
    print(tt)
}
```

Option的map、and_then以及很多其他组合算子也为Rust实现了

```
use std::num::ParseIntError;

fn multiply(first_num_str: &str, second_num_str: &str) -> Result<i32, ParseIntError> {
    first_num_str.parse::<i32>().and_then(|first_number| {
        second_num_str
            .parse::<i32>()
            .map(|second_number| first_number * second_number)
    })
}

fn print(result: Result<i32, ParseIntError>) {
    match result {
        Ok(n) => println!("{}", n),
        Err(e) => println!("{}", e),
    }
}

fn main() {
    let twenty = multiply("10", "2");
    print(twenty);

    let tt = multiply("t", "2");
    print(tt)
}
```

### 18.3.2 给Result起别名

在模块层面创建别名非常有帮助，同一模块中的错误通常是同种类型

```
use std::num::ParseIntError;

type AliaseResult<T> = Result<T, ParseIntError>;
fn multiply(first_num_str: &str, second_num_str: &str) -> AliaseResult<i32> {
    first_num_str.parse::<i32>().and_then(|first_number| {
        second_num_str
            .parse::<i32>()
            .map(|second_number| first_number * second_number)
    })
}

fn print(result: AliaseResult<i32>) {
    match result {
        Ok(n) => println!("{}", n),
        Err(e) => println!("{}", e),
    }
}

fn main() {
    print(multiply("10", "2"));
    print(multiply("t", "2"))
}
```

### 18.3.3 提前返回

发生错误时我们可以让函数提前返回，处理逻辑和上面的有点像的

```
use std::num::ParseIntError;

fn multiply(first_num_str: &str, second_num_str: &str) -> Result<i32, ParseIntError> {
    let first_number = match first_num_str.parse::<i32>() {
        Ok(first_number) => first_number,
        Err(e) => return Err(e),
    };

    let second_number = match second_num_str.parse::<i32>() {
        Ok(second_number) => second_number,
        Err(e) => return Err(e),
    };

    Ok(first_number * second_number)
}

fn print(result: Result<i32, ParseIntError>) {
    match result {
        Ok(n) => println!("{}", n),
        Err(e) => println!("{}", e),
    }
}

fn main() {
    print(multiply("10", "2"));
    print(multiply("t", "2"))
}
```

### 18.3.4 引入？

上面遇到错误我们都是Panic了，现在我们使用？来避免Panic，这种方式更简单

```
use std::num::ParseIntError;

fn multiply(first_num_str: &str, second_num_str: &str) -> Result<i32, ParseIntError> {
    let first_number = first_num_str.parse::<i32>()?;
    let second_number = second_num_str.parse::<i32>()?;
    Ok(first_number * second_number)
}

fn print(result: Result<i32, ParseIntError>) {
    match result {
        Ok(n) => println!("{}", n),
        Err(e) => println!("{}", e),
    }
}

fn main() {
    print(multiply("10", "2"));
    print(multiply("t", "2"))
}
```

**try！宏**

在？出现之前，我们是使用try！完成的，现在我们推荐使用？

## 18.4 处理更多错误类型

### 18.4.1 从Option中取出Result

```
fn double_first(vec: Vec<&str>) -> i32 {
    let first = vec.first().unwrap();
    2 * first.parse::<i32>().unwrap()
}

fn main() {
    let nums = vec!["32", "46", "78"];
    let empty = vec![];

    let string = vec!["todo", "32", "78"];

    println!("The first double is {}", double_first(nums));
    println!("The second double is {}", double_first(empty));
    println!("the first double is {}", double_first(string));
}
```

处理混合错误类型最基本的就是使他们相互包含

```
use std::num::ParseIntError;

fn double_first(vec: Vec<&str>) -> Result<Option<i32>, ParseIntError> {
    let opt = vec.first().map(|first| first.parse::<i32>().map(|n| 2 * n));

    opt.map_or(Ok(None), |r| r.map(Some))
}

fn main() {
    let nums = vec!["32", "46", "78"];
    let empty = vec![];
    let string = vec!["todo", "32", "78"];

    println!("the first vec is {:?}", double_first(nums));
    println!("the first vec is {:?}", double_first(empty));
    println!("the first vec is {:?}", double_first(string))
}
```

### 18.4.2 定义一种错误类型

把不同的错误类型视为一种错误类型会简化代码

```
use std::error;
use std::fmt;

type Result<T> = std::result::Result<T, DoubleError>;

#[derive(Debug, Clone)]
struct DoubleError;

impl fmt::Display for DoubleError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Invalid first item to double")
    }
}

impl error::Error for DoubleError {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        None
    }
}

fn double_first(vec: Vec<&str>) -> Result<i32> {
    vec.first()
        .ok_or(DoubleError)
        .and_then(|s| s.parse::<i32>().map_err(|_| DoubleError).map(|i| 2 * i))
}

fn print(result: Result<i32>) {
    match result {
        Ok(n) => println!("the first doubed is {}", n),
        Err(e) => println!("Error {}", e),
    }
}

fn main() {
    let numbers = vec!["42", "93", "18"];
    let empty = vec![];
    let strings = vec!["tofu", "93", "18"];

    print(double_first(numbers));
    print(double_first(empty));
    print(double_first(strings))
}
```

### 18.4.3 把错误装箱

想简单但又想保留原始错误信息，可以把错误装箱，坏处，该错误类型只能在运行时了解，而不能被静态的判别

只要实现了Error trait类型，Box就可以通过From将其转换为Box<Error>,注意写法

```
use std::error;
use std::fmt;

type Result<T> = std::result::Result<T, Box<dyn error::Error>>;

#[derive(Debug, Clone)]
struct EmptyVec;

impl fmt::Display for EmptyVec {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Invalid first to double")
    }
}

impl error::Error for EmptyVec {
    fn description(&self) -> &str {
        "Invalid first item to double"
    }

    fn cause(&self) -> Option<&dyn error::Error> {
        None
    }
}

fn double_first(vec: Vec<&str>) -> Result<i32> {
    vec.first()
        .ok_or_else(|| EmptyVec.into())
        .and_then(|s| s.parse::<i32>().map_err(|e| e.into()).map(|i| 2 * i))
}

fn print(result: Result<i32>) {
    match result {
        Ok(n) => println!("the first double is {}", n),
        Err(e) => println!("Error:{}", e),
    }
}

fn main() {
    let numbers = vec!["42", "93", "18"];

    let empty = vec![];

    let strings = vec!["tofu", "93", "18"];
    print(double_first(numbers));
    print(double_first(empty));
    print(double_first(strings));
}
```

### 18.4.4 ？的其他用法

小技巧太多，要熟练甄别

```
use std::error;
use std::fmt;

type Result<T> = std::result::Result<T, Box<dyn error::Error>>;

#[derive(Debug, Clone)]
struct EmptyVec;

impl fmt::Display for EmptyVec {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Invalid first to double")
    }
}

impl error::Error for EmptyVec {}

fn double_first(vec: Vec<&str>) -> Result<i32> {
    let first = vec.first().ok_or(EmptyVec)?;
    let parsed = first.parse::<i32>()?;
    Ok(2 * parsed)
}

fn print(result: Result<i32>) {
    match result {
        Ok(n) => println!("the first double is {}", n),
        Err(e) => println!("Error:{}", e),
    }
}

fn main() {
    let numbers = vec!["42", "93", "18"];

    let empty = vec![];

    let strings = vec!["tofu", "93", "18"];
    print(double_first(numbers));
    print(double_first(empty));
    print(double_first(strings));
}
```

### 18.4.5 包裹错误

把错误包装也可以把错误包装到自己的错误类型中

```
use std::error;
use std::fmt;
use std::num::ParseIntError;

type Result<T> = std::result::Result<T, Box<dyn error::Error>>;

#[derive(Debug, Clone)]
enum DoubleError {
    EmptyVec,
    Parse(ParseIntError),
}

impl fmt::Display for DoubleError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DoubleError::EmptyVec => write!(f, "please use a vector with at least one element"),
            DoubleError::Parse(ref e) => e.fmt(f),
        }
    }
}

impl error::Error for DoubleError {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        match *self {
            DoubleError::EmptyVec => None,

            DoubleError::Parse(ref e) => Some(e),
        }
    }
}

impl From<ParseIntError> for DoubleError {
    fn from(err: ParseIntError) -> DoubleError {
        DoubleError::Parse(err)
    }
}

fn double_first(vec: Vec<&str>) -> Result<i32> {
    let first = vec.first().ok_or(DoubleError::EmptyVec)?;
    let parsed = first.parse::<i32>()?;
    Ok(2 * parsed)
}

fn print(result: Result<i32>) {
    match result {
        Ok(n) => println!("the first double is {}", n),
        Err(e) => println!("Error:{}", e),
    }
}

fn main() {
    let numbers = vec!["42", "93", "18"];

    let empty = vec![];

    let strings = vec!["tofu", "93", "18"];
    print(double_first(numbers));
    print(double_first(empty));
    print(double_first(strings));
}
```

## 18.5 遍历Result

只能迭代获取后面两个元素

```
fn main() {
    let strings = vec!["tofu", "93", "18"];
    let nums: Vec<_> = strings.into_iter().map(|s| s.parse::<i32>().ok()).collect();
    println!("Results: {:?}", nums)
}
```

使用filter忽略失败的项

```
fn main() {
    let strings = vec!["tofu", "93", "18"];
    let nums: Vec<_> = strings
        .into_iter()
        .filter_map(|s| s.parse::<i32>().ok())
        .collect();
    println!("Results: {:?}", nums)
}
```

使用collect使整个操作失败

```
fn main() {
    let strings = vec!["tofu", "93", "18"];
    let nums: Result<Vec<_>, _> = strings.into_iter().map(|s| s.parse::<i32>()).collect();
    println!("Results: {:?}", nums)
}
```

同样的技巧也可以对Option使用

使用Partition() 收集所有合法的值和错误

```
fn main() {
    let strings = vec!["tofu", "43", "89"];
    let (nums, err): (Vec<_>, Vec<_>) = strings
        .into_iter()
        .map(|s| s.parse::<i32>())
        .partition(Result::is_ok);
    println!("{:?}", nums);
    println!("{:?}", err);
}
```

上述的值还保存在Result中，可以通过一些模式将其取出

```
fn main() {
    let strings = vec!["tofu", "43", "89"];
    let (nums, err): (Vec<_>, Vec<_>) = strings
        .into_iter()
        .map(|s| s.parse::<i32>())
        .partition(Result::is_ok);

    let numbers: Vec<_> = nums.into_iter().map(Result::unwrap).collect();
    let errors: Vec<_> = err.into_iter().map(Result::unwrap_err).collect();
    println!("{:?}", numbers);
    println!("{:?}", errors);
}
```

# 19 标准库类型

标准库提供了很多自定义的类型，在原生类型基础上进行了大量的扩充

- 可增长的 `String`（字符串），如: `"hello world"`
- 可增长的向量（vector）: `[1, 2, 3]`
- 选项类型（optional types）: `Option<i32>`
- 错误处理类型（error handling types）: `Result<i32, i32>`
- 堆分配的指针（heap allocated pointers）: `Box<i32>`

## 19.1 箱子、栈和堆

箱子Box<T>是一个指针，数据存放在堆山，可以使用*解引用

```
use std::mem;

#[allow(dead_code)]
#[derive(Debug, Clone, Copy)]
struct Point {
    x: f64,
    y: f64,
}

#[allow(dead_code)]
struct Rectangle {
    p1: Point,
    p2: Point,
}

fn origin() -> Point {
    Point { x: 0.0, y: 0.0 }
}

fn boxed_origin() -> Box<Point> {
    Box::new(Point { x: 0.0, y: 0.0 })
}

fn main() {
    let point: Point = origin();
    let rectangle: Rectangle = Rectangle {
        p1: origin(),
        p2: Point { x: 3.0, y: 4.0 },
    };

    let boxed_rectangle: Box<Rectangle> = Box::new(Rectangle {
        p1: origin(),
        p2: origin(),
    });

    let boxed_point: Box<Point> = Box::new(origin());

    println!(
        "Point occupies {} bytes in the stack",
        mem::size_of_val(&point)
    );
    println!(
        "Rectangle occupies {} bytes in the stack",
        mem::size_of_val(&rectangle)
    );
    println!(
        "box rectangle occupies {} bytes in the stack",
        mem::size_of_val(&boxed_rectangle)
    );

    let unboxed_point: Point = *boxed_point;
    println!(
        "Unboxed point occupies {} bytes in the stack",
        mem::size_of_val(&unboxed_point)
    )
}
```

## 19.2 动态数组

动态数组三要素，指向数据的指针、长度和容量。超过容量会重新分配更大的内存，基本操作没有什么特别的

```
fn main() {
    let collected_iterator: Vec<i32> = (0..10).collect();
    println!("{:?}", collected_iterator);

    let mut xs = vec![1i32, 2, 3];
    println!("vec is {:?}", xs);

    xs.push(4i32);
    println!("vec is {:?}", xs);

    println!("the popped element is {:?}", xs.pop());

    for x in xs.iter() {
        println!("> {}", x);
    }

    for (i, x) in xs.iter().enumerate() {
        println!("in position {} we have value {}", i, x);
    }

    for x in xs.iter_mut() {
        *x *= 3;
    }
    println!("xs is  {:?}", xs)
}
```

## 19.3 字符串

两种字符串类型

String 和 &str ，Str是一个Vec(Vec<u8>),而 &str指向的是一个切片

```
fn main() {
    let pangram: &'static str = "the quick brown fox jumps over the lazy dog";
    println!("{}", pangram);

    for word in pangram.split_whitespace().rev() {
        println!("{}", word)
    }

    let mut chars: Vec<char> = pangram.chars().collect();
    chars.sort(); //排序
    chars.dedup(); //去重

    println!("{:?}", chars);

    let mut string = String::new();
    for c in chars {
        string.push(c);
        string.push_str(",")
    }

    let chars_to_trim: &[char] = &[' ', ','];
    let trimed_str: &str = string.trim_matches(chars_to_trim);
    println!("{}", trimed_str);

    let alice = String::from("I like dogs");
    let bob: String = alice.replace("dog", "cat");

    println!("bob says I like dogs {}", bob);
    println!("alice says I like dogs {}", alice);

    println!("")
}
```

字面量与转义字符

这个以后再说吧了解吧，感觉目前实用不是很大

## 19.4 选项Option

Option前面我们详细讲过了

## 19.5 Result

Result能够返回为什么会失败的问题，前面的我们也详细讲过了

## 19.6 Panic！

产生恐慌并会退栈，运行时通过调用析构函数释放所有资源

## 19.7 Hash表

哈希表的键可以是任意实现了Eq或者Hash trait的类型

两种创建方式

```
HashMap::with_capacity(uint) 创建具有初始容量的HashMap，也可以使用HashMap::new()来获得带有默认容量的HashMap
```

```
use std::collections::HashMap;

fn call(number: &str) -> &str {
    match number {
        "798" => "sorry",
        "675" => "hello",
        _ => " Hi ",
    }
}

fn main() {
    let mut contacts = HashMap::new();

    contacts.insert("da", "798");
    contacts.insert("fa", "675");

    match contacts.get(&"da") {
        Some(&number) => println!(" calling da: {}", call(number)),
        _ => println!("wrong number"),
    }

    contacts.remove(&"da");

    for (contact, &number) in contacts.iter() {
        println!(" calling fa: {}:{}", contact, call(number));
    }
}
```

### 19.7.1 更改类型或者自定义关键字类型

f32和f64没有实现Hash，给自定义类型实现Eq和Hash，使用宏即可

```
#[derive(PartialEq,Eq,Hash)]
```

```
use std::collections::HashMap;

#[derive(PartialEq, Eq, Hash)]
struct Account<'a> {
    username: &'a str,
    password: &'a str,
}

struct AccountInfo<'a> {
    name: &'a str,
    email: &'a str,
}

type Accounts<'a> = HashMap<Account<'a>, AccountInfo<'a>>;

fn try_logon<'a>(accounts: &Accounts<'a>, username: &'a str, password: &'a str) {
    println!("Username: {}", username);
    println!("Password: {}", password);
    println!("attempting logon...");

    let logon = Account {
        username: username,
        password: password,
    };

    match accounts.get(&logon) {
        Some(account_info) => {
            println!("Successful logon");
            println!("Name:{}", account_info.name);
            println!("Email: {}", account_info.email);
        }
        _ => println!("Login failed"),
    }
}
fn main() {
    let mut accounts: Accounts = HashMap::new();

    let account = Account {
        username: "shiyivei",
        password: "123456",
    };

    let account_info = AccountInfo {
        name: "jianquan",
        email: "jianquan@gmail.com",
    };

    accounts.insert(account, account_info);

    try_logon(&accounts, "shiyivei", "123456");
    try_logon(&accounts, "shiyivei", "123456");
}
```

### 19.7.2 散列集HashSet

关于集合的操作，差集，并集，交集，对称差，牛的，Rust居然在标准库里实现了这些

```
use std::collections::HashSet;

fn main() {
    let mut a: HashSet<i32> = vec![1i32, 2, 3].into_iter().collect();
    let mut b: HashSet<i32> = vec![2i32, 3, 4].into_iter().collect();

    assert!(a.insert(4));
    assert!(a.contains(&4));

    // assert!(b.insert(4), "the value 4 already in b");

    b.insert(5);

    println!("A: {:?}", a);
    println!("B: {:?}", b);

    
    println!("Union: {:?}", a.union(&b).collect::<Vec<&i32>>());
    println!("Difference: {:?}", a.difference(&b).collect::<Vec<&i32>>());
    println!(
        "intersection: {:?}",
        a.intersection(&b).collect::<Vec<&i32>>()
    );
    println!(
        "Symmetric Difference: {:?}",
        a.symmetric_difference(&b).collect::<Vec<&i32>>()
    );
}
```

## 19.8 引用计数Rc

这个也是神奇，为了所有权这件事真是绞尽脑汁

```
use std::rc::Rc;

fn main() {
    let rc_examples = "Rc example".to_string();

    {
        println!("-----------start----------------");

        let rc_a: Rc<String> = Rc::new(rc_examples);

        println!("Reference Count of rc_a : {}", Rc::strong_count(&rc_a));

        {
            println!("---------------clone---------------");

            let rc_b: Rc<String> = Rc::clone(&rc_a);

            println!("Reference Count of rc_b : {}", Rc::strong_count(&rc_b));
            println!("Reference Count of rc_a : {}", Rc::strong_count(&rc_a));

            println!("equal {}", rc_a.eq(&rc_b));

            println!("length {}", rc_a.len());
            println!("value rc_b {}", rc_b);

            println!("-----------end--------------")
        }

        println!("Reference Count of rc_a : {}", Rc::strong_count(&rc_a));

        println!("-----------outofscope-----------")
    }
}
```

## 19.9 共享的引用计数Arc

线程之间所有权需要共享时，可以使用Arc（这个结构通过Clone实现）

```
use std::sync::Arc;
use std::thread;

fn main() {
    let apple = Arc::new("the same apple");

    for _ in 0..10 {
        let apple = Arc::clone(&apple);
        thread::spawn(move || {
            println!("{:?}", apple);
        });
    }
}
```

# 20 标准库的更多介绍

## 20.1 线程

rust通过使用spawn函数提供了创建本地操作系统的（native OS）线程的机制，该函数的餐食是一个通过值捕获变变量的闭包。线程由操作系统调度

```
use std::thread;
static NTHEADS: i32 = 10;
fn main() {
    let mut children = vec![];

    for i in 0..NTHEADS {
        //
        children.push(thread::spawn(move || {
            println!("this is thread number {}", i)
        }));
    }

    for child in children {
        let _ = child.join();
    }
}
```

### 20.1.1 测试实例

```
use std::thread;

fn main() {
    //定义了一个变量
    let data = "2452015845283756927556438457";
    //定义了一个空数组
    let mut children = vec![];

    //把数据分段
    let chunked_data = data.split_whitespace();

    for (i, data_segment) in chunked_data.enumerate() {
        println!("data segment {} is \"{}\"", i, data_segment);

        children.push(thread::spawn(move || -> u32 {
            let result = data_segment
                .chars()
                .map(|c| c.to_digit(10).expect("should be a digit"))
                .sum();

            println!("process segment {},result: {}", i, result);
            result
        }));
    }

    let mut intermediate_sums = vec![];
    for child in children {
        let intermediate_sum = child.join().unwrap();
        intermediate_sums.push(intermediate_sum);
    }

    let final_result = intermediate_sums.iter().sum::<u32>();

    println!("inal result: {}", final_result)
}
```

## 20.2 通道

rust为线程之间的通信提供了异步通道，通道允许两个端点之间的信息单向流动，sender和receiver

```
use std::sync::mpsc;
use std::sync::mpsc::{Receiver, Sender};
use std::thread;

static NTHEADS: i32 = 3;
fn main() {
    let (tx, rx): (Sender<i32>, Receiver<i32>) = mpsc::channel();

    for id in 0..NTHEADS {
        let thread_tx = tx.clone();

        thread::spawn(move || {
            thread_tx.send(id).unwrap();

            println!("thread {} finished", id);
        });
    }

    let mut ids = Vec::with_capacity(NTHEADS as usize);
    for _ in 0..NTHEADS {
        ids.push(rx.recv());
    }

    println!("{:?}", ids)
}
```

## 20.3 路径

Path结构体代表了底层文件系统的文件路径。 Path分为两种：UNIX和Windows::Path,prelude 会选择并输出符合平台类型的Path种类

Path可以从OsStr类型创建，并且它提供了数种方法，用户获取路径指向的文件/目录的信息

Path在内部是存储为若干字节的（Vec<u8>）的vector。因此将Path转换为&str并非零开销（free），且可能失败（返回的是一个Option）

```
use std::path::Path;
fn main() {
    let path = Path::new(".");

    let display = path.display();

    let new_path = path.join("a").join("b").join("c");
    match new_path.to_str() {
        None => panic!("new path is not valid UTF-8 sequence"),
        Some(s) => println!("new path is {}", s),
    }
}
```

## 20.4 文件输出

File 的所有方法都返回的是io::Result<T> 类型，它是Result<T,io::Error> 的别名

### 20.4.1 打开文件

open以只读模式打开文件

```
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

fn main() {
    let path = Path::new("hello.txt");

    let display = path.display();

    let mut file = match File::open(&path) {
        Err(why) => panic!("could open {}: {:?}", display, why),
        Ok(file) => file,
    };

    let mut s = String::new();
    match file.read_to_string(&mut s) {
        Err(why) => panic!("could not read {} : {:?}", display, why),
        Ok(_) => print!("{} contains \n{}", display, s),
    }
}
```

### 20.4.2 创建文件create

create静态方法以只写模式打开一个文件，文件存在会销毁旧内容，否则新建文件

```
static LOREM_IPSUM: &'static str =
    "Lorem ipsum dolor sit amet, consectetur adipisicing elit, sed do eiusmod
tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim veniam,
quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea commodo
consequat. Duis aute irure dolor in reprehenderit in voluptate velit esse
cillum dolore eu fugiat nulla pariatur. Excepteur sint occaecat cupidatat non
proident, sunt in culpa qui officia deserunt mollit anim id est laborum.
";

use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

fn main() {
    let path = Path::new("out/lorem_ipsum.txt");
    let display = path.display();

    let mut file = match File::create(&path) {
        Err(why) => panic!("couldn't create {}: {:?}", display, why),
        Ok(file) => file,
    };

    match file.write_all(LOREM_IPSUM.as_bytes()) {
        Err(why) => {
            panic!("couldn't write to {}: {:?}", display, why)
        }
        Ok(_) => println!("successfully wrote to {}", display),
    }
}
```

## 20.4.3 读取行

方法lines（）在文件行上返回一个迭代器

```
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    if let Ok(lines) = read_lines("./hosts") {
        for line in lines {
            if let Ok(ip) = line {
                println!("{}", ip);
            }
        }
    }
}
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
```

这个过程比再内存中创建String更有效，特别是处理更大的文件

## 20.5 子进程

```
use std::process::Command;

fn main() {
    let output = Command::new("rustc")
        .arg("--version")
        .output()
        .unwrap_or_else(|e| panic!("failed to execute process: {}", e));

    if output.status.success() {
        let s = String::from_utf8_lossy(&output.stdout);
        print!("rustc succeed and stdout was:\n{}", s);
    } else {
        let s = String::from_utf8_lossy(&output.stderr);
        print!("rustc failed and stderr was: \n {}", s)
    }
}
```

### 20.5.1 管道

```
use std::io::prelude::*;

use std::process::{Command, Stdio};

static PANGRAM: &'static str = "the quick brown fox jumped over the lazy dog\n";
fn main() {
    let proccess = match Command::new("wc")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
    {
        Err(why) => panic!("couldn't spawn wc: {:?}", why),
        Ok(process) => process,
    };

    match proccess.stdin.unwrap().write_all(PANGRAM.as_bytes()) {
        Err(why) => panic!("couldn't write to wc stdin: {:?}", why),
        Ok(_) => println!("sent pangram to wc"),
    }

    let mut s = String::new();

    match proccess.stdout.unwrap().read_to_string(&mut s) {
        Err(why) => panic!("couldn't read  wc from stdout: {:?}", why),
        Ok(_) => print!("wc responded with: \n{:?}", s),
    }
}
```

### 20.5.2 等待

如果想等待一个process;;Child完成，必须调用Child::wait,这会返回一个process::ExitStatus

```
use std::process::Command;

fn main() {
    let mut child = Command::new("sleep").arg("5").spawn().unwrap();
    let _resukt = child.wait().unwrap();

    println!("reached end of main");
}
```

### 20.6 文件系统操作

牛的，Rust居然有一系列的命令可以用于文件的操作

```
use std::fs;
use std::fs::{File, OpenOptions};
use std::io;
use std::io::prelude::*;
use std::os::unix;
use std::path::Path;

fn cat(path: &Path) -> io::Result<String> {
    let mut f = File::open(path)?;
    let mut s = String::new();
    match f.read_to_string(&mut s) {
        Ok(_) => Ok(s),
        Err(e) => Err(e),
    }
}

fn echo(s: &str, path: &Path) -> io::Result<()> {
    let mut f = File::create(path)?;

    f.write_all(s.as_bytes())
}

fn touch(path: &Path) -> io::Result<()> {
    match OpenOptions::new().create(true).write(true).open(path) {
        Ok(_) => Ok(()),
        Err(e) => Err(e),
    }
}
fn main() {
    println!("`mkdir a`");

    match fs::create_dir("a") {
        Err(why) => println!("! {:?}", why.kind()),
        Ok(_) => {}
    }

    println!("`echo hello > a/b.txt`");
    echo("hello", &Path::new("a/b.txt")).unwrap_or_else(|why| println!("! {:?}", why.kind()));

    println!("`mkdir -p a/c/d`");
    fs::create_dir_all("a/c/d").unwrap_or_else(|why| println!("! {:?}", why.kind()));

    println!("`touch a/c/e.txt");
    touch(&Path::new("a/c/e.txt")).unwrap_or_else(|why| println!("! {:?}", why.kind()));

    println!("`ln -s ../b.txt a/c/b.txt`");
    if cfg!(target_family = "unix") {
        unix::fs::symlink("../b.txt", "a/c/b.txt").unwrap_or_else(|why| {
            println!("! {:?}", why.kind());
        });
    }

    println!("`cat a/c/b.txt`");
    match cat(&Path::new("a/c/b.txt")) {
        Err(why) => println!("! {:?}", why.kind()),
        Ok(s) => println!("> {}", s),
    }
    println!("`ls a`");

    match fs::read_dir("a") {
        Err(why) => println!("! {:?}", why.kind()),
        Ok(paths) => {
            for path in paths {
                println!("> {:?}", path.unwrap().path())
            }
        }
    }

    println!("`rm a/c/e.txt`");
    fs::remove_file("a/c/e.txt").unwrap_or_else(|why| {
        println!("! {:?}", why.kind());
    });

    println!("`rmdir a/c/d`");

    fs::remove_dir("a/c/d").unwrap_or_else(|why| {
        println!("! {:?}", why.kind());
    });
}
```

另外就可以使用cat函数用？标记

## 20.7.1 程序参数

命令行参数使用

```
std::env::args进行接收，返回一个迭代器
```

```
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    println!("My path is {}", args[0]);

    println!("I got {:?} arguments: {:?}", args.len() - 1, &args[1..]);
}
```

## 20.7.1 参数解析

可以用模式匹配来接些简单的参数，这在web服务中非常常用

```
use std::env;

fn increase(number: i32) {
    println!("{}", number + 1)
}

fn decrease(number: i32) {
    println!("{}", number - 1)
}

fn help() {
    println!(
        "usage:
    match_args <string> Check whether given string is the answer.
    match_args {{increase|decrease}} <integer>
    ncrease or decrease given integer by one"
    )
}
fn main() {
    let args: Vec<String> = env::args().collect();

    match args.len() {
        1 => {
            println!("My name is 'match_args', Try pass some arguments");
        }

        2 => match args[1].parse() {
            Ok(42) => println!("This is the answer"),
            _ => println!("This is not the answer"),
        },

        3 => {
            let cmd = &args[1];
            let num = &args[2];

            let number: i32 = match num.parse() {
                Ok(n) => n,
                Err(_) => {
                    println!("error: second argument is not an integer");
                    help();
                    return;
                }
            };

            match &cmd[..] {
                "increase" => increase(number),
                "decrease" => decrease(number),
                _ => {
                    println!("error:invalid argument");
                    help();
                }
            }
        }

        _ => {
            help();
        }
    }
}
```

## 20.8 外部语言函数接口

Rust提供了到C语言库的外部函数接口。外部语言函数必须在一个external代码块中声明，且该代码块要带有一个包含库名称的#[link]属性

```
use std::fmt;

#[link(name = "m")]

extern "C" {
    fn csqrtf(z: Complex) -> Complex;

    fn ccosf(z: Complex) -> Complex;
}

fn cos(z: Complex) -> Complex {
    unsafe { ccosf(z) }
}

fn main() {
    let z = Complex { re: -1., im: 0. };

    let z_sqrt = unsafe { csqrtf(z) };

    println!("the square root of {:?} is {:?}", z, z_sqrt);

    println!("cos({:?}) = {:?}", z, cos(z));
}

#[repr(C)]
#[derive(Clone, Copy)]
struct Complex {
    re: f32,
    im: f32,
}

impl fmt::Debug for Complex {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.im < 0. {
            write!(f, "{}-{}i", self.re, -self.im)
        } else {
            write!(f, "{}+{}i", self.re, self.im)
        }
    }
}
```

# 21 测试

Rust测试有三种，大家都知道的

# 22 不安全的操作

```
解引用裸指针
通过 FFI 调用函数（这已经在之前的章节介绍过了）
调用不安全的函数
内联汇编（inline assembly）
```

**原始指针**

解引用裸指针只能通过不安全的代码块执行

```
fn main() {
    let raw_p: *const u32 = &10;

    unsafe { assert!(*raw_p == 10) }
}
```

**调用不安全的函数**

这块知识还是比较模糊的，后面结合实际案例再继续研究了

```
use std::slice;

fn main() {
    let some_vector = vec![1, 2, 3, 4];

    let pointer = some_vector.as_ptr();

    let length = some_vector.len();

    unsafe {
        let my_slice: &[u32] = slice::from_raw_parts(pointer, length);

        assert_eq!(some_vector.as_slice(), my_slice);
    }
}
```

# 23 兼容性

## 23.1 原始标志符

原始标志符允许我们使用通常不允许的关键字

```
extern crate foo;

fn main() {

    foo::r#try():
}
```

# 24 补充

基础设施

```
文档：通过附带的 rustdoc 生成库文档给用户
测试：为库创建测试套件，确保库准确地实现了你想要的功能
基准测试（benchmark）：对功能进行基准测试，保证其运行速度足够快
```

## 24.1 文档

注释

文档属性

上述都比较简单，就不一一罗列了

## 24.2 Playpen

即Rust PlayGround，想要踏实学习Rust的话还会老老实实配置开发环境吧，也不是很难
