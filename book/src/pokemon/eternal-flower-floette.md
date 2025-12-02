<details class="pokemon-card-container">
<summary>Eternal Flower Floette (#313)</summary>
Types: Fairy • Egg Groups: -

<div class="pokemon-card">
<div class="card-column">
**Abilities**
- Flower Veil
- Symbiosis *(Hidden)*

**Type Matchups**

*Resists / Immune to*
- Fighting (0.5×)
- Bug (0.5×)
- Dragon (0×)
- Dark (0.5×)

*Weak to*
- Poison (2×)
- Steel (2×)

**TM/HM Moves**
- TM01 - Wish
- TM04 - Calm Mind
- TM06 - Toxic
- TM11 - Sunny Day
- TM15 - Draining Kiss
- TM16 - Light Screen
- TM17 - Protect
- TM18 - Rain Dance
- TM19 - Giga Drain
- TM22 - Solar Beam
- TM29 - Psychic
- TM32 - Double Team
- TM42 - Facade
- TM44 - Rest
- TM45 - Attract
- TM48 - Skill Swap
- TM54 - Dazzling Gleam
- HM05 - Flash
</div>
<div class="card-column">
<label><input type="checkbox" class="caught-check" data-species="eternal-flower-floette" /> Caught</label>

**Base Stats**

| Stat | Value |
| --- | --- |
| HP | <span class="stat-value stat-mid">74</span> |
| Attack | <span class="stat-value stat-mid">65</span> |
| Defense | <span class="stat-value stat-mid">67</span> |
| Sp. Atk | <span class="stat-value stat-high">125</span> |
| Sp. Def | <span class="stat-value stat-high">128</span> |
| Speed | <span class="stat-value stat-high">92</span> |
| Total | <span class="stat-value stat-high">551</span> |

**Level-Up Moves**
- Tackle (Lv 1)
- Vine Whip (Lv 1)
- Fairy Wind (Lv 6)
- Lucky Chant (Lv 10)
- Draining Kiss (Lv 12)
- Razor Leaf (Lv 15)
- Wish (Lv 20)
- Magical Leaf (Lv 23)
- Grassy Terrain (Lv 25)
- Synthesis (Lv 28)
- Petal Blizzard (Lv 33)
- Aromatherapy (Lv 35)
- Pollen Puff (Lv 38)
- Misty Terrain (Lv 43)
- Moonblast (Lv 46)
- Light of Ruin (Lv 50)
- Petal Dance (Lv 51)
- Solar Beam (Lv 58)

**Egg Moves**
- Incompatible

**Tutor Moves**
- Endure
- Metronome
- Sleep Talk
- Snore
- Swagger
- Swift
</div>
</div>
<script>
(function() {
  if (window.__lazarusCaughtInit) return; window.__lazarusCaughtInit = true;
  const STORAGE_KEY = 'lazarusCaught';
  function loadState() {
    try { return JSON.parse(localStorage.getItem(STORAGE_KEY) || '{}'); } catch (_) { return {}; }
  }
  function saveState(state) {
    try { localStorage.setItem(STORAGE_KEY, JSON.stringify(state)); } catch (_) {}
  }
  function applyState() {
    const state = loadState();
    const boxes = Array.from(document.querySelectorAll('.caught-check'));
    const bySpecies = boxes.reduce((m, cb) => {
      const s = cb.dataset.species; if (!s) return m; (m[s] ||= []).push(cb); return m; }, {});
    boxes.forEach(cb => {
      const key = cb.dataset.species;
      cb.checked = !!state[key];
      cb.onchange = () => {
        const checked = cb.checked;
        if (checked) state[key] = true; else delete state[key];
        saveState(state);
        (bySpecies[key] || []).forEach(other => { if (other !== cb) other.checked = checked; });
      };
    });
  }
  if (document.readyState === 'loading') {
    document.addEventListener('DOMContentLoaded', applyState);
  } else {
    applyState();
  }
})();
</script>
</details>
