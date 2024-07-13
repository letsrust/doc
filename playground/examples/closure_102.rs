use anyhow::Result;

fn main() -> Result<()> {
    // 这个闭包实现了 Interceptor trait，可以被当作 Interceptor trait 来调用
    let mut auth_interceptor = |mut request: String| -> Result<String, String> {
        request.push_str(" world");
        Ok(format!("Request: {}", request))
    };
    // 当作 Interceptor trait 调用
    let res = auth_interceptor.intercept("hello".into());
    println!("{:?}", res);

    // 把 interceptor 放在 vec 中
    let mut interceptors: Vec<Box<dyn Interceptor>> = Vec::new();
    interceptors.push(Box::new(auth_interceptor));
    interceptors.push(Box::new(|mut request: String| -> Result<String, String> {
        request.push_str(" uuid");
        Ok(format!("Request: {}", request))
    }));

    // 这是一个实现了 Interceptor trait 的结构体，放在 vec 中，和闭包可以放在一起，统一了结构体和闭包的调用方式
    interceptors.push(Box::new(UserAgent));

    let mut res = "hello".into();
    for ele in interceptors.iter_mut() {
        let inter_res = ele.intercept(res);
        res = inter_res.unwrap();
    }

    println!("{:?}", res);

    Ok(())
}

// 为闭包实现某个 trait

trait Interceptor {
    fn intercept(&mut self, request: String) -> Result<String, String>;
}

impl<F> Interceptor for F
where
    F: FnMut(String) -> Result<String, String>,
{
    fn intercept(&mut self, request: String) -> Result<String, String> {
        self(request)
    }
}

struct UserAgent;
impl Interceptor for UserAgent {
    fn intercept(&mut self, request: String) -> Result<String, String> {
        Ok(format!("{} user-agent", request))
    }
}
