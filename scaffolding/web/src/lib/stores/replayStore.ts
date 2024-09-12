import { writable } from 'svelte/store';

export type Gem = "sapphire" | "emerald" | "ruby" | "onyx" | "diamond" | "gold";

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

// TODO: update this to use enums or string so we don't have to look at this reference again
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
