
pub struct Hand 
{
    hand_size : u32,
    cards_in_hand : Vec<u32>,
}

impl Hand
{
    pub fn build_hand(m_hand_size : u32) -> Self
    {
        let vec_size = m_hand_size.try_into().unwrap();
        
        Self{
            hand_size : m_hand_size,
            cards_in_hand : Vec::<u32>::with_capacity(vec_size),
        }

    }

    pub fn add_card_to_hand(&mut self, dealt_card : u32)
    {

        
    }




    //Accessor Methods

    pub fn get_amount_of_cards(&self) -> u32
    {
        self.cards_in_hand.len().try_into().unwrap()
    }

   


}
