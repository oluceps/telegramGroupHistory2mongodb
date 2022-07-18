use bson::{doc, Document};
use mongodb::Client;
use serde::Deserialize;
use serde_json::Value;
use std::fs;
use std::fs::File;
use std::io::Write;
use uuid::Uuid;
use std::env;


//#[derive(Deserialize, Debug)]
//struct Result {
//    name: String,
//    group_type: String,
//    id: u32,
//    messages: Vec<Msgs>,
//}
//
//
//#[derive(Deserialize, Debug)]
//struct Msgs {
//    id: u8,
////    msg_type: String,
//    date: String,
//    actor: String,
//    actor_id: String,
//    action: String,
//    title: String,
//    text: String,
//    from: String,
//    from_id: String,
//    photo: String,
//    file: String,
//    thumbnail: String,
//    media_type: String,
//    sticker_emoji: String,
//    reply_to_message_id: u8,
//
//}

#[derive(Deserialize, Debug)]
struct ParseResult {
    messages: Vec<Value>,
}

impl ParseResult {
    fn init(path: String) -> ParseResult {

        let raw_contents = fs::read_to_string(
        &path,
        )
        .expect("Something went wrong reading the file");
        let json_msg_array: ParseResult = serde_json::from_str(&raw_contents).unwrap();

        json_msg_array
    }

    fn to_doc_vecs(msg_array: ParseResult) -> Vec<Document> {
        let mut doc_vec: Vec<Document> = Vec::new();

        for per_msg in msg_array.messages {
            doc_vec.push(bson::to_document(&per_msg).unwrap());
            println!("{:?} convert successfully !", &per_msg);
        }

        doc_vec
    }
}


#[tokio::main]
async fn main() -> std::result::Result<(), Box<dyn std::error::Error>> {

    let args: Vec<String> = env::args().collect();

    let source = &args[1];
    let url = &args[2];

 
    let client = Client::with_uri_str(
        url
    ).await?;
    
    let id = Uuid::new_v4();
    let collection = client.database("telegram_history").collection(&id.to_string());
    collection.insert_many(
        ParseResult::to_doc_vecs(ParseResult::init(source.to_string())), None
    ).await?;
    Ok(())
    
}

//async fn run() -> Result<()> {
//
//    let client = Client::with_uri_str("").await.unwrap();
//
//    let collection = client.database("telegram_history").collection("ava");
//    let handle = 
//        collection.insert_one(doc! { "x": 1 }, None); 
//
//    handle.await.unwrap();
//    Ok(())
//
//}
//


#[allow(unused)]
fn json_process(raw_contents: String, result_file: File) {
    let json_msg_array: ParseResult = serde_json::from_str(&raw_contents).unwrap();

    //    let bson = bson::to_bson(&json_msg_array["messages"]).unwrap();

    // let obj: Map<String, Value> = json_msg_array["messages"].as_object().unwrap().clone();

    //    println!(
    //        "{:?}",
    //        bson::to_vec(&bson::to_bson(&json_msg_array["messages"]).unwrap()).unwrap()
    //    );

    //write_to(bson);

    //
    //    for per_msg in json_msg_array.messages {
    //        // println!("{:?}", per_msg);
    //        let per_bson_doc: Document = bson::to_document(&per_msg).unwrap();
    //        write_to(per_bson_doc, result_file);
   //    }

    //    for index in 0..50092 {
    //
    //                //..48017
    //    }

    //    bson::to_vec(
    //        &bson::to_document(&serde_json::to_vec(&json_responses["messages"]).unwrap()).unwrap(),
    //    )
    //    .unwrap();
    //
    //mid = bson::to_document(&final_vec).unwrap();

    // bson::to_vec(&mid).unwrap()
}

#[allow(unused)]
fn write_to(bson_doc: Document, mut result_file: File) {
    //    let doc = Document::new();
    //    let mut buf = Vec::new();
    //
    //    encode_document(&mut buf, &doc).unwrap();
    //
    //    println!("{:?} {:?}", &result_file, &json_serialized_vec);

    //    std::fs::write("./result.bson", bson_value);
    //
    let bson_vec: Vec<u8> = bson::to_vec(&bson_doc).unwrap();

    match result_file.write_all(&bson_vec) {
        Err(why) => panic!("couldn't write  {}", why),
        Ok(_) => println!("successfully wrote "),
    };
}
