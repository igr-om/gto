pub fn remove_blocked_combos(
    deck: &mut Deck,
    hero_hole: &[Card; 4],
    board: &Board,
) {
    deck.remove_many(hero_hole);
    deck.remove_many(&board.cards);
}
