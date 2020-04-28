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
