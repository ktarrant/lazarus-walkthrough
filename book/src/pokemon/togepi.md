<details class="pokemon-card-container">
<summary>Togepi (#164)</summary>
<div class="pokemon-tabs" id="pokemon-tabs-togepi">
<input type="radio" name="pokemon-tabs-togepi-group" id="pokemon-tabs-togepi-tab-0" checked>
<label for="pokemon-tabs-togepi-tab-0">Togepi</label>
<input type="radio" name="pokemon-tabs-togepi-group" id="pokemon-tabs-togepi-tab-1">
<label for="pokemon-tabs-togepi-tab-1">Togetic</label>
<input type="radio" name="pokemon-tabs-togepi-group" id="pokemon-tabs-togepi-tab-2">
<label for="pokemon-tabs-togepi-tab-2">Togekiss</label>
<div class="pokemon-tab-panels">
<div class="pokemon-tab-panel" id="pokemon-tabs-togepi-panel-0">
Types: Fairy • Egg Groups: -

<div class="pokemon-card">
<div class="card-column">
**Abilities**
- Serene Grace
- Super Luck *(Hidden)*

**Type Matchups**

*Resists / Immune to*
- Fighting (0.5×)
- Bug (0.5×)
- Dragon (0×)
- Dark (0.5×)

*Weak to*
- Poison (2×)
- Steel (2×)

**TM/HM Moves**
- TM01 - Wish
- TM03 - Water Pulse
- TM04 - Calm Mind
- TM05 - Psyshock
- TM06 - Toxic
- TM11 - Sunny Day
- TM15 - Draining Kiss
- TM16 - Light Screen
- TM17 - Protect
- TM18 - Rain Dance
- TM22 - Solar Beam
- TM29 - Psychic
- TM30 - Shadow Ball
- TM32 - Double Team
- TM33 - Reflect
- TM34 - Shock Wave
- TM35 - Flamethrower
- TM38 - Fire Blast
- TM40 - Aerial Ace
- TM42 - Facade
- TM44 - Rest
- TM45 - Attract
- TM54 - Dazzling Gleam
- TM58 - Thunder Wave
- HM05 - Flash
- HM06 - Rock Smash

**Encounter Locations**
- Erinys Path (West) — Grass (Day) (2%)
- Erinys Path (West) — Grass (Night) (2%)
- Sea of Asteri (East) — Grass (Night) (10%)
</div>
<div class="card-column">
<label><input type="checkbox" class="caught-check" data-species="togepi" /> Caught</label>

**Base Stats**

| Stat | Value |
| --- | --- |
| HP | <span class="stat-value stat-low">35</span> |
| Attack | <span class="stat-value stat-low">20</span> |
| Defense | <span class="stat-value stat-mid">65</span> |
| Sp. Atk | <span class="stat-value stat-low">40</span> |
| Sp. Def | <span class="stat-value stat-mid">65</span> |
| Speed | <span class="stat-value stat-low">20</span> |
| Total | <span class="stat-value stat-low">245</span> |

**Level-Up Moves**
- Growl (Lv 1)
- Charm (Lv 1)
- Metronome (Lv 5)
- Sweet Kiss (Lv 9)
- Yawn (Lv 13)
- Encore (Lv 17)
- Follow Me (Lv 21)
- Bestow (Lv 25)
- Wish (Lv 29)
- Ancient Power (Lv 33)
- Safeguard (Lv 37)
- Baton Pass (Lv 41)
- Double-Edge (Lv 45)
- Last Resort (Lv 49)
- After You (Lv 53)

**Egg Moves**
- Present
- Mirror Move
- Peck
- Foresight
- Future Sight
- Nasty Plot
- Psycho Shift
- Lucky Chant
- Extransensory
- Secret Power
- Stored Power
- Morning Sun

**Tutor Moves**
- Body Slam
- Counter
- Defense Curl
- Double-Edge
- Dream Eater
- Endure
- Mega Kick
- Mega Punch
- Metronome
- Mud-Slap
- Psych Up
- Rollout
- Seismic Toss
- Sleep Talk
- Snore
- Soft-Boiled
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
<div class="pokemon-tab-panel" id="pokemon-tabs-togepi-panel-1">
Types: Fairy / Flying • Egg Groups: Flying / Fairy

<div class="pokemon-card">
<div class="card-column">
**Abilities**
- Serene Grace
- Super Luck *(Hidden)*

**Type Matchups**

*Resists / Immune to*
- Grass (0.5×)
- Fighting (0.25×)
- Ground (0×)
- Bug (0.25×)
- Dragon (0×)
- Dark (0.5×)

*Weak to*
- Electric (2×)
- Ice (2×)
- Poison (2×)
- Rock (2×)
- Steel (2×)

**TM/HM Moves**
- TM01 - Wish
- TM03 - Water Pulse
- TM04 - Calm Mind
- TM05 - Psyshock
- TM06 - Toxic
- TM11 - Sunny Day
- TM15 - Draining Kiss
- TM16 - Light Screen
- TM17 - Protect
- TM18 - Rain Dance
- TM22 - Solar Beam
- TM29 - Psychic
- TM30 - Shadow Ball
- TM31 - Brick Break
- TM32 - Double Team
- TM33 - Reflect
- TM34 - Shock Wave
- TM35 - Flamethrower
- TM38 - Fire Blast
- TM40 - Aerial Ace
- TM42 - Facade
- TM44 - Rest
- TM45 - Attract
- TM47 - Steel Wing
- TM54 - Dazzling Gleam
- TM57 - Roost
- TM58 - Thunder Wave
- HM02 - Fly
- HM05 - Flash
- HM06 - Rock Smash

**Evolution Info**
Lv. 20

**Encounter Locations**
- Port Pello — Grass (Day) (20%)
</div>
<div class="card-column">
<label><input type="checkbox" class="caught-check" data-species="togetic" /> Caught</label>

**Base Stats**

| Stat | Value |
| --- | --- |
| HP | <span class="stat-value stat-mid">65</span> |
| Attack | <span class="stat-value stat-low">40</span> |
| Defense | <span class="stat-value stat-mid">85</span> |
| Sp. Atk | <span class="stat-value stat-mid">80</span> |
| Sp. Def | <span class="stat-value stat-high">105</span> |
| Speed | <span class="stat-value stat-low">40</span> |
| Total | <span class="stat-value stat-mid">415</span> |

**Level-Up Moves**
- Air Slash (Lv Evo)
- Magical Leaf (Lv 1)
- Growl (Lv 1)
- Charm (Lv 1)
- Metronome (Lv 5)
- Sweet Kiss (Lv 9)
- Yawn (Lv 13)
- Fairy Wind (Lv 14)
- Encore (Lv 17)
- Follow Me (Lv 21)
- Bestow (Lv 25)
- Wish (Lv 27)
- Ancient Power (Lv 29)
- Dazzling Gleam (Lv 30)
- Extrasensory (Lv 33)
- Moonblast (Lv 36)
- Safeguard (Lv 37)
- Baton Pass (Lv 41)
- Sparkly Swirl (Lv 45)
- Last Resort (Lv 49)
- After You (Lv 53)

**Egg Moves**
- Present
- Mirror Move
- Peck
- Foresight
- Future Sight
- Nasty Plot
- Psycho Shift
- Lucky Chant
- Extransensory
- Secret Power
- Stored Power
- Morning Sun

**Tutor Moves**
- Body Slam
- Counter
- Defense Curl
- Double-Edge
- Dream Eater
- Endure
- Mega Kick
- Mega Punch
- Metronome
- Mud-Slap
- Psych Up
- Rollout
- Seismic Toss
- Sleep Talk
- Snore
- Soft-Boiled
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
<div class="pokemon-tab-panel" id="pokemon-tabs-togepi-panel-2">
Types: Fairy / Flying • Egg Groups: Flying / Fairy

<div class="pokemon-card">
<div class="card-column">
**Abilities**
- Serene Grace
- Super Luck *(Hidden)*

**Type Matchups**

*Resists / Immune to*
- Grass (0.5×)
- Fighting (0.25×)
- Ground (0×)
- Bug (0.25×)
- Dragon (0×)
- Dark (0.5×)

*Weak to*
- Electric (2×)
- Ice (2×)
- Poison (2×)
- Rock (2×)
- Steel (2×)

**TM/HM Moves**
- TM01 - Wish
- TM03 - Water Pulse
- TM04 - Calm Mind
- TM05 - Psyshock
- TM06 - Toxic
- TM11 - Sunny Day
- TM15 - Draining Kiss
- TM16 - Light Screen
- TM17 - Protect
- TM18 - Rain Dance
- TM22 - Solar Beam
- TM29 - Psychic
- TM30 - Shadow Ball
- TM31 - Brick Break
- TM32 - Double Team
- TM33 - Reflect
- TM34 - Shock Wave
- TM35 - Flamethrower
- TM38 - Fire Blast
- TM40 - Aerial Ace
- TM42 - Facade
- TM44 - Rest
- TM45 - Attract
- TM47 - Steel Wing
- TM54 - Dazzling Gleam
- TM57 - Roost
- TM58 - Thunder Wave
- HM02 - Fly
- HM05 - Flash
- HM06 - Rock Smash

**Evolution Info**
Shiny Stone
</div>
<div class="card-column">
<label><input type="checkbox" class="caught-check" data-species="togekiss" /> Caught</label>

**Base Stats**

| Stat | Value |
| --- | --- |
| HP | <span class="stat-value stat-high">95</span> |
| Attack | <span class="stat-value stat-low">50</span> |
| Defense | <span class="stat-value stat-high">95</span> |
| Sp. Atk | <span class="stat-value stat-high">120</span> |
| Sp. Def | <span class="stat-value stat-high">115</span> |
| Speed | <span class="stat-value stat-mid">80</span> |
| Total | <span class="stat-value stat-high">555</span> |

**Level-Up Moves**
- Aura Sphere (Lv Evo)
- Air Slash (Lv 1)
- Magical Leaf (Lv 1)
- Growl (Lv 1)
- Charm (Lv 1)
- Metronome (Lv 5)
- Sweet Kiss (Lv 9)
- Yawn (Lv 13)
- Fairy Wind (Lv 14)
- Encore (Lv 17)
- Follow Me (Lv 21)
- Bestow (Lv 25)
- Wish (Lv 27)
- Ancient Power (Lv 29)
- Dazzling Gleam (Lv 30)
- Extrasensory (Lv 33)
- Moonblast (Lv 36)
- Safeguard (Lv 37)
- Baton Pass (Lv 41)
- Sparkly Swirl (Lv 45)
- Last Resort (Lv 49)
- After You (Lv 53)

**Egg Moves**
- Present
- Mirror Move
- Peck
- Foresight
- Future Sight
- Nasty Plot
- Psycho Shift
- Lucky Chant
- Extransensory
- Secret Power
- Stored Power
- Morning Sun

**Tutor Moves**
- Body Slam
- Double-Edge
- Dream Eater
- Endure
- Mega Kick
- Mega Punch
- Metronome
- Mud-Slap
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
#pokemon-tabs-togepi-tab-0:checked ~ .pokemon-tab-panels #pokemon-tabs-togepi-panel-0 { display: block; }
#pokemon-tabs-togepi-tab-1:checked ~ .pokemon-tab-panels #pokemon-tabs-togepi-panel-1 { display: block; }
#pokemon-tabs-togepi-tab-2:checked ~ .pokemon-tab-panels #pokemon-tabs-togepi-panel-2 { display: block; }
</style>
</details>
