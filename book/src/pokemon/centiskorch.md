<details class="pokemon-card-container">
<summary>Centiskorch (#225)</summary>
<div class="pokemon-tabs" id="pokemon-tabs-centiskorch">
<input type="radio" name="pokemon-tabs-centiskorch-group" id="pokemon-tabs-centiskorch-tab-0">
<label for="pokemon-tabs-centiskorch-tab-0">Sizzlipede</label>
<input type="radio" name="pokemon-tabs-centiskorch-group" id="pokemon-tabs-centiskorch-tab-1" checked>
<label for="pokemon-tabs-centiskorch-tab-1">Centiskorch</label>
<input type="radio" name="pokemon-tabs-centiskorch-group" id="pokemon-tabs-centiskorch-tab-2">
<label for="pokemon-tabs-centiskorch-tab-2">Mega Centiskorch</label>
<div class="pokemon-tab-panels">
<div class="pokemon-tab-panel" id="pokemon-tabs-centiskorch-panel-0">
Types: Fire / Bug • Egg Groups: Bug

<div class="pokemon-card">
<div class="card-column">
**Abilities**
- Flash Fire
- White Smoke
- Flame Body *(Hidden)*

**Type Matchups**

*Resists / Immune to*
- Grass (0.25×)
- Ice (0.5×)
- Fighting (0.5×)
- Bug (0.5×)
- Steel (0.5×)
- Fairy (0.5×)

*Weak to*
- Water (2×)
- Flying (2×)
- Rock (4×)

**TM/HM Moves**
- <a href="move-lookup.html?q=sunny-day">TM11 - Sunny Day</a>
- <a href="move-lookup.html?q=protect">TM17 - Protect</a>
- <a href="move-lookup.html?q=facade">TM42 - Facade</a>
- <a href="move-lookup.html?q=rest">TM44 - Rest</a>
- <a href="move-lookup.html?q=attract">TM45 - Attract</a>
- <a href="move-lookup.html?q=scald">TM56 - Scald</a>

**Encounter Locations**
- Marmaro Island — Grass (Day) (10%)
- Pythios Cemetery — Grass (Day) (10%)
- Pythios Cemetery — Grass (Night) (10%)
- Sea of Vulcai — Grass (Day) (10%)
</div>
<div class="card-column">
<label><input type="checkbox" class="caught-check" data-species="sizzlipede" /> Caught</label>

**Base Stats**

| Stat | Value |
| --- | --- |
| HP | <span class="stat-value stat-low">50</span> |
| Attack | <span class="stat-value stat-mid">65</span> |
| Defense | <span class="stat-value stat-low">45</span> |
| Sp. Atk | <span class="stat-value stat-low">50</span> |
| Sp. Def | <span class="stat-value stat-low">50</span> |
| Speed | <span class="stat-value stat-low">45</span> |
| Total | <span class="stat-value stat-low">305</span> |

**Level-Up Moves**
- <a href="move-lookup.html?q=ember">Ember</a> (Lv 1)
- <a href="move-lookup.html?q=smokescreen">Smokescreen</a> (Lv 1)
- <a href="move-lookup.html?q=wrap">Wrap</a> (Lv 5)
- <a href="move-lookup.html?q=bite">Bite</a> (Lv 10)
- <a href="move-lookup.html?q=flame-wheel">Flame Wheel</a> (Lv 15)
- <a href="move-lookup.html?q=bug-bite">Bug Bite</a> (Lv 20)
- <a href="move-lookup.html?q=coil">Coil</a> (Lv 23)
- <a href="move-lookup.html?q=slam">Slam</a> (Lv 27)
- <a href="move-lookup.html?q=fire-spin">Fire Spin</a> (Lv 30)
- <a href="move-lookup.html?q=crunch">Crunch</a> (Lv 34)
- <a href="move-lookup.html?q=leech-life">Leech Life</a> (Lv 37)
- <a href="move-lookup.html?q=fell-stinger">Fell Stinger</a> (Lv 41)
- <a href="move-lookup.html?q=fire-lash">Fire Lash</a> (Lv 45)
- <a href="move-lookup.html?q=lunge">Lunge</a> (Lv 48)
- <a href="move-lookup.html?q=electroweb">Electroweb</a> (Lv 53)
- <a href="move-lookup.html?q=burn-up">Burn Up</a> (Lv 55)

**Egg Moves**
- <a href="move-lookup.html?q=knock-off">Knock Off</a>
- <a href="move-lookup.html?q=struggle-bug">Struggle Bug</a>
- <a href="move-lookup.html?q=rollout">Rollout</a>
- <a href="move-lookup.html?q=defense-curl">Defense Curl</a>

**Tutor Moves**
- <a href="move-lookup.html?q=defense-curl">Defense Curl</a>
- <a href="move-lookup.html?q=endure">Endure</a>
- <a href="move-lookup.html?q=rollout">Rollout</a>
- <a href="move-lookup.html?q=sleep-talk">Sleep Talk</a>
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
<div class="pokemon-tab-panel" id="pokemon-tabs-centiskorch-panel-1">
Types: Fire / Bug • Egg Groups: Bug

<div class="pokemon-card">
<div class="card-column">
**Abilities**
- Flash Fire
- White Smoke
- Flame Body *(Hidden)*

**Type Matchups**

*Resists / Immune to*
- Grass (0.25×)
- Ice (0.5×)
- Fighting (0.5×)
- Bug (0.5×)
- Steel (0.5×)
- Fairy (0.5×)

*Weak to*
- Water (2×)
- Flying (2×)
- Rock (4×)

**TM/HM Moves**
- <a href="move-lookup.html?q=sunny-day">TM11 - Sunny Day</a>
- <a href="move-lookup.html?q=protect">TM17 - Protect</a>
- <a href="move-lookup.html?q=solar-beam">TM22 - Solar Beam</a>
- <a href="move-lookup.html?q=flamethrower">TM35 - Flamethrower</a>
- <a href="move-lookup.html?q=fire-blast">TM38 - Fire Blast</a>
- <a href="move-lookup.html?q=facade">TM42 - Facade</a>
- <a href="move-lookup.html?q=rest">TM44 - Rest</a>
- <a href="move-lookup.html?q=attract">TM45 - Attract</a>
- <a href="move-lookup.html?q=will-o-wisp">TM51 - Will-O-Wisp</a>
- <a href="move-lookup.html?q=scald">TM56 - Scald</a>

**Evolution Info**
Lv. 28
</div>
<div class="card-column">
<label><input type="checkbox" class="caught-check" data-species="centiskorch" /> Caught</label>

**Base Stats**

| Stat | Value |
| --- | --- |
| HP | <span class="stat-value stat-high">105</span> |
| Attack | <span class="stat-value stat-high">115</span> |
| Defense | <span class="stat-value stat-mid">70</span> |
| Sp. Atk | <span class="stat-value stat-mid">85</span> |
| Sp. Def | <span class="stat-value stat-mid">90</span> |
| Speed | <span class="stat-value stat-mid">60</span> |
| Total | <span class="stat-value stat-mid">525</span> |

**Level-Up Moves**
- <a href="move-lookup.html?q=fire-fang">Fire Fang</a> (Lv Evo)
- <a href="move-lookup.html?q=inferno">Inferno</a> (Lv 1)
- <a href="move-lookup.html?q=ember">Ember</a> (Lv 1)
- <a href="move-lookup.html?q=smokescreen">Smokescreen</a> (Lv 1)
- <a href="move-lookup.html?q=wrap">Wrap</a> (Lv 5)
- <a href="move-lookup.html?q=bite">Bite</a> (Lv 10)
- <a href="move-lookup.html?q=flame-wheel">Flame Wheel</a> (Lv 15)
- <a href="move-lookup.html?q=bug-bite">Bug Bite</a> (Lv 20)
- <a href="move-lookup.html?q=coil">Coil</a> (Lv 23)
- <a href="move-lookup.html?q=slam">Slam</a> (Lv 27)
- <a href="move-lookup.html?q=fire-spin">Fire Spin</a> (Lv 30)
- <a href="move-lookup.html?q=crunch">Crunch</a> (Lv 34)
- <a href="move-lookup.html?q=leech-life">Leech Life</a> (Lv 37)
- <a href="move-lookup.html?q=fell-stinger">Fell Stinger</a> (Lv 41)
- <a href="move-lookup.html?q=fire-lash">Fire Lash</a> (Lv 45)
- <a href="move-lookup.html?q=lunge">Lunge</a> (Lv 48)
- <a href="move-lookup.html?q=electroweb">Electroweb</a> (Lv 53)
- <a href="move-lookup.html?q=burn-up">Burn Up</a> (Lv 55)
- <a href="move-lookup.html?q=scald">Scald</a> (Lv 60)

**Egg Moves**
- <a href="move-lookup.html?q=knock-off">Knock Off</a>
- <a href="move-lookup.html?q=struggle-bug">Struggle Bug</a>
- <a href="move-lookup.html?q=rollout">Rollout</a>
- <a href="move-lookup.html?q=defense-curl">Defense Curl</a>

**Tutor Moves**
- <a href="move-lookup.html?q=defense-curl">Defense Curl</a>
- <a href="move-lookup.html?q=endure">Endure</a>
- <a href="move-lookup.html?q=rollout">Rollout</a>
- <a href="move-lookup.html?q=sleep-talk">Sleep Talk</a>
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
<div class="pokemon-tab-panel" id="pokemon-tabs-centiskorch-panel-2">
Types: Fire / Bug • Egg Groups: Bug

<div class="pokemon-card">
<div class="card-column">
**Abilities**
- Armor Tail

**Type Matchups**

*Resists / Immune to*
- Grass (0.25×)
- Ice (0.5×)
- Fighting (0.5×)
- Bug (0.5×)
- Steel (0.5×)
- Fairy (0.5×)

*Weak to*
- Water (2×)
- Flying (2×)
- Rock (4×)

**TM/HM Moves**
- <a href="move-lookup.html?q=sunny-day">TM11 - Sunny Day</a>
- <a href="move-lookup.html?q=protect">TM17 - Protect</a>
- <a href="move-lookup.html?q=solar-beam">TM22 - Solar Beam</a>
- <a href="move-lookup.html?q=flamethrower">TM35 - Flamethrower</a>
- <a href="move-lookup.html?q=fire-blast">TM38 - Fire Blast</a>
- <a href="move-lookup.html?q=facade">TM42 - Facade</a>
- <a href="move-lookup.html?q=rest">TM44 - Rest</a>
- <a href="move-lookup.html?q=attract">TM45 - Attract</a>
- <a href="move-lookup.html?q=will-o-wisp">TM51 - Will-O-Wisp</a>
- <a href="move-lookup.html?q=scald">TM56 - Scald</a>

**Evolution Info**
Centiskite
</div>
<div class="card-column">
<label><input type="checkbox" class="caught-check" data-species="mega-centiskorch" /> Caught</label>

**Base Stats**

| Stat | Value |
| --- | --- |
| HP | <span class="stat-value stat-high">105</span> |
| Attack | <span class="stat-value stat-high">145</span> |
| Defense | <span class="stat-value stat-mid">85</span> |
| Sp. Atk | <span class="stat-value stat-high">100</span> |
| Sp. Def | <span class="stat-value stat-high">100</span> |
| Speed | <span class="stat-value stat-mid">90</span> |
| Total | <span class="stat-value stat-high">625</span> |

**Level-Up Moves**
- <a href="move-lookup.html?q=fire-fang">Fire Fang</a> (Lv Evo)
- <a href="move-lookup.html?q=inferno">Inferno</a> (Lv 1)
- <a href="move-lookup.html?q=ember">Ember</a> (Lv 1)
- <a href="move-lookup.html?q=smokescreen">Smokescreen</a> (Lv 1)
- <a href="move-lookup.html?q=wrap">Wrap</a> (Lv 5)
- <a href="move-lookup.html?q=bite">Bite</a> (Lv 10)
- <a href="move-lookup.html?q=flame-wheel">Flame Wheel</a> (Lv 15)
- <a href="move-lookup.html?q=bug-bite">Bug Bite</a> (Lv 20)
- <a href="move-lookup.html?q=coil">Coil</a> (Lv 23)
- <a href="move-lookup.html?q=slam">Slam</a> (Lv 27)
- <a href="move-lookup.html?q=fire-spin">Fire Spin</a> (Lv 30)
- <a href="move-lookup.html?q=crunch">Crunch</a> (Lv 34)
- <a href="move-lookup.html?q=leech-life">Leech Life</a> (Lv 37)
- <a href="move-lookup.html?q=fell-stinger">Fell Stinger</a> (Lv 41)
- <a href="move-lookup.html?q=fire-lash">Fire Lash</a> (Lv 45)
- <a href="move-lookup.html?q=lunge">Lunge</a> (Lv 48)
- <a href="move-lookup.html?q=electroweb">Electroweb</a> (Lv 53)
- <a href="move-lookup.html?q=burn-up">Burn Up</a> (Lv 55)
- <a href="move-lookup.html?q=scald">Scald</a> (Lv 60)

**Egg Moves**
- <a href="move-lookup.html?q=knock-off">Knock Off</a>
- <a href="move-lookup.html?q=struggle-bug">Struggle Bug</a>
- <a href="move-lookup.html?q=rollout">Rollout</a>
- <a href="move-lookup.html?q=defense-curl">Defense Curl</a>

**Tutor Moves**
- <a href="move-lookup.html?q=defense-curl">Defense Curl</a>
- <a href="move-lookup.html?q=endure">Endure</a>
- <a href="move-lookup.html?q=rollout">Rollout</a>
- <a href="move-lookup.html?q=sleep-talk">Sleep Talk</a>
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
</div>
</div>
<style>
#pokemon-tabs-centiskorch-tab-0:checked ~ .pokemon-tab-panels #pokemon-tabs-centiskorch-panel-0 { display: block; }
#pokemon-tabs-centiskorch-tab-1:checked ~ .pokemon-tab-panels #pokemon-tabs-centiskorch-panel-1 { display: block; }
#pokemon-tabs-centiskorch-tab-2:checked ~ .pokemon-tab-panels #pokemon-tabs-centiskorch-panel-2 { display: block; }
</style>
</details>
