use rand::Rng;

pub struct Deck 
{   
    //How many decks are we using?
    deck_count : u32,
    //How many cards are in the deck?
    deck_size : u32,
    //Keeps track of all cards that have been 'dealt'
    dealt_cards : Vec<u32>,
    //How many cards are in a suit?
    //Used for things like tarot decks,
    cards_in_a_suit : u32,
    //Honestly these last two don't really need commenting
    suits_in_a_deck : u32,
}

impl Deck
{
	///Constructor for Deck, instantiates a deck of cards but doesn't 
	///handle game logic or how individual hands are done; instead serves
	///as an API for modules that do so.
	///build_deck(deck_count,deck_size,cards_in_a_suit,suits_in_a_deck)
	///all args are in u32.
    pub fn build_deck( m_deck_count : u32, m_deck_size : u32,
                   m_cards_in_a_suit : u32, m_suits_in_a_deck : u32) -> Self
    {
        Self {
            deck_count : m_deck_count,
            deck_size : m_deck_size,
            dealt_cards : vec![0],
            cards_in_a_suit : m_cards_in_a_suit,
            suits_in_a_deck : m_suits_in_a_deck,
        }
        
       
    }
	///generates a random number, bounded between 1 and deck_size
	///this number maps to a card, the number is passed to
	///check_card() to prevent duplicate cards, and valid numbers 
	///are then passed to the caller for whatever purpose.

	///If all available cards have been played then method returns
	///-1 and doesn't run RNG.
    pub fn draw_card(&mut self) -> i32
    {
		if self.dealt_cards.len() == (self.deck_count*self.deck_size + 1).
													try_into().unwrap() 	
		{
			return -1
		}	
		let mut draw = 256_000; //arbitrary number idk
		let mut test_passed = false;		
		
		//loops until a generated number passes check_card()
		//will currently endlessly loop if all available 'cards'
		//have been played
		while(!test_passed)
		{
			draw = rand::thread_rng().gen_range(1..=self.deck_size);
		 	test_passed = self.check_card(draw);   
   		}
		self.dealt_cards.push(draw);
		draw.try_into().unwrap()
	}
	
	///iterates through dealt_cards vector, checking the generated number
	///against it's elements. Checks to make sure there are only n duplicates
	///of a number (number), where n = deck_count.
	
	//AGAIN, DOESNT CHECK RANGE
	//OF NUMBERS ONLY CHECKS HOW MANY TIMES A NUMBER IS REPEATED IN 
	//dealt_cards
 
	fn check_card(&self, drawn_card : u32) -> bool 
    {
		//counter variable
        let mut x : u32 = 0;
		
        for i in self.dealt_cards.iter()
        {
			println!("{}", i);

			//every element of dealt_cards that matches the 
			//generated number increments the counter
            if *i == drawn_card
            {   x = x + 1;   }
        }
		//if there are as many duplicates of the generated 
		//number in dealt_cardsas there are decks being used, 
		//then the function returns false and another number is generated
        if x == self.deck_count
        {   return false }
		//
		//
        else 
        {   return true  }
    }
	
	
	//ACCESSOR METHODS

	///clears dealt_cards vector, resetting deck 	
	pub fn shuffle_deck(&mut self)
	{
		self.dealt_cards.clear();
	}
	///accessor method for deck_count
	pub fn get_deck_count(&self) -> u32
	{
		self.deck_count
	}
	///accessor method for deck_size
	pub fn get_deck_size(&self) -> u32
	{
		self.deck_size
	}
	///accessor method for cards_in_a_suit
	pub fn get_cards_in_a_suit(&self) -> u32
	{
		self.cards_in_a_suit
	}
	///accessor method for suits_in_a_deck
	pub fn get_suits_in_a_deck(&self) -> u32
	{
		self.suits_in_a_deck
	}
}

