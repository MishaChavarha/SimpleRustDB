
    #[derive(PartialEq, Eq, Hash, PartialOrd, Ord, Debug)]
    pub struct Block {
        file: String,
        id: u64
    }
    
    impl Block{
        pub fn new(file: &str, id: u64) -> Block {
            Block {
                file: String::from(file),
                id
            }
        }
    
        pub fn id(&self) -> u64{
            return self.id;
        }
    
        pub fn file(&self) -> &str{
            return &self.file;
        }
    }
    
    #[cfg(test)]
    mod test {
        use super::{Block};
    
        #[test]
        fn test_block_number(){
            let block = Block::new("abcd",23);
            assert_eq!(block.id,23)
        }
    
        #[test]
        fn test_block_number_err(){
            let block = Block::new("abcd",23);
            assert_ne!(block.id,25)
        }
    
    }