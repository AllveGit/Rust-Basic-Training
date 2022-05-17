fn main() {
    // rust의 변수들은 기본적으로 불변성이다.
    // 한번 값이 대입되고 나서 바꿀수 없다는 걸 의미한다.
    // const랑 헷갈리면 안된다.

    /*
    ===== Test Case1 =====
    그래서 이런식으로 변수를 할당하고 변경시 컴파일 에러가 발생하게 된다.
    */

    let x = 5;
    println!("The value of x is : {}", x);
    // x = 6; (에러 발생!)
    // println!("The value of x is : {}", x);

    /*
    ===== Test Case2 =====
    변수명의 접두어로 mut를 추가해주면 가변성의 변수를 만들 수 있음.
    */

    let mut x = 5;
    println!("The value of x is : {}", x);
    x = 6;
    println!("The value of x is : {}", x);

    /*
    ===== Test Case3 =====
    완전한 상수를 정의할려면 let키워드 대신에 const키워드를 사용하며, 값의 유형까지 정해줘야 한다.
    함수 호출의 결과값이나 컴파일 타임 외의 실행시간에 결정되는 값으로 설정불가능
    */

    const MAX_POINTS: u32 = 100_000;


    /*
    ===== Test Case4 =====
    let을 이용하여 이미 있는 변수와 같은 이름의 변수를 선언하였을 때 그 변수는 Shadowed 되었다고 한다.
    mut으로 값을 바꾸는 것과 다른점은 다른 유형(타입)의 변수를 같은이름으로 만들어낼 수 있다는 점이다.
    */

    let spaces = "   ";
    let spaces = _spaces.len();

    // let mut spaces = "   "
    // spaces = spaces.len(); (에러 발생!)
}
