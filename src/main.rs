fn main() {
    //[0; 3] 创建3个元素为0
    let mut array: [i32; 3] = [0; 3];
    array[1] = 1;
    array[2] = 2;

    assert_eq!([1, 2], &array[1..]);

    for x in &array {
        println!("{}", x);
    }

    //创建空vec
    let v: Vec<i32> = Vec::new();
    //使用宏创建vec
    let v: Vec<i32> = vec![];

    //创建包含5个元素的Vec
    let v = vec![1, 2, 3, 4, 5];

    //创建10个零
    let v = vec![0, 10];

    //创建可变的Vec 压入元素3
    let mut v = vec![1, 2];
    v.push(3);
    for x in &v {
        println!("{}", x);
    }

    //可变Vec v pop一个值
    let mut v = vec!["1", "2"];
    let two = v.pop();
    for x in &v {
        println!("{}", x);
    }

    //索引使用，改变vec中的值
    let mut v = vec![1, 2, 3];
    let three = v[2];
    v[1] = v[1] + 5;

    //字符串面量值
    let hello = "Hello, world!";
    //其实带类型标识就是
    let hello: &'static str = "Hello, world!";

    //创建一个空的字符串
    let mut s = String::new();
    //从`&str` 类型转为`String`类型
    let mut hello = String::from("Hello, ");
    //压入字符和压入字符串切片
    hello.push('w');
    hello.push_str('orld!');
    //弹出字符
    let mut s = String::from("foo");
    assert_eq!(s.pop(), Some('o'));
    assert_eq!(s.pop(), Some('o'));
    assert_eq!(s.pop(), Some('f'));
    assert_eq!(s.pop(), None);
}
