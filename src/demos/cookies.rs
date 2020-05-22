// Cookies
//
// Cookies ia built-in request guard: it allows you to get, set, and 
// remove cookies. Because Cookies is a request guard, an argument of 
// its type can simply be added to a handler.
//
// The code below results in the incoming request's cookies being 
// accessible from the handler. The example above retrieves a cookie 
// named "message". Cookies can also be set and removed using the 
// Cookies guard. The cookies example on GitHub illustrates further
//  use of the Cookies type to get and set cookies, while the Cookies
// documentation contains complete usage information.

use rocket::http::{Cookie,Cookies};

#[get("/get-cookie")]
pub fn get_cookie(cookies: Cookies) -> Option<String> {
    cookies.get("alpha").map(|cookie| 
        format!("Get cookie... name:{} value:{}", cookie.name(), cookie.value()))
}

#[get("/set-cookie")]
pub fn set_cookie(mut cookies: Cookies) -> Option<String> {
    cookies.add(Cookie::new("alpha", "bravo"));
    cookies.get("alpha").map(|cookie| 
        format!("Set cookie... name:{} value:{}", cookie.name(), cookie.value())
    )
}
