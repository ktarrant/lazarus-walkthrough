# Acrisia City & Bronze Fields

## Acrisia City

You begin in Acrisia City, awakened by the Professor’s urgent call for a new
Pokédex initiative across the Lazarus region.

Inside the annex you’ll choose one of nine Lazarus starters:

{{#include ./pokemon/popplio.md}}

{{#include ./pokemon/rowlet.md}}

{{#include ./pokemon/froakie.md}}

{{#include ./pokemon/chespin.md}}

{{#include ./pokemon/litten.md}}

{{#include ./pokemon/quaxly.md}}

{{#include ./pokemon/fennekin.md}}

{{#include ./pokemon/sprigatito.md}}

{{#include ./pokemon/fuecoco.md}}

Next, stock up on Pokeballs at the PokeMart.
I like to catch new Pokemon whenever they come up.
The road between Acrisia City and Jusmail Town is all the way on the left
and down.

### Pokemon Spotlight: Munna
Munna is a great Pokemon to use for catching other Pokemon.
The ability Synchronize will make it so wild Pokemon will have the same nature as your Munna.
So for example if you can find a Jolly Munna, then when the Munna is at the head of your
party, wild Pokemon that are encountered will also have the Jolly ability.
Also, Munna learns Yawn at level 7 and Hypnosis at level 19.
Sleep offers the highest catch rate increase of all status effects.

{{#include ./pokemon/munna.md}}

### Quests
- <label><input type="checkbox" class="quest-check" data-quest="acrisia-city-find-pokedex"> **Find Pokédex** — Return Parcel to the Professor _(Reward: Rotom Phone, Exp. Share, Outfit Box; Split: Polymnia Lvl 12)_.</label>
- <label><input type="checkbox" class="quest-check" data-quest="acrisia-city-a-girlypop-dream"> **A Girlypop Dream** — Register Togepi in the Pokédex _(Reward: Eviolite; Split: Polymnia Lvl 12)_.</label>
  - Encounter routes: Erinys Path (West), Sea of Asteri (East)
- <label><input type="checkbox" class="quest-check" data-quest="acrisia-city-rocknroll-pokemon"> **Rock'n'Roll Pokémon** — Show Blitzle in party to the lady in SW house _(Reward: Rare Candy; Split: Polymnia Lvl 12)_.</label>
  - Encounter routes: Wanderer's Woods (North)
- **Species Quest milestones** _(Split: Kleio Lvl 22)_:
  - <label><input type="checkbox" class="quest-check" data-quest="acrisia-city-species-quest">10: Nugget</label>
  - <label><input type="checkbox" class="quest-check" data-quest="acrisia-city-species-quest-2">25: Loaded Dice</label>
  - <label><input type="checkbox" class="quest-check" data-quest="acrisia-city-species-quest-3">50: TM27 Return</label>
  - <label><input type="checkbox" class="quest-check" data-quest="acrisia-city-species-quest-4">100: Rowlet, Litleo, Popplio</label>
  - <label><input type="checkbox" class="quest-check" data-quest="acrisia-city-species-quest-5">135: Chespin, Fennekin, Froakie</label>
  - <label><input type="checkbox" class="quest-check" data-quest="acrisia-city-species-quest-6">170: Sprigatito, Fuecoco, Quaxly</label>
  - <label><input type="checkbox" class="quest-check" data-quest="acrisia-city-species-quest-7">200: Oval Charm</label>
  - <label><input type="checkbox" class="quest-check" data-quest="acrisia-city-species-quest-8">250: Leftovers x3</label>
  - <label><input type="checkbox" class="quest-check" data-quest="acrisia-city-species-quest-9">300: Shiny Charm</label>
  - <label><input type="checkbox" class="quest-check" data-quest="acrisia-city-complete-the-pokedex">430: Trainer Card Gold Star</label>

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

### Encounters
{{#include ./encounters/acrisia-city.md}}
**University Student Reward (Acrisia City):** 15 Nest Balls.

## Bronze Fields
- Catch Pokemon and fight trainers on this path.
- On the way, you will find the delivery man being attacked and help him.
Once you win, you can bring the package back to the Prof.
- When you have returned the package, you can continue south to Jusmail Town.

### Pokemon Spotlight: Comfey
Comfey does not evolve, but can provide some surprisingly good early game value.
Comfey is fast and has decent defense, and learns multiple moves to heal itself.

{{#include ./pokemon/comfey.md}}

### Pokemon Spotlight: Ducklett
Ducklett can play the role of the classic early game flying Pokemon.
If you can get one with Drizzle ability, it may even be worth keeping.

{{#include ./pokemon/ducklett.md}}

### Encounters
{{#include ./encounters/bronze-fields-north.md}}

{{#include ./encounters/bronze-fields-south.md}}
**University Student Reward (Bronze Fields):** Bright Powder.
