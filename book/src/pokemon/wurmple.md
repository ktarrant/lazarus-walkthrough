<details class="pokemon-card-container">
<summary>Wurmple (#073)</summary>
<div class="pokemon-tabs" id="pokemon-tabs-wurmple">
<input type="radio" name="pokemon-tabs-wurmple-group" id="pokemon-tabs-wurmple-tab-0" checked>
<label for="pokemon-tabs-wurmple-tab-0">Wurmple</label>
<input type="radio" name="pokemon-tabs-wurmple-group" id="pokemon-tabs-wurmple-tab-1">
<label for="pokemon-tabs-wurmple-tab-1">Silcoon</label>
<input type="radio" name="pokemon-tabs-wurmple-group" id="pokemon-tabs-wurmple-tab-2">
<label for="pokemon-tabs-wurmple-tab-2">Beautifly</label>
<input type="radio" name="pokemon-tabs-wurmple-group" id="pokemon-tabs-wurmple-tab-3">
<label for="pokemon-tabs-wurmple-tab-3">Cascoon</label>
<input type="radio" name="pokemon-tabs-wurmple-group" id="pokemon-tabs-wurmple-tab-4">
<label for="pokemon-tabs-wurmple-tab-4">Dustox</label>
<div class="pokemon-tab-panels">
<div class="pokemon-tab-panel" id="pokemon-tabs-wurmple-panel-0">
Types: Bug • Egg Groups: Bug

<div class="pokemon-card">
<div class="card-column">
**Abilities**
- Shield Dust
- Run Away *(Hidden)*

**Type Matchups**

*Resists / Immune to*
- Grass (0.5×)
- Fighting (0.5×)
- Ground (0.5×)

*Weak to*
- Fire (2×)
- Flying (2×)
- Rock (2×)

**TM/HM Moves**
- <a href="move-lookup.html?q=incompatible">Incompatible</a>

**Held Item**
Pecha Berry

**Encounter Locations**
- Jusmail Town — Grass (Day) (20%)
- Jusmail Town — Grass (Night) (20%)
- Riverwalk Trail (South) — Grass (Day) (10%)
- Riverwalk Trail (South) — Grass (Night) (10%)
</div>
<div class="card-column">
<label><input type="checkbox" class="caught-check" data-species="wurmple" /> Caught</label>

**Base Stats**

| Stat | Value |
| --- | --- |
| HP | <span class="stat-value stat-low">45</span> |
| Attack | <span class="stat-value stat-low">45</span> |
| Defense | <span class="stat-value stat-low">35</span> |
| Sp. Atk | <span class="stat-value stat-low">20</span> |
| Sp. Def | <span class="stat-value stat-low">30</span> |
| Speed | <span class="stat-value stat-low">20</span> |
| Total | <span class="stat-value stat-low">195</span> |

**Level-Up Moves**
- <a href="move-lookup.html?q=tackle">Tackle</a> (Lv 1)
- <a href="move-lookup.html?q=string-shot">String Shot</a> (Lv 1)
- <a href="move-lookup.html?q=poison-sting">Poison Sting</a> (Lv 5)
- <a href="move-lookup.html?q=bug-bite">Bug Bite</a> (Lv 15)

**Egg Moves**
- <a href="move-lookup.html?q=incompatible">Incompatible</a>

**Tutor Moves**
- <a href="move-lookup.html?q=snore">Snore</a>
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
<div class="pokemon-tab-panel" id="pokemon-tabs-wurmple-panel-1">
Types: Bug • Egg Groups: Bug

<div class="pokemon-card">
<div class="card-column">
**Abilities**
- Shed Skin

**Type Matchups**

*Resists / Immune to*
- Grass (0.5×)
- Fighting (0.5×)
- Ground (0.5×)

*Weak to*
- Fire (2×)
- Flying (2×)
- Rock (2×)

**TM/HM Moves**
- <a href="move-lookup.html?q=incompatible">Incompatible</a>

**Evolution Info**
Lv. 7

**Encounter Locations**
- Riverwalk Trail (South) — Grass (Day) (8%)
</div>
<div class="card-column">
<label><input type="checkbox" class="caught-check" data-species="silcoon" /> Caught</label>

**Base Stats**

| Stat | Value |
| --- | --- |
| HP | <span class="stat-value stat-low">50</span> |
| Attack | <span class="stat-value stat-low">35</span> |
| Defense | <span class="stat-value stat-mid">55</span> |
| Sp. Atk | <span class="stat-value stat-low">25</span> |
| Sp. Def | <span class="stat-value stat-low">25</span> |
| Speed | <span class="stat-value stat-low">15</span> |
| Total | <span class="stat-value stat-low">205</span> |

**Level-Up Moves**
- <a href="move-lookup.html?q=harden">Harden</a> (Lv Evo)

**Egg Moves**
- <a href="move-lookup.html?q=incompatible">Incompatible</a>
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
<div class="pokemon-tab-panel" id="pokemon-tabs-wurmple-panel-2">
Types: Bug / Flying • Egg Groups: Bug

<div class="pokemon-card">
<div class="card-column">
**Abilities**
- Swarm
- Rivalry *(Hidden)*

**Type Matchups**

*Resists / Immune to*
- Grass (0.25×)
- Fighting (0.25×)
- Ground (0×)
- Bug (0.5×)

*Weak to*
- Fire (2×)
- Electric (2×)
- Ice (2×)
- Flying (2×)
- Rock (4×)

**TM/HM Moves**
- <a href="move-lookup.html?q=toxic">TM06 - Toxic</a>
- <a href="move-lookup.html?q=sunny-day">TM11 - Sunny Day</a>
- <a href="move-lookup.html?q=protect">TM17 - Protect</a>
- <a href="move-lookup.html?q=giga-drain">TM19 - Giga Drain</a>
- <a href="move-lookup.html?q=solar-beam">TM22 - Solar Beam</a>
- <a href="move-lookup.html?q=psychic">TM29 - Psychic</a>
- <a href="move-lookup.html?q=shadow-ball">TM30 - Shadow Ball</a>
- <a href="move-lookup.html?q=double-team">TM32 - Double Team</a>
- <a href="move-lookup.html?q=aerial-ace">TM40 - Aerial Ace</a>
- <a href="move-lookup.html?q=facade">TM42 - Facade</a>
- <a href="move-lookup.html?q=rest">TM44 - Rest</a>
- <a href="move-lookup.html?q=attract">TM45 - Attract</a>
- <a href="move-lookup.html?q=thief">TM46 - Thief</a>
- <a href="move-lookup.html?q=roost">TM57 - Roost</a>
- <a href="move-lookup.html?q=flash">HM05 - Flash</a>

**Held Item**
Shed Shell

**Evolution Info**
Lv. 10

**Encounter Locations**
- Kipos Town — Grass (Day) (2%)
- Péntepetal City — Grass (Day) (20%)
</div>
<div class="card-column">
<label><input type="checkbox" class="caught-check" data-species="beautifly" /> Caught</label>

**Base Stats**

| Stat | Value |
| --- | --- |
| HP | <span class="stat-value stat-mid">60</span> |
| Attack | <span class="stat-value stat-low">50</span> |
| Defense | <span class="stat-value stat-mid">60</span> |
| Sp. Atk | <span class="stat-value stat-high">115</span> |
| Sp. Def | <span class="stat-value stat-mid">70</span> |
| Speed | <span class="stat-value stat-mid">85</span> |
| Total | <span class="stat-value stat-mid">440</span> |

**Level-Up Moves**
- <a href="move-lookup.html?q=gust">Gust</a> (Lv Evo)
- <a href="move-lookup.html?q=absorb">Absorb</a> (Lv 12)
- <a href="move-lookup.html?q=stun-spore">Stun Spore</a> (Lv 15)
- <a href="move-lookup.html?q=morning-sun">Morning Sun</a> (Lv 17)
- <a href="move-lookup.html?q=air-cutter">Air Cutter</a> (Lv 20)
- <a href="move-lookup.html?q=mega-drain">Mega Drain</a> (Lv 22)
- <a href="move-lookup.html?q=silver-wind">Silver Wind</a> (Lv 25)
- <a href="move-lookup.html?q=attract">Attract</a> (Lv 27)
- <a href="move-lookup.html?q=whirlwind">Whirlwind</a> (Lv 30)
- <a href="move-lookup.html?q=giga-drain">Giga Drain</a> (Lv 32)
- <a href="move-lookup.html?q=bug-buzz">Bug Buzz</a> (Lv 35)
- <a href="move-lookup.html?q=rage">Rage</a> (Lv 37)
- <a href="move-lookup.html?q=quiver-dance">Quiver Dance</a> (Lv 40)

**Egg Moves**
- <a href="move-lookup.html?q=incompatible">Incompatible</a>

**Tutor Moves**
- <a href="move-lookup.html?q=double-edge">Double-Edge</a>
- <a href="move-lookup.html?q=endure">Endure</a>
- <a href="move-lookup.html?q=sleep-talk">Sleep Talk</a>
- <a href="move-lookup.html?q=snore">Snore</a>
- <a href="move-lookup.html?q=swagger">Swagger</a>
- <a href="move-lookup.html?q=swift">Swift</a>
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
<div class="pokemon-tab-panel" id="pokemon-tabs-wurmple-panel-3">
Types: Bug • Egg Groups: Bug

<div class="pokemon-card">
<div class="card-column">
**Abilities**
- Shed Skin

**Type Matchups**

*Resists / Immune to*
- Grass (0.5×)
- Fighting (0.5×)
- Ground (0.5×)

*Weak to*
- Fire (2×)
- Flying (2×)
- Rock (2×)

**TM/HM Moves**
- <a href="move-lookup.html?q=incompatible">Incompatible</a>

**Evolution Info**
Lv. 7

**Encounter Locations**
- Riverwalk Trail (South) — Grass (Night) (8%)
</div>
<div class="card-column">
<label><input type="checkbox" class="caught-check" data-species="cascoon" /> Caught</label>

**Base Stats**

| Stat | Value |
| --- | --- |
| HP | <span class="stat-value stat-low">50</span> |
| Attack | <span class="stat-value stat-low">35</span> |
| Defense | <span class="stat-value stat-mid">55</span> |
| Sp. Atk | <span class="stat-value stat-low">25</span> |
| Sp. Def | <span class="stat-value stat-low">25</span> |
| Speed | <span class="stat-value stat-low">15</span> |
| Total | <span class="stat-value stat-low">205</span> |

**Level-Up Moves**
- <a href="move-lookup.html?q=harden">Harden</a> (Lv Evo)

**Egg Moves**
- <a href="move-lookup.html?q=incompatible">Incompatible</a>
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
<div class="pokemon-tab-panel" id="pokemon-tabs-wurmple-panel-4">
Types: Bug / Poison • Egg Groups: Bug

<div class="pokemon-card">
<div class="card-column">
**Abilities**
- Unaware
- Compound Eyes *(Hidden)*

**Type Matchups**

*Resists / Immune to*
- Grass (0.25×)
- Fighting (0.25×)
- Poison (0.5×)
- Bug (0.5×)
- Fairy (0.5×)

*Weak to*
- Fire (2×)
- Flying (2×)
- Psychic (2×)
- Rock (2×)

**TM/HM Moves**
- <a href="move-lookup.html?q=toxic">TM06 - Toxic</a>
- <a href="move-lookup.html?q=sunny-day">TM11 - Sunny Day</a>
- <a href="move-lookup.html?q=light-screen">TM16 - Light Screen</a>
- <a href="move-lookup.html?q=protect">TM17 - Protect</a>
- <a href="move-lookup.html?q=giga-drain">TM19 - Giga Drain</a>
- <a href="move-lookup.html?q=solar-beam">TM22 - Solar Beam</a>
- <a href="move-lookup.html?q=psychic">TM29 - Psychic</a>
- <a href="move-lookup.html?q=shadow-ball">TM30 - Shadow Ball</a>
- <a href="move-lookup.html?q=double-team">TM32 - Double Team</a>
- <a href="move-lookup.html?q=sludge-bomb">TM36 - Sludge Bomb</a>
- <a href="move-lookup.html?q=aerial-ace">TM40 - Aerial Ace</a>
- <a href="move-lookup.html?q=facade">TM42 - Facade</a>
- <a href="move-lookup.html?q=rest">TM44 - Rest</a>
- <a href="move-lookup.html?q=attract">TM45 - Attract</a>
- <a href="move-lookup.html?q=thief">TM46 - Thief</a>
- <a href="move-lookup.html?q=roost">TM57 - Roost</a>
- <a href="move-lookup.html?q=flash">HM05 - Flash</a>

**Held Item**
Shed Shell

**Evolution Info**
Lv. 10

**Encounter Locations**
- Kipos Town — Grass (Night) (2%)
- Péntepetal City — Grass (Night) (20%)
</div>
<div class="card-column">
<label><input type="checkbox" class="caught-check" data-species="dustox" /> Caught</label>

**Base Stats**

| Stat | Value |
| --- | --- |
| HP | <span class="stat-value stat-mid">80</span> |
| Attack | <span class="stat-value stat-mid">65</span> |
| Defense | <span class="stat-value stat-mid">90</span> |
| Sp. Atk | <span class="stat-value stat-low">50</span> |
| Sp. Def | <span class="stat-value stat-mid">90</span> |
| Speed | <span class="stat-value stat-mid">65</span> |
| Total | <span class="stat-value stat-mid">440</span> |

**Level-Up Moves**
- <a href="move-lookup.html?q=gust">Gust</a> (Lv Evo)
- <a href="move-lookup.html?q=confusion">Confusion</a> (Lv 12)
- <a href="move-lookup.html?q=poison-powder">Poison Powder</a> (Lv 15)
- <a href="move-lookup.html?q=moonlight">Moonlight</a> (Lv 17)
- <a href="move-lookup.html?q=venoshock">Venoshock</a> (Lv 20)
- <a href="move-lookup.html?q=psybeam">Psybeam</a> (Lv 22)
- <a href="move-lookup.html?q=u-turn">U-Turn</a> (Lv 25)
- <a href="move-lookup.html?q=light-screen">Light Screen</a> (Lv 27)
- <a href="move-lookup.html?q=whirlwind">Whirlwind</a> (Lv 30)
- <a href="move-lookup.html?q=toxic">Toxic</a> (Lv 32)
- <a href="move-lookup.html?q=bug-buzz">Bug Buzz</a> (Lv 35)
- <a href="move-lookup.html?q=protect">Protect</a> (Lv 37)
- <a href="move-lookup.html?q=quiver-dance">Quiver Dance</a> (Lv 40)

**Egg Moves**
- <a href="move-lookup.html?q=incompatible">Incompatible</a>

**Tutor Moves**
- <a href="move-lookup.html?q=double-edge">Double-Edge</a>
- <a href="move-lookup.html?q=endure">Endure</a>
- <a href="move-lookup.html?q=sleep-talk">Sleep Talk</a>
- <a href="move-lookup.html?q=snore">Snore</a>
- <a href="move-lookup.html?q=swagger">Swagger</a>
- <a href="move-lookup.html?q=swift">Swift</a>
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
#pokemon-tabs-wurmple-tab-0:checked ~ .pokemon-tab-panels #pokemon-tabs-wurmple-panel-0 { display: block; }
#pokemon-tabs-wurmple-tab-1:checked ~ .pokemon-tab-panels #pokemon-tabs-wurmple-panel-1 { display: block; }
#pokemon-tabs-wurmple-tab-2:checked ~ .pokemon-tab-panels #pokemon-tabs-wurmple-panel-2 { display: block; }
#pokemon-tabs-wurmple-tab-3:checked ~ .pokemon-tab-panels #pokemon-tabs-wurmple-panel-3 { display: block; }
#pokemon-tabs-wurmple-tab-4:checked ~ .pokemon-tab-panels #pokemon-tabs-wurmple-panel-4 { display: block; }
</style>
</details>
