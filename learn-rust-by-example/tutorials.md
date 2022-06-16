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

】**指针和引用**

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

crate是rust编译的独立单元，可以被编译为二进制执行文件或者库文件。可以通过rustc的选项 -- crate-type重载

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

虽然是泛型，但约束要求必须要实现的trait

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

