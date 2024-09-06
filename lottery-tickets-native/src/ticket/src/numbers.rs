use borsh::{BorshDeserialize, BorshSerialize};

//Needed to deserialize the numbers

#[derive(BorshDeserialize, BorshSerialize, Debug)]
pub struct LotteryNumbers {
    pub values: [u8; 6],  // 6 numbers of the lottery
}
