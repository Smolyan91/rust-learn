use std::str::FromStr;

extern crate iron;
#[macro_use]
extern crate mime;

use iron::prelude::*;
use iron::status;

extern crate router;
use router::Router;

extern crate urlencoded;
use urlencoded::UrlEncodedBody;

fn main() {
    print!("Start server on port 3000. Open localhost:3000");

    let mut router = Router::new();

    router.get("/", get_form, "root");
    router.post("/gcd", post_form, "gcd");

    Iron::new(router).http("localhost:3000").unwrap();
}

fn get_form(_req: &mut Request) -> IronResult<Response> {
    let mut response = Response::new();

    set_mime_and_status_success(&mut response);
    response.set_mut(
        r#"
        <title>GCD Calculator</title>
        <form action="/gcd" method="post">
            <input type="text" name="n"/>
            <input type="text" name="n"/>
            <button type="submit">Compute GCD</button>
        </form>
    "#,
    );

    Ok(response)
}

fn post_form(req: &mut Request) -> IronResult<Response> {
    let mut response = Response::new();

    let form_data = match req.get_ref::<UrlEncodedBody>() {
        Err(_) => {
            set_mime_and_status_unsuccess(&mut response, "Error parsing form data");
            return Ok(response);
        }
        Ok(map) => map,
    };
    let unparsed_nums = match form_data.get("n") {
        None => {
            set_mime_and_status_unsuccess(&mut response, "Not fount params in form data");
            return Ok(response);
        }
        Some(nums) => nums,
    };

    let mut nums = Vec::new();
    for unparsed in unparsed_nums {
        match u64::from_str(&unparsed) {
            Err(_) => {
                set_mime_and_status_unsuccess(
                    &mut response,
                    "Value for 'n' parameter not a number",
                );
                return Ok(response);
            }
            Ok(n) => {
                nums.push(n);
            }
        }
    }

    let mut d = nums[0];
    for n in &nums[1..] {
        d += n;
    }

    set_mime_and_status_success(&mut response);
    response.set_mut(format!("Sum is: {}", d));
    Ok(response)
}

fn set_mime_and_status_success(res: &mut Response) -> () {
    res.set_mut(status::Ok);
    res.set_mut(mime!(Text/Html; Charset=Utf8));
}

fn set_mime_and_status_unsuccess(res: &mut Response, str_err: &str) -> () {
    res.set_mut(status::BadRequest);
    res.set_mut(format!("{}", str_err));
}
