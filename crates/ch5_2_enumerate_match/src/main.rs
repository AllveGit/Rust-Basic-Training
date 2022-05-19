// 러스트는 match라고 불리는 강력한 흐름 제어 연산자를 가지고 있다.
// 다른 문법들의 switch랑 비슷한 역할을 한다.
// 그렇지만 switch보다 더욱 많은 기능이 있다.

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}

fn match_test1() {
    println!("Penny is {}", value_in_cents(Coin::Penny));
    println!("Quarter is {}", value_in_cents(Coin::Quarter));
}

// 이렇게 match를 이용하여 Coin enum 타입에 따라 다른 행동을 취할수 있다.
// 이거만 보면 switch문과 똑같다.
fn value_in_cents(coin: Coin) -> u32 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => {
            println!("Lucky Quarter!!");
            25
        }, 
    }
}


// 하지만 rust의 enum 타입은 각각 다른 데이터타입을 저장할수 있었다.
#[derive(Debug)]
enum USState {
    Alabama,
    Alaska,
}

enum USCoin {
    Penny,
    Nickel,
    Dime,
    Quarter(USState),
}

fn match_test2() {
    println!("Penny is {}", values_in_us_cents(USCoin::Penny));
    println!("Quarter is {}", values_in_us_cents(USCoin::Quarter(USState::Alaska)));
}

fn values_in_us_cents(coin: USCoin) -> u32 {
    match coin {
        USCoin::Penny => 1,
        USCoin::Nickel => 5,
        USCoin::Dime => 10,
        // 이렇게 혼자만 데이터가 달라도 match문에 입력시킬수 있다.
        // 자동으로 state에 coin이 가지고 있는 UsState 데이터값을 바인딩 시킬것이다.
        USCoin::Quarter(state) => {
            println!("State quarter from {:?}!", state);
            25
        },
    }
} 


// 저번 cargo에서 Option<T>와 match를 이용하면 강력하다고 하였다.
// 이 함수는 None일 때는 None을 리턴해버리고, 값이 있으면 + 1을 한 후 새로운 some 값을 리턴한다.
fn increase(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}

fn match_test3() {
    let x = increase(Some(5));
    let y = increase(None);
}

// match는 한 케이스라도 빼먹지 않는다. 빼먹으면 에러가 난다.
// 즉, 모든케이스에 대한 대응이 가능해야 한다.
// enum이야 모든 케이스를 적으면 그만이라쳐도, 일반타입에 대해서는 모든 케이스를 적는것이 불가능해보인다.
// 이를 위해 변경자(_)가 있다

fn match_test4() {
    let some_u8_value = 0u8;
    match some_u8_value {
        1 => println!("one"),
        3 => println!("three"),
        5 => println!("five"),
        7 => println!("seven"),
        // 변경자 케이스를 입력하면 정의하지 않은 나머지 케이스는 모두 변경자 케이스로 동작한다.
        // ()는 단지 단위값이므로 아무것도 하고 싶지 않다는 것을 의미한다.
        _ => (),
    }
}

fn main() {
    match_test1();
    match_test2();
    match_test3();
    match_test4();
}
