### 容器与迭代器  


#### 容器  

|  容器   | 描述  |
|  :----: | :---- |
|  Vec         |  可变长数组 连续存储  |
|  VecDeque    |  双向队列 适用于从头部或尾部删除插入数据  |
|  LinkedList  |  双向链表 非连续存储  |
|  HashMap     |  基于 Hash 算法存储一系列键值对 |
|  BTreeMap    |  基于 B 树存储一系列键值对 |
|  HashSet     |  基于 Hash 算法的集合 相当于没有值的 HashMap |
|  BTreeSet    |  基于 B 树的集合 相当于没有值的 BTreeMap |
|  BinaryHeap  |  基于二叉堆实现的优先级队列 |


```rs

fn main() {

  let v1 = Vec::<i32>::new();   //  or  let v1 = Vec::<i32>::default()

  let v2 = Vec::<String>::with_capacity(1000);   // with_capacity 方法可以预先分配一个较大空间，避免插入数据动态扩容

  let v3 = vec![1, 2, 3, 4];   //  也可以利用宏来初始化
  
  let mut v4 = Vec::new();
  v4.push(1);
  v4.extend_from_slice(&[10, 20, 30, 40, 50]);
  v4.insert(2, 100);
  println!("capacity: {}, length: {}", v4.capacity(), v4.len());
  
  v4[5] = 55;          //  调用 IndexMut 运算符， 可以写入数据
  let i = v4[5];
  println!("{:?}", v4);
  println!("{}", i);
  
  //    Index 运算符直接访问， 如果越界则会造成 panic 而 get 方法不会，  返回一个 Option<T>
   if let Some(i) = v4.get(6) {
    println!("{}", i);
  }

  let slice = &v4[4..];
  println!("{:?}", slice);
 
}


```



```rs

use std::collections::HashMap;

#[derive(Hash, Eq, PartialEq, Debug)]
struct Person {
  first_name: String,
  last_name: String,
}

impl Person {
  fn new(first: &str, last: &str) -> Self {
    Person {
      first_name: first.to_string(),
      last_name: last.to_string(),
    }
  }
}

fn main() {
  let mut book = HashMap::new();
  book.insert(Person::new("AA", "aa"), "111");
  book.insert(Person::new("BB", "bb"), "222");
  book.insert(Person::new("CC", "cc"), "333");

  let p = Person::new("AA", "aa");

  if let Some(number) = book.get(&p) {
    println!("{}", number);
  }

  book.remove(&p);

  println!("{}", book.contains_key(&p));

  book.entry(Person::new("DD", "dd")).or_insert("444");
  println!("{:?}", book);         //  HaspMap 不保证遍历结果是顺序的   BTreeMap 可以
}

```


```rs 

// BTrrMap 比 HashMap 多的一项功能是 不仅可以查询单个 key 的结果， 还可以查询一个区间的结果

use std::collections::BTreeMap;

fn main() {
  let mut map = BTreeMap::new();
  map.insert(1, "a");
  map.insert(2, "b");
  map.insert(3, "c");
  map.insert(4, "d");
  map.insert(5, "e");
  map.insert(6, "f");
  map.insert(7, "g");                 //  相同 key 会被之后的覆盖掉

  for (k, v) in map.range(2..6) {     //  前闭后开
    println!("{}: {}", k, v); 
  }
}

```


