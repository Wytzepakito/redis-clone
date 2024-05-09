use crate::marshall::MessageSegment;



pub struct LinesMarshaller {}



impl LinesMarshaller {
    pub fn new() -> LinesMarshaller {
        LinesMarshaller {}
    }



    pub fn parse_master_commands(&self, lines: String) -> Vec<Result<MessageSegment, String>> {

        let mut vec_lines: Vec<&str> = lines.lines().collect();
        let mut results: Vec<Result<MessageSegment, String>> = Vec::new();
        while vec_lines.len() != 0 {
            results.push(self.parse_segment_strings(&mut vec_lines));
        }

        results
    }

    fn parse_segment_strings(&self, lines: &mut Vec<&str>) -> Result<MessageSegment, String> {
            let mut segment = lines.remove(0); 
            let (segment_type, data) = segment.trim().split_at(1);

            match segment_type {
                "*" => self.parse_array(data, lines),
                "$" => self.parse_bulk_string(lines),
                "+" => self.parse_simple_string(data),
                _ => unimplemented!(),
            }
    }

    fn parse_array(
        &self,
        data: &str,
        lines: &mut Vec<&str>,
    ) -> Result<MessageSegment, String> {
        let element_count: i32 = data
            .parse()
            .map_err(|e| {
                String::from(format!(
                    "Could not parse array length {data} with error {e}."
                ))
            })
            .expect("Couldn't parse this");
        let mut words = Vec::new();

        for _ in 0..element_count {
            words.push(self.parse_segment_strings(lines)?);
        }
        Ok(MessageSegment::Array(words))
    }

    fn parse_bulk_string(
        &self,
        lines: &mut  Vec<&str>,
    ) -> Result<MessageSegment, String> {
        let mut segment = lines.remove(0);
        Ok(MessageSegment::BulkString(
            segment.trim().to_owned().to_ascii_lowercase(),
        ))
    }

    fn parse_simple_string(&self, data: &str) -> Result<MessageSegment, String> {
        Ok(MessageSegment::SimpleString(
            data.to_string().to_ascii_lowercase(),
        ))
    }
}
