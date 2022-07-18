#### 匹配模式

```

fn main() {
  let favorite_color: Option<&str> = None;
  let is_tuesday = false;
  let age: Result<u8, _> = "34".parse();

  if let Some(color) = favorite_color {
    println!("Using your favorite color: {}, as the background color", color);
  } else if is_tuesday {
    println!("Tuesday is green day!");
  } else if let Ok(age) = age {
    if age > 30 {
      println!("Using purple as the background color");
    } else {
      println!("Using orange as the background color");
    }
  } else {
    println!("Using blue as the background color");
  }
}

```


```

fn main() {
  let mut stack = Vec::new();

  stack.push(1);
  stack.push(2);
  stack.push(3);

  while let Some(top) = stack.pop() {
    println!("{}", top);
  }
}

```

> 模式的两种形式
  - 可辩驳：对某些可能的值，无法进行匹配模式：  
    例如：  if let Some(x) = xxx;
    
  - 无可辩驳：能匹配任何可能的值的模式：  
    例如： let x = 5;

- 函数参数， let 语句， for 循环只接收无可辩驳的模式  
- if let 和 while let 可以接收可辩驳和无可辩驳的模式
    
    
    
    

