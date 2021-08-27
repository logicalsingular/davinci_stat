use std::collections::HashSet;

struct Total {
    likes: u32,
    dislikes: u32,
    messages: u32,
    unique_ankets: HashSet<String>,
} 

fn main() {
    let mut total = Total {
        likes: 0,
        dislikes: 0,
        messages: 0,
        unique_ankets: HashSet::new(),
    };

    let path = String::from("C:\\Users\\rr\\Desktop\\result.json");
    let json = get_data(path);
    let json = json["messages"].clone();


    let mut i = 0;

    loop {
        match &json[i] {
            serde_json::Value::Object(block) => calculate(block,&mut total),
            serde_json::Value::Null => break,
            _ => println!("Ошибка, обратитесь к создателю приложения\nКод ошибки: 1"),
        }
        i +=1;
    }

    let ankets = total.unique_ankets.len();
    println!("Анкет понравилось 👍:{}
Отправлено сообщений 💌: {}
Не понравилось 👎: {}
Уникальных анкет: {}",total.likes, total.messages, total.dislikes, ankets);

    // "Есть взаимная симпатия! Начинай общаться 👉 "
}

fn calculate(block: &serde_json::Map<String, serde_json::Value>,total: &mut Total) {
   if block["text"].eq("👍") {
        total.likes += 1;
    } else if block["text"].eq("💌") {
        total.messages += 1;
    } else if block["text"].eq("👎") {
        total.dislikes +=1;
    } else if block.contains_key("photo") || block.contains_key("media_type") {

        // total.unique_ankets.insert(String::from(block["text"].as_str().unwrap()));
        if let Some(data_block) = block["text"].as_str() {
            total.unique_ankets.insert(serde_json::to_string(data_block).unwrap());
        } else if let Some(data_block) = block["text"].as_array() {
                    /////////////////// we need calculate unique ankets. May be use enum. It will consist from string or array
        } else {
            println!("Error");
        }

    }
    // } else { println!("{:?}",block["text"])}
}

fn get_data(path: String) -> serde_json::Value {
    let read = std::fs::read(path).unwrap();
    let contents = std::str::from_utf8(&read).unwrap();
    serde_json::from_str(contents).unwrap()

} 