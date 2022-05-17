// 구조체는 데이터를 모은 데이터 단위이다.
// 튜플과 똑같이 구성요소들이 각자 다른 타입을 가질 수 있지만, 각 구성요소들을 명명할수 있다는 차이점이 있다.

// 구조체 정의법이다.
// 구조체 정의후, 각 field의 이름과 타입을 지정해주면 된다.
// ! 구조체는 소유권을 가질수 있는 데이터타입을 필드로 선언해야 한다.
// ! 소유권이 없는 참조형을 필드로 선언 및 사용하면 라이프타임 에러가 발생한다.
struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

// 구조체를 생성할 때 필드들의 순서가 설계한 필드의 순서와 똑같을 필요가 없다.
fn struct_test1() {
    let user1 = User {
        email: String::from("wowwow@naver.com"),
        username: String::from("wowwow"),
        active: true,
        sign_in_count: 1,
    };

    // 간접 연산자(.) 으로 구조체의 필드에 접근이 가능하다.
    println!("email value is {}", user1.email);
    println!("user name is {}", user1.username);

    // mut을 붙여 구조체의 필드값을 바꿀 수 있다.
    let mut user2 = User {
        email: String::from("wowwow@naver.com"),
        username: String::from("wowwow"),
        active: true,
        sign_in_count: 1,
    };

    user2.email = String::from("owoowo@naver.com");
    println!("change email value is {}", user2.email);
}

// 구조체를 생성할 때 필드의 이름이 변수명과 같으면 축약하여 초기화할 수 있다.
fn struct_test2() {
    let user1 = create_user(String::from("helloworld@gmail.com"), String::from("hw"));
    let mut user2 = create_user(String::from("helloworld@gmail.com"), String::from("hw"));

    println!("email value is {}", user1.email);
    println!("user name value is {}", user1.username);

    user2.username = String::from("swag");
    println!("change user name value is {}", user2.username);
}

fn create_user(email: String, username: String) -> User {
    User{
        // 이런식으로 email: 를 안 붙여도 된다.
        email,
        username,
        active: true,
        sign_in_count: 1,
    }
}

// 구조체 갱신법을 이용하면 기존 구조체 인스턴스로 새 구조체 인스턴스를 생성할 수 있다.
fn struct_test3() {
    let user1 = User {
        email: String::from("wowwow@naver.com"),
        username: String::from("wowwow"),
        active: true,
        sign_in_count: 1,
    };

    let user2 = User {
        email: String::from("owoowo@naver.com"),
        username: String::from("owoowo"),
        // 이렇게 구조체를 갱신 시킬 수 있다.
        active: user1.active,
        sign_in_count: user1.sign_in_count,
    };

    let user3 = User {
        email: String::from("swgswg@naver.com"),
        username: String::from("swgswg"),
        // user2를 갱신시킬때와 똑같은 의미의 구문이다.
        // 초기화 되지 않은 나머지 필드를 user1의 필드와 똑같이 설정한다.
        ..user1
    };

    println!("user2 active is {}", user2.active);
    println!("user3 sign_in_count is {}", user3.sign_in_count);
}

// 튜플 구조체 정의
// 튜플과 똑같이 필드의 타입만 있고 명명은 할수 없다.
struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

fn struct_test4() {
    let black = Color(0,0,0);
    let origin = Point(0,0,0);

    println!("r : {}, g : {}, b : {}", black.0, black.1, black.2);
    println!("x : {}, y : {}, z : {}", origin.0, origin.1, origin.2);
}

struct Rectangle {
    length: u32,
    width: u32,
}

fn struct_test5() {
    let _rect1 = Rectangle { length: 50, width: 30 };

    // 에러 발생! 어떻게 구조체를 출력시켜야할지 모르기 때문이다.
    // println!("rect1 is {}", _rect1);
}

#[derive(Debug)]
struct ModifyRectangle {
    length: u32,
    width:u32,
}

fn struct_test6() {
    let _rect1 = ModifyRectangle { length: 50, width: 30 };
    println!("rect1 is {:?}", _rect1);
}

// 어떤 필드도 없는 구조체도 정의할 수 있다.
// 이를 유사 유닛 구조체라고 명칭한다.
// 특정한 타입의 트레잇을 구현해야하지만, 타입 자체에 데이터를 저장하지 않은 경우에 유용하다는데... 트레잇을 공부할때 다시한번 보자.
struct Empty;

fn main() {
    struct_test1();
    struct_test2();
    struct_test3();
    struct_test4();
    struct_test5();
    struct_test6();
}
