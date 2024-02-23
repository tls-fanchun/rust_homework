use std::ffi::CString;
use std::io;

fn one() {

    let mut s = String::new();
    io::stdin().read_line(&mut s).expect("error");
    let num: i32 = s.trim().parse().expect("error");

    if num > 0 {
        println!("{} is a positive number", num);
    } else if num < 0 {
        println!("{} is a negative number", num);
    } else {
        println!("The number is zero");
    }


    //2. 使⽤ loop 编写⼀个⽆限循环，当循环次数达到10次时，使⽤ break 退出循环。

    let mut i = 0; // 将计数器初始化在loop外部

    loop {
        if i == 10 {
            break; // 当i等于10时跳出循环
        }

        println!("{i}"); // 打印当前的计数
        i += 1; // i增加1
    }

    //3. 使⽤ for 循环遍历 1 到 888 的数字，并只打印出其中的偶数。


    for i in 1..888{

        if i%2==0 {
            print!("{i}\n");
        }

    }




}


fn take_onwership(X: String){
    print!("{X}");
}

fn borrow_string(c: &String){
    print!("{} , {}",c,c.len());
}
fn two() {
    // 1. 创建⼀个函数 take_onwership ，它获取⼀个 String 类型的参数，并打印出来，然后探
    // 索函数调⽤后原变量的状态；创建⼀个函数 borrow_string ，它获取⼀个对 String 的不
    // 可变引⽤，并打印出字符串的⻓度。

    let mut x = String::from("hell\n");
    take_onwership(x);
    let d: String = String::from("ss");
    borrow_string(&d);

}

fn three(){

    //ONE
    //b 是 指向a ， b修改了a 的状态，b的生命周期并没有结束，所以 a不能用
    // let mut a = 10u32;
    // let b = &mut a;
    // *b = 20;
    // let c = &a;
    // println!("{b}");

    //2 因为 Rust 不允许一个变量同时拥有活跃的可变引用和不可变引用
    // let mut a = 10u32;
    // let c = &a;
    // let b = &mut a;
    // *b = 20;
    // println!("{c}");

}


fn four(){

   //  // 1.假设有⼀个结构体 Person ，它包含⼀个对 String 的引⽤。编写⼀个带有⽣命周期注释的
   //  // 结构体，并解释为什么需要⽣命周期。
   //  // 2.实现⼀个返回最⻓字符串切⽚的函数
   //  // 编写⼀个函数 longest ， 它接受两个字符的引⽤，并返回最⻓的字符串的引⽤。尝试调⽤
   //  // longest 函数，并处理可能出现的⽣命周期问题。

// struct Person<'a>{
//     name: &'a String,
// }
//
// fn lens<'a>(x: &'a str, z: &'a str) -> &'a str {
//     if x.len()>z.len() {
//         x
//     }else {
//         z
//     }
// }
//
//
//
//
// fn main() {
//
//     let mut x = String::from("xx");
//     let mut xx = String::from("dddd");
//     let person = Person{name: &x};
//
//     let result = lens(x.as_str(),xx.as_str());
//     print!("{}",result);
//
//
//
// }
}

//
// //4 题
// // 1. 请定义⼀个Person 结构体，它应该包括姓名、年龄和城市等字段。然后编写⼀个关联函数
// // new ，⽤于根据给定的参数创建Person 实例。
// // 2. 为Person 结构体实现⼀个⽅法 introduce ，该⽅法的作⽤是打印出⼀个介绍个⼈信息的
// // 语句。应能够清晰地表达出这个⼈的姓名、年龄和所在的城市。
// struct Person{
//     name: String,
//     age: i32,
//     city: String,
// }
//
// impl Person {
//     fn New(name: String,age: i32, city:String) ->Person{
//         Person{name,age,city}
//     }
//
//     fn introduce(&self){
//         println!("{},{},{}",self.name,self.age,self.city);
//     }
// }
// fn main() {
//
//     let xx = Person::New("li".to_string(),22,"zhuhai".to_string());
//     xx.introduce();
// }

// 定义⼀个名为 TrafficLight 的枚举，它应该包含红灯、⻩灯、绿灯这三种状态。然后为
// TrafficLight 枚举实现⼀个⽅法 duration ，该⽅法返回每种灯持续的时间（以秒为单
// 位）。最后，使⽤ match 表达式来处理TrafficLight 实例，根据不同的灯显示相应的⾏动指
// 示。


enum TrafficLight{
    red,
    yellow,
    green,
}

impl TrafficLight{
    fn duration(&self) -> u8{
        match *self {
            TrafficLight::red => 10,
            TrafficLight::yellow => 3,
            TrafficLight::green => 15,
        }
    }
}


fn main() {
    let red_light = TrafficLight::red;
    let yellow_light = TrafficLight::yellow;
    let green_light = TrafficLight::green;

    println!("red {}",red_light.duration());
    println!("yellow {}",yellow_light.duration());
    println!("green {}",green_light.duration());


    match green_light {
        TrafficLight::red => println!("stop"),
        TrafficLight::yellow => println!("down"),
        TrafficLight::green => println!("go!"),
    }


}