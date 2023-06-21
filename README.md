# Auto Rumble

This is an autochess style version of [Rumble](https://kevan.org/rumble/hero/rules)

Instead of a two-stage game where players first bid on powers then fight it out in autorumble the fighting is automated.
Players bid on powers and their heroes battle it out automatically. After each round of bidding the battle is automatically
executed, but only the final one counts.

## Bidding

Each round there will be `2 * number of players` powers to bid on, drawn from a pool of powers. Players receive 30 coins each round, which accumulate if not spent. Bids are made with these coins for powers, ties result in a copy of the power going to each bidder (except ties for 0 coins, in which case the power goes to no one and stays in the pool). Each player privately submits any number of bids on the available powers. Only the winning bid costs coins, losing bids retain the bid coins. Players may make more bids more than their coin balance (but no individual bid can be more than your balance), however negative balance will accumulate into future rounds too.

## Power submission

Each round players also submit 2 new powers publicly. Inspiration can be drawn from the [Rumble archive](https://kevan.org/rumble/hero/viewall). It is the responsibility of all players to agree on how newly submitted powers will interact with existing powers in the game. While the host will do their best to pre-empt clarifications, if an interaction is not clarified before a power is submitted they will have to make a judgement call on that interaction.

Since fighting is automated, everything must be 100% deterministic. The scope of powers is also more limited than classic Rumble, powers may only affect the battle, they can't influence the bidding process, so 'Flaw' style `Flaw. (All players pay their bids for this, except for the lowest bidder(s), who must take it and cannot discard it.) ` powers are not allowed, as would be a power that increases your coins.

Many powers listed in the Rumble archive are actives that a player would choose when to active on a target during fighting. These powers are not deterministic, instead powers must have rules on whom they target, or target everyone every round (which will be the default interpretation if not specified otherwise). Costs are only paid once, a `Burn 10` power costs 10 Energy every round but could hit every hero for that cost.

The power strength of a power is the number of coins bid on it.

## Fighting

After resolving all bids, heroes gain powers if their players won bids, then the heroes fight it out automatically.

Each hero starts with 100† Energy, which doubles as their HP. Some powers may spend Energy which allocates it for that round but does not use it up, and some powers may burn Energy or inflict damage which uses it up permanently. If a hero runs out of Energy to allocate before they can use all their powers/attack then the remaining powers/attack will not be used in that round. Taking damage during a round **does not** reduce the Energy available to spend on powers and attacks until the following round.

Since each hero is effectively a mindless zombie, they will spend 10 energy to attack every other hero for 10† damage each round and not defend at all by default (though powers may add defences, and smarter attacks). Defences only block one source of damage, 10 defence blocks 10 damage, not 10 damage per player.

For ease of automation, and also deterministic behaviour, heroes will have an initiative order. This order determines which hero acts first while resolving actions each round of fighting. By default every hero will be assigned a base initiative between 0 and 1 to create tie breakers, the highest initiative goes first, then the second highest, through to the lowest. When resolving attacks or powers that target multiple heroes this is also resolved in initiative order.

† - Unspent coins are not wasted, they will increase your hero's starting Energy and initiative by the unspent balance each, as well as your hero's base attack damage by 1/3 of the coin balance, rounding **up** (away from zero). This means bidding on lots of powers will make your hero lower in the initiative, so holding some coins back may be beneficial. These Energy, initiative and attack bonuses also apply to negative coin balance.

### Round resolution

1 - In initiative order, heroes use powers that activate at the start of the round

2 - In initiative order, heroes make their attack and use powers which activate in each round

3 - In initiative order, heroes use powers which activate at the end of the round

If a hero's energy drops equal to or below 0 at any point during resolution they immediately die and cease to take any further actions, but may still be affected by actions from other heroes. They will not be present in the subsequent rounds. This might be changed by powers which change the energy number at which the player dies or revive them.

No fractions are permitted for any stats at any time (except base initiative), including Energy, damage and defence, if taking division that would create a fraction, it will always be rounded up (away from zero).

To try to avoid stalemates if no hero loses Energy for three consecutive rounds, all heroes will lose half their Energy at the end of that round (`1 - 0.5` rounds to `1 - 1` which is `0`). If we're still stuck in a stalemate, whichever hero has the most Energy after round 30 wins, if Energy is tied, ties are broken by who has the highest initiative.

## Power use order

Since powers may spend energy and the base attack also spends 10 energy each round, players may privately submit their desired powers/attack order. This order is fixed for every round of fighting. The powers/attack order is public information and revealed when the heroes fight it out automatically after each bidding round. Any newly gained powers (during automatic fighting or from the powers just bid on) will automatically go to the bottom of this list order (highest bid to lowest bid).

## Overall game order

0 - `3 * number of players` powers will be seeded into the game's pool of powers. These will not be public.

1 - Bidding round x of 10: `2 * number of players` powers will be randomly chosen from the pool of powers for bidding and revealed to all players to bid on. Players submit 2 powers publicly to the pool, bid on the available powers privately, and may rearrange the order in which their hero will use their currently acquired powers/attack.

2 - Bidding round x of 10 resolution: power use order is revealed (newly won powers enter the bottom of the list order), won bids deduct coins from player's balances, heroes fight automatically, provisional winner is declared.

3 - Bidding round x+1 of 10: repeat

4 - Bidding round 10 of 10: `2 * number of players` powers will be randomly chosen from the pool of powers for bidding and revealed to all players to bid on. Players bid on the available powers privately, and may rearrange the order in which their hero will use their currently acquired powers/attack. The 10+ remaining powers in the pool are no longer obtainable through bidding.

5 - Bidding round 10 of 10 resolution: power use order is revealed (newly won powers enter the bottom of the list order (highest bid to lowest bid)), won bids deduct coins from player's balances, heroes fight automatically, winner is declared.

## Sample round 1
Alice's hero got a base initiative of 0.25, Bob's hero got a base initiative of 0.75, and Charlie's hero got a base initiative of 0.5.

Players Alice, Bob and Charlie are bidding on:
> Amoeba: At the end of each round you ~may opt to~ multiply if your Energy is higher than 5 - divide your Energy by 2 and add 5. Create another of yourself with that same Energy *and powers*.

> Big, Gnashy Claws: If you use no other Powers in a given round, add 20 to each of your Attacks.

> Cosmic Shield: You get +10 to Defense for every Hero ~in play~ *alive at the start of the round*.

> Souleater: Gain 30 Energy *at the end of the round* whenever an opponent is eliminated *in the round*.

> Crystallize: Spend 20: Add +X to your Defense. X is initially 1, and doubles every time you use this Power.

> Titanium Skin: Prevent the first 50 damage you take during the game.

Italics and strikethroughs indicate where powers taken from the Rumble Archive were made less ambiguous.

Alice bids 6 coins on Souleater and Titanium Skin and 4 coins on each of the 4 powers. Bob bids 5 coins on Amoeba and Cosmic Shield, 10 on Crystallize, and 3 coins on the others. Charlie bids 7 coins on Big, Gnashy Claws, 10 coins on Crystallize, and 2 coins on the others.

Bob and Charlie win Crystallize, Bob wins Amoeba and Cosmic Shield, Charlie wins Big, Gnashy Claws, Alice wins Souleater and Titanium Skin.

Alice now has 18 coins, Bob now has 10 coins and Charlie now has 13 coins.

Alice enters round 1's battle with 118 Energy, a 10 energy attack that hits for 16 damage and 18.25 initiative. Bob enters with 110 Energy, a 10 energy attack that hits for 14 damage, and 10.75 initiative. Charlie enters with 113 Energy, a 15 damage attack, and 13.5 initiative.

Alice's power use order is Attack(10)
Bob's is Attack(10), Crystallize(20)
Charlie's is Attack(10), Crystallize(20)

At the start of the round Alice does nothing.
At the start of the round Charlie does nothing.
At the start of the round Bob does nothing.

During the round Alice spends 10 Energy (out of 118) to attack Charlie for 16 damage, then Bob for 16 damage. Charlie has no defence so loses 16 Energy (97). Bob has 30 defence from Cosmic Shield so loses 0 Energy and has 14 defence left over.

During the round Charlie spends 10 Energy (out of 113) to attack Alice for 15 damage, then Bob for 15 damage. Alice has Titanium skin so the 15 damage is prevented. Bob still has 14 defence so only loses 1 Energy (109).
During the round Charlie spends 20 Energy on Crystallize, gaining 1 defence.

During the round Bob spends 10 Energy (out of 110) to attack Alice for 14 damage, then Charlie for 14 damage. Alice has Titanium skin so the 14 damage is prevented. Charlie has 1 defence so only loses 13 Energy (84).
During the round Bob spends 20 Energy on Crystallize, gaining 1 defence.

At the end of the round Alice does nothing.
At the end of the round Charlie does nothing. Big, Gnashy Claws did not apply this round because Charlie had enough Energy to use Crystallize.
At the end of the round Bob uses Amoeba: Bob 1 (60), Bob 2 (60).

Round 2
Alice: 118 Energy, Titanium skin 29/50.
Charlie: 84 Energy.
Bob 1: 60 Energy.
Bob 2: 60 Energy.  

At the start of the round Alice does nothing.
At the start of the round Charlie does nothing.
At the start of the round Bob does nothing.

During the round Alice spends 10 Energy (out of 118) to attack Charlie for 16 damage, Bob 1 for 16 damage, Bob 2 for 16 damage. Charlie has no defence so loses 16 Energy (68). Bob 1 and 2 have 40 defence each from Cosmic Shield so lose 0 Energy and have 24 defence left over.

During the round Charlie spends 10 Energy (out of 84) to attack Alice, Bob 1 and Bob 2 for 15 damage. Alice has Titanium skin so the 15 damage is prevented. Bob 1 and 2 still have 24 defence so lose 0 Energy.
During the round Charlie spends 20 Energy on Crystallize, gaining 2 defence.

During the round Bob 1 spends 10 Energy (out of 60) to attack Alice for 14 damage, then Charlie for 14 damage. Alice has Titanium skin so 6 damage is prevented (110). Charlie has 2 defence so only loses 12 Energy (56).
During the round Bob 1 spends 20 Energy on Crystallize, gaining 2 defence.

During the round Bob 2 spends 10 Energy (out of 60) to attack Alice for 14 damage, then Charlie for 14 damage. Alice (96), Charlie (42).
During the round Bob 2 spends 20 Energy on Crystallize, gaining 2 defence.

At the end of the round Alice does nothing.
At the end of the round Charlie does nothing. Big, Gnashy Claws did not apply this round because Charlie had enough Energy to use Crystallize.
At the end of the round Bob 1 uses Amoeba: Bob 1A (35), Bob 1B (35).
At the end of the round Bob 2 uses Amoeba: Bob 2A (35), Bob 2B (35).

Round 3
Alice: 96 Energy
Charlie: 42 Energy.
Bobs: 35 Energy each

At the start of the round Alice does nothing.
At the start of the round Charlie does nothing.
At the start of the round Bobs do nothing.

During the round Alice spends 10 Energy (out of 96) to attack everyone else for 16 damage each. Charlie has no defence so loses 16 Energy (26). Each Bob  has 70 defence so lose 0 Energy and have 54 defence left over.

During the round Charlie spends 10 Energy (out of 42) to attack everyone else for 15 damage each. Alice (81), bobs still have 54 defence so lose 0 Energy.
During the round Charlie spends 20 Energy on Crystallize, gaining 4 defence.

During the round each Bob spends 10 Energy (out of 35) to attack Alice and Charlie for 14 damage each, then 20 Energy on Crystallize, gaining 2 defence. Bob 1A: Alice (67) Charlie (12). Bob 1B: Alice (53), Charlie: (0). Charlie is dead! Bob 2A: Alice (39), Bob 2B: Alice (25).

At the end of the round Alice uses Souleater because Charlie is dead and gains 30 Energy.
At the end of the round Charlie does nothing (dead).
At the end of the round Bobs use Amoeba. Bob 1A1: (23), Bob 1A2: (23), Bob 1B1: (23), Bob 1B2: (23), Bob 2A1: (23), Bob 2A2: (23), Bob 2B1: (23), Bob 2B2: (23).

Round 4
Alice: 55 Energy
Bobs: 23 Energy each

At the start of the round Alice does nothing.
At the start of the round Bobs do nothing.

During the round Alice spends 10 Energy (out of 55) to attack all the Bobs for 16 damage each. Each Bob has more defence than Alice has Energy so this is futile.

Bob 1A1 (still claiming to be the original) starts the dogpile on Alice (41), then is unable to use Crystallize since Bob only has 23 Energy to allocate, and attacked first instead of using Crystallize. Bob 1A2 does the same, Alice (27), Bob 1B1 takes their turn, Alice (13), Bob 1B2 takes the kill on Alice (0). Bob 2A1, Bob 2A2, Bob 2B1 and Bob 2B2 continue anyway, beating a dead body into pulp.

At the end of the round Alice can't use Souleater because Alice is dead.
At the end of the round the Bobs multiply again, creating 16 Bobs each with 17 Energy.

The Bobs win, though they would multiply again at the end of Round 5 to only 9 Energy each and be left unable to attack in Round 6.

The second round of Bidding starts for Alice, Bob and Charlie. Alice and Charlie are probably thinking about how to introduce powers that counter that Amoeba+Cosmic Shield combo.

Alice enters round 2 of bidding with 48 coins, Bob with 40 coins, and Charlie with 43 coins. During this round's bidding, Charlie opts to reorder their power use order to Crystallize(20), Attack(10), which will become public information when round 2 bidding is finished and we see the next simulation.
