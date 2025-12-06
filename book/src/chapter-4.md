# Kalami City

## Kalami City
- Kalami City is home to the second gym, which is Rock, Steel, and Electric types.
Winning allows you to use HM06 Rock Smash outside of battle.
- You can find a fisherman on the east side of the map who will give you an Old Rod.
- A man in one of the houses will give you a quest to complete the Eevee Pokedex entry.
- A fisherman on the east side of town will give you an Old Rod.

### Specialty Pokeballs
The PokeMart in this town sells speciality Pokeballs.
- Level Ball can get up to a very high multiplier, but is less useful because
lower level Pokemon are already easier to catch.
- Quick Ball offers a 4x catchrate on the first turn. Only use on first turn.
- Timer Ball offers a 4x catchrate after 10 turns.
- Lure Ball offers a 4x catchrate when fishing.
- Repeat Ball offers a 3x catchrate on Pokemon caught before,
which is good for nature/IV hunting.

These are my favorites.
Compare to the 2x catchrate of an Ultra Ball.

### Quests
- <label><input type="checkbox" class="quest-check" data-quest="kalami-city-understanding-eeveelution"> **Understanding Eeveelution** — Register Eevee in the Pokédex (house SW of Mart) _(Reward: Exp. Candy S x3)_.</label>
  - Encounter routes: Jusmail Town, Kipos Town
- <label><input type="checkbox" class="quest-check" data-quest="kalami-city-eye-of-the-beholder"> **Eye of the Beholder** — Get Heart Scale for Artist in SE of City _(Reward: Opal Fish Poster)_.</label>

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
{{#include ./encounters/kalami-city.md}}
**University Student Reward (Kalami City):** TM Whirlpool & East Blue Ilios Outfit (if not already unlocked).
