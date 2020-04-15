#### 闭包  

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
