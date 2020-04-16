### 闭包  

和 `javascript`, `Golang` 不同  
`rust` 一定要写成 `lambda` 表达式才能捕获外部变量  

```rs

fn main() {
  let x: i32 = 1;

  fn inner() -> i32 {
    x + 1            // <-----------  编译错误 can't capture dynamic enviroment in a fn item
  }

  let y = inner();
  println!("{}", y)
}

///////////////

fn main() {
  let x: i32 = 1;
  let inner = || { x + 1 };   // <------------- 编程成功

  let y = inner();
  println!("{}", y)
}


```


#### escaping closure 

有些时候 我们的闭包的生命周期可能会超过一个函数，比如 我们可以将此闭包存储到某个数据结构中，  
在当前函数返回之后继续使用。闭包作为函数返回值被传递出去。  
使用 `move` 关键词 一般用于此种情况

```rs

fn make_adder(x: i32) -> Box<dyn Fn(i32) -> i32> {
  Box::new(move |y| x + y)   //  <----------  move
}

fn main() {
  let f = make_adder(3);
  println!("{}", f(1));
  println!("{}", f(10));
}

```




