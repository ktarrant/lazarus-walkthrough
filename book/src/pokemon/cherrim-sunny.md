<details class="pokemon-card-container">
<summary>Cherrim Sunny (#168)</summary>
Types: Grass / Fire • Egg Groups: Grass / Fairy

<div class="pokemon-card">
<div class="card-column">
**Abilities**
- Flower Gift

**Type Matchups**

*Resists / Immune to*
- Electric (0.5×)
- Grass (0.25×)
- Steel (0.5×)
- Fairy (0.5×)

*Weak to*
- Poison (2×)
- Flying (2×)
- Rock (2×)

**TM/HM Moves**
- TM06 - Toxic
- TM09 - Bullet Seed
- TM11 - Sunny Day
- TM15 - Draining Kiss
- TM17 - Protect
- TM19 - Giga Drain
- TM22 - Solar Beam
- TM32 - Double Team
- TM42 - Facade
- TM44 - Rest
- TM45 - Attract
- TM54 - Dazzling Gleam
- HM05 - Flash

**Held Item**
Miracle Seed

**Evolution Info**
Lv. 25
</div>
<div class="card-column">
<label><input type="checkbox" class="caught-check" data-species="cherrim-sunny" /> Caught</label>

**Base Stats**

| Stat | Value |
| --- | --- |
| HP | <span class="stat-value stat-mid">70</span> |
| Attack | <span class="stat-value stat-mid">60</span> |
| Defense | <span class="stat-value stat-mid">70</span> |
| Sp. Atk | <span class="stat-value stat-high">112</span> |
| Sp. Def | <span class="stat-value stat-mid">78</span> |
| Speed | <span class="stat-value stat-high">100</span> |
| Total | <span class="stat-value stat-mid">490</span> |

**Level-Up Moves**
- Petal Dance (Lv Evo)
- Sunny Day (Lv Evo)
- Morning Sun (Lv 1)
- Tackle (Lv 1)
- Growth (Lv 7)
- Leech Seed (Lv 10)
- Helping Hand (Lv 13)
- Magical Leaf (Lv 19)
- Sunny Day (Lv 22)
- Flame Burst (Lv 27)
- Worry Seed (Lv 30)
- Flame Burst (Lv 35)
- Solar Beam (Lv 38)
- Fiery Dance (Lv 43)
- Lucky Chant (Lv 48)
- Petal Blizzard (Lv 50)

**Egg Moves**
- Razor Leaf
- Sweet Scent
- Tickle
- Nature Power
- Grass Whistle
- Aromatherapy
- Weather Ball
- Heal Pulse
- Healing Wish
- Seed Bomb
- Natural Gift
- Defense Curl
- Rollout
- Flower Shield
- Grassy Terrain

**Tutor Moves**
- Defense Curl
- Double-Edge
- Endure
- Rollout
- Sleep Talk
- Snore
- Swagger
- Swords Dance
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
