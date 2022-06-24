### 智能指针

> 智能指针是一种数据结构  
  1. 行为和指针类似
  2. 有额外的元数据和功能

> 引用和智能指针的其他不同  
  1. 引用： 只能借用数据
  2. 智能指针： 很多时候都能拥有所指向的数据

> 智能指针的例子  
  1. String 和 Vec<T>  
  2. 有拥有一块内存区域，且运行用户对其操作  
  3. 拥有元数据（例如容量等）  
  4. 提供额外的功能或保障（String保障其数据是合法的 UTF-8 编码）

> 标准库中的常见智能指针  
  1. Box<T>： 在 heap 内存上分配值  
  2. Rc<T>: 启用多重所有权的引用计数类型  
  3. Ref<T> 和 RefMut<T>, 通过 RefCell<T> 访问： 在运行时而不是编译时强制借用规则的类型




#### Box<T>
- 在 heap 上存储数据 而不是 stack
- 在编译的时候， rust 需要知道一个类型所占的空间大小，但递归类型的大小无法在编译时确定 
  

#### Deref Trait  
##### 解引用与可变性  
  - 可使用 DerefMut trait 重载可变引用的 * 运算符  
  - 在类型和 trait 在以下三种情况发生时，rust 会执行 deref coercion
    1. 当 T: Deref<Target=U>, 允许 &T 转换为 &U  
    2. 当 T: DerefMut<Target=U>, 允许 &mut T 转换为 &mut U  
    3. 当 T: Deref<Target=U>, 允许 &mut T 转换为 &U  

  
