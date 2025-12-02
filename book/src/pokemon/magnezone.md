<details class="pokemon-card-container">
<summary>Magnezone (#373)</summary>
<div class="pokemon-tabs" id="pokemon-tabs-magnezone">
<input type="radio" name="pokemon-tabs-magnezone-group" id="pokemon-tabs-magnezone-tab-0">
<label for="pokemon-tabs-magnezone-tab-0">Magnemite</label>
<input type="radio" name="pokemon-tabs-magnezone-group" id="pokemon-tabs-magnezone-tab-1">
<label for="pokemon-tabs-magnezone-tab-1">Magneton</label>
<input type="radio" name="pokemon-tabs-magnezone-group" id="pokemon-tabs-magnezone-tab-2" checked>
<label for="pokemon-tabs-magnezone-tab-2">Magnezone</label>
<div class="pokemon-tab-panels">
<div class="pokemon-tab-panel" id="pokemon-tabs-magnezone-panel-0">
Types: Electric / Steel • Egg Groups: Mineral

<div class="pokemon-card">
<div class="card-column">
**Abilities**
- Magnet Pull
- Sturdy
- Analytic *(Hidden)*

**Type Matchups**

*Resists / Immune to*
- Normal (0.5×)
- Electric (0.5×)
- Grass (0.5×)
- Ice (0.5×)
- Poison (0×)
- Flying (0.25×)
- Psychic (0.5×)
- Bug (0.5×)
- Rock (0.5×)
- Dragon (0.5×)
- Steel (0.25×)
- Fairy (0.5×)

*Weak to*
- Fire (2×)
- Fighting (2×)
- Ground (4×)

**TM/HM Moves**
- TM06 - Toxic
- TM11 - Sunny Day
- TM16 - Light Screen
- TM17 - Protect
- TM18 - Rain Dance
- TM24 - Thunderbolt
- TM25 - Thunder
- TM32 - Double Team
- TM33 - Reflect
- TM34 - Shock Wave
- TM37 - Sandstorm
- TM42 - Facade
- TM44 - Rest
- TM58 - Thunder Wave
- HM05 - Flash

**Held Item**
Metal Coat

**Encounter Locations**
- Corrin Crossing — Grass (Day) (20%)
- Corrin Crossing — Grass (Night) (20%)
- Kipos Town — Grass (Day) (20%)
- Kipos Town — Grass (Night) (20%)
</div>
<div class="card-column">
<label><input type="checkbox" class="caught-check" data-species="magnemite" /> Caught</label>

**Base Stats**

| Stat | Value |
| --- | --- |
| HP | <span class="stat-value stat-low">25</span> |
| Attack | <span class="stat-value stat-low">35</span> |
| Defense | <span class="stat-value stat-mid">70</span> |
| Sp. Atk | <span class="stat-value stat-high">95</span> |
| Sp. Def | <span class="stat-value stat-mid">55</span> |
| Speed | <span class="stat-value stat-low">45</span> |
| Total | <span class="stat-value stat-mid">325</span> |

**Level-Up Moves**
- Supersonic (Lv 1)
- Tackle (Lv 1)
- Thunder Shock (Lv 5)
- Thunder Wave (Lv 7)
- Magnet Bomb (Lv 11)
- Light Screen (Lv 13)
- Sonic Boom (Lv 17)
- Spark (Lv 19)
- Mirror Shot (Lv 23)
- Metal Sound (Lv 25)
- Electro Ball (Lv 29)
- Flash Cannon (Lv 31)
- Screech (Lv 35)
- Discharge (Lv 37)
- Lock-On (Lv 41)
- Magnet Rise (Lv 43)
- Gyro Ball (Lv 47)
- Zap Cannon (Lv 49)

**Egg Moves**
- Incompatible

**Tutor Moves**
- Double-Edge
- Endure
- Explosion
- Psych Up
- Rollout
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
</div>
<div class="pokemon-tab-panel" id="pokemon-tabs-magnezone-panel-1">
Types: Electric / Steel • Egg Groups: Mineral

<div class="pokemon-card">
<div class="card-column">
**Abilities**
- Magnet Pull
- Sturdy
- Analytic *(Hidden)*

**Type Matchups**

*Resists / Immune to*
- Normal (0.5×)
- Electric (0.5×)
- Grass (0.5×)
- Ice (0.5×)
- Poison (0×)
- Flying (0.25×)
- Psychic (0.5×)
- Bug (0.5×)
- Rock (0.5×)
- Dragon (0.5×)
- Steel (0.25×)
- Fairy (0.5×)

*Weak to*
- Fire (2×)
- Fighting (2×)
- Ground (4×)

**TM/HM Moves**
- TM06 - Toxic
- TM11 - Sunny Day
- TM16 - Light Screen
- TM17 - Protect
- TM18 - Rain Dance
- TM24 - Thunderbolt
- TM25 - Thunder
- TM32 - Double Team
- TM33 - Reflect
- TM34 - Shock Wave
- TM37 - Sandstorm
- TM42 - Facade
- TM44 - Rest
- TM58 - Thunder Wave
- HM05 - Flash

**Held Item**
Metal Coat

**Evolution Info**
Lv. 30
</div>
<div class="card-column">
<label><input type="checkbox" class="caught-check" data-species="magneton" /> Caught</label>

**Base Stats**

| Stat | Value |
| --- | --- |
| HP | <span class="stat-value stat-low">50</span> |
| Attack | <span class="stat-value stat-mid">60</span> |
| Defense | <span class="stat-value stat-high">95</span> |
| Sp. Atk | <span class="stat-value stat-high">120</span> |
| Sp. Def | <span class="stat-value stat-mid">70</span> |
| Speed | <span class="stat-value stat-mid">70</span> |
| Total | <span class="stat-value stat-mid">465</span> |

**Level-Up Moves**
- Tri Attack (Lv Evo)
- Zap Cannon (Lv 1)
- Electric Terrain (Lv 1)
- Supersonic (Lv 1)
- Tackle (Lv 1)
- Thunder Shock (Lv 5)
- Thunder Wave (Lv 7)
- Magnet Bomb (Lv 11)
- Light Screen (Lv 13)
- Sonic Boom (Lv 17)
- Spark (Lv 19)
- Mirror Shot (Lv 23)
- Metal Sound (Lv 25)
- Electro Ball (Lv 29)
- Flash Cannon (Lv 33)
- Screech (Lv 39)
- Discharge (Lv 43)
- Lock-On (Lv 49)
- Magnet Rise (Lv 53)
- Gyro Ball (Lv 59)
- Zap Cannon (Lv 63)

**Egg Moves**
- Incompatible

**Tutor Moves**
- Double-Edge
- Endure
- Explosion
- Psych Up
- Rollout
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
</div>
<div class="pokemon-tab-panel" id="pokemon-tabs-magnezone-panel-2">
Types: Electric / Steel • Egg Groups: Mineral

<div class="pokemon-card">
<div class="card-column">
**Abilities**
- Magnet Pull
- Sturdy
- Analytic *(Hidden)*

**Type Matchups**

*Resists / Immune to*
- Normal (0.5×)
- Electric (0.5×)
- Grass (0.5×)
- Ice (0.5×)
- Poison (0×)
- Flying (0.25×)
- Psychic (0.5×)
- Bug (0.5×)
- Rock (0.5×)
- Dragon (0.5×)
- Steel (0.25×)
- Fairy (0.5×)

*Weak to*
- Fire (2×)
- Fighting (2×)
- Ground (4×)

**TM/HM Moves**
- TM06 - Toxic
- TM11 - Sunny Day
- TM16 - Light Screen
- TM17 - Protect
- TM18 - Rain Dance
- TM24 - Thunderbolt
- TM25 - Thunder
- TM32 - Double Team
- TM33 - Reflect
- TM34 - Shock Wave
- TM37 - Sandstorm
- TM42 - Facade
- TM44 - Rest
- TM58 - Thunder Wave
- HM05 - Flash

**Held Item**
Metal Coat

**Evolution Info**
Thunder Stone
</div>
<div class="card-column">
<label><input type="checkbox" class="caught-check" data-species="magnezone" /> Caught</label>

**Base Stats**

| Stat | Value |
| --- | --- |
| HP | <span class="stat-value stat-mid">70</span> |
| Attack | <span class="stat-value stat-mid">70</span> |
| Defense | <span class="stat-value stat-high">115</span> |
| Sp. Atk | <span class="stat-value stat-high">130</span> |
| Sp. Def | <span class="stat-value stat-mid">90</span> |
| Speed | <span class="stat-value stat-mid">60</span> |
| Total | <span class="stat-value stat-mid">535</span> |

**Level-Up Moves**
- Tri Attack (Lv 1)
- Zap Cannon (Lv 1)
- Barrier (Lv 1)
- Electric Terrain (Lv 1)
- Magnetic Flux (Lv 1)
- Mirror Coat (Lv 1)
- Supersonic (Lv 1)
- Tackle (Lv 1)
- Thunder Shock (Lv 5)
- Thunder Wave (Lv 7)
- Magnet Bomb (Lv 11)
- Light Screen (Lv 13)
- Sonic Boom (Lv 17)
- Spark (Lv 19)
- Mirror Shot (Lv 23)
- Metal Sound (Lv 25)
- Electro Ball (Lv 29)
- Flash Cannon (Lv 33)
- Screech (Lv 39)
- Discharge (Lv 43)
- Lock-On (Lv 49)
- Magnet Rise (Lv 53)
- Gyro Ball (Lv 59)
- Zap Cannon (Lv 63)

**Egg Moves**
- Incompatible

**Tutor Moves**
- Body Slam
- Double-Edge
- Endure
- Explosion
- Psych Up
- Rollout
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
</div>
</div>
</div>
<style>
#pokemon-tabs-magnezone-tab-0:checked ~ .pokemon-tab-panels #pokemon-tabs-magnezone-panel-0 { display: block; }
#pokemon-tabs-magnezone-tab-1:checked ~ .pokemon-tab-panels #pokemon-tabs-magnezone-panel-1 { display: block; }
#pokemon-tabs-magnezone-tab-2:checked ~ .pokemon-tab-panels #pokemon-tabs-magnezone-panel-2 { display: block; }
</style>
</details>
