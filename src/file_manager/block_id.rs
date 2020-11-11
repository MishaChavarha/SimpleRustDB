#[derive(PartialEq, Eq, Hash, PartialOrd, Ord, Debug)]
pub struct BlockId {
    file: String,
    number: u64
}

impl BlockId {
    pub fn new(file:String, number: u64) -> BlockId {
        BlockId{
            file,
            number
        }
    }

    pub fn number(&self) -> u64{
        return self.number;
    }

    pub fn file(&self) -> &str{
        return &self.file;
    }
}

#[cfg(test)]
mod test {
    use super::{BlockId};

    #[test]
    fn test_block_number(){
        let block = BlockId::new(String::from("abcd"),23);
        assert_eq!(block.number,23)
    }

    #[test]
    fn test_block_number_err(){
        let block = BlockId::new(String::from("abcd"),23);
        assert_ne!(block.number,25)
    }

}