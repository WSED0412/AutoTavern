# TavernGym 37.0.3 hotfix

TavernGym 1.1.1 applies the Battlegrounds **37.0.3** server hotfix (2026-11-17) on top of the
37.0.2 client data (**37.0.2.256589**, unchanged). The changes are recorded in
[tools/overlays/37.0.3.json](../tools/overlays/37.0.3.json), applied after the 37.0.2 overlay.

- **Snarky Shark**: Tier 4 → Tier 5.
- **Felboar**: "After you cast 4 spells..." (was 3).
- **Sanguine Champion**: Blood Gems give an extra +1/+1 (Golden +2/+2; was +2/+1 and +4/+2).
- **Trinket costs**: 24 Trinkets (13 Lesser, 11 Greater) as listed in the hotfix notes.
- **Emergency Gearblade** is no longer in the Lesser Trinket pool.
- **Kith'ix** is only offered in lobbies with Aberrations.
- **Dark Gift Discovers** after Turn 9 offer Tyrael, Falling Sky Golem, Cataclysmic Harbinger and
  De-volition-ist only when they match your most common minion type (`content::late_gift_ok`).
  The count uses the living warband in the lobby's types; a tie allows each tied type, and a
  minion with no type never matches. Both readings are simulator assumptions.
- **Orb of the Unknown** could already not give Wand of Divination in Quilboar lobbies: its
  offer uses the same lobby rule as Wand's own offer.

Golden values are not in the notes: Golden Felboar keeps the 4-spell count and Golden Sanguine
Champion doubles the normal bonus, as before the hotfix.

## Sources

- [37.0.3 hotfix (HearthPwn, quoting the official notes)](https://www.hearthpwn.com/news/12781-37-0-3-hotfix-patch-battlegrounds-balance-changes)
- [37.0.3 patch notes (hsbg.cards)](https://hsbg.cards/patch-notes/37.0.3)
