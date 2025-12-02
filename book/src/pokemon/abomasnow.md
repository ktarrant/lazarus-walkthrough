<details class="pokemon-card-container">
<summary>Abomasnow (#393)</summary>
<div class="pokemon-tabs" id="pokemon-tabs-abomasnow">
<input type="radio" name="pokemon-tabs-abomasnow-group" id="pokemon-tabs-abomasnow-tab-0">
<label for="pokemon-tabs-abomasnow-tab-0">Snover</label>
<input type="radio" name="pokemon-tabs-abomasnow-group" id="pokemon-tabs-abomasnow-tab-1" checked>
<label for="pokemon-tabs-abomasnow-tab-1">Abomasnow</label>
<input type="radio" name="pokemon-tabs-abomasnow-group" id="pokemon-tabs-abomasnow-tab-2">
<label for="pokemon-tabs-abomasnow-tab-2">Mega Abomasnow</label>
<div class="pokemon-tab-panels">
<div class="pokemon-tab-panel" id="pokemon-tabs-abomasnow-panel-0">
Types: Grass / Ice • Egg Groups: Grass / Monster

<div class="pokemon-card">
<div class="card-column">
**Abilities**
- Snow Warning
- Long Reach *(Hidden)*

**Type Matchups**

*Resists / Immune to*
- Water (0.5×)
- Electric (0.5×)
- Grass (0.5×)
- Ground (0.5×)

*Weak to*
- Fire (4×)
- Fighting (2×)
- Poison (2×)
- Flying (2×)
- Bug (2×)
- Rock (2×)
- Steel (2×)

**TM/HM Moves**
- TM03 - Water Pulse
- TM06 - Toxic
- TM09 - Bullet Seed
- TM13 - Ice Beam
- TM14 - Blizzard
- TM16 - Light Screen
- TM17 - Protect
- TM18 - Rain Dance
- TM19 - Giga Drain
- TM22 - Solar Beam
- TM30 - Shadow Ball
- TM32 - Double Team
- TM42 - Facade
- TM44 - Rest
- TM45 - Attract
- TM52 - Frost Breath
- HM05 - Flash

**Held Item**
Never-Melt Ice

**Encounter Locations**
- Acrisia Mountains (Peak) — Grass (Day) (10%)
- Acrisia Mountains (Peak) — Grass (Night) (10%)
- Nyx Trails — Grass (Day) (20%)
- Nyx Trails — Grass (Night) (20%)
</div>
<div class="card-column">
<label><input type="checkbox" class="caught-check" data-species="snover" /> Caught</label>

**Base Stats**

| Stat | Value |
| --- | --- |
| HP | <span class="stat-value stat-mid">60</span> |
| Attack | <span class="stat-value stat-mid">62</span> |
| Defense | <span class="stat-value stat-low">50</span> |
| Sp. Atk | <span class="stat-value stat-mid">62</span> |
| Sp. Def | <span class="stat-value stat-mid">60</span> |
| Speed | <span class="stat-value stat-low">40</span> |
| Total | <span class="stat-value stat-mid">334</span> |

**Level-Up Moves**
- Powder Snow (Lv 1)
- Leer (Lv 1)
- Razor Leaf (Lv 5)
- Icy Wind (Lv 9)
- Grass Whistle (Lv 13)
- Swagger (Lv 17)
- Mist (Lv 21)
- Ice Shard (Lv 24)
- Avalanche (Lv 28)
- Ingrain (Lv 31)
- Wood Hammer (Lv 34)
- Horn Leech (Lv 38)
- Blizzard (Lv 41)
- Power-Up Punch (Lv 44)
- Sheer Cold (Lv 46)

**Egg Moves**
- Leech Seed
- Magical Leaf
- Seed Bomb
- Growth
- Double-Edge
- Mist
- Stomp
- Skull Bash
- Avalanche
- Natural Gift
- Bullet Seed

**Tutor Moves**
- Body Slam
- Double-Edge
- Endure
- Ice Punch
- Icy Wind
- Mega Punch
- Mud-Slap
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
</div>
<div class="pokemon-tab-panel" id="pokemon-tabs-abomasnow-panel-1">
Types: Grass / Ice • Egg Groups: Grass / Monster

<div class="pokemon-card">
<div class="card-column">
**Abilities**
- Snow Warning
- Long Reach *(Hidden)*

**Type Matchups**

*Resists / Immune to*
- Water (0.5×)
- Electric (0.5×)
- Grass (0.5×)
- Ground (0.5×)

*Weak to*
- Fire (4×)
- Fighting (2×)
- Poison (2×)
- Flying (2×)
- Bug (2×)
- Rock (2×)
- Steel (2×)

**TM/HM Moves**
- TM03 - Water Pulse
- TM06 - Toxic
- TM09 - Bullet Seed
- TM13 - Ice Beam
- TM14 - Blizzard
- TM16 - Light Screen
- TM17 - Protect
- TM18 - Rain Dance
- TM19 - Giga Drain
- TM22 - Solar Beam
- TM26 - Earthquake
- TM30 - Shadow Ball
- TM31 - Brick Break
- TM32 - Double Team
- TM39 - Rock Tomb
- TM42 - Facade
- TM44 - Rest
- TM45 - Attract
- TM49 - Bulldoze
- TM52 - Frost Breath
- HM04 - Strength
- HM05 - Flash
- HM06 - Rock Smash

**Held Item**
Never-Melt Ice

**Evolution Info**
Lv. 36

**Encounter Locations**
- Nyx Trails — Grass (Day) (5%)
- Nyx Trails — Grass (Night) (5%)
</div>
<div class="card-column">
<label><input type="checkbox" class="caught-check" data-species="abomasnow" /> Caught</label>

**Base Stats**

| Stat | Value |
| --- | --- |
| HP | <span class="stat-value stat-high">100</span> |
| Attack | <span class="stat-value stat-high">102</span> |
| Defense | <span class="stat-value stat-mid">75</span> |
| Sp. Atk | <span class="stat-value stat-high">102</span> |
| Sp. Def | <span class="stat-value stat-mid">85</span> |
| Speed | <span class="stat-value stat-mid">60</span> |
| Total | <span class="stat-value stat-mid">524</span> |

**Level-Up Moves**
- Ice Punch (Lv 1)
- Powder Snow (Lv 1)
- Leer (Lv 1)
- Razor Leaf (Lv 5)
- Icy Wind (Lv 9)
- Grass Whistle (Lv 13)
- Swagger (Lv 17)
- Mist (Lv 21)
- Ice Shard (Lv 24)
- Avalanche (Lv 28)
- Ingrain (Lv 31)
- Wood Hammer (Lv 34)
- Horn Leech (Lv 38)
- Blizzard (Lv 41)
- Power-Up Punch (Lv 44)
- Sheer Cold (Lv 46)

**Egg Moves**
- Leech Seed
- Magical Leaf
- Seed Bomb
- Growth
- Double-Edge
- Mist
- Stomp
- Skull Bash
- Avalanche
- Natural Gift
- Bullet Seed

**Tutor Moves**
- Body Slam
- Double-Edge
- Endure
- Ice Punch
- Icy Wind
- Mega Kick
- Mega Punch
- Mud-Slap
- Rock Slide
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
</div>
<div class="pokemon-tab-panel" id="pokemon-tabs-abomasnow-panel-2">
Types: Grass / Ice • Egg Groups: Grass / Monster

<div class="pokemon-card">
<div class="card-column">
**Abilities**
- Snow Warning

**Type Matchups**

*Resists / Immune to*
- Water (0.5×)
- Electric (0.5×)
- Grass (0.5×)
- Ground (0.5×)

*Weak to*
- Fire (4×)
- Fighting (2×)
- Poison (2×)
- Flying (2×)
- Bug (2×)
- Rock (2×)
- Steel (2×)

**TM/HM Moves**
- TM03 - Water Pulse
- TM06 - Toxic
- TM09 - Bullet Seed
- TM13 - Ice Beam
- TM14 - Blizzard
- TM16 - Light Screen
- TM17 - Protect
- TM18 - Rain Dance
- TM19 - Giga Drain
- TM22 - Solar Beam
- TM26 - Earthquake
- TM30 - Shadow Ball
- TM31 - Brick Break
- TM32 - Double Team
- TM39 - Rock Tomb
- TM42 - Facade
- TM44 - Rest
- TM45 - Attract
- TM49 - Bulldoze
- TM52 - Frost Breath
- HM04 - Strength
- HM05 - Flash
- HM06 - Rock Smash

**Held Item**
Never-Melt Ice

**Evolution Info**
Abomasite
</div>
<div class="card-column">
<label><input type="checkbox" class="caught-check" data-species="mega-abomasnow" /> Caught</label>

**Base Stats**

| Stat | Value |
| --- | --- |
| HP | <span class="stat-value stat-high">100</span> |
| Attack | <span class="stat-value stat-high">132</span> |
| Defense | <span class="stat-value stat-high">115</span> |
| Sp. Atk | <span class="stat-value stat-high">132</span> |
| Sp. Def | <span class="stat-value stat-high">115</span> |
| Speed | <span class="stat-value stat-low">30</span> |
| Total | <span class="stat-value stat-high">624</span> |

**Level-Up Moves**
- Ice Punch (Lv 1)
- Powder Snow (Lv 1)
- Leer (Lv 1)
- Razor Leaf (Lv 5)
- Icy Wind (Lv 9)
- Grass Whistle (Lv 13)
- Swagger (Lv 17)
- Mist (Lv 21)
- Ice Shard (Lv 24)
- Avalanche (Lv 28)
- Ingrain (Lv 31)
- Wood Hammer (Lv 34)
- Horn Leech (Lv 38)
- Blizzard (Lv 41)
- Power-Up Punch (Lv 44)
- Sheer Cold (Lv 46)

**Egg Moves**
- Leech Seed
- Magical Leaf
- Seed Bomb
- Growth
- Double-Edge
- Mist
- Stomp
- Skull Bash
- Avalanche
- Natural Gift
- Bullet Seed

**Tutor Moves**
- Body Slam
- Double-Edge
- Endure
- Ice Punch
- Icy Wind
- Mega Kick
- Mega Punch
- Mud-Slap
- Rock Slide
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
</div>
</div>
</div>
<style>
#pokemon-tabs-abomasnow-tab-0:checked ~ .pokemon-tab-panels #pokemon-tabs-abomasnow-panel-0 { display: block; }
#pokemon-tabs-abomasnow-tab-1:checked ~ .pokemon-tab-panels #pokemon-tabs-abomasnow-panel-1 { display: block; }
#pokemon-tabs-abomasnow-tab-2:checked ~ .pokemon-tab-panels #pokemon-tabs-abomasnow-panel-2 { display: block; }
</style>
</details>
