// 지금까지의 cargo는 rust 실행파일을 제작하는 cargo들이었다.
// 이번에는 rust 라이브러리를 제작하는 cargo이다.

// client.rs의 client 모듈을 선언하는 것이다.
// 선언을 하면 다른 파일내에서 client 모듈을 찾는다.
// ! client 모듈과 똑같은 이름의 파일내에서 찾는다.
// ! mod client {} 와 똑같은 의미이다. 이 안의 내용물을 다른 파일에서 찾는 것이다.
// ! 그렇기에 client.rs에서 mod client를 또 정의하면 client::client::로 접근을 하게 된다. 그냥 함수를 client.rs에 정의하면 client::로 접근이 가능해진다.
mod client; 

// 모듈정의를 하는 법은 mod 뒤에 이름을 붙이면 된다.
// 모듈에 접근하는법은 네임스페이스 연산자(::)를 통해 접근할 수 있다.
mod network {
    fn connect() {
    }

    // 모듈안에 모듈을 정의할수도 있다.
    // 바깥의 client와는 network::client::connect(), client::connect() 이런식으로
    // 접근 방식이 다르기 때문에 서로 부딪힐일이 없다.
    mod client {
        fn connect() {
        }
    }
}

// 같은 파일 내에 여러개의 모듈을 정의할 수 있다.
// mod client {
//     fn connect() {
//     }
// }