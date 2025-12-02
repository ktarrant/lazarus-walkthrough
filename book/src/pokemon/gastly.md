<details class="pokemon-card-container">
<summary>Gastly (#155)</summary>
<div class="pokemon-tabs" id="pokemon-tabs-gastly">
<input type="radio" name="pokemon-tabs-gastly-group" id="pokemon-tabs-gastly-tab-0" checked>
<label for="pokemon-tabs-gastly-tab-0">Gastly</label>
<input type="radio" name="pokemon-tabs-gastly-group" id="pokemon-tabs-gastly-tab-1">
<label for="pokemon-tabs-gastly-tab-1">Haunter</label>
<input type="radio" name="pokemon-tabs-gastly-group" id="pokemon-tabs-gastly-tab-2">
<label for="pokemon-tabs-gastly-tab-2">Gengar</label>
<input type="radio" name="pokemon-tabs-gastly-group" id="pokemon-tabs-gastly-tab-3">
<label for="pokemon-tabs-gastly-tab-3">Mega Gengar</label>
<div class="pokemon-tab-panels">
<div class="pokemon-tab-panel" id="pokemon-tabs-gastly-panel-0">
Types: Ghost / Poison • Egg Groups: Amorphous

<div class="pokemon-card">
<div class="card-column">
**Abilities**
- Levitate

**Type Matchups**

*Resists / Immune to*
- Normal (0×)
- Grass (0.5×)
- Fighting (0×)
- Poison (0.25×)
- Bug (0.25×)
- Fairy (0.5×)

*Weak to*
- Ground (2×)
- Psychic (2×)
- Ghost (2×)
- Dark (2×)

**TM/HM Moves**
- TM06 - Toxic
- TM11 - Sunny Day
- TM12 - Taunt
- TM17 - Protect
- TM18 - Rain Dance
- TM19 - Giga Drain
- TM20 - Poison Jab
- TM23 - Hex
- TM24 - Thunderbolt
- TM25 - Thunder
- TM29 - Psychic
- TM30 - Shadow Ball
- TM32 - Double Team
- TM36 - Sludge Bomb
- TM41 - Torment
- TM42 - Facade
- TM44 - Rest
- TM45 - Attract
- TM46 - Thief
- TM48 - Skill Swap
- TM51 - Will-O-Wisp
- TM54 - Dazzling Gleam
- TM59 - Dark Pulse

**Encounter Locations**
- Erinys Path (East) — Grass (Day) (20%)
- Erinys Path (East) — Grass (Night) (20%)
</div>
<div class="card-column">
<label><input type="checkbox" class="caught-check" data-species="gastly" /> Caught</label>

**Base Stats**

| Stat | Value |
| --- | --- |
| HP | <span class="stat-value stat-low">30</span> |
| Attack | <span class="stat-value stat-low">35</span> |
| Defense | <span class="stat-value stat-low">30</span> |
| Sp. Atk | <span class="stat-value stat-high">100</span> |
| Sp. Def | <span class="stat-value stat-low">35</span> |
| Speed | <span class="stat-value stat-mid">80</span> |
| Total | <span class="stat-value stat-mid">310</span> |

**Level-Up Moves**
- Hypnosis (Lv 1)
- Lick (Lv 1)
- Spite (Lv 5)
- Mean Look (Lv 8)
- Curse (Lv 12)
- Night Shade (Lv 15)
- Sludge (Lv 17)
- Confuse Ray (Lv 19)
- Sucker Punch (Lv 22)
- Payback (Lv 26)
- Shadow Ball (Lv 29)
- Sludge Bomb (Lv 36?)
- Dream Eater (Lv 33)
- Dark Pulse (Lv 36)
- Destiny Bond (Lv 40)
- Hex (Lv 43)
- Nightmare (Lv 47)

**Egg Moves**
- Psywave
- Perish Song
- Haze
- Astonish
- Grudge
- Fire Punch
- Ice Punch
- Thunder Punch
- Disable
- Scary Face
- Clear Smog
- Smog
- Reflect Type

**Tutor Moves**
- Acid Spray
- Dream Eater
- Endure
- Explosion
- Fire Punch
- Ice Punch
- Icy Wind
- Psych Up
- Sleep Talk
- Snore
- Swagger
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
<div class="pokemon-tab-panel" id="pokemon-tabs-gastly-panel-1">
Types: Ghost / Poison • Egg Groups: Amorphous

<div class="pokemon-card">
<div class="card-column">
**Abilities**
- Levitate

**Type Matchups**

*Resists / Immune to*
- Normal (0×)
- Grass (0.5×)
- Fighting (0×)
- Poison (0.25×)
- Bug (0.25×)
- Fairy (0.5×)

*Weak to*
- Ground (2×)
- Psychic (2×)
- Ghost (2×)
- Dark (2×)

**TM/HM Moves**
- TM06 - Toxic
- TM11 - Sunny Day
- TM12 - Taunt
- TM17 - Protect
- TM18 - Rain Dance
- TM19 - Giga Drain
- TM20 - Poison Jab
- TM23 - Hex
- TM24 - Thunderbolt
- TM25 - Thunder
- TM29 - Psychic
- TM30 - Shadow Ball
- TM32 - Double Team
- TM36 - Sludge Bomb
- TM41 - Torment
- TM42 - Facade
- TM44 - Rest
- TM45 - Attract
- TM46 - Thief
- TM48 - Skill Swap
- TM51 - Will-O-Wisp
- TM54 - Dazzling Gleam
- TM59 - Dark Pulse

**Evolution Info**
Lv. 25
</div>
<div class="card-column">
<label><input type="checkbox" class="caught-check" data-species="haunter" /> Caught</label>

**Base Stats**

| Stat | Value |
| --- | --- |
| HP | <span class="stat-value stat-low">45</span> |
| Attack | <span class="stat-value stat-low">50</span> |
| Defense | <span class="stat-value stat-low">45</span> |
| Sp. Atk | <span class="stat-value stat-high">115</span> |
| Sp. Def | <span class="stat-value stat-mid">55</span> |
| Speed | <span class="stat-value stat-high">95</span> |
| Total | <span class="stat-value stat-mid">405</span> |

**Level-Up Moves**
- Shadow Punch (Lv Evo)
- Hypnosis (Lv 1)
- Lick (Lv 1)
- Spite (Lv 5)
- Mean Look (Lv 8)
- Curse (Lv 12)
- Night Shade (Lv 15)
- Sludge (Lv 17)
- Confuse Ray (Lv 19)
- Sucker Punch (Lv 22)
- Payback (Lv 28)
- Shadow Ball (Lv 33)
- Sludge Bomb (Lv 36)
- Dream Eater (Lv 39)
- Dark Pulse (Lv 44)
- Destiny Bond (Lv 50)
- Hex (Lv 55)
- Nightmare (Lv 61)

**Egg Moves**
- Psywave
- Perish Song
- Haze
- Astonish
- Grudge
- Fire Punch
- Ice Punch
- Thunder Punch
- Disable
- Scary Face
- Clear Smog
- Smog
- Reflect Type

**Tutor Moves**
- Acid Spray
- Dream Eater
- Endure
- Explosion
- Fire Punch
- Ice Punch
- Icy Wind
- Metronome
- Psych Up
- Sleep Talk
- Snore
- Swagger
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
<div class="pokemon-tab-panel" id="pokemon-tabs-gastly-panel-2">
Types: Ghost / Poison • Egg Groups: Amorphous

<div class="pokemon-card">
<div class="card-column">
**Abilities**
- Levitate

**Type Matchups**

*Resists / Immune to*
- Normal (0×)
- Grass (0.5×)
- Fighting (0×)
- Poison (0.25×)
- Bug (0.25×)
- Fairy (0.5×)

*Weak to*
- Ground (2×)
- Psychic (2×)
- Ghost (2×)
- Dark (2×)

**TM/HM Moves**
- TM06 - Toxic
- TM11 - Sunny Day
- TM12 - Taunt
- TM17 - Protect
- TM18 - Rain Dance
- TM19 - Giga Drain
- TM20 - Poison Jab
- TM23 - Hex
- TM24 - Thunderbolt
- TM25 - Thunder
- TM29 - Psychic
- TM30 - Shadow Ball
- TM31 - Brick Break
- TM32 - Double Team
- TM36 - Sludge Bomb
- TM41 - Torment
- TM42 - Facade
- TM44 - Rest
- TM45 - Attract
- TM46 - Thief
- TM48 - Skill Swap
- TM51 - Will-O-Wisp
- TM53 - Power-Up Punch
- TM54 - Dazzling Gleam
- TM58 - Thunder Wave
- TM59 - Dark Pulse
- HM04 - Strength
- HM06 - Rock Smash

**Evolution Info**
Linking Cord
</div>
<div class="card-column">
<label><input type="checkbox" class="caught-check" data-species="gengar" /> Caught</label>

**Base Stats**

| Stat | Value |
| --- | --- |
| HP | <span class="stat-value stat-mid">60</span> |
| Attack | <span class="stat-value stat-mid">65</span> |
| Defense | <span class="stat-value stat-mid">60</span> |
| Sp. Atk | <span class="stat-value stat-high">130</span> |
| Sp. Def | <span class="stat-value stat-mid">75</span> |
| Speed | <span class="stat-value stat-high">110</span> |
| Total | <span class="stat-value stat-mid">500</span> |

**Level-Up Moves**
- Shadow Punch (Lv Evo)
- Hypnosis (Lv 1)
- Lick (Lv 1)
- Spite (Lv 5)
- Mean Look (Lv 8)
- Curse (Lv 12)
- Night Shade (Lv 15)
- Sludge (Lv 17)
- Confuse Ray (Lv 19)
- Sucker Punch (Lv 22)
- Payback (Lv 28)
- Shadow Ball (Lv 33)
- Sludge Bomb (Lv 36)
- Dream Eater (Lv 39)
- Dark Pulse (Lv 44)
- Destiny Bond (Lv 50)
- Hex (Lv 55)
- Nightmare (Lv 61)

**Egg Moves**
- Psywave
- Perish Song
- Haze
- Astonish
- Grudge
- Fire Punch
- Ice Punch
- Thunder Punch
- Disable
- Scary Face
- Clear Smog
- Smog
- Reflect Type

**Tutor Moves**
- Acid Spray
- Body Slam
- Counter
- Double-Edge
- Dream Eater
- Dynamic Punch
- Endure
- Explosion
- Fire Punch
- Ice Punch
- Icy Wind
- Mega Kick
- Mega Punch
- Metronome
- Psych Up
- Seismic Toss
- Sleep Talk
- Snore
- Swagger
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
<div class="pokemon-tab-panel" id="pokemon-tabs-gastly-panel-3">
Types: Ghost / Poison • Egg Groups: Amorphous

<div class="pokemon-card">
<div class="card-column">
**Abilities**
- Shadow Tag

**Type Matchups**

*Resists / Immune to*
- Normal (0×)
- Grass (0.5×)
- Fighting (0×)
- Poison (0.25×)
- Bug (0.25×)
- Fairy (0.5×)

*Weak to*
- Ground (2×)
- Psychic (2×)
- Ghost (2×)
- Dark (2×)

**TM/HM Moves**
- TM06 - Toxic
- TM11 - Sunny Day
- TM12 - Taunt
- TM17 - Protect
- TM18 - Rain Dance
- TM19 - Giga Drain
- TM20 - Poison Jab
- TM23 - Hex
- TM24 - Thunderbolt
- TM25 - Thunder
- TM29 - Psychic
- TM30 - Shadow Ball
- TM31 - Brick Break
- TM32 - Double Team
- TM36 - Sludge Bomb
- TM41 - Torment
- TM42 - Facade
- TM44 - Rest
- TM45 - Attract
- TM46 - Thief
- TM48 - Skill Swap
- TM51 - Will-O-Wisp
- TM53 - Power-Up Punch
- TM54 - Dazzling Gleam
- TM58 - Thunder Wave
- TM59 - Dark Pulse
- HM04 - Strength
- HM06 - Rock Smash

**Evolution Info**
Gengarite
</div>
<div class="card-column">
<label><input type="checkbox" class="caught-check" data-species="mega-gengar" /> Caught</label>

**Base Stats**

| Stat | Value |
| --- | --- |
| HP | <span class="stat-value stat-mid">60</span> |
| Attack | <span class="stat-value stat-mid">65</span> |
| Defense | <span class="stat-value stat-mid">80</span> |
| Sp. Atk | <span class="stat-value stat-high">170</span> |
| Sp. Def | <span class="stat-value stat-high">95</span> |
| Speed | <span class="stat-value stat-high">130</span> |
| Total | <span class="stat-value stat-high">600</span> |

**Level-Up Moves**
- Shadow Punch (Lv Evo)
- Hypnosis (Lv 1)
- Lick (Lv 1)
- Spite (Lv 5)
- Mean Look (Lv 8)
- Curse (Lv 12)
- Night Shade (Lv 15)
- Sludge (Lv 17)
- Confuse Ray (Lv 19)
- Sucker Punch (Lv 22)
- Payback (Lv 28)
- Shadow Ball (Lv 33)
- Sludge Bomb (Lv 36)
- Dream Eater (Lv 39)
- Dark Pulse (Lv 44)
- Destiny Bond (Lv 50)
- Hex (Lv 55)
- Nightmare (Lv 61)

**Egg Moves**
- Psywave
- Perish Song
- Haze
- Astonish
- Grudge
- Fire Punch
- Ice Punch
- Thunder Punch
- Disable
- Scary Face
- Clear Smog
- Smog
- Reflect Type

**Tutor Moves**
- Acid Spray
- Body Slam
- Counter
- Double-Edge
- Dream Eater
- Dynamic Punch
- Endure
- Explosion
- Fire Punch
- Ice Punch
- Icy Wind
- Mega Kick
- Mega Punch
- Metronome
- Psych Up
- Seismic Toss
- Sleep Talk
- Snore
- Swagger
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
#pokemon-tabs-gastly-tab-0:checked ~ .pokemon-tab-panels #pokemon-tabs-gastly-panel-0 { display: block; }
#pokemon-tabs-gastly-tab-1:checked ~ .pokemon-tab-panels #pokemon-tabs-gastly-panel-1 { display: block; }
#pokemon-tabs-gastly-tab-2:checked ~ .pokemon-tab-panels #pokemon-tabs-gastly-panel-2 { display: block; }
#pokemon-tabs-gastly-tab-3:checked ~ .pokemon-tab-panels #pokemon-tabs-gastly-panel-3 { display: block; }
</style>
</details>
