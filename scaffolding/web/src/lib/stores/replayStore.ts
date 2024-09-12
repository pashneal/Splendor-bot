import { writable } from 'svelte/store';

export type Gem = "sapphire" | "emerald" | "ruby" | "onyx" | "diamond" | "gold";
export type Cost = "sapphire" | "emerald" | "ruby" | "onyx" | "diamond" ;


export type DeckBackendDesc = { 
  cardCount: number,
  tier: number,
}

//TODO: unify the types of the backend and frontend
export type CardBackendDesc = {
  tier: number,
  points: number,
  colorIndex: number,
  tokens: Array<[number, number]>,
}

export type CardDesc = {
  tier: number,
  points: number,
  gem: Cost,
  cost: Map<Cost, number>,
}

export type PlayerBackendDesc = {
  developments: Array<[number, number]>,
  gems: Array<[number, number]>,
  totalGems: number,
  reservedCards: Array<any>,
  totalPoints: number,
  noblePoints: number,
}

export type PlayerDesc = {
  name: string,
  developments: Map<Gem, number>,
  gems: Array<BankDesc>,
  totalGems: number,
  numReservedCards: number
  totalPoints: number,
  noblePoints: number,
}

export type NobleReq = {
  gemName: Gem,
  gemCount: number,
}

export type NobleDesc = {
  requirements: Array<NobleReqs>,
}

export type BankDesc = {
  gemName: Gem,
  gemCount: number,
}

export let turnNumber = writable(0);
export let nobles = writable(Array<NobleDesc>());
export let bank = writable(Array<BankDesc>());
export let number_players = writable(4);
export let players = writable(Array<PlayerDesc>());
export let cards = writable(Array<Array<CardDesc>>());
export let deckCounts = writable([0, 0, 0]);

// TODO: update this to use enums or string so we don't have to look at this reference again
// TODO: merge into one big API call rather than multiple to reduce latency 
// Match the conventions of the frontend gems
//
//          color    : index
//	 white (diamond) : 0
//	 blue (sapphire) : 1
//	 green (emerald) : 2
//	 red (ruby)      : 3
//	 black (onyx)    : 4
//	 yellow (gold)   : 5
export function indexToGem(index) : Gem | undefined {
    switch (index) {
        case 0:
            return 'diamond';
        case 1:
            return 'sapphire';
        case 2:
            return 'emerald';
        case 3:
            return 'ruby';
        case 4:
            return 'onyx';
        case 5:
            return 'gold';
    }
}



export function updateGameBanks() {
  fetch("/replay/bank")
    .then(response => response.json())
    .then(response => {
      let bankDescriptions : BankDesc[] = Array<BankDesc>();

      // Fill in the bank descriptions with the default values
      for (let i = bankDescriptions.length; i < 6; i++) {
        bankDescriptions.push({gemName: indexToGem(i), gemCount: 0});
      }

      // Then update the bank descriptions from backend
      response.success.bank.forEach(([gemIndex, gemCount] : [number, number]) => {
          let gemName = indexToGem(gemIndex)!;
          bankDescriptions[gemIndex] = {gemName: gemName, gemCount: gemCount};
      });

      bank.update(() => bankDescriptions);
    }).catch(error => {
      console.error('/replay/bank Error:', error);
    });
}

export function updateGameNobles() {
  fetch("/replay/nobles")
    .then(response => response.json())
    .then(response => {

      let new_nobles : Array<NobleDesc> = Array();
      response.success.nobles.forEach( (nobleReqs : Array<[number, number]>) => {
        let noble : NobleDesc = {
          requirements : Array(),
        }

        nobleReqs.forEach( ([gemIndex, gemCount] : [number, number]) => {
          let nobleReq : NobleReq = {
            gemName : indexToGem(gemIndex)!,
            gemCount : gemCount,
          }

          noble.requirements.push(nobleReq);
        });

        new_nobles.push(noble);

      });

      nobles.update(() => new_nobles);

    }).catch(error => {
      console.error('/replay/nobles/ Error:', error);
    })
}

export function updateGamePlayers() {
  fetch("/replay/players")
    .then(response => response.json())
    .then(response => {
      let newPlayers = Array<PlayerDesc>();
      response.success.players.forEach((player : PlayerBackendDesc, id :number) => {

        let developments = new Map<Gem, number>();
        let personalBank = Array<BankDesc>();
        let totalPoints = player.totalPoints;
        let noblePoints = player.noblePoints;
        let totalGems = player.totalGems
        let numReservedCards = player.reservedCards.length;
         
        player.developments.forEach(([gemIndex, count], _) => {
          let gemName = indexToGem(gemIndex)!;
          developments.set(gemName, count);
        });

        player.gems.forEach(([gemIndex, count], _) => {
          let gemName = indexToGem(gemIndex)!;
          let bankDesc = {gemName : gemName,  gemCount : count};
          personalBank.push(bankDesc);
        });

        let playerDesc = {name : "Player " + id, 
                          developments : developments, 
                          gems : personalBank, 
                          totalPoints : totalPoints, 
                          noblePoints : noblePoints, 
                          totalGems : totalGems, 
                          numReservedCards : numReservedCards};

        newPlayers.push(playerDesc);
      });

      players.update(() => newPlayers);
    });
}

export function updateGameCards() {
    fetch("/replay/cards")
      .then(response => response.json())
      .then(response => {
        let new_cards : Array<Array<CardDesc>> = [];

        response.success.cards.forEach((row : Array<CardBackendDesc>) => {
          let cardRow : Array<CardDesc> = [];
          row.forEach((card, _) => {

            let cost = new Map<Gem, number>();
            card.tokens.forEach(([gemIndex, count])  => {
              cost.set(indexToGem(gemIndex)! as Cost, count);
            });

            let cardDesc : CardDesc = {
              tier: card.tier,
              points: card.points,
              gem: indexToGem(card.colorIndex)!,
              cost: cost
            }
            cardRow.push(cardDesc);
          })
          new_cards.push(cardRow);

        });
       cards.update(() => new_cards); 
      });
  }

export function updateGameDeckCounts() {
  fetch("/replay/decks")
    .then(response => response.json())
    .then(response => {
      let new_decks = [0, 0, 0];
      
      response.success.decks.forEach((deck : DeckBackendDesc) => {
          // Map tiers 0 to 2 to indices 2 to 0 for the frontend view
          // TODO: this isn't that nice
          let index = deck.tier;
          index = 2 - index;
          new_decks[index] = deck.cardCount;
      });

      deckCounts.update(() => new_decks);
    });
}

