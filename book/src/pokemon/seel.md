<details class="pokemon-card-container">
<summary>Seel (#192)</summary>
<div class="pokemon-tabs" id="pokemon-tabs-seel">
<input type="radio" name="pokemon-tabs-seel-group" id="pokemon-tabs-seel-tab-0" checked>
<label for="pokemon-tabs-seel-tab-0">Seel</label>
<input type="radio" name="pokemon-tabs-seel-group" id="pokemon-tabs-seel-tab-1">
<label for="pokemon-tabs-seel-tab-1">Dewgong</label>
<div class="pokemon-tab-panels">
<div class="pokemon-tab-panel" id="pokemon-tabs-seel-panel-0">
Types: Water / Ice • Egg Groups: Water 1 / Field

<div class="pokemon-card">
<div class="card-column">
**Abilities**
- Thick Layers
- Ice Scales
- Ice Body *(Hidden)*

**Type Matchups**

*Resists / Immune to*
- Water (0.5×)
- Ice (0.25×)

*Weak to*
- Electric (2×)
- Grass (2×)
- Fighting (2×)
- Rock (2×)

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
- TM46 - Thief
- TM50 - Deepwater Curse
- HM03 - Surf
- HM04 - Strength
- HM07 - Waterfall
- HM08 - Dive

**Encounter Locations**
- Erinys Path (East) — Surfing (30%)
- Froslass Cavern BF2 — Grass (Day) (10%)
- Froslass Cavern F1 — Grass (Day) (10%)
- Kipos Town — Fishing (40%)
- Pythios Town — Surfing (30%)
- Péntepetal City — Fishing (60%)
</div>
<div class="card-column">
<label><input type="checkbox" class="caught-check" data-species="seel" /> Caught</label>

**Base Stats**

| Stat | Value |
| --- | --- |
| HP | <span class="stat-value stat-mid">70</span> |
| Attack | <span class="stat-value stat-low">45</span> |
| Defense | <span class="stat-value stat-mid">55</span> |
| Sp. Atk | <span class="stat-value stat-low">45</span> |
| Sp. Def | <span class="stat-value stat-mid">65</span> |
| Speed | <span class="stat-value stat-low">45</span> |
| Total | <span class="stat-value stat-mid">325</span> |

**Level-Up Moves**
- Headbutt (Lv 1)
- Growl (Lv 3)
- Water Sport (Lv 7)
- Icy Wind (Lv 11)
- Encore (Lv 13)
- Ice Shard (Lv 17)
- Rest (Lv 21)
- Aqua Ring (Lv 23)
- Aurora Beam (Lv 27)
- Aqua Jet (Lv 31)
- Brine (Lv 33)
- Take Down (Lv 37)
- Dive (Lv 41)
- Aqua Tail (Lv 43)
- Ice Beam (Lv 47)
- Deepwater Curse (Lv 48)
- Safeguard (Lv 51)
- Hail (Lv 53)

**Egg Moves**
- Lick
- Perish Song
- Disable
- Horn Drill
- Slam
- Encore
- Fake Out
- Icicle Spear
- Signal Beam
- Stockpile
- Swallow
- Spit Up
- Water Pulse
- Iron Tail
- Sleep Talk
- Belch
- Entrainment

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
<div class="pokemon-tab-panel" id="pokemon-tabs-seel-panel-1">
Types: Water / Ice • Egg Groups: Water 1 / Field

<div class="pokemon-card">
<div class="card-column">
**Abilities**
- Thick Layers
- Ice Scales
- Ice Body *(Hidden)*

**Type Matchups**

*Resists / Immune to*
- Water (0.5×)
- Ice (0.25×)

*Weak to*
- Electric (2×)
- Grass (2×)
- Fighting (2×)
- Rock (2×)

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
- TM46 - Thief
- TM50 - Deepwater Curse
- TM52 - Frost Breath
- HM03 - Surf
- HM04 - Strength
- HM07 - Waterfall
- HM08 - Dive

**Evolution Info**
Lv. 34
</div>
<div class="card-column">
<label><input type="checkbox" class="caught-check" data-species="dewgong" /> Caught</label>

**Base Stats**

| Stat | Value |
| --- | --- |
| HP | <span class="stat-value stat-high">115</span> |
| Attack | <span class="stat-value stat-mid">75</span> |
| Defense | <span class="stat-value stat-mid">80</span> |
| Sp. Atk | <span class="stat-value stat-mid">75</span> |
| Sp. Def | <span class="stat-value stat-mid">90</span> |
| Speed | <span class="stat-value stat-mid">65</span> |
| Total | <span class="stat-value stat-mid">500</span> |

**Level-Up Moves**
- Sheer Cold (Lv Evo)
- Headbutt (Lv 1)
- Growl (Lv 3)
- Signal Beam (Lv 7)
- Icy Wind (Lv 11)
- Encore (Lv 13)
- Ice Shard (Lv 17)
- Rest (Lv 21)
- Aqua Ring (Lv 23)
- Aurora Beam (Lv 27)
- Aqua Jet (Lv 31)
- Brine (Lv 33)
- Megahorn (Lv 35)
- Take Down (Lv 39)
- Dive (Lv 45)
- Aqua Tail (Lv 49)
- Ice Beam (Lv 55)
- Deepwater Curse (Lv 55)
- Safeguard (Lv 61)
- Hail (Lv 65)

**Egg Moves**
- Lick
- Perish Song
- Disable
- Horn Drill
- Slam
- Encore
- Fake Out
- Icicle Spear
- Signal Beam
- Stockpile
- Swallow
- Spit Up
- Water Pulse
- Iron Tail
- Sleep Talk
- Belch
- Entrainment

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
</div>
</div>
<style>
#pokemon-tabs-seel-tab-0:checked ~ .pokemon-tab-panels #pokemon-tabs-seel-panel-0 { display: block; }
#pokemon-tabs-seel-tab-1:checked ~ .pokemon-tab-panels #pokemon-tabs-seel-panel-1 { display: block; }
</style>
</details>
