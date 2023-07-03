use reqwest::Client;
use headless_chrome::{Browser};
use std::thread;
use std::time::Duration;
type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;
use std::error::Error;

#[tokio::main]
async fn main() -> Result<()> {

    let _url = "https://diablo4.life/trackers/overview";
    let browser = Browser::default().unwrap();
    let tab = browser.new_tab().unwrap();
    tab.navigate_to(_url).unwrap();

    // ブラウザの読み込みを待つ
    // let wait_result = tab.wait_until_navigated();
    // match wait_result {
    //     Ok(_) => println!("ロード成功"),
    //     Err(err) => eprintln!("ロードエラー: {:?}", err),
    // }

    thread::sleep(Duration::from_secs(1));

    // let p_class_name = ".EventCountdown_countdown__sl85X";
    let p_class_name = ".EventCountdown_digits__6xesm";
    let parent_elements = match tab.wait_for_elements(p_class_name) {
        Ok(elements) => elements,
        Err(err) => {
            eprintln!("エレメント取得エラー: {:?}", err);
            return Err(err.into());
        },
    };

    for parent_element in parent_elements {
        // Get the first span child element
        let result = parent_element.call_js_fn("function() { return this.textContent; }", vec![], false)?;
        if let Some(text) = result.value {
            if let serde_json::Value::String(text) = text {
                println!("Node text: {}", text);
            }
        }

        // let description = match parent_element.get_description() {
        //     Ok(desc) => desc,
        //     Err(err) => {
        //         eprintln!("子の詳細: {:?}", err);
        //         continue;
        //     }
        // };
        // println!("Span text: {}", description.node_value);
        // let child_element = match parent_element.find_element("span") {
        //     Ok(element) => element,
        //     Err(err) => {
        //         eprintln!("子のエレメント取得エラー: {:?}", err);
        //         continue;
        //     }
        // };
        //
        // let description = match child_element.get_description() {
        //     Ok(desc) => desc,
        //     Err(err) => {
        //         eprintln!("子の詳細: {:?}", err);
        //         continue;
        //     }
        // };
        //
        // println!("Span text: {}", description.node_value);
    }
    // match tab.wait_for_element("#your-element-id") {
    //     Ok(_) => {
    //         println!("The element is loaded!");
    //         // Proceed with your code here
    //     },
    //     Err(err) => {
    //         eprintln!("An error occurred while waiting for the element: {:?}", err);
    //         // Handle error accordingly
    //     }
    // }

    // tab.navigate_to("https://diablo4.life/trackers/overview")?;
    // tab.wait_until_navigated()?;
    // let parent_elements = tab.wait_for_element(".EventCountdown_countdown__sl85X");
    // for parent_element in parent_elements {
    //     println!("{:#?}", parent_element);
    // }

    // let parent_elements = tab.wait_for_element(".EventCountdown_countdown__sl85X");

    // for parent_element in parent_elements {
    //     // 子要素 (span) を全て取得
    //     let child_elements = parent_element.find_elements("span");
    //
    //     for child_element in child_elements {
    //         // span のテキストを取得
    //         let text = child_element.get_description();
    //
    //         println!("Span text: {}", text);
    //     }
    // }
    send_discord_message("hogehoge").await?;
    Ok(())

}

async fn send_discord_message(msg: &str) -> Result<()> {
    let client = Client::new();
    let url = "https://discord.com/api/webhooks/1125345907735543858/bV9GucNfJC6OExrDi-yIAhiJhGyhZFGvgZpGjzI9O-tW_phqNhYek6J05S4nABllvDdX";
    let json_request = format!(
        r#"{{
              "content": "{}"
            }}"#,
        msg
    );
    let response = client
        .post(url)
        .header(reqwest::header::CONTENT_TYPE, "application/json")
        .body(json_request)
        .send()
        .await?;
    let body = response.text().await?;
    println!("{}", body);
    Ok(())
}

