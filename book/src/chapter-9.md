# Kipos Town

## Kipos Town
When you head south out of Sofos City, the old sailor will take you to Kipos Town.
This zoo hub has several sidequests and unique rewards tied to donating Pokémon.
You can't proceed south through Lastlight Road yet.

- In the Northeast of the city you can find a TM03 Water Pulse.
- In the Southwest of the conservation center you can find a PP Max.
- On the east side of the North area of the conservation center you can find a Nugget.

### Quests
- <label><input type="checkbox" class="quest-check" data-quest="kipos-town-zookeeper"> **Zookeeper** — Fill all exhibits with Pokémon _(Reward: Ogerpon; Split: Euterpe Lvl 31)_.</label>
- <label><input type="checkbox" class="quest-check" data-quest="kipos-town-zoo-donations"> **Zoo Donations** — Donate 5 Pokémon _(Reward: Zookeeper Outfit; Split: Euterpe Lvl 31)_.</label>
- <label><input type="checkbox" class="quest-check" data-quest="kipos-town-zoo-donations-2"> **Zoo Donations** — Donate 10 Pokémon _(Reward: Nest Ball x5, Repeat Ball x5; Split: Euterpe Lvl 31)_.</label>
- <label><input type="checkbox" class="quest-check" data-quest="kipos-town-zoo-donations-3"> **Zoo Donations** — Donate 20 Pokémon _(Reward: Master Ball; Split: Euterpe Lvl 31)_.</label>
- <label><input type="checkbox" class="quest-check" data-quest="kipos-town-zoo-donations-4"> **Zoo Donations** — Donate 30 Pokémon _(Reward: Cornerstone/Wellspring/Hearthflame Masks; Split: Euterpe Lvl 31)_.</label>
- <label><input type="checkbox" class="quest-check" data-quest="kipos-town-domestic-pokemon"> **Domestic Pokémon** — Donate 3 Pokémon to the Suburbs Enclosure _(Reward: TM01 Wish; Split: Euterpe Lvl 31)_.</label>
- <label><input type="checkbox" class="quest-check" data-quest="kipos-town-a-paleo-nerds-dream"> **A Paleo Nerd's Dream** — Donate a Lazarus Taxa Pokémon _(Reward: Exp. Candy L; Split: Euterpe Lvl 31)_.</label>
- <label><input type="checkbox" class="quest-check" data-quest="kipos-town-cross-species-crossing"> **Cross-Species Crossing** — Register Braviary in the Pokédex for the zoo scientist _(Reward: Thick Club; Split: Euterpe Lvl 31)_.</label>
- <label><input type="checkbox" class="quest-check" data-quest="kipos-town-kalos-curiosity"> **Kalos Curiosity** — Show Tyrunt in party to the scientist in Kipos Town _(Reward: Protein; Split: Euterpe Lvl 31)_.</label>

### Encounters

{{#include ./encounters/kipos-town.md}}

**University Student Reward (Kipos Town):** Pokémon Box Link (Portable PC).

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
