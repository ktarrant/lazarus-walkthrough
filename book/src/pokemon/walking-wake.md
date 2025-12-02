<details class="pokemon-card-container">
<summary>Walking Wake (#426)</summary>
<div class="pokemon-tabs" id="pokemon-tabs-walking-wake">
<input type="radio" name="pokemon-tabs-walking-wake-group" id="pokemon-tabs-walking-wake-tab-0">
<label for="pokemon-tabs-walking-wake-tab-0">Suicune</label>
<input type="radio" name="pokemon-tabs-walking-wake-group" id="pokemon-tabs-walking-wake-tab-1" checked>
<label for="pokemon-tabs-walking-wake-tab-1">Walking Wake</label>
<div class="pokemon-tab-panels">
<div class="pokemon-tab-panel" id="pokemon-tabs-walking-wake-panel-0">
Types: Water • Egg Groups: -

<div class="pokemon-card">
<div class="card-column">
**Abilities**
- Pressure
- Inner Focus *(Hidden)*

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
- TM04 - Calm Mind
- TM06 - Toxic
- TM07 - Whirlpool
- TM11 - Sunny Day
- TM13 - Ice Beam
- TM14 - Blizzard
- TM17 - Protect
- TM18 - Rain Dance
- TM28 - Dig
- TM30 - Shadow Ball
- TM32 - Double Team
- TM33 - Reflect
- TM37 - Sandstorm
- TM42 - Facade
- TM44 - Rest
- TM49 - Bulldoze
- TM55 - Snarl
- TM56 - Scald
- HM01 - Cut
- HM03 - Surf
- HM06 - Rock Smash
- HM07 - Waterfall
- HM08 - Dive
</div>
<div class="card-column">
<label><input type="checkbox" class="caught-check" data-species="suicune" /> Caught</label>

**Base Stats**

| Stat | Value |
| --- | --- |
| HP | <span class="stat-value stat-high">100</span> |
| Attack | <span class="stat-value stat-mid">65</span> |
| Defense | <span class="stat-value stat-high">105</span> |
| Sp. Atk | <span class="stat-value stat-mid">90</span> |
| Sp. Def | <span class="stat-value stat-high">105</span> |
| Speed | <span class="stat-value stat-high">95</span> |
| Total | <span class="stat-value stat-high">560</span> |

**Level-Up Moves**
- Sheer Cold (Lv 1)
- Bite (Lv 1)
- Leer (Lv 1)
- Rain Dance (Lv 1)
- Bubble Beam (Lv 8)
- Rain Dance (Lv 15)
- Gust (Lv 20)
- Aurora Beam (Lv 25)
- Mist (Lv 30)
- Mirror Coat (Lv 35)
- Ice Fang (Lv 40)
- Tailwind (Lv 45)
- Extrasensory (Lv 50)
- Hydro Pump (Lv 55)
- Dragon Energy (Lv 60)
- Calm Mind (Lv 65)
- Blizzard (Lv 70)

**Egg Moves**
- Incompatible

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
<div class="pokemon-tab-panel" id="pokemon-tabs-walking-wake-panel-1">
Types: Water / Dragon • Egg Groups: -

<div class="pokemon-card">
<div class="card-column">
**Abilities**
- Protosynthesis
- Protosynthesis *(Hidden)*

**Type Matchups**

*Resists / Immune to*
- Fire (0.25×)
- Water (0.25×)
- Steel (0.5×)

*Weak to*
- Dragon (2×)
- Fairy (2×)

**TM/HM Moves**
- TM02 - Dragon Claw
- TM03 - Water Pulse
- TM07 - Whirlpool
- TM11 - Sunny Day
- TM17 - Protect
- TM18 - Rain Dance
- TM35 - Flamethrower
- TM42 - Facade
- TM44 - Rest
- TM55 - Snarl
- TM56 - Scald
- TM60 - Dragon Dance
- HM03 - Surf
- HM07 - Waterfall

**Held Item**
Booster Energy

**Evolution Info**
Lv. after using Dragon Energy 20x
</div>
<div class="card-column">
<label><input type="checkbox" class="caught-check" data-species="walking-wake" /> Caught</label>

**Base Stats**

| Stat | Value |
| --- | --- |
| HP | <span class="stat-value stat-high">99</span> |
| Attack | <span class="stat-value stat-high">93</span> |
| Defense | <span class="stat-value stat-high">91</span> |
| Sp. Atk | <span class="stat-value stat-high">125</span> |
| Sp. Def | <span class="stat-value stat-mid">83</span> |
| Speed | <span class="stat-value stat-high">109</span> |
| Total | <span class="stat-value stat-high">600</span> |

**Level-Up Moves**
- Hydro Steam (Lv Evo)
- Sunny Day (Lv 1)
- Hone Claws (Lv 1)
- Leer (Lv 1)
- Roar (Lv 1)
- Twister (Lv 1)
- Aqua Jet (Lv 1)
- Bite (Lv 7)
- Water Pulse (Lv 14)
- Noble Roar (Lv 21)
- Dragon Breath (Lv 28)
- Breaking Swipe (Lv 35)
- Dragon Rush (Lv 42)
- Hydro Steam (Lv 56)
- Dragon Pulse (Lv 63)
- Outrage (Lv 70)
- Flamethrower (Lv 77)
- Hydro Pump (Lv 84)

**Egg Moves**
- Incompatible

**Tutor Moves**
- Body Slam
- Double-Edge
- Endure
- Sleep Talk
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
#pokemon-tabs-walking-wake-tab-0:checked ~ .pokemon-tab-panels #pokemon-tabs-walking-wake-panel-0 { display: block; }
#pokemon-tabs-walking-wake-tab-1:checked ~ .pokemon-tab-panels #pokemon-tabs-walking-wake-panel-1 { display: block; }
</style>
</details>
