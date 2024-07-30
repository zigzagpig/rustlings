// move_semantics2.rs
// 不修改第13行以及不移动第10行的前提下让我可以编译！
// 执行 `rustlings hint move_semantics2` 或在观察模式下使用 `hint` 子命令来获取提示。

fn main() {
    let mut vec0 = Vec::new();

    // let mut vec1 = fill_vec(vec0.clone());//方法1
    fill_vec(&mut vec0); //方法2
    let mut vec1 = vec0.clone();
    // let mut vec1 = fill_vec(vec0.clone());//方法1

    // 不要改变下面这一行！
    println!("{} has length {} content `{:?}`", "vec0", vec0.len(), vec0);

    vec1.push(88);

    println!("{} has length {} content `{:?}`", "vec1", vec1.len(), vec1);
}

// fn fill_vec(vec: Vec<i32>) -> Vec<i32> {
//     let mut vec = vec;

//     vec.push(22);
//     vec.push(44);
//     vec.push(66);

//     vec
// }//方法1
// fn fill_vec(vec0: &Vec<i32>) -> Vec<i32> {
//     let mut vec = Vec::new();
//     for i in vec0 {
//         vec.push(i.clone());
//     }

//     vec.push(22);
//     vec.push(44);
//     vec.push(66);

//     vec
// }//方法2
fn fill_vec(vec: &mut Vec<i32>) {
    // let mut vec = Vec::new();
    // for i in vec0 {
    //     vec.push(i.clone());
    // }

    vec.push(22);
    vec.push(44);
    vec.push(66);

    // vec
}
