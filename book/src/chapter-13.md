# Pentepetal City

## Pentepetal City
Use the old sailor to get to Pentepetal City.
Once there you will have to fight Rania and then the Chimera Admin.

### Quests
- <label><input type="checkbox" class="quest-check" data-quest="pentepetal-city-the-big-ask"> **The Big Ask** — Bring a Hisuian Arcanine Doll to the lady in the house north of the berry shop _(Reward: Dragon Poster)_.</label>
- <label><input type="checkbox" class="quest-check" data-quest="pentepetal-city-a-winning-painting"> **A Winning Painting** — Show the artist outside the Contest Hall a Pokémon with a Contest Ribbon _(Reward: —)_.</label>
- <label><input type="checkbox" class="quest-check" data-quest="pentepetal-city-the-long-lost-friend"> **The Long-Lost Friend** — Bring 12 Wishing Stones to the woman in the northwest house _(Reward: Eternal Flower Floette)_.</label>

### Encounters

{{#include ./encounters/pentepetal-city.md}}

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
