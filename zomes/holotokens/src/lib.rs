use hdk::prelude::*;
use std::str;
use rand::seq::SliceRandom;

#[derive(Serialize, Deserialize, SerializedBytes, Debug, Clone)]
pub struct ZomeInput {
    pub tradeyear: String,
}

// data we want back from holochain
#[derive(Serialize, Deserialize, SerializedBytes, Debug, Clone)]
pub struct ZomeOutput {
    pub hot_tradingprice: String,    
}

#[hdk_extern]
pub fn fetch_averagehot(input: ZomeInput) -> ExternResult<ZomeOutput> {

let  choices2022 = vec!["$0.0062", "$0.0064", "$0.0060"];
let  mut rng = rand::thread_rng();
let choice2022 = choices2022.choose(&mut rng).unwrap();

let mut partialtradingprice = "unknown".to_string();

if input.tradeyear.starts_with("2021"){
    partialtradingprice ="$0.016".to_string() 
   }
  else if input.tradeyear.starts_with("2022"){
    partialtradingprice = choice2022.to_string()
  }

   Ok(ZomeOutput {
    hot_tradingprice : partialtradingprice 
   })

}