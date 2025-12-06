# Sea of Asteri

## Sea of Asteri

There are several islands and trainers on your way.
The first island has three items and a quest to pick up.
South of that island is a smaller island with an item and two trainers.

### Quests
- <label><input type="checkbox" class="quest-check" data-quest="sea-of-asteri-west-lost-at-sea"> **Lost at Sea** — Find Pearl String for the old man on the NE island _(Reward: Exp. Candy L; Split: Euterpe Lvl 31)_.</label>
- <label><input type="checkbox" class="quest-check" data-quest="sea-of-asteri-east-another-chance"> **Another Chance** — Get a Revive for the diver (requires Dive) _(Reward: TM18 Rain Dance; Split: Thalia & Melpomene Lvl 36)_.</label>

### Encounters

{{#include ./encounters/sea-of-asteri-west.md}}

{{#include ./encounters/sea-of-asteri-east.md}}

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
