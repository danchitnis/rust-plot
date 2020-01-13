extern crate web_view;


use web_view::*;




fn main() {
    web_view::builder()
        .title("Change background color")
        .content(Content::Html(HTML))
        .size(400, 300)
        .resizable(true)
        .debug(true)
        .user_data("")
        .invoke_handler(|webview, arg| {
            match arg {
                "red" => {
                    webview.set_color((255, 0, 0));
                    println!("I am here 🥙");
                    webview.set_title("Hello!")?;
                }
                "green" => {
                    
                    //let a:f32 = 1.25;
                    let array = vec![1.25f32, 0.0, 0.08, 5.0, 54.0, -100.1, -14.07];
                    
                    
                    let str = float_to_str_array(&array);
                    println!("{}", str);
                    webview.eval(&format!("dostuff('{}')", str))?;
                },
                "blue" => {
                    webview.eval(include_str!("./script.js"))?;
                },
                _ => ()
            }

            Ok(())
        })
        .run()
        .unwrap();
}

fn float_to_str_array(array: &Vec<f32>) -> String {
    
    let mut char_array = Vec::<char>::new();
    
    for e in array {
        //let s = float_to_str(*e);
        let tmp = float_to_char(*e);
        for t in tmp {
            char_array.push(t);
        }
    }

    return char_array.into_iter().collect();
}

fn float_to_char(num: f32) -> Vec<char> {
    let b = num.to_be_bytes();
    println!("{:?}", b);

    
    let s = String::from(&format!("{:02x}{:02x}{:02x}{:02x}", b[0], b[1], b[2], b[3]));
    println!("{:?}",s);
    return s.chars().collect()
}

const HTML: &str = r#"
<!DOCTYPE html>
<html>
    <head>
    </head>
	<body>
		<button onclick="external.invoke('red')">red</button>
        <button onclick="external.invoke('green')">green</button>
        <button onclick="external.invoke('blue')">blue</button>
        <p id="info"></p>
        <p id="float"></p>

        <script type="text/javascript">
            console.log(window.location.href);
            console.log("hi!");
        </script>
    </body>
    
</html>
"#;