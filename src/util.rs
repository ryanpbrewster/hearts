use data::{Card, Hand};

/// Return the subset of cards in `hand` that are legal to play,
/// given that the cards in `trick` have been played already.
/// Requires: `trick.len() < 4`
pub fn legal_plays(hand: &Hand, trick: &[Card]) -> Vec<Card> {
    assert!(trick.len() < 4);

    let asdf = trick.first().and_then(|&lead_card| {
        let follow_suit: Vec<Card> = hand.cards
            .iter()
            .filter(|&card| card.suit == lead_card.suit)
            .cloned()
            .collect();
        if follow_suit.is_empty() {
            None
        } else {
            Some(follow_suit)
        }
    });
    unimplemented!()
}
