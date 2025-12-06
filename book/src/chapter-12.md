# Myrrini Island

## Myrrini Island
- In one of the houses is TM05.
- The lady with the bee sells nectars.
- The artist will give you TM43.
- The daycare is on this island.
- North of the daycare is a Carbos.
- East of the daycare is TM17 Protect.

### Quests
- <label><input type="checkbox" class="quest-check" data-quest="myrrini-island-rift-in-the-heart"> **Rift in the Heart** — Get Heart Scale for Punk Rocker in NW house _(Reward: TM36 Sludge Bomb)_.</label>
- <label><input type="checkbox" class="quest-check" data-quest="myrrini-island-allergies"> **Allergies** — Bring a Full Heal to the camper south of the gym _(Reward: Silk Scarf)_.</label>
- <label><input type="checkbox" class="quest-check" data-quest="myrrini-island-second-battle-pavilion"> **Second Battle Pavilion** — Participate in the Myrrini Battle Pavilion _(Reward: Reaper Cloth)_.</label>
- <label><input type="checkbox" class="quest-check" data-quest="myrrini-island-gym-6"> **Gym 6** — Defeat Gym 6 on the Fresco Isles _(Reward: TM49 Bulldoze)_.</label>
- <label><input type="checkbox" class="quest-check" data-quest="myrrini-island-mischief-on-marmaro"> **Mischief on Marmaro** — Chase Team Chimera from the Marmaro Mine _(Reward: HM08 Dive, Marmaro Mining Minigame)_.</label>
- <label><input type="checkbox" class="quest-check" data-quest="myrrini-island-5-star-recovery"> **5-Star Recovery** — Visit Rania in Péntepetal City _(Reward: See Nature of wild Pokémon)_.</label>

### Encounters

{{#include ./encounters/myrrini-island.md}}

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
