<details class="pokemon-card-container">
<summary>Primeape (#361)</summary>
<div class="pokemon-tabs" id="pokemon-tabs-primeape">
<input type="radio" name="pokemon-tabs-primeape-group" id="pokemon-tabs-primeape-tab-0">
<label for="pokemon-tabs-primeape-tab-0">Mankey</label>
<input type="radio" name="pokemon-tabs-primeape-group" id="pokemon-tabs-primeape-tab-1" checked>
<label for="pokemon-tabs-primeape-tab-1">Primeape</label>
<input type="radio" name="pokemon-tabs-primeape-group" id="pokemon-tabs-primeape-tab-2">
<label for="pokemon-tabs-primeape-tab-2">Annihilape</label>
<div class="pokemon-tab-panels">
<div class="pokemon-tab-panel" id="pokemon-tabs-primeape-panel-0">
Types: Fighting • Egg Groups: Field

<div class="pokemon-card">
<div class="card-column">
**Abilities**
- Vital Spirit
- Anger Point
- Gorilla Tactics *(Hidden)*

**Type Matchups**

*Resists / Immune to*
- Bug (0.5×)
- Rock (0.5×)
- Dark (0.5×)

*Weak to*
- Flying (2×)
- Psychic (2×)
- Fairy (2×)

**TM/HM Moves**
- TM06 - Toxic
- TM08 - Bulk Up
- TM11 - Sunny Day
- TM12 - Taunt
- TM17 - Protect
- TM18 - Rain Dance
- TM20 - Poison Jab
- TM24 - Thunderbolt
- TM25 - Thunder
- TM26 - Earthquake
- TM28 - Dig
- TM31 - Brick Break
- TM32 - Double Team
- TM39 - Rock Tomb
- TM40 - Aerial Ace
- TM42 - Facade
- TM44 - Rest
- TM45 - Attract
- TM46 - Thief
- TM49 - Bulldoze
- TM53 - Power-Up Punch
- HM04 - Strength
- HM06 - Rock Smash

**Encounter Locations**
- Acrisia Mountains — Grass (Day) (5%)
- Acrisia Mountains — Grass (Night) (5%)
</div>
<div class="card-column">
<label><input type="checkbox" class="caught-check" data-species="mankey" /> Caught</label>

**Base Stats**

| Stat | Value |
| --- | --- |
| HP | <span class="stat-value stat-low">40</span> |
| Attack | <span class="stat-value stat-mid">80</span> |
| Defense | <span class="stat-value stat-low">35</span> |
| Sp. Atk | <span class="stat-value stat-low">35</span> |
| Sp. Def | <span class="stat-value stat-low">45</span> |
| Speed | <span class="stat-value stat-mid">70</span> |
| Total | <span class="stat-value stat-low">305</span> |

**Level-Up Moves**
- Covet (Lv 1)
- Scratch (Lv 1)
- Low Kick (Lv 1)
- Leer (Lv 1)
- Focus Energy (Lv 1)
- Fury Swipes (Lv 5)
- Karate Chop (Lv 8)
- Pursuit (Lv 12)
- Seismic Toss (Lv 15)
- Swagger (Lv 19)
- Cross Chop (Lv 22)
- Assurance (Lv 26)
- Punishment (Lv 29)
- Thrash (Lv 33)
- Close Combat (Lv 36)
- Screech (Lv 40)
- Stomping Tantrum (Lv 43)
- Outrage (Lv 47)
- Final Gambit (Lv 50)

**Egg Moves**
- Foresight
- Meditate
- Counter
- Reversal
- Beat Up
- Revenge
- Smelling Salts
- Close Combat
- Encore
- Focus Punch
- Sleep Talk
- Night Slash
- Power Trip

**Tutor Moves**
- Body Slam
- Counter
- Defense Curl
- Double-Edge
- Dynamic Punch
- Endure
- Fire Punch
- Ice Punch
- Mega Kick
- Mega Punch
- Metronome
- Mud-Slap
- Psych Up
- Rock Slide
- Seismic Toss
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
<div class="pokemon-tab-panel" id="pokemon-tabs-primeape-panel-1">
Types: Fighting • Egg Groups: Field

<div class="pokemon-card">
<div class="card-column">
**Abilities**
- Vital Spirit
- Anger Point
- Gorilla Tactics *(Hidden)*

**Type Matchups**

*Resists / Immune to*
- Bug (0.5×)
- Rock (0.5×)
- Dark (0.5×)

*Weak to*
- Flying (2×)
- Psychic (2×)
- Fairy (2×)

**TM/HM Moves**
- TM06 - Toxic
- TM08 - Bulk Up
- TM11 - Sunny Day
- TM12 - Taunt
- TM17 - Protect
- TM18 - Rain Dance
- TM20 - Poison Jab
- TM24 - Thunderbolt
- TM25 - Thunder
- TM26 - Earthquake
- TM28 - Dig
- TM31 - Brick Break
- TM32 - Double Team
- TM39 - Rock Tomb
- TM40 - Aerial Ace
- TM42 - Facade
- TM44 - Rest
- TM45 - Attract
- TM46 - Thief
- TM49 - Bulldoze
- TM53 - Power-Up Punch
- HM04 - Strength
- HM06 - Rock Smash

**Evolution Info**
Lv. 28

**Encounter Locations**
- Areios Hideout — Grass (Day) (10%)
- Areios Hideout — Grass (Night) (10%)
</div>
<div class="card-column">
<label><input type="checkbox" class="caught-check" data-species="primeape" /> Caught</label>

**Base Stats**

| Stat | Value |
| --- | --- |
| HP | <span class="stat-value stat-mid">65</span> |
| Attack | <span class="stat-value stat-high">105</span> |
| Defense | <span class="stat-value stat-mid">60</span> |
| Sp. Atk | <span class="stat-value stat-mid">60</span> |
| Sp. Def | <span class="stat-value stat-mid">70</span> |
| Speed | <span class="stat-value stat-high">95</span> |
| Total | <span class="stat-value stat-mid">455</span> |

**Level-Up Moves**
- Rage (Lv Evo)
- Revenge (Lv 1)
- Final Gambit (Lv 1)
- Fling (Lv 1)
- Scratch (Lv 1)
- Low Kick (Lv 1)
- Leer (Lv 1)
- Focus Energy (Lv 1)
- Fury Swipes (Lv 5)
- Karate Chop (Lv 8)
- Pursuit (Lv 12)
- Seismic Toss (Lv 15)
- Swagger (Lv 19)
- Cross Chop (Lv 22)
- Assurance (Lv 26)
- Punishment (Lv 30)
- Thrash (Lv 32)
- Rage Fist (Lv 35)
- Close Combat (Lv 39)
- Screech (Lv 44)
- Stomping Tantrum (Lv 48)
- Outrage (Lv 53)
- Final Gambit (Lv 57)

**Egg Moves**
- Foresight
- Meditate
- Counter
- Reversal
- Beat Up
- Revenge
- Smelling Salts
- Close Combat
- Encore
- Focus Punch
- Sleep Talk
- Night Slash
- Power Trip

**Tutor Moves**
- Body Slam
- Counter
- Defense Curl
- Double-Edge
- Dynamic Punch
- Endure
- Fire Punch
- Ice Punch
- Mega Kick
- Mega Punch
- Metronome
- Mud-Slap
- Psych Up
- Rock Slide
- Seismic Toss
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
<div class="pokemon-tab-panel" id="pokemon-tabs-primeape-panel-2">
Types: Fighting / Ghost • Egg Groups: Field

<div class="pokemon-card">
<div class="card-column">
**Abilities**
- Vital Spirit
- Inner Focus
- Gorilla Tactics *(Hidden)*

**Type Matchups**

*Resists / Immune to*
- Normal (0×)
- Fighting (0×)
- Poison (0.5×)
- Bug (0.25×)
- Rock (0.5×)

*Weak to*
- Flying (2×)
- Psychic (2×)
- Ghost (2×)
- Fairy (2×)

**TM/HM Moves**
- TM08 - Bulk Up
- TM11 - Sunny Day
- TM12 - Taunt
- TM17 - Protect
- TM18 - Rain Dance
- TM20 - Poison Jab
- TM24 - Thunderbolt
- TM25 - Thunder
- TM26 - Earthquake
- TM28 - Dig
- TM30 - Shadow Ball
- TM31 - Brick Break
- TM39 - Rock Tomb
- TM42 - Facade
- TM44 - Rest
- TM46 - Thief
- TM49 - Bulldoze

**Evolution Info**
Lv. after using Rage Fist 20x

**Encounter Locations**
- Wakewater Isle — Grass (Night) (2%)
</div>
<div class="card-column">
<label><input type="checkbox" class="caught-check" data-species="annihilape" /> Caught</label>

**Base Stats**

| Stat | Value |
| --- | --- |
| HP | <span class="stat-value stat-high">110</span> |
| Attack | <span class="stat-value stat-high">115</span> |
| Defense | <span class="stat-value stat-mid">80</span> |
| Sp. Atk | <span class="stat-value stat-low">50</span> |
| Sp. Def | <span class="stat-value stat-mid">90</span> |
| Speed | <span class="stat-value stat-mid">90</span> |
| Total | <span class="stat-value stat-mid">535</span> |

**Level-Up Moves**
- Shadow Punch (Lv Evo)
- Revenge (Lv 1)
- Scratch (Lv 1)
- Leer (Lv 1)
- Counter (Lv 1)
- Fling (Lv 1)
- Focus Energy (Lv 1)
- Fury Swipes (Lv 5)
- Low Kick (Lv 8)
- Seismic Toss (Lv 12)
- Swagger (Lv 17)
- Cross Chop (Lv 22)
- Assurance (Lv 26)
- Thrash (Lv 30)
- Rage Fist (Lv 35)
- Close Combat (Lv 39)
- Screech (Lv 44)
- Stomping Tantrum (Lv 48)
- Outrage (Lv 53)
- Final Gambit (Lv 57)

**Egg Moves**
- Foresight
- Meditate
- Counter
- Reversal
- Beat Up
- Revenge
- Smelling Salts
- Close Combat
- Encore
- Focus Punch
- Sleep Talk
- Night Slash
- Power Trip

**Tutor Moves**
- Body Slam
- Counter
- Double-Edge
- Endure
- Fire Punch
- Ice Punch
- Metronome
- Rock Slide
- Seismic Toss
- Sleep Talk
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
#pokemon-tabs-primeape-tab-0:checked ~ .pokemon-tab-panels #pokemon-tabs-primeape-panel-0 { display: block; }
#pokemon-tabs-primeape-tab-1:checked ~ .pokemon-tab-panels #pokemon-tabs-primeape-panel-1 { display: block; }
#pokemon-tabs-primeape-tab-2:checked ~ .pokemon-tab-panels #pokemon-tabs-primeape-panel-2 { display: block; }
</style>
</details>
