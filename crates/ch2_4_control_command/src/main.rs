// 제어문은 코드의 흐름을 제어해주는 역할을 해준다.

fn main() {
    if_test();
    loop_test();
    while_test();
    for_test();
}

/*
===== if문 =====
조건은 항상 boolean 타입이어야 한다.
*/
fn if_test() {
    let number1 = 3;
    let number2 = 7;

    if number1 > 5 {
        println!("case1 cond is true");
    } else {
        println!("case1 cond is false");
    }

    if number2 > 5 {
        println!("case2 cond is true");
    } else {
        println!("case2 cond is false");
    }
    
    // 에러발생! 다른 언어들처럼 생략체크방식으로 체크가안됨
    // if number1 {
    //     println!("Can?");
    // }

    if number1 > 5 {
        println!("case3 cond is true");
    } else if number2 > 5 {
        println!("case3 cond is true");
    } else {
        println!("case3 cond is false");
    }

    // let 구문에서 if문을 사용하여 나타낼수도있다. (삼항연산자 비슷한느낌)
    let condition = true;
    let value = if condition {
        5
    } else {
        6
    };

    println!("case4 value is {}", value);

    // 하지만 이렇게 반환하는 값이 다르면 에러가 발생한다. 
    // 무조건 모두 같아야 한다.
    // let condition = false;
    // let value = if condition {
    //     5
    // } else {
    //     "six"
    // };
}

/*
===== 반복문 : loop =====
loop 반복문은 rust에게 그만두라고 명시하여 알려주기 전까지 계속 반복수행한다.
그만두라고 알려주는 법은 코드에서 break문을 이용하거나, 터미널에서 Ctrl+C를 입력한다.
*/
fn loop_test() {
    let mut cnt: i32 = 0;
    loop {
        if cnt >= 10 {
            break;
        }

        cnt += 1;

        println!("loop 반복문 테스트 {}", cnt);
    }
}

/*
===== 반복문 : while =====
상황에 따라 loop 반복문보다 더 깔끔하게 사용 가능
*/
fn while_test() {
    let mut cnt: i32 = 0;
    while cnt < 10 {
        cnt += 1;

        println!("while 반복문 테스트 {}", cnt)
    }
}

/*
===== 반복문 : for =====
for 반복문이 다른 반복문과 차이가 있는 장점은 바로 콜렉션의 각 요소에 안전하게 접근이 가능하다는 것이다.
while문 같이 인덱스를 하나씩 올리며 순회하는 방식은 콜렉션의 길이를 알아야하지만, for문은 iterator방식으로 순회가 가능하다.

게다가 range를 통한 특정횟수만큼 순회하는 반복문도 손쉽게 만들수 있다.
*/
fn for_test() {
    // iterator 순회
    let arr = [10, 20, 30, 40, 50];
    for element in arr.iter() {
        println!("for iterator 반복문 테스트, element value is {}", element);
    }

    // range를 통한 특정횟수 순회
    for element in 1..11 {
        println!("for range 반복문 테스트, element value is {}", element);
    }

    // range를 통한 특정횟수 역순 순회
    for element in (1..11).rev() {
        println!("for range 역순 반복문 테스트, element value is {}", element);
    }
}
