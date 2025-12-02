<details class="pokemon-card-container">
<summary>Poliwag (#118)</summary>
<div class="pokemon-tabs" id="pokemon-tabs-poliwag">
<input type="radio" name="pokemon-tabs-poliwag-group" id="pokemon-tabs-poliwag-tab-0" checked>
<label for="pokemon-tabs-poliwag-tab-0">Poliwag</label>
<input type="radio" name="pokemon-tabs-poliwag-group" id="pokemon-tabs-poliwag-tab-1">
<label for="pokemon-tabs-poliwag-tab-1">Poliwhirl</label>
<input type="radio" name="pokemon-tabs-poliwag-group" id="pokemon-tabs-poliwag-tab-2">
<label for="pokemon-tabs-poliwag-tab-2">Poliwrath</label>
<input type="radio" name="pokemon-tabs-poliwag-group" id="pokemon-tabs-poliwag-tab-3">
<label for="pokemon-tabs-poliwag-tab-3">Politoed</label>
<div class="pokemon-tab-panels">
<div class="pokemon-tab-panel" id="pokemon-tabs-poliwag-panel-0">
Types: Water • Egg Groups: Water 1

<div class="pokemon-card">
<div class="card-column">
**Abilities**
- Water Absorb
- Damp
- Swift Swim *(Hidden)*

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
- TM28 - Dig
- TM29 - Psychic
- TM32 - Double Team
- TM42 - Facade
- TM44 - Rest
- TM45 - Attract
- TM46 - Thief
- TM49 - Bulldoze
- TM50 - Deepwater Curse
- TM56 - Scald
- HM03 - Surf
- HM07 - Waterfall
- HM08 - Dive

**Encounter Locations**
- Acrisia City — Fishing (60%)
- Acrisia City — Grass (Day) (10%)
- Acrisia City — Grass (Night) (10%)
- Acrisia Mountains — Fishing (60%)
- Erinys Path (East) — Fishing (60%)
- Pythios Town — Fishing (60%)
- Riverwalk Trail (West) — Fishing (30%)
- Wanderer's Woods (North) — Fishing (60%)
- Wanderer's Woods (North) — Grass (Day) (5%)
- Wanderer's Woods (North) — Grass (Night) (5%)
</div>
<div class="card-column">
<label><input type="checkbox" class="caught-check" data-species="poliwag" /> Caught</label>

**Base Stats**

| Stat | Value |
| --- | --- |
| HP | <span class="stat-value stat-low">40</span> |
| Attack | <span class="stat-value stat-low">50</span> |
| Defense | <span class="stat-value stat-low">40</span> |
| Sp. Atk | <span class="stat-value stat-low">40</span> |
| Sp. Def | <span class="stat-value stat-low">40</span> |
| Speed | <span class="stat-value stat-mid">90</span> |
| Total | <span class="stat-value stat-low">300</span> |

**Level-Up Moves**
- Water Sport (Lv 1)
- Water Gun (Lv 5)
- Hypnosis (Lv 8)
- Bubble (Lv 11)
- Double Slap (Lv 15)
- Rain Dance (Lv 18)
- Body Slam (Lv 21)
- Bubble Beam (Lv 25)
- Mud Shot (Lv 28)
- Belly Drum (Lv 31)
- Deepwater Curse (Lv 34)
- Wake-Up Slap (Lv 35)
- Hydro Pump (Lv 38)
- Mud Bomb (Lv 41)

**Egg Moves**
- Mist
- Splash
- Bubble Beam
- Haze
- Mind Reader
- Water Sport
- Ice Ball
- Mud Shot
- Refresh
- Endeavor
- Encore
- Endure
- Water Pulse

**Tutor Moves**
- Body Slam
- Defense Curl
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
<div class="pokemon-tab-panel" id="pokemon-tabs-poliwag-panel-1">
Types: Water • Egg Groups: Water 1

<div class="pokemon-card">
<div class="card-column">
**Abilities**
- Water Absorb
- Damp
- Swift Swim *(Hidden)*

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
- TM26 - Earthquake
- TM28 - Dig
- TM29 - Psychic
- TM31 - Brick Break
- TM32 - Double Team
- TM42 - Facade
- TM44 - Rest
- TM45 - Attract
- TM46 - Thief
- TM49 - Bulldoze
- TM50 - Deepwater Curse
- TM53 - Power-Up Punch
- TM56 - Scald
- HM03 - Surf
- HM04 - Strength
- HM06 - Rock Smash
- HM07 - Waterfall
- HM08 - Dive

**Held Item**
King`s Rock

**Evolution Info**
Lv. 25

**Encounter Locations**
- Froslass Cavern BF2 — Fishing (60%)
- Pythios Cemetery — Fishing (40%)
- Péntepetal City — Fishing (40%)
</div>
<div class="card-column">
<label><input type="checkbox" class="caught-check" data-species="poliwhirl" /> Caught</label>

**Base Stats**

| Stat | Value |
| --- | --- |
| HP | <span class="stat-value stat-mid">65</span> |
| Attack | <span class="stat-value stat-mid">65</span> |
| Defense | <span class="stat-value stat-mid">65</span> |
| Sp. Atk | <span class="stat-value stat-low">50</span> |
| Sp. Def | <span class="stat-value stat-low">50</span> |
| Speed | <span class="stat-value stat-mid">90</span> |
| Total | <span class="stat-value stat-mid">385</span> |

**Level-Up Moves**
- Water Sport (Lv 1)
- Water Gun (Lv 5)
- Hypnosis (Lv 8)
- Bubble (Lv 11)
- Double Slap (Lv 15)
- Rain Dance (Lv 18)
- Body Slam (Lv 21)
- Bubble Beam (Lv 27)
- Mud Shot (Lv 32)
- Deepwater Curse (Lv 34)
- Belly Drum (Lv 37)
- Wake-Up Slap (Lv 43)
- Hydro Pump (Lv 48)
- Mud Bomb (Lv 53)

**Egg Moves**
- Mist
- Splash
- Bubble Beam
- Haze
- Mind Reader
- Water Sport
- Ice Ball
- Mud Shot
- Refresh
- Endeavor
- Encore
- Endure
- Water Pulse

**Tutor Moves**
- Body Slam
- Counter
- Defense Curl
- Double-Edge
- Endure
- Ice Punch
- Icy Wind
- Mega Kick
- Mega Punch
- Metronome
- Mud-Slap
- Psych Up
- Seismic Toss
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
<div class="pokemon-tab-panel" id="pokemon-tabs-poliwag-panel-2">
Types: Water / Fighting • Egg Groups: Water 1

<div class="pokemon-card">
<div class="card-column">
**Abilities**
- Water Absorb
- Damp
- Swift Swim *(Hidden)*

**Type Matchups**

*Resists / Immune to*
- Fire (0.5×)
- Water (0.5×)
- Ice (0.5×)
- Bug (0.5×)
- Rock (0.5×)
- Dark (0.5×)
- Steel (0.5×)

*Weak to*
- Electric (2×)
- Grass (2×)
- Flying (2×)
- Psychic (2×)
- Fairy (2×)

**TM/HM Moves**
- TM03 - Water Pulse
- TM06 - Toxic
- TM07 - Whirlpool
- TM08 - Bulk Up
- TM12 - Taunt
- TM13 - Ice Beam
- TM14 - Blizzard
- TM17 - Protect
- TM18 - Rain Dance
- TM20 - Poison Jab
- TM26 - Earthquake
- TM28 - Dig
- TM29 - Psychic
- TM31 - Brick Break
- TM32 - Double Team
- TM39 - Rock Tomb
- TM42 - Facade
- TM44 - Rest
- TM45 - Attract
- TM46 - Thief
- TM49 - Bulldoze
- TM50 - Deepwater Curse
- TM53 - Power-Up Punch
- TM56 - Scald
- HM03 - Surf
- HM04 - Strength
- HM06 - Rock Smash
- HM07 - Waterfall
- HM08 - Dive

**Held Item**
King`s Rock

**Evolution Info**
Water Stone
</div>
<div class="card-column">
<label><input type="checkbox" class="caught-check" data-species="poliwrath" /> Caught</label>

**Base Stats**

| Stat | Value |
| --- | --- |
| HP | <span class="stat-value stat-high">95</span> |
| Attack | <span class="stat-value stat-high">110</span> |
| Defense | <span class="stat-value stat-high">95</span> |
| Sp. Atk | <span class="stat-value stat-mid">65</span> |
| Sp. Def | <span class="stat-value stat-mid">90</span> |
| Speed | <span class="stat-value stat-mid">70</span> |
| Total | <span class="stat-value stat-mid">525</span> |

**Level-Up Moves**
- Circle Throw (Lv Evo)
- Water Sport (Lv 1)
- Water Gun (Lv 5)
- Hypnosis (Lv 8)
- Bubble (Lv 11)
- Double Slap (Lv 15)
- Rain Dance (Lv 18)
- Body Slam (Lv 21)
- Bubble Beam (Lv 27)
- Mud Shot (Lv 32)
- Deepwater Curse (Lv 34)
- Waterfall (Lv 36)
- Belly Drum (Lv 37)
- Brick Break (Lv 41)
- Wake-Up Slap (Lv 43)
- Hydro Pump (Lv 48)
- Earthquake (Lv 51)

**Egg Moves**
- Mist
- Splash
- Bubble Beam
- Haze
- Mind Reader
- Water Sport
- Ice Ball
- Mud Shot
- Refresh
- Endeavor
- Encore
- Endure
- Water Pulse

**Tutor Moves**
- Body Slam
- Counter
- Defense Curl
- Double-Edge
- Dynamic Punch
- Endure
- Ice Punch
- Icy Wind
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
<div class="pokemon-tab-panel" id="pokemon-tabs-poliwag-panel-3">
Types: Water / Normal • Egg Groups: Water 1

<div class="pokemon-card">
<div class="card-column">
**Abilities**
- Water Absorb
- Damp
- Drizzle *(Hidden)*

**Type Matchups**

*Resists / Immune to*
- Fire (0.5×)
- Water (0.5×)
- Ice (0.5×)
- Ghost (0×)
- Steel (0.5×)

*Weak to*
- Electric (2×)
- Grass (2×)
- Fighting (2×)

**TM/HM Moves**
- TM03 - Water Pulse
- TM06 - Toxic
- TM07 - Whirlpool
- TM13 - Ice Beam
- TM14 - Blizzard
- TM17 - Protect
- TM18 - Rain Dance
- TM26 - Earthquake
- TM28 - Dig
- TM29 - Psychic
- TM31 - Brick Break
- TM32 - Double Team
- TM42 - Facade
- TM44 - Rest
- TM45 - Attract
- TM46 - Thief
- TM49 - Bulldoze
- TM50 - Deepwater Curse
- TM53 - Power-Up Punch
- TM56 - Scald
- HM03 - Surf
- HM04 - Strength
- HM06 - Rock Smash
- HM07 - Waterfall
- HM08 - Dive

**Held Item**
King`s Rock

**Evolution Info**
King's Rock
</div>
<div class="card-column">
<label><input type="checkbox" class="caught-check" data-species="politoed" /> Caught</label>

**Base Stats**

| Stat | Value |
| --- | --- |
| HP | <span class="stat-value stat-high">100</span> |
| Attack | <span class="stat-value stat-mid">75</span> |
| Defense | <span class="stat-value stat-mid">75</span> |
| Sp. Atk | <span class="stat-value stat-high">95</span> |
| Sp. Def | <span class="stat-value stat-high">100</span> |
| Speed | <span class="stat-value stat-mid">70</span> |
| Total | <span class="stat-value stat-mid">515</span> |

**Level-Up Moves**
- Bounce (Lv Evo)
- Perish Song (Lv 1)
- Water Sport (Lv 1)
- Water Gun (Lv 5)
- Hypnosis (Lv 8)
- Bubble (Lv 11)
- Double Slap (Lv 15)
- Rain Dance (Lv 18)
- Body Slam (Lv 21)
- Bubble Beam (Lv 27)
- Mud Shot (Lv 32)
- Deepwater Curse (Lv 34)
- Waterfall (Lv 36)
- Body Slam (Lv 37)
- Uproar (Lv 41)
- Hydro Pump (Lv 43)
- Hyper Voice (Lv 47)
- Perish Song (Lv 50)

**Egg Moves**
- Mist
- Splash
- Bubble Beam
- Haze
- Mind Reader
- Water Sport
- Ice Ball
- Mud Shot
- Refresh
- Endeavor
- Encore
- Endure
- Water Pulse

**Tutor Moves**
- Body Slam
- Counter
- Defense Curl
- Double-Edge
- Dynamic Punch
- Endure
- Ice Punch
- Icy Wind
- Mega Kick
- Mega Punch
- Metronome
- Mud-Slap
- Psych Up
- Seismic Toss
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
#pokemon-tabs-poliwag-tab-0:checked ~ .pokemon-tab-panels #pokemon-tabs-poliwag-panel-0 { display: block; }
#pokemon-tabs-poliwag-tab-1:checked ~ .pokemon-tab-panels #pokemon-tabs-poliwag-panel-1 { display: block; }
#pokemon-tabs-poliwag-tab-2:checked ~ .pokemon-tab-panels #pokemon-tabs-poliwag-panel-2 { display: block; }
#pokemon-tabs-poliwag-tab-3:checked ~ .pokemon-tab-panels #pokemon-tabs-poliwag-panel-3 { display: block; }
</style>
</details>
