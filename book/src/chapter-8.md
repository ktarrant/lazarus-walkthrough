# Sofos City

## Sofos City
- Sofos City is home to the fourth gym. She uses Fighting and Psychic types.
This gym uses a Gauntlet style, where you have to fight all the trainers and the
leader without returning to the PokeCenter.
This Gauntlet style can actually be very useful for training up new Pokemon,
since you can fight all the trainers in there and then leave and then fight
them again.
- The street vendor sells apples to evolve Applin.
- There is an Arcade/Game Corner. You can play pinball to win points
to buy rare items and EV items.
It seems like you can you only get at most 1 pinball point per game played.
Since you can finish the Diglett and Gengar pinballs early, I think they are the
fastest for grinding.
I think the Gengar one may be the easiest, but the Diglett one is the fastest
if you can consistently win.
- A fisherman to the northeast of the city will give you the Good Rod.
- To unlock the gym, you will have to first go north to Satyr's Lair
north of the city to fight Team Chimera. They will give you the
Key Stone to show to Niko. Niko will give you HM03 Surf.

### Quests
- <label><input type="checkbox" class="quest-check" data-quest="sofos-city-no-nodding-off"> **No Nodding Off!** — Bring an Awakening to the old lady west of the gym _(Reward: Rare Candy)_.</label>
- <label><input type="checkbox" class="quest-check" data-quest="sofos-city-arcade-escapade"> **Arcade Escapade** — Bring 5 Pinball Points to the computer guy in the west of the city _(Reward: Covert Cloak)_.</label>
- <label><input type="checkbox" class="quest-check" data-quest="sofos-city-master-of-terrain"> **Master of Terrain** — Show Alolan Raichu to the man by the grass _(Reward: Raichu Doll)_.</label>
  - Encounter routes: Port Pello, Wakewater Isle
- <label><input type="checkbox" class="quest-check" data-quest="sofos-city-bird-brained"> **Bird-Brained** — Register Noctowl and Xatu for the bird lady _(Reward: Wise Glasses)_.</label>
  - Encounter routes (Noctowl): Sofos City, Sea of Vulcai
  - Encounter routes (Xatu): Nyx Trails
- <label><input type="checkbox" class="quest-check" data-quest="sofos-city-nemos-birds"> **Nemo's Birds** — Show all 4 Oricorio forms to Nemo in the SE house _(Reward: Glimmering Charm, Oricorio Doll)_.</label>
  - Oricorio Baile: Jusmail Town, Myrrini Island
  - Oricorio Pom-Pom: Myrrini Island
  - Oricorio Pa'u: Myrrini Island
  - Oricorio Sensu: Myrrini Island, Wanderer's Woods (South)

<script>
(function() {
  if (window.__lazarusQuestInit) return; window.__lazarusQuestInit = true;
  const KEY = 'lazarusQuests';
  function load() { try { return JSON.parse(localStorage.getItem(KEY) || '{}'); } catch (_) { return {}; } }
  function save(state) { try { localStorage.setItem(KEY, JSON.stringify(state)); } catch (_) {} }
  function apply() {
    const state = load();
    document.querySelectorAll('.quest-check').forEach(cb => {
      const key = cb.dataset.quest;
      cb.checked = !!state[key];
      cb.addEventListener('change', () => {
        if (cb.checked) state[key] = true; else delete state[key];
        save(state);
      });
    });
  }
  if (document.readyState === 'loading') document.addEventListener('DOMContentLoaded', apply); else apply();
})();
</script>
- <label><input type="checkbox" class="quest-check" data-quest="sofos-city-first-battle-pavilion"> **First Battle Pavilion** — Participate in the Sofos Battle Pavilion _(Reward: Exp. Candy M x2)_.</label>
- <label><input type="checkbox" class="quest-check" data-quest="sofos-city-the-legend-of-kyogre"> **The Legend of Kyogre** — Find Kyogre and show your strength _(Reward: Unlock Kyogre encounter)_.</label>
- <label><input type="checkbox" class="quest-check" data-quest="sofos-city-the-legend-of-groudon"> **The Legend of Groudon** — Find Groudon and show your strength _(Reward: Unlock Groudon encounter)_.</label>
- <label><input type="checkbox" class="quest-check" data-quest="sofos-city-the-legend-of-rayquaza"> **The Legend of Rayquaza** — Find Rayquaza and show your strength _(Reward: Unlock Rayquaza encounter)_.</label>

### Battle Pavilion
In the Battle Pavilion you take three rental Pokemon and then fight
three battles with those Pokemon, with an option to swap one after
each fight.
These fights can be a little tricky. I lost a few times.
I think the best strategy is to spam status moves.
I found Dedenne with Nuzzle and
Ralts with the Hypnosis/Dream Eater combo to be particularly effective.
If you win the three fights you will be awarded a Wishing Star.

### Encounters
{{#include ./encounters/sofos-city.md}}
**University Student Reward (Sofos City):** Traditional Outfit.
