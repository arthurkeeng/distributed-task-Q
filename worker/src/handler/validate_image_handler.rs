use std::collections::HashMap;

use base64::{Engine, engine::general_purpose};
use common::{PayloadField, Task, TaskPayloadSchema , FieldType};
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
        // the image must be encoded in base 64
        let payload = &task.payload;
        let payload_schema = serde_json::json!({
            "task_type": "validate_image",
            "description" : "This handler helps you verify if an image is truly an image",
            "fields" : {
                "field_type" : "string", 
                "required" : "true"
            }
        });

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
impl ValidateImageHandler{
    pub fn schema () -> TaskPayloadSchema {
        TaskPayloadSchema { task_type: "validate_image".to_string(), description: "Validate that a base64 image is real and large enough".to_string(),fields: 
            HashMap::from([
                (
                    "image".to_string(), 
                    PayloadField {
                        field_type : FieldType::String, 
                        required : true , 
                        description : Some("Base 64 Encoded image".to_string()), 
                        example : Some(json!("1vsdf3409l;a dl...."))
                    }
                )
            ])
     }
    }

    

}

