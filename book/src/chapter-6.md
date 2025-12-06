# Pythios Town

## Pythios Town
- Pythios Town is home to the third gym.
The gym leader specializes in Ghost and Psychic type Pokemon.
Winning allows you to use HM05 Flash outside of battle.
- Don't do the gym until you have cleared the Pythios Cemetery, which is to the west.
- You can also catch Pokemon to the east in Erinys Path, but there is nothing else to do there.
If you want to look at what can be caught there, see Chapter 7.
- The man in the hat near the gym will give you Will-o-Wisp.
- The woman with the spider will sell you an outfit.
- The Name Rater lives in one of the houses.
- On the beach in the south, a Move Relearner is available if you have 20 Pokemon registered.

### Quests
- <label><input type="checkbox" class="quest-check" data-quest="pythios-town-impatience-is-a-virtue"> **Impatience is a Virtue** — Get 3 Quick Balls for Athlete _(Reward: Ultra Ball x3)_.</label>
- <label><input type="checkbox" class="quest-check" data-quest="pythios-town-save-our-sails"> **Save our Sails!** — Defeat Team Chimera Admin east of town _(Reward: Unlock Delphis transport ship)_.</label>

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
{{#include ./encounters/pythios-town.md}}
**University Student Reward (Pythios Town):** Traditional Outfit.

## Pythios Cemetery
- This area provides some extra training.
- There are also some Chimera grunts who will give you HM05 Flash when you beat them.
- There is an old lady looking at a gravestone in the north that wants to see a Snom.
She will give you an Ice Stone if you talk to her with one in your party.

### Quests
- <label><input type="checkbox" class="quest-check" data-quest="pythios-cemetery-snom-infestation"> **Snom Infestation** — Show Snom in party to the old lady in the north _(Reward: Ice Stone)_.</label>

{{#include ./encounters/pythios-cemetery.md}}
