<details class="pokemon-card-container">
<summary>Gouging Fire (#424)</summary>
<div class="pokemon-tabs" id="pokemon-tabs-gouging-fire">
<input type="radio" name="pokemon-tabs-gouging-fire-group" id="pokemon-tabs-gouging-fire-tab-0">
<label for="pokemon-tabs-gouging-fire-tab-0">Entei</label>
<input type="radio" name="pokemon-tabs-gouging-fire-group" id="pokemon-tabs-gouging-fire-tab-1" checked>
<label for="pokemon-tabs-gouging-fire-tab-1">Gouging Fire</label>
<div class="pokemon-tab-panels">
<div class="pokemon-tab-panel" id="pokemon-tabs-gouging-fire-panel-0">
Types: Fire • Egg Groups: -

<div class="pokemon-card">
<div class="card-column">
**Abilities**
- Pressure
- Inner Focus *(Hidden)*

**Type Matchups**

*Resists / Immune to*
- Fire (0.5×)
- Grass (0.5×)
- Ice (0.5×)
- Bug (0.5×)
- Steel (0.5×)
- Fairy (0.5×)

*Weak to*
- Water (2×)
- Ground (2×)
- Rock (2×)

**TM/HM Moves**
- TM04 - Calm Mind
- TM06 - Toxic
- TM11 - Sunny Day
- TM17 - Protect
- TM18 - Rain Dance
- TM22 - Solar Beam
- TM28 - Dig
- TM30 - Shadow Ball
- TM32 - Double Team
- TM33 - Reflect
- TM35 - Flamethrower
- TM37 - Sandstorm
- TM38 - Fire Blast
- TM42 - Facade
- TM44 - Rest
- TM49 - Bulldoze
- TM51 - Will-O-Wisp
- TM55 - Snarl
- HM01 - Cut
- HM04 - Strength
- HM05 - Flash
- HM06 - Rock Smash
</div>
<div class="card-column">
<label><input type="checkbox" class="caught-check" data-species="entei" /> Caught</label>

**Base Stats**

| Stat | Value |
| --- | --- |
| HP | <span class="stat-value stat-high">115</span> |
| Attack | <span class="stat-value stat-high">115</span> |
| Defense | <span class="stat-value stat-mid">85</span> |
| Sp. Atk | <span class="stat-value stat-mid">90</span> |
| Sp. Def | <span class="stat-value stat-mid">65</span> |
| Speed | <span class="stat-value stat-mid">90</span> |
| Total | <span class="stat-value stat-high">560</span> |

**Level-Up Moves**
- Sacred Fire (Lv 1)
- Eruption (Lv 1)
- Extrasensory (Lv 1)
- Lava Plume (Lv 1)
- Bite (Lv 1)
- Leer (Lv 1)
- Ember (Lv 8)
- Roar (Lv 15)
- Fire Spin (Lv 20)
- Stomp (Lv 25)
- Flamethrower (Lv 30)
- Swagger (Lv 35)
- Fire Fang (Lv 40)
- Lava Plume (Lv 45)
- Extrasensory (Lv 50)
- Fire Blast (Lv 55)
- Dragon Energy (Lv 60)
- Calm Mind (Lv 65)
- Eruption (Lv 70)

**Egg Moves**
- Incompatible

**Tutor Moves**
- Body Slam
- Double-Edge
- Endure
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
<div class="pokemon-tab-panel" id="pokemon-tabs-gouging-fire-panel-1">
Types: Fire / Dragon • Egg Groups: -

<div class="pokemon-card">
<div class="card-column">
**Abilities**
- Protosynthesis
- Protosynthesis *(Hidden)*

**Type Matchups**

*Resists / Immune to*
- Fire (0.25×)
- Electric (0.5×)
- Grass (0.25×)
- Bug (0.5×)
- Steel (0.5×)

*Weak to*
- Ground (2×)
- Rock (2×)
- Dragon (2×)

**TM/HM Moves**
- TM02 - Dragon Claw
- TM11 - Sunny Day
- TM17 - Protect
- TM26 - Earthquake
- TM35 - Flamethrower
- TM38 - Fire Blast
- TM42 - Facade
- TM44 - Rest
- TM49 - Bulldoze
- TM55 - Snarl
- TM60 - Dragon Dance

**Held Item**
Booster Energy

**Evolution Info**
Lv. after using Dragon Energy 20x
</div>
<div class="card-column">
<label><input type="checkbox" class="caught-check" data-species="gouging-fire" /> Caught</label>

**Base Stats**

| Stat | Value |
| --- | --- |
| HP | <span class="stat-value stat-high">105</span> |
| Attack | <span class="stat-value stat-high">115</span> |
| Defense | <span class="stat-value stat-high">131</span> |
| Sp. Atk | <span class="stat-value stat-mid">65</span> |
| Sp. Def | <span class="stat-value stat-high">93</span> |
| Speed | <span class="stat-value stat-high">91</span> |
| Total | <span class="stat-value stat-high">600</span> |

**Level-Up Moves**
- Burning Bulwark (Lv Evo)
- Double Kick (Lv 1)
- Ancient Power (Lv 1)
- Noble Roar (Lv 1)
- Stomp (Lv 1)
- Leer (Lv 1)
- Incinerate (Lv 1)
- Sunny Day (Lv 1)
- Fire Fang (Lv 7)
- Howl (Lv 14)
- Bite (Lv 21)
- Dragon Claw (Lv 28)
- Crush Claw (Lv 35)
- Morning Sun (Lv 42)
- Burning Bulwark (Lv 49)
- Dragon Rush (Lv 56)
- Fire Blast (Lv 63)
- Lava Plume (Lv 70)
- Outrage (Lv 77)
- Flare Blitz (Lv 84)
- Raging Fury (Lv 91)

**Egg Moves**
- Incompatible

**Tutor Moves**
- Body Slam
- Double-Edge
- Endure
- Sleep Talk
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
#pokemon-tabs-gouging-fire-tab-0:checked ~ .pokemon-tab-panels #pokemon-tabs-gouging-fire-panel-0 { display: block; }
#pokemon-tabs-gouging-fire-tab-1:checked ~ .pokemon-tab-panels #pokemon-tabs-gouging-fire-panel-1 { display: block; }
</style>
</details>
