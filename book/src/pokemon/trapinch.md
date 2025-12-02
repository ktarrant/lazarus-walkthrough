<details class="pokemon-card-container">
<summary>Trapinch (#321)</summary>
<div class="pokemon-tabs" id="pokemon-tabs-trapinch">
<input type="radio" name="pokemon-tabs-trapinch-group" id="pokemon-tabs-trapinch-tab-0" checked>
<label for="pokemon-tabs-trapinch-tab-0">Trapinch</label>
<input type="radio" name="pokemon-tabs-trapinch-group" id="pokemon-tabs-trapinch-tab-1">
<label for="pokemon-tabs-trapinch-tab-1">Vibrava</label>
<input type="radio" name="pokemon-tabs-trapinch-group" id="pokemon-tabs-trapinch-tab-2">
<label for="pokemon-tabs-trapinch-tab-2">Flygon</label>
<input type="radio" name="pokemon-tabs-trapinch-group" id="pokemon-tabs-trapinch-tab-3">
<label for="pokemon-tabs-trapinch-tab-3">Mega Flygon</label>
<div class="pokemon-tab-panels">
<div class="pokemon-tab-panel" id="pokemon-tabs-trapinch-panel-0">
Types: Ground • Egg Groups: Bug / Dragon

<div class="pokemon-card">
<div class="card-column">
**Abilities**
- Hyper Cutter
- Arena Trap
- Sheer Force *(Hidden)*

**Type Matchups**

*Resists / Immune to*
- Electric (0×)
- Poison (0.5×)
- Rock (0.5×)

*Weak to*
- Water (2×)
- Grass (2×)
- Ice (2×)

**TM/HM Moves**
- TM06 - Toxic
- TM11 - Sunny Day
- TM17 - Protect
- TM19 - Giga Drain
- TM22 - Solar Beam
- TM26 - Earthquake
- TM28 - Dig
- TM32 - Double Team
- TM37 - Sandstorm
- TM39 - Rock Tomb
- TM42 - Facade
- TM44 - Rest
- TM45 - Attract
- TM49 - Bulldoze
- HM04 - Strength
- HM06 - Rock Smash

**Held Item**
Soft Sand

**Encounter Locations**
- Fresco Isles — Grass (Day) (20%)
- Fresco Isles — Grass (Night) (20%)
- Pythios Cemetery — Grass (Day) (10%)
- Pythios Cemetery — Grass (Night) (10%)
</div>
<div class="card-column">
<label><input type="checkbox" class="caught-check" data-species="trapinch" /> Caught</label>

**Base Stats**

| Stat | Value |
| --- | --- |
| HP | <span class="stat-value stat-low">45</span> |
| Attack | <span class="stat-value stat-high">100</span> |
| Defense | <span class="stat-value stat-low">45</span> |
| Sp. Atk | <span class="stat-value stat-low">45</span> |
| Sp. Def | <span class="stat-value stat-low">45</span> |
| Speed | <span class="stat-value stat-low">10</span> |
| Total | <span class="stat-value stat-low">290</span> |

**Level-Up Moves**
- Sand Attack (Lv 1)
- Bite (Lv 1)
- Feint Attack (Lv 1)
- Bide (Lv 1)
- Mud-Slap (Lv 5)
- Bulldoze (Lv 8)
- Sand Tomb (Lv 12)
- Rock Slide (Lv 15)
- Dig (Lv 19)
- Crunch (Lv 22)
- Stomping Tantrum (Lv 26)
- Feint (Lv 29)
- Earth Power (Lv 33)
- Sandstorm (Lv 36)
- Superpower (Lv 40)
- Attack Order (Lv 43)
- Fissure (Lv 47)

**Egg Moves**
- Focus Energy
- Quick Attack
- Gust
- Flail
- Fury Cutter
- Mud Shot
- Endure
- Earth Power
- Bug Bite
- Signal Beam

**Tutor Moves**
- Body Slam
- Double-Edge
- Endure
- Fury Cutter
- Mud-Slap
- Rock Slide
- Sleep Talk
- Snore
- Swagger
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
<div class="pokemon-tab-panel" id="pokemon-tabs-trapinch-panel-1">
Types: Ground / Dragon • Egg Groups: Bug / Dragon

<div class="pokemon-card">
<div class="card-column">
**Abilities**
- Levitate
- Sheer Force *(Hidden)*

**Type Matchups**

*Resists / Immune to*
- Fire (0.5×)
- Electric (0×)
- Poison (0.5×)
- Rock (0.5×)

*Weak to*
- Ice (4×)
- Dragon (2×)
- Fairy (2×)

**TM/HM Moves**
- TM02 - Dragon Claw
- TM06 - Toxic
- TM11 - Sunny Day
- TM17 - Protect
- TM19 - Giga Drain
- TM22 - Solar Beam
- TM26 - Earthquake
- TM28 - Dig
- TM32 - Double Team
- TM37 - Sandstorm
- TM39 - Rock Tomb
- TM40 - Aerial Ace
- TM42 - Facade
- TM44 - Rest
- TM45 - Attract
- TM47 - Steel Wing
- TM49 - Bulldoze
- TM57 - Roost
- HM02 - Fly
- HM04 - Strength
- HM06 - Rock Smash

**Evolution Info**
Lv. 22
</div>
<div class="card-column">
<label><input type="checkbox" class="caught-check" data-species="vibrava" /> Caught</label>

**Base Stats**

| Stat | Value |
| --- | --- |
| HP | <span class="stat-value stat-low">50</span> |
| Attack | <span class="stat-value stat-mid">75</span> |
| Defense | <span class="stat-value stat-mid">55</span> |
| Sp. Atk | <span class="stat-value stat-low">50</span> |
| Sp. Def | <span class="stat-value stat-mid">55</span> |
| Speed | <span class="stat-value stat-mid">85</span> |
| Total | <span class="stat-value stat-mid">370</span> |

**Level-Up Moves**
- Leech Life (Lv Evo)
- Dragon Breath (Lv 1)
- Sand Attack (Lv 1)
- Sonic Boom (Lv 1)
- Feint Attack (Lv 1)
- Bide (Lv 1)
- Mud-Slap (Lv 5)
- Bulldoze (Lv 8)
- Sand Tomb (Lv 12)
- Rock Slide (Lv 15)
- Supersonic (Lv 19)
- Screech (Lv 22)
- Stomping Tantrum (Lv 26)
- Bug Buzz (Lv 28)
- Breaking Swipe (Lv 30)
- Earth Power (Lv 33)
- Sandstorm (Lv 36)
- Hyper Voice (Lv 38)
- Dragon Dance (Lv 41)
- Leech Life (Lv 43)
- Attack Order (Lv 49)

**Egg Moves**
- Focus Energy
- Quick Attack
- Gust
- Flail
- Fury Cutter
- Mud Shot
- Endure
- Earth Power
- Bug Bite
- Signal Beam

**Tutor Moves**
- Body Slam
- Double-Edge
- Endure
- Fury Cutter
- Mud-Slap
- Rock Slide
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
<div class="pokemon-tab-panel" id="pokemon-tabs-trapinch-panel-2">
Types: Ground / Dragon • Egg Groups: Bug / Dragon

<div class="pokemon-card">
<div class="card-column">
**Abilities**
- Levitate
- Sheer Force *(Hidden)*

**Type Matchups**

*Resists / Immune to*
- Fire (0.5×)
- Electric (0×)
- Poison (0.5×)
- Rock (0.5×)

*Weak to*
- Ice (4×)
- Dragon (2×)
- Fairy (2×)

**TM/HM Moves**
- TM02 - Dragon Claw
- TM06 - Toxic
- TM11 - Sunny Day
- TM17 - Protect
- TM19 - Giga Drain
- TM22 - Solar Beam
- TM26 - Earthquake
- TM28 - Dig
- TM32 - Double Team
- TM35 - Flamethrower
- TM37 - Sandstorm
- TM38 - Fire Blast
- TM39 - Rock Tomb
- TM40 - Aerial Ace
- TM42 - Facade
- TM44 - Rest
- TM45 - Attract
- TM47 - Steel Wing
- TM49 - Bulldoze
- TM53 - Power-Up Punch
- TM57 - Roost
- TM60 - Dragon Dance
- HM02 - Fly
- HM04 - Strength
- HM06 - Rock Smash

**Evolution Info**
Lv. 45
</div>
<div class="card-column">
<label><input type="checkbox" class="caught-check" data-species="flygon" /> Caught</label>

**Base Stats**

| Stat | Value |
| --- | --- |
| HP | <span class="stat-value stat-mid">80</span> |
| Attack | <span class="stat-value stat-high">100</span> |
| Defense | <span class="stat-value stat-mid">80</span> |
| Sp. Atk | <span class="stat-value stat-mid">85</span> |
| Sp. Def | <span class="stat-value stat-mid">80</span> |
| Speed | <span class="stat-value stat-high">100</span> |
| Total | <span class="stat-value stat-mid">525</span> |

**Level-Up Moves**
- Dragon Claw (Lv Evo)
- Dragon Breath (Lv 1)
- Dragon Dance (Lv 1)
- Sand Attack (Lv 1)
- Sonic Boom (Lv 1)
- Feint Attack (Lv 1)
- Bide (Lv 1)
- Mud-Slap (Lv 5)
- Bulldoze (Lv 8)
- Sand Tomb (Lv 12)
- Rock Slide (Lv 15)
- Supersonic (Lv 19)
- Screech (Lv 22)
- Stomping Tantrum (Lv 26)
- Dragon Tail (Lv 28)
- Breaking Swipe (Lv 30)
- Earth Power (Lv 33)
- Sandstorm (Lv 36)
- Hyper Voice (Lv 38)
- Dragon Dance (Lv 41)
- Leech Life (Lv 43)
- Attack Order (Lv 49)

**Egg Moves**
- Focus Energy
- Quick Attack
- Gust
- Flail
- Fury Cutter
- Mud Shot
- Endure
- Earth Power
- Bug Bite
- Signal Beam

**Tutor Moves**
- Body Slam
- Double-Edge
- Endure
- Fire Punch
- Fury Cutter
- Mega Kick
- Mega Punch
- Mud-Slap
- Rock Slide
- Sleep Talk
- Snore
- Swagger
- Swift
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
</div>
<div class="pokemon-tab-panel" id="pokemon-tabs-trapinch-panel-3">
Types: Bug / Dragon • Egg Groups: Bug / Dragon

<div class="pokemon-card">
<div class="card-column">
**Abilities**
- Burrower (Boosts Ground moves, like Dragon's Maw?)

**Type Matchups**

*Resists / Immune to*
- Water (0.5×)
- Electric (0.5×)
- Grass (0.25×)
- Fighting (0.5×)
- Ground (0.5×)

*Weak to*
- Ice (2×)
- Flying (2×)
- Rock (2×)
- Dragon (2×)
- Fairy (2×)

**TM/HM Moves**
- TM02 - Dragon Claw
- TM06 - Toxic
- TM11 - Sunny Day
- TM17 - Protect
- TM19 - Giga Drain
- TM22 - Solar Beam
- TM26 - Earthquake
- TM28 - Dig
- TM32 - Double Team
- TM35 - Flamethrower
- TM37 - Sandstorm
- TM38 - Fire Blast
- TM39 - Rock Tomb
- TM40 - Aerial Ace
- TM42 - Facade
- TM44 - Rest
- TM45 - Attract
- TM47 - Steel Wing
- TM49 - Bulldoze
- TM53 - Power-Up Punch
- TM57 - Roost
- TM60 - Dragon Dance
- HM02 - Fly
- HM04 - Strength
- HM06 - Rock Smash

**Evolution Info**
Flygonite
</div>
<div class="card-column">
<label><input type="checkbox" class="caught-check" data-species="mega-flygon" /> Caught</label>

**Base Stats**

| Stat | Value |
| --- | --- |
| HP | <span class="stat-value stat-mid">80</span> |
| Attack | <span class="stat-value stat-high">120</span> |
| Defense | <span class="stat-value stat-mid">90</span> |
| Sp. Atk | <span class="stat-value stat-high">135</span> |
| Sp. Def | <span class="stat-value stat-mid">90</span> |
| Speed | <span class="stat-value stat-high">110</span> |
| Total | <span class="stat-value stat-high">625</span> |

**Level-Up Moves**
- Dragon Claw (Lv Evo)
- Dragon Breath (Lv 1)
- Dragon Dance (Lv 1)
- Sand Attack (Lv 1)
- Sonic Boom (Lv 1)
- Feint Attack (Lv 1)
- Bide (Lv 1)
- Mud-Slap (Lv 5)
- Bulldoze (Lv 8)
- Sand Tomb (Lv 12)
- Rock Slide (Lv 15)
- Supersonic (Lv 19)
- Screech (Lv 22)
- Stomping Tantrum (Lv 26)
- Dragon Tail (Lv 28)
- Breaking Swipe (Lv 30)
- Earth Power (Lv 33)
- Sandstorm (Lv 36)
- Hyper Voice (Lv 38)
- Dragon Dance (Lv 41)
- Leech Life (Lv 43)
- Attack Order (Lv 49)

**Egg Moves**
- Focus Energy
- Quick Attack
- Gust
- Flail
- Fury Cutter
- Mud Shot
- Endure
- Earth Power
- Bug Bite
- Signal Beam

**Tutor Moves**
- Body Slam
- Double-Edge
- Endure
- Fire Punch
- Fury Cutter
- Mega Kick
- Mega Punch
- Mud-Slap
- Rock Slide
- Sleep Talk
- Snore
- Swagger
- Swift
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
</div>
</div>
</div>
<style>
#pokemon-tabs-trapinch-tab-0:checked ~ .pokemon-tab-panels #pokemon-tabs-trapinch-panel-0 { display: block; }
#pokemon-tabs-trapinch-tab-1:checked ~ .pokemon-tab-panels #pokemon-tabs-trapinch-panel-1 { display: block; }
#pokemon-tabs-trapinch-tab-2:checked ~ .pokemon-tab-panels #pokemon-tabs-trapinch-panel-2 { display: block; }
#pokemon-tabs-trapinch-tab-3:checked ~ .pokemon-tab-panels #pokemon-tabs-trapinch-panel-3 { display: block; }
</style>
</details>
