// 다른 언어에서 사용하는 enum을 생각하면 되지만,
// rust의 enum은 더욱 강력하다.

// enum 정의법
enum IPAddrKind {
    V4,
    V6,
}

// enum 사용법
fn enum_test1() {
    let _four = IPAddrKind::V4;
    let _six = IPAddrKind::V6;

    // 이렇게 함수의 인자로도 전달이 가능하다.
    route(IPAddrKind::V4);
    route(_six);
}

fn route(ip_type: IPAddrKind) {
}

// 현재 IPAddrKind는 주소값의 타입만 저장할수 있지, 값을 저장할 수 없다.
// struct에 enum타입 필드를 하나두고 값을 저장하는 필드를 두면 해결할 수 있지만, 더욱 좋은 방법이 있다.

// 이렇게 enum 필드 괄호안에 타입을 넣으면 그 타입의 데이터값을 저장할 수 있다.
enum IPAddrKind2 {
    V4(String),
    V6(String),
}

fn enum_test2() {
    let local = IPAddrKind2::V4(String::from("127.0.0.1"));
    let loopback = IPAddrKind2::V6(String::from("::1"));

    // enum의 데이터가 String이므로 소유권이 이동된다. 그렇기에 소유권을 가지지 않도록 참조자를 붙였다.
    if let IPAddrKind2::V4(val) = &local {
        println!("local is {}", val);
    }

    if let IPAddrKind2::V6(val) = &loopback {
        println!("loopback is {}", val);
    }
}

// 게다가 struct로 만들면, enum 타입이 달라도 모두 같은 데이터필드를 사용해야 하지만,
// enum은 enum필드마다 각각 다른 타입의 데이터들을 저장할 수 있다.
enum IPAddrKind3 {
    V4(u8, u8, u8, u8),
    V6(String),
}

fn enum_test3() {
    let local = IPAddrKind3::V4(127, 0, 0, 1);
    let loopback = IPAddrKind3::V6(String::from("::1"));

    if let IPAddrKind3::V4(val1, val2, val3, val4) = &local {
        println!("local is {}.{}.{}.{}", val1, val2, val3, val4);
    }

    if let IPAddrKind3::V6(val) = &loopback {
        println!("loopback is {}", val);
    }
}

// enum의 저장 데이터값은 소유권을 가질수 있는 타입이라면 어떤 타입이든 저장할 수 있다.
enum IPAddrKind4 {
    V4(IPV4Addr),
    V6(IPV6Addr),
}

struct IPV4Addr {
}
struct IPV6Addr {
}

// enum의 강력한점은 또 있다.
// 예를 들어 이런 struct들이 있다고 해보자.

struct QuitMessage;                         // 유닛 구조체
struct WriteMessage(String);                // 튜플 구조체
struct ChangeColorMessage(i32, i32, i32);   // 튜플 구조체
struct MoveMessage {                        // 일반 구조체
    x: i32,
    y: i32,
}

// 이 메시지들에게 공통으로 함수를 만들기는 어렵다.
// 각각 다른 타입을 가지고 있기 때문이다.
// 하지만 enum을 사용하면 이 문제는 한번에 해결이 된다.

enum Message {
    Quit,
    Write(String),
    ChangeColor(i32, i32, i32),
    Move { x: i32, y: i32 },
}

// impl 블록을 활용하여 enum 필드들에게 공통 함수를 정의할 수 있다.
impl Message {
    // Message들의 공통 함수
    // self에는 Message의 열거형 필드값이 들어가게 될것이다.
    fn call(&self) {
        println!("Hello!");
    }
}

fn enum_test4() {
    let m = Message::Write(String::from("Hello"));
    // 공통 함수 호출
    m.call();
}

// rust에는 null이 없다. 
// null 값을 사용하면 null 값을 null이 아닌 값처럼 사용하려고할때(nullptr 참조) 많은 오류들이 발생하고, 이런 상황이 많이 나오게 된다.
// (그래서 null을 만든 Tony Hoare은 null이 자신의 10억달러의 실수라고 칭한다.)

// 러스트에는 null이 없는데 값의 존재와, 유효성을 어떻게 판단할까?
// 바로, Option<T> 라는 표준라이브러리 enum 타입을 통하여 확인할 수 있다.

/*
enum Option<T> {
    Some(T),
    None,  
} 

이 형식으로 표준라이브러리에 정의가 되어있다.
Option은 기본적으로 스코프에 포함되어, 명시적으로 로드하지 않아도 사용 가능
Option::를 안붙여도 되며 바로 Some과 None을 사용할수 있다.

! Some은 값이 있음을 나타내며, None은 값이 없음을 나타낸다.
*/

fn enum_test5() {
    let some_number = Some(5);
    let some_string = Some("a string");

    // 여기에만 타입을 지정한 이유는 None은 값을 가지지 않기에, 타입도 없다.
    // ! None을 지정하면 타입 추론이 불가능해져 에러가 발생함, 그렇기에 None은 타입을 임의로라도 지정해주어야 한다.
    let absent_number: Option<i32> = None;
}

// Some 자체를 값으로 사용할수 없다. 
// Some은 단지 Option이라는 enum의 필드 타입일 뿐이다.
fn enum_test6() {
    let x = 5;
    let y = Some(5);

    // 에러 발생!
    // i32와 Option<i32>는 더할수 없기 때문이다.
    // let sum = x + y;

    // Option 타입 열거형은 다음에 설명할 match라는 연산자를 통해 강력해질수 있다.
}

fn main() {
    enum_test1();
    enum_test2();
    enum_test3();
    enum_test4();
    enum_test5();
    enum_test6();
}
