use base64::{Engine, engine::general_purpose};
use common::Task;
use async_trait::async_trait;
use image::GenericImageView;
use serde_json::{Value, json};
use crate::handler::registry::{HandlerResult, TaskHandler};
pub struct ValidateImageHandler;

// request struct

// {
//     task_type : String , 
//     image : String
// }

#[async_trait]
impl TaskHandler for ValidateImageHandler{
    async fn handle( &self , task : &Task) -> HandlerResult{
        let payload = &task.payload;

        let img_b64 = match payload.get("image").and_then(|v| v.as_str()){
            Some(v) => v,
            None => return HandlerResult::err("Missing image field".to_string())
        };

        // here , we decode from base 64 to bytes
        let decoded = match general_purpose::STANDARD.decode(img_b64){
            Ok(d) => d , 
            Err(e) => return HandlerResult::err(format!("Base 64 decode failed"))
        };

        let img = match image::load_from_memory(&decoded){
            Ok(i) => i , 
            Err(e) => return HandlerResult::err(format!("image decode failed"))
        };

        let (width , height) = img.dimensions();

        if width < 100 || height < 100{
            return HandlerResult::err(format!("Image resolution too low : {} {}" , width , height))
        }
        
        HandlerResult::ok(serde_json::json!(
            {
                "valid" : true , 
                "width" : width , 
                "height" : height
            }
        ))
    }
}