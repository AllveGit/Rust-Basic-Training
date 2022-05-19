// match는 한가지 경우일때만 고려하였을 때는 너무 장황한 감이 있다
// switch문도 case가 하나만 있을때 쓰기엔 조금 찜찜한 느낌이랑 마찬가지..

// 그럴 때 사용하라고 if let이 있다.

fn iflet_test1() {
    let some_u8_value = Some(3);

    // some_u8_value 가 Some(3)와 똑같을때만 동작
    if let Some(3) = some_u8_value {
        println!("three");
    } else {
        println!("not three");
    }

    let some_string_value = Some(String::from("HelloWorld"));

    // some_string_value의 저장하는 데이터는 string이므로 소유권이 이동될 수 있다.
    // some_string_value의 소유권이 i로 이동되게 된다.
    // 그래서 이 이후 some_string_value에 접근하면 컴파일 에러가 난다.
    // 이를 해결하려면 some_string_value 앞에 참조자(&)를 붙여주면 된다.
    if let Some(i) = some_string_value {
        println!("Some {}", i);
    } else {
        println!("None");
    }
}

fn main() {
    iflet_test1();
}
