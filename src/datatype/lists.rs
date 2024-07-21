#[derive(Clone, Debug)]
pub struct Lists(Vec<Vec<u8>>);

impl Lists {
    pub fn lpush(&mut self, data: Vec<u8>) {
        self.0.push(data)
    }
    pub fn lpop(&mut self) {

    }
    pub fn llen(&self) -> usize {
        self.0.len()
    }

    pub async fn blpop(&mut self) {}
    pub async fn blmove(&mut self, another: &mut Lists) {}
}

