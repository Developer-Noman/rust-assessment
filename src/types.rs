#[derive(Clone, Debug, PartialEq)]
pub struct Transaction {
    pub id: u64,
    pub wallet: String,
    pub value: u64,
}

#[derive(Clone, Debug)]
pub struct Batch {
    pub batch_id: u64,
    pub transactions: Vec<Transaction>,
}
#[derive(Clone, Debug)]
pub struct Message {
    pub id: u64,
    pub sender: String,
    pub content: String,
}

#[derive(Clone, Debug)]
pub struct Group {
    pub messages: Vec<Message>,
}