# Jusmail Town

## Jusmail Town

Jusmail Town is the home of the first gym.
The gym leader specializes in Grass and Bug type Pokemon.
Winning allows you to use HM01 Cut outside of battle.

### Quests
- <label><input type="checkbox" class="quest-check" data-quest="jusmail-town-lovestruck-biker"> **Lovestruck Biker** — Get Heart Scale for Cycling Man in north of town _(Reward: Love Ball x5; Split: Polymnia Lvl 12)_.</label>
- <label><input type="checkbox" class="quest-check" data-quest="jusmail-town-talk-to-prof-elia"> **Talk to Prof. Elia** — Go back to Acrisia City _(Reward: Ability Capsule; Split: Ourani Lvl 19)_.</label>
- **Berry milestones** _(Split: Ourani Lvl 19)_:
  - <label><input type="checkbox" class="quest-check" data-quest="jusmail-town-berry-20">Berry Novice (20 berries): Chilan/Haban/Roseli Berries</label>
  - <label><input type="checkbox" class="quest-check" data-quest="jusmail-town-berry-50">Berry Enthusiast (50 berries): Level 20 Woobat with special moves</label>
  - <label><input type="checkbox" class="quest-check" data-quest="jusmail-town-berry-100">Berry Expert (100 berries): Amaze Mulch x10, Shiny Charm</label>
  - <label><input type="checkbox" class="quest-check" data-quest="jusmail-town-berry-200">Berry Master (200 berries): Potted Berry, Trainer Card Gold Star</label>

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


### Pokemon Spotlight: Eevee
Eevee is a Pokemon with many possibilities.
But an extra twist in this game is that if you can find one with the hidden
ability Anticipation, then you can evolve it into a Vaporeon to get access
to the hidden ability Piscivore. Piscivore gives you advantages with fishing.

{{#include ./pokemon/eevee.md}}

### Encounters
{{#include ./encounters/jusmail-town.md}}
**University Student Reward (Jusmail Town):** Casual Coral Outfit.
