// 슬라이스는 선형 콜렉션 객체의 일부를 나타내는 데이터이다. 

fn string_slice_test1() {
    let s = String::from("hello world");

    // 슬라이스 정의 0~4
    let _hello = &s[0..5];
    // 슬라이스 정의 6~10
    let _world = &s[6..11];

    // string 슬라이스는 string에 대한 참조자가 아닌 string의 일부분에 대한 참조자이다.
    // string 슬라이스는 힙데이터가 복사되지 않고, 스택데이터를 새로 생성후, 원래 데이터의 참조 범위를 설정하는 식으로 생성이 된다.
}

fn string_slice_test2() {
    let s = String::from("hello world");
    let len = s.len();

    // 둘이 똑같은 의미의 구문이다.
    let _slice = &s[0..2];
    let _slice = &s[..2];

    // 둘이 똑같은 의미의 구문이다.
    let _slice = &s[3..len];
    let _slice = &s[3..];

    // 둘이 똑같은 의미의 구문이다. (전체 스트링의 슬라이스)
    let _slice = &s[0..len];
    let _slice = &s[..];
}

fn string_slice_test3() {
    let mut s = String::from("Hello World!");

    let word1 = first_word(&s);

    // word1는 s의 스트링 슬라이스타입이며 동시에 불변참조자이다.
    // 불변참조자를 가지고 있을 때 그 참조대상의 가변참조자를 만들 수 없다.
    // clear() 함수는 string을 변경하는 함수이기 때문에 가변참조자를 만들어 에러가 발생한다.
    // s.clear();  // 컴파일 에러 발생!

    println!("the first word is: {}", word1);
}

fn string_slice_test4() {
    // 이제야 스트링 리터럴을 변경할 수 없는 이유가 이해가 간다.
    // 스트링 리터럴은 스트링 슬라이스이기 때문이다.

    // 여기서 _s의 타입은 &str이다.
    // &str은 불변참조자이기에 변경할수가 없다.
    // 그렇기에 스트링 리터럴은 불변이다.
    let _s = "Hello, world!";
}

// 스트링 슬라이스의 타입은 &str이다.
// 첫번째 단어를 찾아 스트링슬라이스를 리턴해주는 함수이다. 공백문자가 없으면 스트링자체를 단어로 간주
fn first_word(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    return &s[..]
}

// 배열등 모든 컬렉션 타입에 슬라이스를 사용가능하다.
// 스트링 슬라이스랑 똑같이 동작한다.
fn collection_slice_test() {
    let a = [1, 2, 3, 4, 5];
    let _slice = &a[1..3]; 
}


// ! 소유권, 빌림, 슬라이스의 개념은 rust 프로그램의 메모리 안정성을 컴파일 타임에 보장해주는 러스트의 핵심 특성이라고 할 수 있다.
fn main() {
    string_slice_test1();
    string_slice_test2();
    string_slice_test3();
    string_slice_test4();
    collection_slice_test();
}
