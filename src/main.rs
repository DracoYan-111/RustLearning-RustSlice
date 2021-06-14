fn main() {
    let mut s = String::from("aaaaa");
    let world = first_world(&s);
    println!("{}",world);
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
