//use std::io;
//use std::cmp::Ordering;
use rand::Rng;
use std::fmt;

use List::*;

#[derive(Debug)]
struct MinMax(i64, i64);

impl fmt::Display for MinMax{
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result{
		write!(f, "({}, {})", self.0, self.1)
	}
}

#[derive(Debug)]
struct Point2D {
    x: f64,
    y: f64,
}
// 类似地对 `Point2D` 实现 `Display`
impl fmt::Display for Point2D {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // 自定义格式，使得仅显示 `x` 和 `y` 的值。
        write!(f, "x: {}, y: {}", self.x, self.y)
    }
}



#[derive(Debug)]
struct Printable(i32);

//#[derive(Debug)]
//struct DebugPrintable(i32);
struct List1(Vec<i32>);

impl fmt::Display for List1 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // 使用元组的下标获取值，并创建一个 `vec` 的引用。
        let vec = &self.0;

        write!(f, "[")?;

        // 使用 `v` 对 `vec` 进行迭代，并用 `count` 记录迭代次数。
        for (count, v) in vec.iter().enumerate() {
            // 对每个元素（第一个元素除外）加上逗号。
            // 使用 `?` 或 `try!` 来返回错误。
            if count != 0 { write!(f, ", ")?; }
            write!(f, "{}: {}", count, v)?;
        }

        // 加上配对中括号，并返回一个 fmt::Result 值。
        write!(f, "]")
    }
}

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

	fn len(&self) -> u32 {
		match *self {
			Cons(_, ref tail) => 1 + tail.len(),
			Nil => 0
		}
	}

	fn stringify(&self) -> String {
		match *self {
			Cons(head, ref tail) => {
				format!("{}, {}", head, tail.stringify())
			},
			Nil => {
				format!("Nil")
			},
		}
	}
}

fn main() {
    

    let secret_number = rand::thread_rng().gen_range(1, 101);

	println!("Hello, world! {}", secret_number);

	println!("Now {:?} will print!", Printable(3));
	let minmax = MinMax(0, 14);
    println!("Compare structures:");
    println!("Display: {}", minmax);
    println!("Debug: {:?}", minmax);

    let big_range =   MinMax(-300, 300);
    let small_range = MinMax(-3, 3);

    println!("The big range is {big} and the small is {small}",
             small = small_range,
             big = big_range);


    let point = Point2D { x: 3.3, y: 7.2 };

    println!("Compare points:");
    println!("Display: {}", point);
    println!("Debug: {:?}", point);


    let v = List1(vec![1, 2, 3]);
    println!("{}", v);


    // loop{
    //   	println!("Input your guess. ");
	   //  let mut guess = String::new();

	   //  io::stdin().read_line(&mut guess)
	   //  	.expect("Failed to read Line");
	    	
	   //  let guess: u32 = match guess.trim().parse(){
	   //  	Ok(num) => num, 
	   //  	Err(_) => continue,
	   //  };
	   //  println!("You guessed: {}", guess);

	   //  match guess.cmp(&secret_number){
	   //  	Ordering::Less => println!("Too small!"),
	   //  	Ordering::Greater => println!("Too big!"),
	   //  	Ordering::Equal => {
	   //  		println!("You win!");
	   //  		break;
	   //  	}
	   //  }

	   //  //println!("You guessed: {}", guess);  	
    // }

    //linked-list
    let mut list = List::new();

    list = list.prepend(1);
    list = list.prepend(2);
    list = list.prepend(3);

    println!("linked list has length: {}", list.len());
    println!("{}", list.stringify());


    // 通过闭包和函数分别实现自增。
    // 译注：下面这行是使用函数的实现
    fn  function_inc           (i: i32) -> i32 { i + 1 }

    // 闭包是匿名的，这里我们将它们绑定到引用。
    // 类型标注和函数的一样，不过类型标注和使用 `{}` 来围住函数体都是可选的。
    // 这些匿名函数（nameless function）被赋值给合适地命名的变量。
    let closure_annotated = |i: i32| -> i32 { i + 1 };
    let closure_inferred  = |i     |          i + 1  ;
    
    // 译注：将闭包绑定到引用的说法可能不准。
    // 据[语言参考](https://doc.rust-lang.org/beta/reference/types.html#closure-types)
    // 闭包表达式产生的类型就是 “闭包类型”，不属于引用类型，而且确实无法对上面两个
    // `closure_xxx` 变量解引用。
    
    let i = 1;
    // 调用函数和闭包。
    println!("function: {}", function_inc(i));
    println!("closure_annotated: {}", closure_annotated(i));
    println!("closure_inferred: {}", closure_inferred(i));

    // 没有参数的闭包，返回一个 `i32` 类型。
    // 返回类型是自动推导的。
    let one = || 1;
    println!("closure returning one: {}", one());



    let color = "green";
    // 这个闭包打印 `color`。它会立即借用（通过引用，`&`）`color` 并将该借用和
    // 闭包本身存储到 `print` 变量中。`color` 会一直保持被借用状态直到
    // `print` 离开作用域。
    // `println!` 只需传引用就能使用，而这个闭包捕获的也是变量的引用，因此无需
    // 进一步处理就可以使用 `println!`。
    let print = || println!("`color`: {}", color);
    // 调用闭包，闭包又借用 `color`。
    print();
    print();

    let mut count = 0;

    // 这个闭包使 `count` 值增加。要做到这点，它需要得到 `&mut count` 或者
    // `count` 本身，但 `&mut count` 的要求没那么严格，所以我们采取这种方式。
    // 该闭包立即借用 `count`。
    //
    // `inc` 前面需要加上 `mut`，因为闭包里存储着一个 `&mut` 变量。调用闭包时，
    // 该变量的变化就意味着闭包内部发生了变化。因此闭包需要是可变的。
    let mut inc = || {
        count += 1;
        println!("`count`: {}", count);
    };

    // 调用闭包。
    inc();
    inc();


    use std::mem;
    //let reborrow = &mut count;
    // ^ 试一试：将此行注释去掉。
    
    // 不可复制类型（non-copy type）。
    let movable = Box::new(3);

    // `mem::drop` 要求 `T` 类型本身，所以闭包将会捕获变量的值。这种情况下，
    // 可复制类型将会复制给闭包，从而原始值不受影响。不可复制类型必须移动
    // （move）到闭包中，因而 `movable` 变量在这里立即移动到了闭包中。
    let consume = || {
        println!("`movable`: {:?}", movable);
        mem::drop(movable);
    };

    // `consume` 消耗了该变量，所以该闭包只能调用一次。
    consume();
    //consume();
    // ^ 试一试：将此行注释去掉。



    let haystack = vec![1, 2, 3];

    let contains = move |needle| haystack.contains(needle);

    println!("{}", contains(&1));
    println!("{}", contains(&4));

    //println!("There're {} elements in vec", haystack.len());
    // ^ 取消上面一行的注释将导致编译时错误，因为借用检查不允许在变量被移动走
    // 之后继续使用它。
    
    // 在闭包的签名中删除 `move` 会导致闭包以不可变方式借用 `haystack`，因此之后
    // `haystack` 仍然可用，取消上面的注释也不会导致错误。

    // 类型匿名
    fn apply<F>(f: F) where
    	//F: FnOnce() {
    	F: Fn() {
    	f();
    	}
    let x = 7;
    // 捕获 `x` 到匿名类型中，并为它实现 `Fn`。
    // 将闭包存储到 `print` 中。
    let print = || println!("Hi, this is {}", x);
    apply(print);


	    // 定义一个函数，可以接受一个由 `Fn` 限定的泛型 `F` 参数并调用它。
	fn call_me<F: Fn()>(f: F) {
	    f()
	}
	// 定义一个满足 `Fn` 约束的封装函数（wrapper function）。
	fn function2() {
	    println!("I'm a function!");
	}
    // 定义一个满足 `Fn` 约束的闭包。
    let closure = || println!("I'm a closure!");
    call_me(closure);
    call_me(function2);


	fn create_fn() -> Box<Fn()> {
	    let text = "Fn".to_owned();

	    Box::new(move || println!("This is a: {}", text))
	}

	fn create_fnmut() -> Box<FnMut()> {
	    let text = "FnMut".to_owned();

	    Box::new(move || println!("This is a: {}", text))
	}

    let fn_plain = create_fn();
    let mut fn_mut = create_fnmut();

    fn_plain();
    fn_mut();



    let vec1 = vec![1, 2, 3];
    let vec2 = vec![4, 5, 6];

    // 对 vec 的 `iter()` 举出 `&i32`。（通过用 `&x` 匹配）把它解构成 `i32`。
    // 译注：注意 `any` 方法会自动地把 `vec.iter()` 举出的迭代器的元素一个个地
    // 传给闭包。因此闭包接收到的参数是 `&i32` 类型的。
    //println!("2 in vec1: {}", vec1.iter()     .any(|&x| x == 2));
    // 对 vec 的 `into_iter()` 举出 `i32` 类型。无需解构。
    //println!("2 in vec2: {}", vec2.into_iter().any(| x| x == 2));


    // 对 vec1 的 `iter()` 举出 `&i32` 类型。
    let mut iter = vec1.iter();
    // 对 vec2 的 `into_iter()` 举出 `i32` 类型。
    let mut into_iter = vec2.into_iter();

    // 对迭代器举出的元素的引用是 `&&i32` 类型。解构成 `i32` 类型。
    // 译注：注意 `find` 方法会把迭代器元素的引用传给闭包。迭代器元素自身
    // 是 `&i32` 类型，所以传给闭包的是 `&&i32` 类型。
    println!("Find 2 in vec1: {:?}", iter     .find(|&&x| x == 2));
    // 对迭代器举出的元素的引用是 `&i32` 类型。解构成 `i32` 类型。
    println!("Find 2 in vec2: {:?}", into_iter.find(| &x| x == 2));



    let array1 = [1, 2, 3];
    let array2 = [4, 5, 6];

    // 对数组的 `iter()` 举出 `&i32`。
    println!("2 in array1: {}", array1.iter()     .any(|&x| x == 2));
    // 对数组的 `into_iter()` 通常举出 `&i32`。
    println!("2 in array2: {}", array2.into_iter().any(|&x| x == 2));





    fn is_odd(n: u32) -> bool {
    	n % 2 == 1
	}
	println!("Find the sum of all the squared odd numbers under 1000");
    let upper = 1000;

    // 命令式（imperative）的写法
    // 声明累加器变量
    let mut acc = 0;
    // 迭代：0，1, 2, ... 到无穷大
    for n in 0.. {
        // 数字的平方
        let n_squared = n * n;

        if n_squared >= upper {
            // 若大于上限则退出循环
            break;
        } else if is_odd(n_squared) {
            // 如果是奇数就计数
            acc += n_squared;
        }
    }
    println!("imperative style: {}", acc);


    // 函数式的写法
    let sum_of_squared_odd_numbers: u32 =
        (0..).map(|n| n * n)             // 所有自然数取平方
             .take_while(|&n| n < upper) // 取小于上限的
             .filter(|&n| is_odd(n))     // 取奇数
             .fold(0, |sum, i| sum + i); // 最后加起来
    println!("functional style: {}", sum_of_squared_odd_numbers);



   fn sum_odd_numbers(up_to: u32) -> u32 {
        let mut acc = 0;
        for i in 0..up_to {
            // 注意这个 match 表达式的返回值必须为 u32，
            // 因为 “addition” 变量是这个类型。
            let addition: u32 = match i%2 == 1 {
                // “i” 变量的类型为 u32，这毫无问题。
                true => i,
                // 另一方面，“continue” 表达式不返回 u32，但它仍然没有问题，
                // 因为它永远不会返回，因此不会违反匹配表达式的类型要求。
                false => continue,
            };
            acc += addition;
        }
        acc
    }
    println!("Sum of odd numbers up to 9 (excluding): {}", sum_odd_numbers(9));


	mod my {
	    // 一个公有的结构体，带有一个公有的字段（类型为泛型 `T`）
	    pub struct OpenBox<T> {
	        pub contents: T,
	    }

	    // 一个公有的结构体，带有一个私有的字段（类型为泛型 `T`）    
	    #[allow(dead_code)]
	    pub struct ClosedBox<T> {
	        contents: T,
	    }

	    impl<T> ClosedBox<T> {
	        // 一个公有的构造器方法
	        pub fn new(contents: T) -> ClosedBox<T> {
	            ClosedBox {
	                contents: contents,
	            }
	        }
	    }
	}


	let open_box = my::OpenBox { contents: "pulic imformation"};
	println!("The open box contains: {}", open_box.contents);

    // 带有私有字段的公有结构体不能使用字段名来构造。
    // 报错！`ClosedBox` 含有私有字段。
	// let close_box = my::CloseBox { contents: "close imformation"};
	// println!("The close box contains: {}", close_box.contents)

    // 不过带有私有字段的结构体可以使用公有的构造器来创建。
    let _closed_box = my::ClosedBox::new("classified information");

    // 并且一个结构体中的私有字段不能访问到。
    // 报错！`content` 字段是私有的。
    //println!("The closed box contains: {}", _closed_box.contents);
    // 试一试 ^ 取消此行注释    



    ///可以在路径中使用 super （父级）和 self（自身）关键字，从而在访问项时消除 歧义，以及防止不必要的路径硬编码。
	// fn function() {
	//     println!("called `function()`");
	// }

	mod cool {
	    // pub fn function() {
	    //     println!("called `cool::function()`");
	    // }
	}

	mod my2 {
	    fn function() {
	        println!("called `my::function()`");
	    }
	    
	    mod cool {
	        pub fn function() {
	            println!("called `my::cool::function()`");
	        }
	    }
	    
	    pub fn indirect_call() {
	        // 让我们从这个作用域中访问所有名为 `function` 的函数！
	        print!("called `my::indirect_call()`, that\n> ");
	        
	        // `self` 关键字表示当前的模块作用域——在这个例子是 `my`。
	        // 调用 `self::function()` 和直接调用 `function()` 都得到相同的结果，
	        // 因为他们表示相同的函数。
	        self::function();
	        function();
	        
	        // 我们也可以使用 `self` 来访问 `my` 内部的另一个模块：
	        self::cool::function();
	        
	        // `super` 关键字表示父作用域（在 `my` 模块外面）。
	        //super::function();
	        //super::function();
	        
	        // 这将在 *crate* 作用域内绑定 `cool::function` 。
	        // 在这个例子中，crate 作用域是最外面的作用域。
	        {
	            use cool::function as root_function;
	            root_function();
	        }
	    }
	}


	my2::indirect_call();





	//test multi files
	println!("test mod ");
	mymod::function();
	mymod::indirect_access();
	mymod::nested::function();

	// 这个函数仅当目标系统是 Linux 的时候才会编译
	#[cfg(target_os = "linux")]
	fn are_you_on_linux() {
	    println!("You are running linux!")
	}

	// 而这个函数仅当目标系统 **不是** Linux 时才会编译
	#[cfg(not(target_os = "linux"))]
	fn are_you_on_linux() {
	    println!("You are *not* running linux!")
	}

	are_you_on_linux();
    if cfg!(target_os = "linux") {
        println!("Yes. It's definitely linux!");
    } else {
        println!("Yes. It's definitely *not* linux!");
    }
    //functionMod();

	// 一个具体类型 `A`。
	struct A;

	// 在定义类型 `Single` 时，第一次使用类型 `A` 之前没有写 `<A>`。
	// 因此，`Single` 是个具体类型，`A` 取上面的定义。
	struct Single(A);
	//            ^ 这里是 `Single` 对类型 `A` 的第一次使用。

	// 此处 `<T>` 在第一次使用 `T` 前出现，所以 `SingleGen` 是一个泛型类型。
	// 因为 `T` 是泛型的，所以它可以是任何类型，包括在上面定义的具体类型 `A`。
	struct SingleGen<T>(T);
    // `Single` 是具体类型，并且显式地使用类型 `A`。
    let _s = Single(A);
    
    // 创建一个 `SingleGen<char>` 类型的变量 `_char`，并令其值为 `SingleGen('a')`
    // 这里的 `SingleGen` 的类型参数是显式指定的。
    let _char: SingleGen<char> = SingleGen('a');

    // `SingleGen` 的类型参数也可以隐式地指定。
    let _t    = SingleGen(A); // 使用在上面定义的 `A`。
    let _i32  = SingleGen(6); // 使用 `i32` 类型。
    let _char = SingleGen('a'); // 使用 `char`。



	//struct A;          // 具体类型 `A`。
	struct S(A);       // 具体类型 `S`。
	struct SGen<T>(T); // 泛型类型 `SGen`。

	// 下面全部函数都得到了变量的所有权，并立即使之离开作用域，将变量释放。

	// 定义一个函数 `reg_fn`，接受一个 `S` 类型的参数 `_s`。
	// 因为没有 `<T>` 这样的泛型类型参数，所以这不是泛型函数。
	fn reg_fn(_s: S) {}

	// 定义一个函数 `gen_spec_t`，接受一个 `SGen<A>` 类型的参数 `_s`。
	// `SGen<>` 显式地接受了类型参数 `A`，且在 `gen_spec_t` 中，`A` 没有被用作
	// 泛型类型参数，所以函数不是泛型的。
	fn gen_spec_t(_s: SGen<A>) {}

	// 定义一个函数 `gen_spec_i32`，接受一个 `SGen<i32>` 类型的参数 `_s`。
	// `SGen<>` 显式地接受了类型参量 `i32`，而 `i32` 是一个具体类型。
	// 由于 `i32` 不是一个泛型类型，所以这个函数也不是泛型的。
	fn gen_spec_i32(_s: SGen<i32>) {}

	// 定义一个函数 `generic`，接受一个 `SGen<T>` 类型的参数 `_s`。
	// 因为 `SGen<T>` 之前有 `<T>`，所以这个函数是关于 `T` 的泛型函数。
	fn generic<T>(_s: SGen<T>) {}
	    // 使用非泛型函数
    reg_fn(S(A));          // 具体类型。
    gen_spec_t(SGen(A));   // 隐式地指定类型参数 `A`。
    gen_spec_i32(SGen(6)); // 隐式地指定类型参数 `i32`。

	//generic(Sgen('abc'))
	generic::<char>(SGen('a'));
	generic(SGen('b'));


	//struct s;
	struct GenericVal<T>(T,);
	impl GenericVal<f32>{}
	impl GenericVal<S>{}

	//impl <T> GenericVal<T> {}

	struct Val {
		val: f64
	}

	struct GenVal<T>{
		gen_val: T
	}

	impl Val {
		fn value(&self) -> &f64 { &self.val }
	}

	impl <T> GenVal<T> {
		fn value(&self) -> &T { &self.gen_val}
	}

	let x = Val {val: 3.0};
	let y = GenVal { gen_val: "abc" };

	println!(" {}, {}", x.value(), y.value());




	// 这个 trait 用来实现打印标记：`{:?}`。
	use std::fmt::Debug;

	trait HasArea {
	    fn area(&self) -> f64;
	}

	impl HasArea for Rectangle {
	    fn area(&self) -> f64 { self.length * self.height }
	}

	impl HasArea for Triangle {
	    fn area(&self) -> f64 { self.length * self.height / 2.0 }
	}

	#[derive(Debug)]
	struct Rectangle { length: f64, height: f64 }
	#[allow(dead_code)]
	struct Triangle  { length: f64, height: f64 }

	// 泛型 `T` 必须实现 `Debug` 。只要满足这点，无论什么类型
	// 都可以让下面函数正常工作。
	fn print_debug<T: Debug>(t: &T) {
	    println!("{:?}", t);
	}

	// `T` 必须实现 `HasArea`。任意符合该约束的泛型的实例
	// 都可访问 `HasArea` 的 `area` 函数
	fn area<T: HasArea>(t: &T) -> f64 { t.area() }
  	let rectangle = Rectangle { length: 3.0, height: 4.0 };
    let _triangle = Triangle  { length: 3.0, height: 4.0 };
    print_debug(&rectangle);
    println!("Area: {}", area(&rectangle));
    println!("Area of Triangle: {}", area(&_triangle));


	//use std::fmt::{Debug, Display};
	use std::fmt::Display;
	fn compare_prints<T: Debug + Display>(t: &T) {
	    println!("Debug: `{:?}`", t);
	    println!("Display: `{}`", t);
	}


	fn compare_types<T: Debug, U: Debug>(t: &T, u: &U) {
	    println!("t: `{:?}", t);
	    println!("u: `{:?}", u);
	}

	let string = "words";
    let array = [1, 2, 3];
    let vec = vec![1, 2, 3];

    compare_prints(&string);
    //compare_prints(&array);
    // 试一试 ^ 将此行注释去掉。
    compare_types(&array, &vec);

    //use std::fmt::Debug;
	trait PrintInOption {
	    fn print_in_option(self);
	}
	// 这里需要一个 `where` 从句，否则就要表达成 `T: Debug`（这样意思就变了），
	// 或着改用另一种间接的方法。
	impl<T> PrintInOption for T where
	    Option<T>: Debug {
	    // 我们要将 `Option<T>: Debug` 作为约束，因为那是要打印的内容。
	    // 否则我们会给出错误的约束。
	    fn print_in_option(self) {
	        println!("{:?}", Some(self));
	    }
	}
	let vec1 = vec![1, 2, 3];

    vec1.print_in_option();


    struct Years(i64);
    struct Days(i64);

    impl Years{
    	pub fn to_days(&self) -> Days {
    		Days(self.0 * 365)
    	}
    }

    impl Days {
    	pub fn to_years(&self) -> Years {
    		Years(self.0 / 365)
    	}
    }

    impl fmt::Display for Days {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // 自定义格式，使得仅显示 `x` 和 `y` 的值。
        write!(f, "{}", self.0)
    }
}

    fn old_enough(age: &Years) -> bool {
    	age.0 >= 18
    }
    let age = Years(5);
    let age_days = age.to_days();
    println!("ages_dyas: {}", age_days);
    println!("Old enough {}", old_enough(&age));
    println!("Old enough {}", old_enough(&age_days.to_years()));
    // println!("Old enough {}", old_enough(&age_days));



 //    struct Container(i32, i32);

	// // 这个 trait 检查给定的 2 个项是否储存于容器中
	// // 并且能够获得容器的第一个或最后一个值。
	// trait Contains<A, B> {
	//     fn contains(&self, &A, &B) -> bool; // 显式地要求 `A` 和 `B`
	//     fn first(&self) -> i32; // 未显式地要求 `A` 或 `B`
	//     fn last(&self) -> i32;  // 未显式地要求 `A` 或 `B`
	// }

	// impl Contains<i32, i32> for Container {
	//     // 如果存储的数字和给定的相等则为真。
	//     fn contains(&self, number_1: &i32, number_2: &i32) -> bool {
	//         (&self.0 == number_1) && (&self.1 == number_2)
	//     }

	//     // 得到第一个数字。
	//     fn first(&self) -> i32 { self.0 }

	//     // 得到最后一个数字。
	//     fn last(&self) -> i32 { self.1 }
	// }

	// // 容器 `C` 就包含了 `A` 和 `B` 类型。鉴于此，必须指出 `A` 和 `B` 显得很麻烦。
	// fn difference<A, B, C>(container: &C) -> i32 where
	//     C: Contains<A, B> {
	//     container.last() - container.first()
	// }

	//struct Container(i32, i32);

	// 这个 trait 检查给定的 2 个项是否储存于容器中
	// 并且能够获得容器的第一个或最后一个值。
	// trait Contains {
	//     // 在这里定义可以被方法使用的泛型类型。
	//     type A;
	//     type B;

	//     fn contains(&self, &Self::A, &Self::B) -> bool;
	//     fn first(&self) -> i32;
	//     fn last(&self) -> i32;
	// }

	// impl Contains for Container {
	//     // 指出 `A` 和 `B` 是什么类型。如果 `input`（输入）类型
	//     // 为 `Container(i32, i32)`，那么 `output`（输出）类型
	//     // 会被确定为 `i32` 和 `i32`。
	//     type A = i32;
	//     type B = i32;

	//     // `&Self::A` 和 `&Self::B` 在这里也是合法的类型。
	//     fn contains(&self, number_1: &i32, number_2: &i32) -> bool {
	//         (&self.0 == number_1) && (&self.1 == number_2)
	//     }

	//     // 得到第一个数字。
	//     fn first(&self) -> i32 { self.0 }

	//     // 得到最后一个数字。
	//     fn last(&self) -> i32 { self.1 }
	// }

	// fn difference<C: Contains>(container: &C) -> i32 {
	//     container.last() - container.first()
	// }


	use std::ops::Add;
	use std::marker::PhantomData;

	/// 创建空枚举类型来表示单位。
	#[derive(Debug, Clone, Copy)]
	enum Inch {}
	#[derive(Debug, Clone, Copy)]
	enum Mm {}

	/// `Length` 是一个带有虚类型参数 `Unit` 的类型，
	/// 而且对于表示长度的类型（即 `f64`）而言，`Length` 不是泛型的。
	///
	/// `f64` 已经实现了 `Clone` 和 `Copy` trait.
	#[derive(Debug, Clone, Copy)]
	struct Length<Unit>(f64, PhantomData<Unit>);

	/// `Add` trait 定义了 `+` 运算符的行为。
	impl<Unit> Add for Length<Unit> {
	     type Output = Length<Unit>;

	    // add() 返回一个含有和的新的 `Length` 结构体。
	    fn add(self, rhs: Length<Unit>) -> Length<Unit> {
	        // `+` 调用了针对 `f64` 类型的 `Add` 实现。
	        Length(self.0 + rhs.0, PhantomData)
	    }
	}
	let one_foot:  Length<Inch> = Length(12.0, PhantomData);
    // `one_meter` 拥有虚类型参数 `Mm`。
    let one_meter: Length<Mm>   = Length(1000.0, PhantomData);

    // `+` 调用了我们对 `Length<Unit>` 实现的 `add()` 方法。
    //
    // 由于 `Length` 了实现了 `Copy`，`add()` 不会消耗 `one_foot`
    // 和 `one_meter`，而是复制它们作为 `self` 和 `rhs`。
    let two_feet = one_foot + one_foot;
    let two_meters = one_meter + one_meter;

    // 加法正常执行。
    println!("one foot + one_foot = {:?} in", two_feet.0);
    println!("one meter + one_meter = {:?} mm", two_meters.0);

    // 无意义的运算当然会失败：
    // 编译期错误：类型不匹配。
    //let one_feter = one_foot + one_meter;


    // raii.rs
	fn create_box() {
	    // 在堆上分配一个整型数据
	    let _box1 = Box::new(3i32);

	    // `_box1` 在这里被销毁，内存得到释放
	}
	// 在堆上分配一个整型数据    
    let _box2 = Box::new(5i32);

    // 嵌套作用域：
    {
        // 在堆上分配一个整型数据
        let _box3 = Box::new(4i32);

        // `_box3` 在这里被销毁，内存得到释放        
    }

    // 创建一大堆 box（只是因为好玩）。
    // 完全不需要手动释放内存！
    for _ in 0u32..1_000 {
        create_box();
    }
    // `_box2` 在这里被销毁，内存得到释放    

    struct ToDrop;
	impl Drop for ToDrop {
	    fn drop(&mut self) {
	        println!("ToDrop is being dropped");
	    }
	}

	let x = ToDrop;
    println!("Made a ToDrop!");

    for _ in 0u32..1_0 {
        let x = ToDrop;
        println!("Made a ToDrop!");
    }



	    // 此函数取得堆分配的内存的所有权
	fn destroy_box(c: Box<i32>) {
	    println!("Destroying a box that contains {}", c);

	    // `c` 被销毁且内存得到释放
	}
	    // 栈分配的整型
    let x = 5u32;

    // 将 `x` *复制*到 `y`——不存在资源移动
    let y = x;

    // 两个值各自都可以使用
    println!("x is {}, and y is {}", x, y);

    // `a` 是一个指向堆分配的整数的指针
    let a = Box::new(5i32);

    println!("a contains: {}", a);

    // *移动* `a` 到 `b`
    let b = a;
    // 把 `a` 的指针地址（而非数据）复制到 `b`。现在两者都指向
    // 同一个堆分配的数据，但是现在是 `b` 拥有它。
    
    // 报错！`a` 不能访问数据，因为它不再拥有那部分堆上的内存。
    // println!("a contains: {}", a);
    // 试一试 ^ 去掉此行注释

    // 此函数从 `b` 中取得堆分配的内存的所有权
    destroy_box(b);

    // 此时堆内存已经被释放，这个操作会导致解引用已释放的内存，而这是编译器禁止的。
    // 报错！和前面出错的原因一样。
    //println!("b contains: {}", b);
    // 试一试 ^ 去掉此行注释



    let immutable_box = Box::new(5u32);
    println!("immutable_box contains {}", immutable_box);

    // 可变性错误
    //*immutable_box = 4;

    // *移动* box，改变所有权（和可变性）
    let mut mutable_box = immutable_box;

    println!("mutable_box contains {}", mutable_box);

    // 修改 box 的内容
    *mutable_box = 4;

    println!("mutable_box now contains {}", mutable_box);



    // 此函数取得一个 box 的所有权并销毁它
	fn eat_box_i32(boxed_i32: Box<i32>) {
	    println!("Destroying box that contains {}", boxed_i32);
	}

	// 此函数借用了一个 i32 类型
	fn borrow_i32(borrowed_i32: &i32) {
	    println!("This int is: {}", borrowed_i32);
	}
	// 创建一个装箱的 i32 类型，以及一个存在栈中的 i32 类型。
    let boxed_i32 = Box::new(5_i32);
    let stacked_i32 = 6_i32;

    // 借用了 box 的内容，但没有取得所有权，所以 box 的内容之后可以再次借用。
    // 译注：请注意函数自身就是一个作用域，因此下面两个函数运行完成以后，
    // 在函数中临时创建的引用也就不复存在了。
    borrow_i32(&boxed_i32);
    borrow_i32(&stacked_i32);

    {
        // 取得一个对 box 中数据的引用
        let _ref_to_i32: &i32 = &boxed_i32;

        // 报错！
        // 当 `boxed_i32` 里面的值被借用时，不能销毁 `boxed_int`。
        // eat_box_i32(boxed_i32);
        // 改正 ^ 注释掉此行

        // `_ref_to_i32` 离开作用域且不再被借用。
    }

    // `boxed_i32` 现在可以将所有权交给 `eat_i32` 并被销毁。
    //（译注：能够销毁是因为已经不存在对 `boxed_i32` 的引用）
    eat_box_i32(boxed_i32);


    #[allow(dead_code)]
	#[derive(Clone, Copy)]
	struct Book {
	    // `&'static str` 是一个对分配在只读内存区的字符串的引用
	    author: &'static str,
	    title: &'static str,
	    year: u32,
	}

	// 此函数接受一个对 Book 类型的引用
	fn borrow_book(book: &Book) {
	    println!("I immutably borrowed {} - {} edition", book.title, book.year);
	}

	// 此函数接受一个对可变的 Book 类型的引用，它把年份 `year` 改为 2014 年
	fn new_edition(book: &mut Book) {
	    book.year = 2014;
	    println!("I mutably borrowed {} - {} edition", book.title, book.year);
	}

	// 创建一个名为 `immutabook` 的不可变的 Book 实例
    let immutabook = Book {
        // 字符串字面量拥有 `&'static str` 类型
        author: "Douglas Hofstadter",
        title: "Gödel, Escher, Bach",
        year: 1979,
    };

    // 创建一个 `immutabook` 的可变拷贝，命名为 `mutabook`
    let mut mutabook = immutabook;
    
    // 不可变地借用一个不可变对象
    borrow_book(&immutabook);

    // 不可变地借用一个可变对象
    borrow_book(&mutabook);
    
    // 可变地借用一个可变对象
    new_edition(&mut mutabook);
    
    // 报错！不能可变地借用一个不可变对象
    //new_edition(&mut immutabook);
    // 改正 ^ 注释掉此行



    let mut _mutable_integer = 7i32;

    {
        // 借用 `_mutable_integer`
        let _large_integer = &_mutable_integer;

        // 报错！`_mutable_integer` 在本作用域被冻结
        _mutable_integer = 50;
        // 改正 ^ 注释掉此行

        // `_large_integer` 离开作用域
    }

    // 正常运行！`_mutable_integer` 在这作用域没有冻结
    _mutable_integer = 3;



    struct Point { x: i32, y: i32, z: i32 }
    let mut point = Point { x: 0, y: 0, z: 0 };

    {
        let borrowed_point = &point;
        let another_borrow = &point;

        // 通过引用和原始所有者来访问数据
        println!("Point has coordinates: ({}, {}, {})",
                 borrowed_point.x, another_borrow.y, point.z);

        // 报错！不能可变地借用 `point` ，因为现在它有不可变的借用。
        //let mutable_borrow = &mut point;
        // 试一试 ^ 取消此行注释。

        // 不可变引用离开作用域
    }

    {
        let mutable_borrow = &mut point;

        // 通过可变引用来改变数据
        mutable_borrow.x = 5;
        mutable_borrow.y = 2;
        mutable_borrow.z = 1;

        // 报错！不能不可变地借用 `point`，因为现在它有可变的借用。
        //let y = &point.y;
        // 试一试 ^ 取消此行注释。

        // 报错！不能打印，因为 `println!` 会创建一个不可变引用。
        //println!("Point Z coordinate is {}", point.z);
        // 试一试 ^ 取消此行注释。

        // 可以工作！可变引用可以作为不可变的传给 `println!`。
        println!("Point has coordinates: ({}, {}, {})",
                 mutable_borrow.x, mutable_borrow.y, mutable_borrow.z);

        // 可变引用离开作用域
    }

    // 现在又可以不可变地借用 `point` 了。
    let borrowed_point = &point;
    println!("Point now has coordinates: ({}, {}, {})",
             borrowed_point.x, borrowed_point.y, borrowed_point.z);




 //    #[derive(Clone, Copy)]
	// struct Point { x: i32, y: i32 }

		// let xs = vec!["sam"];

		// xs.iter().filter(|item| item.ends_with('m')).take(5).collect();

		#[derive(Clone, Copy)]
		struct Point2 { x: i32, y: i32 }
		    let c = 'Q';

	    // 赋值语句中左边的 `ref` 关键字等价于右边的 `&` 符号。
	    let ref ref_c1 = c;
	    let ref_c2 = &c;

	    println!("ref_c1 equals ref_c2: {}", *ref_c1 == *ref_c2);

	    let point = Point2 { x: 0, y: 0 };

	    // 在解构一个结构体时 `ref` 同样有效。
	    let _copy_of_x = {
	        // `ref_to_x` 是一个指向 `point` 的 `x` 字段的引用。
	        let Point2 { x: ref ref_to_x, y: _ } = point;

	        // 返回一个 `point` 的 `x` 字段的拷贝。
	        *ref_to_x
	    };

	    // `point` 的可变拷贝
	    let mut mutable_point = point;

	    {
	        // `ref` 可以与 `mut` 结合以创建可变引用。
	        let Point2 { x: _, y: ref mut mut_ref_to_y } = mutable_point;

	        // 通过可变引用来改变 `mutable_point` 的字段 `y`。
	        *mut_ref_to_y = 1;
	    }

	    println!("point is ({}, {})", point.x, point.y);
	    println!("mutable_point is ({}, {})", mutable_point.x, mutable_point.y);

	    // 包含一个指针的可变元组
	    let mut mutable_tuple = (Box::new(5u32), 3u32);
	    
	    {
	        // 解构 `mutable_tuple` 来改变 `last` 的值。
	        let (_, ref mut last) = mutable_tuple;
	        *last = 2u32;
	    }
	    
	    println!("tuple is {:?}", mutable_tuple);

	    

	    
}

mod mymod;
// fn functionMod() {
//     println!("called `function()`");
// }


