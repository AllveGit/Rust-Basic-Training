// 구조체안에 함수를 정의하여 구조체 인스턴스를 통해 함수를 호출할 수 있다.
struct Rectangle {
    length: u32,
    width: u32
}

// impl블록 (구현블록) 을 통해 구조체의 함수들을 정의할 수 있다.
impl Rectangle {
    // 구조체 함수들의 첫번째 파라미터는 무조건 self 여야 한다.
    // impl 블록 안에 있어 self의 타입이 Rectangle이라는 것을 알 수 있고,
    // self의 값은 이 함수를 호출한 인스턴스로 자동으로 채워진다.
    // self 앞에 &을 생략할수도, 붙일수도, &mut를 붙일수도 있다.
    // 즉, 함수가 self의 소유권을 가져갈수도, 변경할수도 있다는 소리다.
    fn area(&self) -> u32 {
        self.length * self.width
    }

    // self를 정의 안하는 함수도 있다.
    // 이를 연관함수로 명칭한다.
    // 연관함수는 구조체인스턴스에서 호출하는 것이 아닌 구조체 자체에서 네임스페이스 연산자(::)를 통해 호출을 할 수 있다.
    // ex) Rectangle::square(3);
    fn square(size: u32) -> Rectangle {
        Rectangle { length: size, width: size }
    }
}

fn struct_method_test1() {
    let rect1 = Rectangle { length: 50, width: 30 };
    let rect2 = Rectangle::square(10);

    println!("The area of the rect is {} square pixels.", rect1.area());
    println!("The area of the rect is {} square pixels.", rect2.area());
}

// -> 연산자와 동치 연산자가 러스트에는 없다.
// 다만, 러스트에는 자동 참조 및 자동 역참조 기능을 가지고 있다.
// 메소드를 호출하면, 러스트는 자동적으로 &나 &mut, 혹은 *을 붙여서 해당 메소드의 시그니처와 맞도록 호출할 때 자동으로 바꾼다.
// 이게 가능한 이유는 impl 블록안에 self를 self, &self, &mut self처럼 어떤식으로 정의했는지에 따라 시그니처를 다르게 하면되기 때문이다.

// self         : 소비한다.
// &self        : 읽는다.
// &mut self    : 쓴다.

fn main() {
    struct_method_test1();
}
