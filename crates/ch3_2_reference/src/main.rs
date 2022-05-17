// 저번 cargo 마지막 테스트케이스와 똑같은 함수이지만, 하나 다른점은 참조자를 썼다는 것이다.
// 참조자를 사용하면 값을 참조하지만 소유권은 가지고있지 않게 된다. 그렇기에 함수안으로 이동하고 빠져나올때도 값이 drop되지 않는다.  
// 즉, 호출때 인자로 넣은 값이 계속 유효한 값으로 유지가 된다. 
// 함수의 매개변수로 참조자를 사용하는 걸 빌림이라고 한다. 

// 빌린 값은 수정할 수 없다. 수정을 하면 컴파일 에러가 난다.
fn reference_test1() {
    let s1 = String::from("hello");

    let len = calculate_strlen(&s1);

    println!("The length of '{}' is {}.", s1, len);
}

fn calculate_strlen(s: &String) -> usize {
    s.len()
}

// 빌린 값을 수정할 수 있게 할려면 참조자 뒤에 mut을 붙이면 된다.
// 이를 가변 참조자라고 한다. 
// ! 가변 참조자를 사용하기 위해선 참조하는 값 자체도 가변적이어야 한다.
fn reference_test2() {
    let mut s = String::from("hello");
    
    change(&mut s);

    println!("{}", s);
}

fn change(s: &mut String) {
    s.push_str(", world");
}

// 가변 참조자는 같은 스코프 내에서 특정 데이터의 가변참조자를 딱 하나만 생성할 수 있다는 제한이 있다.
fn reference_test3() {
    let mut s = String::from("hello");

    let r1 = &mut s;
    // 같은 스코프내에서 s의 가변참조자가 두개이상이 되었으므로 컴파일 에러가 난다.
    // let r2 = &mut s;

    let mut s2 = String::from("world");

    {
        let sr1 = &mut s2;
    } // 여기서 sr1은 스코프 밖으로 벗어난다.

    // sr1이 스코프밖으로 벗어나 없어졌으므로 가변참조자를 생성할 수 있다.
    let sr2 = &mut s2;


    // 이런식으로 rust는 가변을 허용하지만 매우 통제된 형식으로 허용한다.
    // 이러한 제한이 가지는 이점은 바로 "데이터 레이스(경쟁)"를 방지해준다.
    // rust는 데이터레이스를 발생할 수 있는 코드 자체를 컴파일 통과시켜주지 않는다.
}

// 가변 참조자와 불변 참조자를 혼용하면 에러가 난다.
fn reference_test4() {
    let mut s = String::from("hello");

    let r1 = &s; // 문제 없음
    let r2 = &s; // 문제 없음
    let r3 = &mut s; // 에러 발생

    // rust는 불변 참조자를 사용중일 때 값이 바뀌리라고 예상하지 못하기에,
    // 불변 참조자를 사용중일 때 가변 참조자를 사용하면 에러를 발생시킨다.
}

// 댕글링 참조자가 발생하면 에러가 난다.
// 댕글링 참조란, 이미 해제된 메모리를 참조하고 있는 상황을 말한다.
// 포인터를 사용하는 언어는 댕글링 포인터라고 칭한다.
fn reference_test5() {
    let _dangle_ref = dangle();
}

fn dangle() -> String { // &String {
    // 값을 정의하였고
    let s = String::from("hello");

    //&s    // 에러 발생!
            // 할당이 해제된 리턴값을 받아왔기 때문에 댕글링 참조자가 반환이 된것이다.
            // 그렇기에 rust는 이같은 상황이 일어날 것을 알기에 컴파일 에러를 발생시킨다.

    s       // 그냥 s를 반환하면 컴파일 에러가 나지 않고 정상작동한다.

} // s의 참조자를 리턴하면 소유권이 s에게 있으므로 s는 할당해제 된다.


// 참조자는 항상 유효해야되고, 
// 하나의 가변참조자, 임의 개수의 불변참조자들, 이 두 케이스 중 하나만 사용할 수 있다.
fn main() {
    reference_test1();
    reference_test2();
    reference_test3();
    reference_test4();
    reference_test5();
}
