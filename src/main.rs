extern crate rand;
extern crate web_view;

use rand::prelude::*;
use web_view::*;

fn main() {
    let html = format!(
        r#"
    <html>
        <head>
        </head>
        <body>
            <button class="button" onclick="external.invoke('red')">red</button>
            <button class="button" onclick="external.invoke('green')">green</button>
            <button class="button" onclick="external.invoke('blue')">blue</button>
            <div class="canvas" id="myDiv"></div>
            <p id="info"></p>
            <p id="float"></p>

            <style>
                {}
            </style>

            <script type="text/javascript">
                {}
            </script>
            
            <script type="text/javascript">
                {}                
            </script>
        </body>
        
    </html>"#,
        include_str!("./style.css"),
        include_str!("../dist/bundle.js"),
        r#"function comm(str){  EntryPoint.dostuff(str); }"#
    );

    println!("{}", r#"function call(str){}"#);

    web_view::builder()
        .title("Change background color")
        .content(Content::Html(html))
        .size(800, 400)
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
                    //let array = vec![1.25f32, 0.0, 0.08, 5.0, 54.0, -100.1, -14.07, 1.047];
                    let mut array = vec![0f32; 100];

                    array_gen(&mut array);
                    let str = float_to_str_array(&array);
                    println!("{}", str);
                    webview.eval(&format!("comm('{}')", str))?;

                    webview.eval(&format!("EntryPoint.plot()"))?;
                }
                "blue" => {
                    webview.eval(&format!("EntryPoint.plot()"))?;
                    //webview.exit();
                }
                _ => (),
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
    println!("{:?}", s);
    return s.chars().collect();
}

fn array_gen(array: &mut Vec<f32>) {
    let mut rng = rand::thread_rng();

    for e in array {
        let r: f32 = rng.gen();
        *e = r;
    }
}
