use std::collections::HashSet;

struct Total {
    put_likes: u32,
    dislikes: u32,
    messages: u32,
    maches: u32,
    liked: u32,
    unique_ankets: HashSet<String>,
} 

fn main() {
    let mut total = Total {
        put_likes: 0,
        dislikes: 0,
        messages: 0,
        maches: 0,
        liked: 0,
        unique_ankets: HashSet::new(),
    };

    let path = String::from("C:\\Users\\logic\\Desktop\\result.json");
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
Уникальных анкет: {}
Взаимных симпатий 👉: {}
Ты понравился, и вы начали общение ❤️: {}",total.put_likes, total.messages, total.dislikes, ankets, total.maches, total.liked);
}

fn calculate(block: &serde_json::Map<String, serde_json::Value>,total: &mut Total) {
   if block["text"].eq("👍") {
        total.put_likes += 1;
    } else if block["text"].eq("💌") {
        total.messages += 1;
    } else if block["text"].eq("👎") {
        total.dislikes +=1;
    } else if block["text"][0].eq("Есть взаимная симпатия! Начинай общаться 👉 "){
        total.maches +=1;
    } else if block["text"][0].eq("Отлично! Надеюсь хорошо проведете время ;) Начинай общаться 👉 "){
        total.liked +=1;
    } else if block.contains_key("photo") || block.contains_key("media_type") {
        
        if let Some(string_block) = block["text"].as_str() {
            total.unique_ankets.insert(serde_json::to_string(string_block).unwrap());
        } else if let Some(arr_block) = block["text"].as_array() {
                // iterate array, that concatinate strings. If concatinated string is object, get     
        }

    }
}

fn get_data(path: String) -> serde_json::Value {
    let read = std::fs::read(path).unwrap();
    let contents = std::str::from_utf8(&read).unwrap();
    serde_json::from_str(contents).unwrap()

} 