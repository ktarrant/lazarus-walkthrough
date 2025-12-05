# Erinys Path, Froslass Cavern, & Asfal Hills

## The Old Sailor
Before leaving town, talk to the old sailor at the south edge of town once you
beat the Gym. He will ask you to defeat the Chimera Grunt to the east in the
Erinys Path.
Once you beat the grunt, you will be able to sail with Piraeus.
Until you beat more gyms, your only options are Kalami City and Pythios Town.

## Erinys Path
Once you save the sailor you can continue east through Erinys Path.
At the end is Froslass Cavern, with even more trainers, so bring healing
if you can afford it as it is a long way to go back and forth to the
PokeCenter.

### Quests
- <label><input type="checkbox" class="quest-check" data-quest="erinys-path-chimeras-mega-plan"> **Chimera's Mega Plan** — Defeat Team Chimera in Sofos City, return to Niko _(Reward: HM03 Surf; Split: Terpsikore Lvl 27)_.</label>

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
{{#include ./encounters/erinys-path-west.md}}

{{#include ./encounters/erinys-path-east.md}}

**University Student Reward (Erinys Path):** 2 Focus Sash.

## Froslass Cavern
At the end of Froslass Cavern is a king of tough double battle.
Something resistant to electricity will help.
You can't complete the Froslass Quest until you get Surf.

### Quests
- <label><input type="checkbox" class="quest-check" data-quest="froslass-cavern-b2f-keeper-of-the-cave"> **Keeper of the Cave** — Catch the Froslass in Froslass Cavern _(Reward: Exp. Candy M x3; Split: Terpsikore Lvl 27)_.</label>

{{#include ./encounters/froslass-cavern-f1.md}}

{{#include ./encounters/froslass-cavern-bf1.md}}

{{#include ./encounters/froslass-cavern-bf2.md}}

## Asfal Hills
Mercifully a nurse will heal your Pokemon once you leave the cave.

Niko will fight you on this route, then ask him to get the Key Stone
from Team Chimera in Sofos City.

### Quests
- <label><input type="checkbox" class="quest-check" data-quest="asfal-hills-a-tall-order"> **A Tall Order** — Register Girafarig in the Pokédex for the boy in the house _(Reward: TM34 Shock Wave; Split: Terpsikore Lvl 27)_.</label>
  - Encounter routes: Corrin Crossing, Palati City, Sofos City

### Encounters
{{#include ./encounters/asfal-hills.md}}
