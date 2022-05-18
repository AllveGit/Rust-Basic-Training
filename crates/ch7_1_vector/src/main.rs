// 컬렉션들은 스택이 아닌 힙에 저장되는 타입들이다. 
// 그 컬렉션 타입중 벡터타입을 사용할 것이다.

fn vector_test1() {
    // 새 벡터를 정의하는 법

    // i32 타입들을 저장하는 벡터를 생성한 것이다. 
    // Vec::new()를 이용하여 생성하면 타입을 알수없기에 타입을 명시해주는 것이 좋다.
    // 값을 추가할 때 추가하는 값으로 타입추론이 되긴 한다.
    let v: Vec<i32> = Vec::new();

    // 똑같이 i32 타입들을 저장하는 벡터를 생성한 것이다.
    // 하지만 초기 값을 가지고 있으며, 초기 값의 타입을 알 수 있기에 타입명시는 해주지 않아도 된다.
    // vec! 매크로를 이용하여 생성할 수 있다.
    let v = vec![1, 2, 3];
}

fn vector_test2() {
    // 벡터도 마찬가지로 요소들을 추가, 제거, 변경을 하려면 mut을 해주어야 한다.
    let mut v = Vec::new();
    // push로 벡터에 값을 추가 시킬수 있다.
    v.push(1);
    v.push(2);
    v.push(3);
    v.push(4);
} // 벡터도 마찬가지로 스코프밖으로 벗어나면 벡터를 drop시키며, 그 안의 내용물까지 전부 drop이 된다. 이때 발생할 수 있는 이슈들이 있다.

fn vector_test3() {
    let v = vec![1, 2, 3, 4, 5];

    // 이렇게 첨자연산자([])로 요소를 가져온다.
    let third: &i32 = &v[2];
    // 벡터의 get함수에 인덱스를 넣어 호출하면 Option<&T>타입으로 요소의 값을 리턴해준다.
    let third: Option<&i32> = v.get(2);

    // 두가지 방식을 제공하는 이유는 벡터가 가지고 있지 않은 인덱스의 값을 가져올려 하였을 때, 처리를 선택할 수 있게 하기 위함.
    // 첫번째 방법은 바로 panic이 일어나고, 두번째 방법은 그냥 None이 리턴될 뿐이다. 크래시가 나냐 안나냐 차이..
}

fn vector_test4() {
    let mut v = vec![1, 2, 3, 4, 5];

    // 불변 참조자로 벡터의 첫번째 요소를 참조
    let first = &v[0];

    // 가변 참조자를 이용하여 벡터에 요소를 추가한다.
    // 에러 발생! 
    // 같은 스코프내에서 불변 참조자와 가변 참조자는 동시에 있을 수 없기 때문이다.
    // 이는 벡터의 요소들의 참조자가 계속 유효한 값을 가질 수 있도록 보장해준다.
    /* v.push(6); */ 
}

fn vector_test5() {
    let v = vec![100, 32, 57];

    // for루프를 이용하여 컬렉션들은 iterator 반복을 사용할 수 있다.
    // 루프할때마다 각각의 요소를 순서대로 매핑한다.
    for i in &v {
        println!("{}", i);
    }

    // 이런식으로 호출하면 v가 iterator에 소유권이 이동이 된다.
    // 그래서 다음에 v에 접근하면 컴파일 에러가 난다.
    /* 
    for i in v {
        println!("{}", i);
    }
    */

    let mut v2 = vec![100,32, 57];

    // 반복을 하며 요소들을 변경시킬려면 가변 참조자를 붙여야 한다.
    for i in &mut v2 {
        // 가변 참조자가 값을 참조하고 있으므로 역참조를 하여 값에 접근하고 바꾼다.
        *i += 50;
    }

    for i in &v2 {
        println!("{}", i);
    }
}

enum SpreadSheetCell {
    Int(i32),
    Float(f64),
    Text(String),
}

fn vector_test6() {
    // 열거형 타입을 벡터에 저장하여 서로 다른 데이터타입을 저장하게 할수도 있다.
    // 이런 방식은 런타임에 벡터에 저장하게 될 타입의 모든 경우를 알아야 한다.
    // 런타임에 모든 경우를 알지 못하면 트레잇 객체를 이용하면 된다고 한다.. TODO
    let row = vec![
        SpreadSheetCell::Int(3),
        SpreadSheetCell::Text(String::from("blue")),
        SpreadSheetCell::Float(10.12),
    ];
}

fn main() {
    vector_test1();
    vector_test2();
    vector_test3();
    vector_test4();
    vector_test5();
    vector_test6();
}
