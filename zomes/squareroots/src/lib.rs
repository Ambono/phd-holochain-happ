use hdk::prelude::*;

#[derive(Serialize, Deserialize, SerializedBytes, Debug, Clone)]
pub struct ZomeInput {
    pub number: i32,
}

// data we want back from holochain
#[derive(Serialize, Deserialize, SerializedBytes, Debug, Clone)]
pub struct ZomeOutput {
    pub square_root: i32,
}

#[hdk_extern]
pub fn square_root(input: ZomeInput) -> ExternResult<ZomeOutput> {
    Ok(ZomeOutput {
        square_root: input.number * input.number
    })
}
