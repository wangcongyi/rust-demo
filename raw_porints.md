### 原始指针  

> Raw Pointer 原始指针 是没有 rust 标准保障的内存地址。  

> 语法  
  1. 不可变 raw pointer: *const T  
  2. 可变 raw pointer: *mut T  

```rs

fn main() {
  let a: i64 = 42;
  let a_ptr: *const i64 = &a as *const i64;
  println!("a: {} ({:p})", a, a_ptr);
}

```

> 解引用 (dereference): 通过指针从RAM 内存提取数据的过程叫做对指针进行解引用 (dereferencing a pointer) 

```rs

fn main() {
  let a: i64 = 42;
  let a_ptr = &a as *const i64;
  let a_addr = unsafe { std::mem::transmute(a_ptr) };
  println!("a: {} ({:p} ... 0x{:x})", a, a_ptr, a_addr + 7);
}

```

|  名称  |  简介  |  强项  |  弱项  |
|  ----  | ----  |  ----  | ----  |
| Raw Pointer | *mut T 和 *const T                                     | 速度快                                          |  Unsafe |
| Box<T>      | 可以把任何东西都放在 Box 里，接收几乎任何类型的长期存储    | 将值集中存储在 Heap                              | 大小增加  |
| Rc<T>       | 是 rust 能干而吝啬的薄记员，知道谁借了什么，何时借了什么   | 对值的共享访问                                   | 大小增加，运行时成本，线程不安全 |  
| Arc<T>      | 可以跨线程共享值，保证这些值不会相互干扰                  | 对值的共享访问，线程安全                          | 大小增加，运行时成本  |
| Cell<T>     | 具有可变不可变值的能力                                  | 内部可变性                                       | 大小增加，性能 |  
| RefCell<T>  | 对不可变引用执行改变                                    | 内部可变性，可与仅接收不可变引用的 Rc、Arc 嵌套使用 | 大小增加，运行时成本，缺乏编译时保障 |  





