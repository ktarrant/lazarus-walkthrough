<details class="pokemon-card-container">
<summary>Nosepass (#102)</summary>
Types: Rock • Egg Groups: Mineral

<div class="pokemon-card">
<div class="card-column">
**Abilities**
- Sturdy
- Magnet Pull
- Sand Force *(Hidden)*

**Type Matchups**

*Resists / Immune to*
- Normal (0.5×)
- Fire (0.5×)
- Poison (0.5×)
- Flying (0.5×)

*Weak to*
- Water (2×)
- Grass (2×)
- Fighting (2×)
- Ground (2×)
- Steel (2×)

**TM/HM Moves**
- TM06 - Toxic
- TM11 - Sunny Day
- TM12 - Taunt
- TM17 - Protect
- TM24 - Thunderbolt
- TM25 - Thunder
- TM26 - Earthquake
- TM32 - Double Team
- TM34 - Shock Wave
- TM37 - Sandstorm
- TM39 - Rock Tomb
- TM41 - Torment
- TM42 - Facade
- TM44 - Rest
- TM45 - Attract
- TM49 - Bulldoze
- TM54 - Dazzling Gleam
- TM58 - Thunder Wave
- HM04 - Strength
- HM06 - Rock Smash

**Held Item**
Magnet

**Encounter Locations**
- Acrisia Mountains — Grass (Day) (10%)
- Sofos City — Grass (Day) (5%)
- Sofos City — Grass (Night) (5%)
</div>
<div class="card-column">
<label><input type="checkbox" class="caught-check" data-species="nosepass" /> Caught</label>

**Base Stats**

| Stat | Value |
| --- | --- |
| HP | <span class="stat-value stat-low">30</span> |
| Attack | <span class="stat-value stat-low">45</span> |
| Defense | <span class="stat-value stat-high">135</span> |
| Sp. Atk | <span class="stat-value stat-low">45</span> |
| Sp. Def | <span class="stat-value stat-mid">90</span> |
| Speed | <span class="stat-value stat-low">30</span> |
| Total | <span class="stat-value stat-mid">375</span> |

**Level-Up Moves**
- Tackle (Lv 1)
- Harden (Lv 4)
- Block (Lv 7)
- Rock Throw (Lv 10)
- Thunder Wave (Lv 13)
- Rest (Lv 16)
- Spark (Lv 19)
- Rock Slide (Lv 22)
- Power Gem (Lv 25)
- Rock Blast (Lv 28)
- Discharge (Lv 31)
- Sandstorm (Lv 34)
- Earth Power (Lv 37)
- Stone Edge (Lv 40)
- Lock-On (Lv 43)
- Zap Cannon (Lv 43)
- Shore Up (Lv 46)
- Electro Drift (Lv 52)

**Egg Moves**
- Magnitude
- Rollout
- Double-Edge
- Block
- Stealth Rock
- Endure
- Wide Guard

**Tutor Moves**
- Body Slam
- Defense Curl
- Double-Edge
- Dynamic Punch
- Endure
- Explosion
- Fire Punch
- Ice Punch
- Mud-Slap
- Rock Slide
- Rollout
- Sleep Talk
- Snore
- Swagger
- Thunder Punch
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
