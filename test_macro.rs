macro_rules! html {
    ($tag:ident) => {
        format!("<{}></{}>", stringify!($tag), stringify!($tag))
    };
    ($tag:ident { $($attr:ident = $val:expr),* }) => {
        {
            let mut result = format!("<{}", stringify!($tag));
            $(result.push_str(&format!(" {}=\"{}\"", stringify!($attr), $val));)*
            result.push_str(">"));
            result.push_str("</");
            result.push_str(stringify!($tag));
            result.push_str(">"));
            result
        }
    };
    ($tag:ident $content:expr) => {
        format!("<{}>{}</{}>", stringify!($tag), $content, stringify!($tag))
    };
}

fn main() {
    let div1 = html!(div);
    let div2 = html!(div { class="container", id="main" });
    let div3 = html!(div html!(p "Hello World"));
    
    println!("div1: {}", div1);
    println!("div2: {}", div2);
    println!("div3: {}", div3);
}