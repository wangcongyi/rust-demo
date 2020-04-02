### partialOrd / Ord

对于集合 X 中的元素 a, b, c:  
> 如果 a < b 则一定有 !(a > b) 反之亦然。                      称之为  **对称性**  
> 如果 a < b 且 b < c 则 a < b                                称之为  **传递性**  
> 如果 X 中所有元素都存在 a < b 或 a > b 或 a == b 三者比其一   称之为  **完整性**  

如果集合 X 中所有元素具备上述前两条特性， 则称 X 是 **偏序**  
同时具备以上所有特性，则称 X 是 **全序**


从以上定义可以看出，浮点数不具备 全序 的特征。因为浮点数中特殊的值 NaN 不满足完整性。

```rs

fn main() {
  let nan = std::f32::NAN;
  let x : f32= 1.0;
  println!("{}",nan < x);       // false
  println!("{}",nan > x);       // false
  println!("{}",nan == x);      // false
}

```
