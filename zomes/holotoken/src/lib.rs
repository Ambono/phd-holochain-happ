use hdk::prelude::*;
use std::str;

#[derive(Serialize, Deserialize, SerializedBytes, Debug, Clone)]
pub struct ZomeInput {
    pub tradeyear: String,
}

// data we want back from holochain
#[derive(Serialize, Deserialize, SerializedBytes, Debug, Clone)]
pub struct ZomeOutput {
    pub hot_trading_price: String,    
}

#[hdk_extern]
pub fn fetch_averagehot(_input: ZomeInput) -> ExternResult<ZomeOutput> {
let x= vec!["$0.0162", "$0.0164", "$0.0160"];
let  mut rng = rand::thread_rng();
let choicex = x.choose(&mut rng).unwrap();

let y= vec!["$0.0062", "$0.0064", "$0.0060"];
let  mut rng = rand::thread_rng();
let choicey = y.choose(&mut rng).unwrap();

   Ok(ZomeOutput {
       //if input.date.starts_with("2021"){
        hot_trading_price: "0.0061".to_string() //choicex.to_string()
       //}
      //else{
       // hot_trading_price: choicey.to_string()
      //}
   })

}