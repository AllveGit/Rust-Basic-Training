fn main() {
    println!("Hello, world!");

    func_test_1();
    func_test_2(1, 2);
    println!("Function Test3!! return value is {}", func_test_3());
    println!("Function Test4!! return value is {}", func_test_4(5));
}

// rust에서 함수와 변수의 이름은 모두 소문자여야 한다. (_으로 단어구분을 해야한다.) 
// 함수 선언은 fn으로 하며 ()안에 인자값을 넣을 수 있다.
// 함수의 선언 순서는 상관없이 정의만 되면 사용가능하다.
fn func_test_1() {
    println!("Function Test1!!");
}

// 함수 매개변수는 괄호안에 넣으면 된다.
// 매개변수에 데이터타입을 명시해야 한다.
fn func_test_2(in_val1: i32, in_val2: i32) {
    println!("Function Test2!! value1 is {}", in_val1);
    println!("Function Test2!! value2 is {}", in_val2);
}

// 함수에 반환값이 있다면 -> 으로 반환값의 타입을 명시해주어야 한다.
// 반환값은 세미콜론을 안붙인 구문이 있으면 그 구문이 리턴하겠다는 표현식이 된다. 즉, 세미콜론을 안붙인 표현식이 리턴값이 된다.
fn func_test_3() -> i32{
    5 // 리턴값
}

fn func_test_4(in_val: i32) -> i32{
    let return_val:i32 = in_val + 1;
    return_val // 리턴값
}