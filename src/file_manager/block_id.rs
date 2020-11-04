#[derive(PartialEq, Eq, Hash, PartialOrd, Ord, Debug)]
pub struct BlockId {
    file_name: String,
    block_number: u64
}

impl BlockId {
    pub fn new(file_name:String, block_number: u64) -> BlockId {
        BlockId{
            file_name,
            block_number
        }
    }

    pub fn block_number(&self) -> u64{
        return self.block_number;
    }
}

#[cfg(test)]
mod test {
    use super::{BlockId};

    #[test]
    fn test_block_number(){
        let block = BlockId::new(String::from("abcd"),23);
        assert_eq!(block.block_number,23)
    }

    #[test]
    fn test_block_number_err(){
        let block = BlockId::new(String::from("abcd"),23);
        assert_ne!(block.block_number,25)
    }

}