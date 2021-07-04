fn main() {
    let mut s = String::from("aaaaa");
    let world = first_world(&s);
    //清空字符串
    s.clear();
    println!("{}", world);

    //===================字符串切片===================
    let a = String::from("Hello world");
    //&a:对某字符串的引用
    //[0..5]:引用该字符串的一部分，起始为0可以不写0[..5],结束为字符最后长度也可不写[6..]
    let b = &a[0..5];
    let c = &a[6..11];
    //切片为整个字符串时,前后都可以省略不写,或者0..a.len();
    let e = &a[..];

    //===================字符串切片重写列子===================
    let mut f = String::from("aa aaa");
    let worldTwo = first_worldTwo(&f);
    //此时清空字符串方法将报错，因为该变量已经成为不可变的
    f.clear();
    println!("{}", worldTwo);

    //===================字符串切片作为参数传递===================
    let mut g = String::from("aa aaa");
    let worldThree = first_worldThree(&g[..]);//传入该字符串的整个字符串切片
    //此时清空字符串方法将报错，因为该变量已经成为不可变的
    g.clear();
    println!("{}", worldThree);

    let mut h = String::from("aa aaa");
    let worldFour = first_worldThree(&h);//传入该字符串的整个字符串
    //此时清空字符串方法将报错，因为该变量已经成为不可变的
    h.clear();
    println!("{}", worldFour);

    //===================字符串切片作为参数传递===================
    let i = [0,1,2,4,5];
    let i1 = &i[..3];//存储为数组的指针和长度

    //===================其他类型的切片===================、
    let j =[1,2,3,4,5];
    let slice = &j[1..3];//存储类型为j数组的指针

}

/*
* 找到字符串中的第一个单词
* 入手点:依次检查每个字节，返回空格位置
*/
fn first_world(s: &String) -> usize {
    //as_bytes():返回字节数组
    let bytes = s.as_bytes();
    //iter():生成一个迭代器
    //enumerate():将结果进行包装，每个包装作为元组的一部分进行返回
    //(i,&item):生成模式匹配，在里面生成两个变量对元组进行解构
    for (i, &item) in bytes.iter().enumerate() {
        //判断是否为空格的字面值:b' '
        if item == b' ' {
            return i;
        }
    }
    s.len()
}

//===================字符串切片重写列子===================
//&str:表示返回字符串切片
fn first_worldTwo(s: &String) -> &str {
    //as_bytes():返回字节数组
    let bytes = s.as_bytes();
    //iter():生成一个迭代器
    //enumerate():将结果进行包装，每个包装作为元组的一部分进行返回
    //(i,&item):生成模式匹配，在里面生成两个变量对元组进行解构
    for (i, &item) in bytes.iter().enumerate() {
        //判断是否为空格的字面值:b' '
        if item == b' ' {
            //找到空格时,截取字符串开始到第i位的切片，并返回
            return &s[..i];
        }
    }
    //如果没有则返回整个字符串切片
    &s[..]
}

//===================字符串切片作为参数传递===================
//&str:表示传入和返回字符串切片
fn first_worldThree(s: &str) -> &str {
    //as_bytes():返回字节数组
    let bytes = s.as_bytes();
    //iter():生成一个迭代器
    //enumerate():将结果进行包装，每个包装作为元组的一部分进行返回
    //(i,&item):生成模式匹配，在里面生成两个变量对元组进行解构
    for (i, &item) in bytes.iter().enumerate() {
        //判断是否为空格的字面值:b' '
        if item == b' ' {
            //找到空格时,截取字符串开始到第i位的切片，并返回
            return &s[..i];
        }
    }
    //如果没有则返回整个字符串切片
    &s[..]
}
