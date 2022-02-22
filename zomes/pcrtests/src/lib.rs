use hdk::prelude::*;
use std::str;
//use std::fmt;
//use rand::seq::SliceRandom;


#[derive(Serialize, Deserialize, SerializedBytes, Debug, Clone)]
pub struct ZomeInput {
    pub patientinfo: String,
}

// impl fmt::Display for ZomeInput{
//     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result{
//         write! (f, "ZomeInput patientinfo {}", self.patientinfo)
//     }
// }
// data we want back from holochain
#[derive(Serialize, Deserialize, SerializedBytes, Debug, Clone)]
pub struct ZomeOutput {
    pub booking_day: String,    
}

#[hdk_extern]
pub fn book_pcrtest(input: ZomeInput) -> ExternResult<ZomeOutput> {
    
// let choicesab = vec!["Monday", "Tuesday"];
// let  mut rng = rand::thread_rng();
// let choiceab = choicesab.choose(&mut rng).unwrap();

// let choicesef = vec!["Wednesday", "Thursday"];
// let  mut rng = rand::thread_rng();
// let choiceef = choicesef.choose(&mut rng).unwrap();

// let choicesuv = vec!["Friday", "Saturday"];
// let  mut rng = rand::thread_rng();
// let choiceuv = choicesuv.choose(&mut rng).unwrap();

let choicetransit :String;

if input.patientinfo.contains(&"ab"){
    choicetransit = "Monday".to_string()
}
else if input.patientinfo.contains(&"ef"){
    choicetransit = "Wednesday".to_string()
}
else if input.patientinfo.contains(&"uv"){
    choicetransit = "Thursday".to_string()
}

else if input.patientinfo.contains(&"st"){
    choicetransit = "Sunday".to_string()
}

else {
    choicetransit = "Friday".to_string()
}
   Ok(ZomeOutput {
    booking_day: choicetransit
   })

}