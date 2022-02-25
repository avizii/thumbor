use std::fmt::Debug;
use std::mem;

// array
fn array_test() {
    // 数组的类型表示为 [类型;长度]，数组的长度也是类型的一部分
    // 如 [i32;4] 和 [i32;5] 并不是同一个类型，因为他们的长度不一样

    // 数组定义1：
    // 类型固定，长度固定，存储在栈上
    let _a = [1, 2, 3, 4, 5];
    let _a: [u8; 5] = [1, 2, 3, 4, 5];

    // 数组定义2：
    // 某个值重复出现n次的数组
    let a = [3_u8; 5];

    // let len = 5;
    // let _b = [5_i32; len];  error

    // 数组元素访问
    let _first = a[0];
    let _second = a[1];

    // slice = ptr + len
    // slice切片 的长度可以与数组不同，并不是固定的，而是取决于你使用时指定的起始和结束位置
    // 创建切片的代价非常小，因为切片只是针对底层数组的一个引用
    // 切片类型 [T] 拥有不固定的大小，而切片引用类型 &[T] 则有固定的大小

    // 数组类型容易跟数组切片混淆，[T;n] 描述了一个数组的类型，
    // 而 [T] 描述了切片的类型，因为切片是运行期的数据结构，他的长度无法在编译期得知，因此不能用 [T;n] 的形式来描述
    // [u8; 3] 和 [u8; 4] 是不同的类型，数组的长度也是类型的一部分
    // 在实际开发中，使用最多的是数组切片 [T]，我们往往通过引用的方式去使用 &[T], 因为后者有固定的类型大小

    // let s: [u8] = a[..]; error
    let s1: &[u8; 5] = &a;
    let s2: &[u8] = &a[..];
    let s3: &[u8] = &a[1..3];
    let len = 4;
    let s4: &[u8] = &a[1..len];
    println!(
        "s1: {}, s2: {}, s3: {}, s4: {}",
        mem::size_of_val(s1), // 5
        mem::size_of_val(s2), // 5
        mem::size_of_val(s3), // 2
        mem::size_of_val(s4), // 3
    );

    let s1 = mem::size_of::<[u64; 3]>(); // 24
    let s2 = mem::size_of::<&[u64; 3]>(); // 8  => ptr
    let s3 = mem::size_of::<&[u64]>(); // 16 => ptr + len
    println!(
        "array size: {}, array ref size: {}, array slice ref size: {}",
        s1, s2, s3
    );
}

// vector - dynamic array
fn vector_test() {
    // vec create
    let _v: Vec<i32> = Vec::new();
    let _v: Vec<i32> = Vec::with_capacity(4);

    let mut v = Vec::new();
    v.push(1);

    let _v = vec![1, 2, 3];

    // vec insert  mut
    let mut v = Vec::new();
    v.push(1);

    // vec read
    let v1 = vec![1, 2, 3, 4, 5];
    let _ele = v1[2];
    let _ele = v1[2];
    let _element = &v1[2];
    let _element = v1.get(2);

    // here: i's type is &i32
    for i in &v1 {
        print!("{}\t", i);
    }
    println!();

    // here: i's type is i32
    for &i in &v1 {
        print!("{}\t", i);
    }
    println!();

    // here: i's type is i32
    for i in v1 {
        print!("{}\t", i);
    }
    println!();
    // from here on, v1 moved

    #[derive(Debug)]
    struct Ele(String);

    let v2 = vec![
        Ele("wu".to_string()),
        Ele("de".to_string()),
        Ele("jin".to_string()),
    ];
    // let e1 = v[1]; // error can't move not implement Copy trait
    let _e2 = &v2[1];
    let _e3 = v2.get(1).unwrap();

    for i in &v2 {
        println!("{:?}", i);
    }

    let v_s_1 = &v2[..];
    let v_s_2 = &v2;
    let v_s_3 = &v2[1..3];

    // for &i in &v2 { // error can't move out of a shared reference - not implement Copy trait
    //     println!("{:?}", i);
    // }

    // for i in v2 {
    //     println!("{:?}", i);
    // }
    // from here on, v2 moved


    let mut v = vec![1, 2, 3];
    let ele = v.get_mut(1).unwrap();
    *ele = 20;
    println!("{:?}", v);

    // mutable iterator
    for i in &mut v {
        *i += 10;
    }
    println!("{:?}", v);

    let mut v = vec![
        Ele("wu".to_string()),
        Ele("de".to_string()),
        Ele("jin".to_string()),
    ];

    let ele = v.get_mut(2).unwrap();
    ele.0 = String::from("qing");
    println!("{:?}", v);
}


// slice
fn slice_test() {
    let arr = [1, 2, 3, 4, 5];
    let vec = vec![1, 2, 3, 4, 5];

    let s1 = &arr[..2];
    let s2 = &vec[..2];

    println!("s1: {:?}, s2: {:?}", s1, s2);

    assert_eq!(s1, s2);
    assert_eq!(&arr[..], vec);
    assert_eq!(&vec[..], arr);
    assert_eq!(&vec, &arr);
}

// iterator
fn iter_test() {
    let v = vec![1, 2, 3, 4, 5];
    for e in v.into_iter() {
        print!("{:?}\t", e);
    }
    println!();

    let v = vec![1, 2, 3, 4, 5];
    for e in v.iter() {
        print!("{:?}\t", e);
    }
    println!();

    let mut v = vec![1, 2, 3, 4, 5];
    for e in v.iter_mut() {
        *e += 10;
    }
    println!("{:?}", v);
}


// map

// str / &str / String / &String
fn str_slice_test() {
    let a = ['h', 'e', 'l', 'l', 'o'];
    let v = vec!['h', 'e', 'l', 'l', 'o'];
    let s = String::from("hello");

    let s1 = &a[1..3];
    let s2 = &v[1..3];
    let s3 = &s[1..3];

    println!("s1: {:?}, s2: {:?}, s3: {:?}", s1, s2, s3);

    assert_eq!(s1, s2);
    assert_eq!(s2, s3.chars().collect::<Vec<_>>());
    assert_eq!(String::from_iter(s2), s3);

    let mut v1 = vec![1, 2, 3, 4];
    v1.push(5);
    println!("cap should be 8: {}", v1.capacity());

    let b1 = v1.into_boxed_slice();
    let mut b2 = b1.clone();

    let v2 = b2.into_vec();
    println!("cap should be exactly 5: {}", v2.capacity());
}



fn main() {
    // array_test();

    // vector_test();

    // iter_test();

    slice_test();

    str_slice_test();

    let v = vec![1, 2, 3, 4, 5];

    print_slice(&v);
    print_slice(&v[..]);

    print_slice2(&v);
    print_slice2(&v[..]);
    print_slice2(v);


    let v = [1, 2, 3, 4, 5];

    print_slice(&v);
    print_slice(&v[..]);

    print_slice2(&v);
    print_slice2(&v[..]);
    print_slice2(v);
}


fn print_slice<T: Debug>(s: &[T]) {
    println!("{:?}", s);
}

fn print_slice2<T, U>(s: T)
where
    T: AsRef<[U]>,
    U: Debug,
{
    println!("{:?}", s.as_ref());
}