#### References adn Borrowing

`&` 符号就是 **引用** 允许你使用值但不获取其所有权  
与 `&` 引用相反的操作就是 **解引用** (dereferencing) 解引用运算符为 `*`

```rs

fn calculate_length(s: &String) -> usize {
  s.len()
}

fn main() {
  let s = String::from("hello");
  let len = calculate_length(&s);
  println!("{} {}", s, len)
}

```

我们将创建一个引用的行为称为 **借用** (borrowing) 正如现实生活中 如果一个人拥有某样东西，可以从那里借来 用完了必须还回去。  
但 变量默认是不可变的，引用也一样 不允许改变借用的变量的值。  
我们必须将变量改为 `mut` 并且  
1. 在同一个时间只能有一个对其可变引用。  
2. 我们也不能同时使用可变与不可变引用

```rs

fn calculate_length(s: &mut String) -> usize {
  s.push_str(" world");
  s.len()
}

fn main() {
  let mut s = String::from("hello");
  let len = calculate_length(&mut s);
  println!("{} {}", s, len)
}

///
///  let mut s = String::from("hello");
///  let r1 = &mut s;   <-----  cannot borrow `s` as mutable more than once at a time
///  let r2 = &mut s;   <-----  cannot borrow `s` as mutable more than once at a time

///
///  let mut s = String::from("hello");
///  let r1 = &s; 
///  let r2 = &s; 
///  let r3 = &mut s;  <-----  cannot borrow `s` as mutable because it is also borrowed as immutable

```

#### recap 

在任意给定时间，要么 只能有一个可变引用，要么 只能有多个不可变引用




