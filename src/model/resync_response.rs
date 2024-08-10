use crate::marshall::MessageSegment;


#[derive(Debug)]
pub struct ResyncResponse {
    response: MessageSegment,
    rdb_file: String
}



impl ResyncResponse {
    pub fn new(response: MessageSegment, rdb_file: String) -> ResyncResponse {
        ResyncResponse {
            response: response,
            rdb_file: rdb_file
        }
    }
}