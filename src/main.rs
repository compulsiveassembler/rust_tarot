//crate::cardengine;

mod cardengine;
mod tarotengine;

fn main() {
	let mut polo =  cardengine::Deck::build_deck(1,52,13,4);
	

	println!("{}", polo.get_deck_size());
	let mut marco = tarotengine::Hand::build_hand(7);

	println!("{}",marco.get_amount_of_cards());

}
