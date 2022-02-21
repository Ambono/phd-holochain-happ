use hdk::prelude::*;
use std::str;
use std::fmt;
use rand::seq::SliceRandom;
use rand::Rng; 


#[derive(Serialize, Deserialize, SerializedBytes, Debug, Clone)]
pub struct ZomeInput {
    pub patientinfo: String,
}

impl fmt::Display for ZomeInput{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result{
        write! (f, "ZomeInput patientinfo {}", self.patientinfo)
    }
}
// data we want back from holochain
#[derive(Serialize, Deserialize, SerializedBytes, Debug, Clone)]
pub struct ZomeOutput {
    pub booking_day: String,    
}

#[hdk_extern]
pub fn book_pcrtest(_input: ZomeInput) -> ExternResult<ZomeOutput> {
    
let choices = vec!["Monday", "Tuesday", "Wednesday", "Thursday", "Friday"];
let  mut rng = rand::thread_rng();
let choice = choices.choose(&mut rng).unwrap();

   Ok(ZomeOutput {
    booking_day: choice.to_string()
   })

}