<details class="pokemon-card-container">
<summary>Huntail (#310)</summary>
<div class="pokemon-tabs" id="pokemon-tabs-huntail">
<input type="radio" name="pokemon-tabs-huntail-group" id="pokemon-tabs-huntail-tab-0">
<label for="pokemon-tabs-huntail-tab-0">Clamperl</label>
<input type="radio" name="pokemon-tabs-huntail-group" id="pokemon-tabs-huntail-tab-1" checked>
<label for="pokemon-tabs-huntail-tab-1">Huntail</label>
<input type="radio" name="pokemon-tabs-huntail-group" id="pokemon-tabs-huntail-tab-2">
<label for="pokemon-tabs-huntail-tab-2">Gorebyss</label>
<div class="pokemon-tab-panels">
<div class="pokemon-tab-panel" id="pokemon-tabs-huntail-panel-0">
Types: Water • Egg Groups: Water 1

<div class="pokemon-card">
<div class="card-column">
**Abilities**
- Shell Armor
- Rattled *(Hidden)*

**Type Matchups**

*Resists / Immune to*
- Fire (0.5×)
- Water (0.5×)
- Ice (0.5×)
- Steel (0.5×)

*Weak to*
- Electric (2×)
- Grass (2×)

**TM/HM Moves**
- TM03 - Water Pulse
- TM06 - Toxic
- TM07 - Whirlpool
- TM13 - Ice Beam
- TM14 - Blizzard
- TM17 - Protect
- TM18 - Rain Dance
- TM32 - Double Team
- TM42 - Facade
- TM44 - Rest
- TM45 - Attract
- TM56 - Scald
- HM03 - Surf
- HM07 - Waterfall
- HM08 - Dive

**Held Item**
Pearl

**Encounter Locations**
- Corrin Crossing — Fishing (20%)
- Corrin Crossing — Fishing (40%)
- Fresco Isles — Fishing (20%)
- Fresco Isles — Fishing (40%)
- Nyx Trails — Fishing (20%)
- Nyx Trails — Fishing (40%)
- Palati City — Fishing (20%)
- Palati City — Fishing (40%)
- Port Pello — Fishing (20%)
- Port Pello — Fishing (40%)
- Pythios Cemetery — Fishing (20%)
- Péntepetal City — Fishing (40%)
- Sea of Asteri (East) — Fishing (20%)
- Sea of Asteri (East) — Fishing (40%)
- Sea of Asteri (West) — Fishing (20%)
- Sea of Asteri (West) — Fishing (40%)
- Sea of Vulcai — Fishing (20%)
- Sea of Vulcai — Fishing (40%)
</div>
<div class="card-column">
<label><input type="checkbox" class="caught-check" data-species="clamperl" /> Caught</label>

**Base Stats**

| Stat | Value |
| --- | --- |
| HP | <span class="stat-value stat-low">35</span> |
| Attack | <span class="stat-value stat-mid">64</span> |
| Defense | <span class="stat-value stat-mid">85</span> |
| Sp. Atk | <span class="stat-value stat-mid">74</span> |
| Sp. Def | <span class="stat-value stat-mid">55</span> |
| Speed | <span class="stat-value stat-low">32</span> |
| Total | <span class="stat-value stat-mid">345</span> |

**Level-Up Moves**
- Clamp (Lv 1)
- Water Gun (Lv 1)
- Whirlpool (Lv 1)
- Iron Defense (Lv 1)
- Shell Smash (Lv 50)

**Egg Moves**
- Refresh
- Mud Sport
- Body Slam
- Supersonic
- Barrier
- Confuse Ray
- Aqua Ring
- Muddy Water
- Water Pulse
- Brine
- Endure

**Tutor Moves**
- Body Slam
- Double-Edge
- Endure
- Icy Wind
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
<div class="pokemon-tab-panel" id="pokemon-tabs-huntail-panel-1">
Types: Water / Dragon • Egg Groups: Water 1

<div class="pokemon-card">
<div class="card-column">
**Abilities**
- Intimidate
- Water Veil *(Hidden)*

**Type Matchups**

*Resists / Immune to*
- Fire (0.25×)
- Water (0.25×)
- Steel (0.5×)

*Weak to*
- Dragon (2×)
- Fairy (2×)

**TM/HM Moves**
- TM03 - Water Pulse
- TM06 - Toxic
- TM07 - Whirlpool
- TM13 - Ice Beam
- TM14 - Blizzard
- TM17 - Protect
- TM18 - Rain Dance
- TM32 - Double Team
- TM39 - Rock Tomb
- TM42 - Facade
- TM44 - Rest
- TM45 - Attract
- TM56 - Scald
- HM03 - Surf
- HM07 - Waterfall
- HM08 - Dive

**Held Item**
Deep Sea Tooth

**Evolution Info**
Deep Sea Tooth

**Encounter Locations**
- Pollen Road — Fishing (5%)
</div>
<div class="card-column">
<label><input type="checkbox" class="caught-check" data-species="huntail" /> Caught</label>

**Base Stats**

| Stat | Value |
| --- | --- |
| HP | <span class="stat-value stat-mid">55</span> |
| Attack | <span class="stat-value stat-high">119</span> |
| Defense | <span class="stat-value stat-high">105</span> |
| Sp. Atk | <span class="stat-value stat-high">94</span> |
| Sp. Def | <span class="stat-value stat-mid">75</span> |
| Speed | <span class="stat-value stat-mid">57</span> |
| Total | <span class="stat-value stat-mid">505</span> |

**Level-Up Moves**
- Breaking Swipe (Lv Evo)
- Whirlpool (Lv 1)
- Bite (Lv 1)
- Screech (Lv 5)
- Scary Face (Lv 9)
- Feint Attack (Lv 11)
- Water Pulse (Lv 14)
- Ice Fang (Lv 16)
- Brine (Lv 19)
- Sucker Punch (Lv 23)
- Dive (Lv 26)
- Dragon Dance (Lv 29)
- Baton Pass (Lv 29)
- Crunch (Lv 32)
- Dragon Rush (Lv 35)
- Aqua Tail (Lv 39)
- Wicked Blow (Lv 43)
- Coil (Lv 45)
- Hydro Pump (Lv 50)

**Egg Moves**
- Refresh
- Mud Sport
- Body Slam
- Supersonic
- Barrier
- Confuse Ray
- Aqua Ring
- Muddy Water
- Water Pulse
- Brine
- Endure

**Tutor Moves**
- Body Slam
- Double-Edge
- Endure
- Icy Wind
- Mud-Slap
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
<div class="pokemon-tab-panel" id="pokemon-tabs-huntail-panel-2">
Types: Water / Fairy • Egg Groups: Water 1

<div class="pokemon-card">
<div class="card-column">
**Abilities**
- Regenerator
- Hydration *(Hidden)*

**Type Matchups**

*Resists / Immune to*
- Fire (0.5×)
- Water (0.5×)
- Ice (0.5×)
- Fighting (0.5×)
- Bug (0.5×)
- Dragon (0×)
- Dark (0.5×)

*Weak to*
- Electric (2×)
- Grass (2×)
- Poison (2×)

**TM/HM Moves**
- TM03 - Water Pulse
- TM06 - Toxic
- TM07 - Whirlpool
- TM13 - Ice Beam
- TM14 - Blizzard
- TM15 - Draining Kiss
- TM17 - Protect
- TM18 - Rain Dance
- TM29 - Psychic
- TM30 - Shadow Ball
- TM32 - Double Team
- TM42 - Facade
- TM44 - Rest
- TM45 - Attract
- TM56 - Scald
- HM03 - Surf
- HM07 - Waterfall
- HM08 - Dive

**Held Item**
Deep Sea Scale

**Evolution Info**
Deep Sea Scale

**Encounter Locations**
- Davosi Straits — Fishing (5%)
</div>
<div class="card-column">
<label><input type="checkbox" class="caught-check" data-species="gorebyss" /> Caught</label>

**Base Stats**

| Stat | Value |
| --- | --- |
| HP | <span class="stat-value stat-mid">75</span> |
| Attack | <span class="stat-value stat-mid">74</span> |
| Defense | <span class="stat-value stat-high">105</span> |
| Sp. Atk | <span class="stat-value stat-high">114</span> |
| Sp. Def | <span class="stat-value stat-mid">85</span> |
| Speed | <span class="stat-value stat-mid">52</span> |
| Total | <span class="stat-value stat-mid">505</span> |

**Level-Up Moves**
- Draining Kiss (Lv Evo)
- Whirlpool (Lv 1)
- Confusion (Lv 1)
- Water Sport (Lv 5)
- Agility (Lv 9)
- Swift (Lv 11)
- Water Pulse (Lv 14)
- Amnesia (Lv 16)
- Aqua Ring (Lv 19)
- Captivate (Lv 23)
- Dive (Lv 26)
- Calm Mind (Lv 29)
- Baton Pass (Lv 29)
- Psychic (Lv 32)
- Dazzling Gleam (Lv 35)
- Aqua Tail (Lv 39)
- Moonblast (Lv 43)
- Coil (Lv 45)
- Hydro Pump (Lv 50)

**Egg Moves**
- Refresh
- Mud Sport
- Body Slam
- Supersonic
- Barrier
- Confuse Ray
- Aqua Ring
- Muddy Water
- Water Pulse
- Brine
- Endure

**Tutor Moves**
- Body Slam
- Double-Edge
- Endure
- Icy Wind
- Mud-Slap
- Psych Up
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
#pokemon-tabs-huntail-tab-0:checked ~ .pokemon-tab-panels #pokemon-tabs-huntail-panel-0 { display: block; }
#pokemon-tabs-huntail-tab-1:checked ~ .pokemon-tab-panels #pokemon-tabs-huntail-panel-1 { display: block; }
#pokemon-tabs-huntail-tab-2:checked ~ .pokemon-tab-panels #pokemon-tabs-huntail-panel-2 { display: block; }
</style>
</details>
