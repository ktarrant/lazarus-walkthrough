<details class="pokemon-card-container">
<summary>Ogerpon (#430)</summary>
<div class="pokemon-tabs" id="pokemon-tabs-ogerpon">
<input type="radio" name="pokemon-tabs-ogerpon-group" id="pokemon-tabs-ogerpon-tab-0" checked>
<label for="pokemon-tabs-ogerpon-tab-0">Ogerpon</label>
<input type="radio" name="pokemon-tabs-ogerpon-group" id="pokemon-tabs-ogerpon-tab-1">
<label for="pokemon-tabs-ogerpon-tab-1">Ogerpon Cornerstone</label>
<input type="radio" name="pokemon-tabs-ogerpon-group" id="pokemon-tabs-ogerpon-tab-2">
<label for="pokemon-tabs-ogerpon-tab-2">Ogerpon Hearthflame</label>
<input type="radio" name="pokemon-tabs-ogerpon-group" id="pokemon-tabs-ogerpon-tab-3">
<label for="pokemon-tabs-ogerpon-tab-3">Ogerpon Wellspring</label>
<div class="pokemon-tab-panels">
<div class="pokemon-tab-panel" id="pokemon-tabs-ogerpon-panel-0">
Types: Grass • Egg Groups: -

<div class="pokemon-card">
<div class="card-column">
**Abilities**
- Defiant
- Embody Aspect (Spe) *(Hidden)*

**Type Matchups**

*Resists / Immune to*
- Water (0.5×)
- Electric (0.5×)
- Grass (0.5×)
- Ground (0.5×)

*Weak to*
- Fire (2×)
- Ice (2×)
- Poison (2×)
- Flying (2×)
- Bug (2×)

**TM/HM Moves**
- <a href="move-lookup.html?q=bullet-seed">TM09 - Bullet Seed</a>
- <a href="move-lookup.html?q=sunny-day">TM11 - Sunny Day</a>
- <a href="move-lookup.html?q=taunt">TM12 - Taunt</a>
- <a href="move-lookup.html?q=protect">TM17 - Protect</a>
- <a href="move-lookup.html?q=rain-dance">TM18 - Rain Dance</a>
- <a href="move-lookup.html?q=giga-drain">TM19 - Giga Drain</a>
- <a href="move-lookup.html?q=solar-beam">TM22 - Solar Beam</a>
- <a href="move-lookup.html?q=brick-break">TM31 - Brick Break</a>
- <a href="move-lookup.html?q=sandstorm">TM37 - Sandstorm</a>
- <a href="move-lookup.html?q=rock-tomb">TM39 - Rock Tomb</a>
- <a href="move-lookup.html?q=facade">TM42 - Facade</a>
- <a href="move-lookup.html?q=rest">TM44 - Rest</a>
</div>
<div class="card-column">
<label><input type="checkbox" class="caught-check" data-species="ogerpon" /> Caught</label>

**Base Stats**

| Stat | Value |
| --- | --- |
| HP | <span class="stat-value stat-mid">80</span> |
| Attack | <span class="stat-value stat-high">120</span> |
| Defense | <span class="stat-value stat-mid">84</span> |
| Sp. Atk | <span class="stat-value stat-mid">60</span> |
| Sp. Def | <span class="stat-value stat-high">96</span> |
| Speed | <span class="stat-value stat-high">110</span> |
| Total | <span class="stat-value stat-high">550</span> |

**Level-Up Moves**
- <a href="move-lookup.html?q=double-kick">Double Kick</a> (Lv 1)
- <a href="move-lookup.html?q=counter">Counter</a> (Lv 1)
- <a href="move-lookup.html?q=retaliate">Retaliate</a> (Lv 1)
- <a href="move-lookup.html?q=horn-leech">Horn Leech</a> (Lv 1)
- <a href="move-lookup.html?q=vine-whip">Vine Whip</a> (Lv 1)
- <a href="move-lookup.html?q=leech-seed">Leech Seed</a> (Lv 1)
- <a href="move-lookup.html?q=quick-attack">Quick Attack</a> (Lv 1)
- <a href="move-lookup.html?q=follow-me">Follow Me</a> (Lv 1)
- <a href="move-lookup.html?q=focus-energy">Focus Energy</a> (Lv 6)
- <a href="move-lookup.html?q=growth">Growth</a> (Lv 12)
- <a href="move-lookup.html?q=slam">Slam</a> (Lv 18)
- <a href="move-lookup.html?q=low-sweep">Low Sweep</a> (Lv 24)
- <a href="move-lookup.html?q=ivy-cudgel">Ivy Cudgel</a> (Lv 30)
- <a href="move-lookup.html?q=throat-chop">Throat Chop</a> (Lv 36)
- <a href="move-lookup.html?q=synthesis">Synthesis</a> (Lv 42)
- <a href="move-lookup.html?q=spiky-shield">Spiky Shield</a> (Lv 48)
- <a href="move-lookup.html?q=power-whip">Power Whip</a> (Lv 54)
- <a href="move-lookup.html?q=superpower">Superpower</a> (Lv 60)
- <a href="move-lookup.html?q=wood-hammer">Wood Hammer</a> (Lv 66)

**Egg Moves**
- <a href="move-lookup.html?q=incompatible">Incompatible</a>

**Tutor Moves**
- <a href="move-lookup.html?q=counter">Counter</a>
- <a href="move-lookup.html?q=endure">Endure</a>
- <a href="move-lookup.html?q=sleep-talk">Sleep Talk</a>
- <a href="move-lookup.html?q=swords-dance">Swords Dance</a>
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
<div class="pokemon-tab-panel" id="pokemon-tabs-ogerpon-panel-1">
Types: Grass / Rock • Egg Groups: -

<div class="pokemon-card">
<div class="card-column">
**Abilities**
- Sturdy
- Embody Aspect (Def) *(Hidden)*

**Type Matchups**

*Resists / Immune to*
- Normal (0.5×)
- Electric (0.5×)

*Weak to*
- Ice (2×)
- Fighting (2×)
- Bug (2×)
- Steel (2×)

**TM/HM Moves**
- <a href="move-lookup.html?q=bullet-seed">TM09 - Bullet Seed</a>
- <a href="move-lookup.html?q=sunny-day">TM11 - Sunny Day</a>
- <a href="move-lookup.html?q=taunt">TM12 - Taunt</a>
- <a href="move-lookup.html?q=protect">TM17 - Protect</a>
- <a href="move-lookup.html?q=rain-dance">TM18 - Rain Dance</a>
- <a href="move-lookup.html?q=giga-drain">TM19 - Giga Drain</a>
- <a href="move-lookup.html?q=solar-beam">TM22 - Solar Beam</a>
- <a href="move-lookup.html?q=brick-break">TM31 - Brick Break</a>
- <a href="move-lookup.html?q=sandstorm">TM37 - Sandstorm</a>
- <a href="move-lookup.html?q=rock-tomb">TM39 - Rock Tomb</a>
- <a href="move-lookup.html?q=facade">TM42 - Facade</a>
- <a href="move-lookup.html?q=rest">TM44 - Rest</a>
</div>
<div class="card-column">
<label><input type="checkbox" class="caught-check" data-species="ogerpon-cornerstone" /> Caught</label>

**Base Stats**

| Stat | Value |
| --- | --- |
| HP | <span class="stat-value stat-mid">80</span> |
| Attack | <span class="stat-value stat-high">120</span> |
| Defense | <span class="stat-value stat-mid">84</span> |
| Sp. Atk | <span class="stat-value stat-mid">60</span> |
| Sp. Def | <span class="stat-value stat-high">96</span> |
| Speed | <span class="stat-value stat-high">110</span> |
| Total | <span class="stat-value stat-high">550</span> |

**Level-Up Moves**
- <a href="move-lookup.html?q=double-kick">Double Kick</a> (Lv 1)
- <a href="move-lookup.html?q=counter">Counter</a> (Lv 1)
- <a href="move-lookup.html?q=retaliate">Retaliate</a> (Lv 1)
- <a href="move-lookup.html?q=horn-leech">Horn Leech</a> (Lv 1)
- <a href="move-lookup.html?q=vine-whip">Vine Whip</a> (Lv 1)
- <a href="move-lookup.html?q=leech-seed">Leech Seed</a> (Lv 1)
- <a href="move-lookup.html?q=quick-attack">Quick Attack</a> (Lv 1)
- <a href="move-lookup.html?q=follow-me">Follow Me</a> (Lv 1)
- <a href="move-lookup.html?q=focus-energy">Focus Energy</a> (Lv 6)
- <a href="move-lookup.html?q=growth">Growth</a> (Lv 12)
- <a href="move-lookup.html?q=slam">Slam</a> (Lv 18)
- <a href="move-lookup.html?q=low-sweep">Low Sweep</a> (Lv 24)
- <a href="move-lookup.html?q=ivy-cudgel">Ivy Cudgel</a> (Lv 30)
- <a href="move-lookup.html?q=throat-chop">Throat Chop</a> (Lv 36)
- <a href="move-lookup.html?q=synthesis">Synthesis</a> (Lv 42)
- <a href="move-lookup.html?q=spiky-shield">Spiky Shield</a> (Lv 48)
- <a href="move-lookup.html?q=power-whip">Power Whip</a> (Lv 54)
- <a href="move-lookup.html?q=superpower">Superpower</a> (Lv 60)
- <a href="move-lookup.html?q=wood-hammer">Wood Hammer</a> (Lv 66)

**Egg Moves**
- <a href="move-lookup.html?q=incompatible">Incompatible</a>

**Tutor Moves**
- <a href="move-lookup.html?q=counter">Counter</a>
- <a href="move-lookup.html?q=endure">Endure</a>
- <a href="move-lookup.html?q=sleep-talk">Sleep Talk</a>
- <a href="move-lookup.html?q=swords-dance">Swords Dance</a>
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
<div class="pokemon-tab-panel" id="pokemon-tabs-ogerpon-panel-2">
Types: Grass / Fire • Egg Groups: -

<div class="pokemon-card">
<div class="card-column">
**Abilities**
- Mold Breaker
- Embody Aspect (Atk) *(Hidden)*

**Type Matchups**

*Resists / Immune to*
- Electric (0.5×)
- Grass (0.25×)
- Steel (0.5×)
- Fairy (0.5×)

*Weak to*
- Poison (2×)
- Flying (2×)
- Rock (2×)

**TM/HM Moves**
- <a href="move-lookup.html?q=bullet-seed">TM09 - Bullet Seed</a>
- <a href="move-lookup.html?q=sunny-day">TM11 - Sunny Day</a>
- <a href="move-lookup.html?q=taunt">TM12 - Taunt</a>
- <a href="move-lookup.html?q=protect">TM17 - Protect</a>
- <a href="move-lookup.html?q=rain-dance">TM18 - Rain Dance</a>
- <a href="move-lookup.html?q=giga-drain">TM19 - Giga Drain</a>
- <a href="move-lookup.html?q=solar-beam">TM22 - Solar Beam</a>
- <a href="move-lookup.html?q=brick-break">TM31 - Brick Break</a>
- <a href="move-lookup.html?q=sandstorm">TM37 - Sandstorm</a>
- <a href="move-lookup.html?q=rock-tomb">TM39 - Rock Tomb</a>
- <a href="move-lookup.html?q=facade">TM42 - Facade</a>
- <a href="move-lookup.html?q=rest">TM44 - Rest</a>
</div>
<div class="card-column">
<label><input type="checkbox" class="caught-check" data-species="ogerpon-hearthflame" /> Caught</label>

**Base Stats**

| Stat | Value |
| --- | --- |
| HP | <span class="stat-value stat-mid">80</span> |
| Attack | <span class="stat-value stat-high">120</span> |
| Defense | <span class="stat-value stat-mid">84</span> |
| Sp. Atk | <span class="stat-value stat-mid">60</span> |
| Sp. Def | <span class="stat-value stat-high">96</span> |
| Speed | <span class="stat-value stat-high">110</span> |
| Total | <span class="stat-value stat-high">550</span> |

**Level-Up Moves**
- <a href="move-lookup.html?q=double-kick">Double Kick</a> (Lv 1)
- <a href="move-lookup.html?q=counter">Counter</a> (Lv 1)
- <a href="move-lookup.html?q=retaliate">Retaliate</a> (Lv 1)
- <a href="move-lookup.html?q=horn-leech">Horn Leech</a> (Lv 1)
- <a href="move-lookup.html?q=vine-whip">Vine Whip</a> (Lv 1)
- <a href="move-lookup.html?q=leech-seed">Leech Seed</a> (Lv 1)
- <a href="move-lookup.html?q=quick-attack">Quick Attack</a> (Lv 1)
- <a href="move-lookup.html?q=follow-me">Follow Me</a> (Lv 1)
- <a href="move-lookup.html?q=focus-energy">Focus Energy</a> (Lv 6)
- <a href="move-lookup.html?q=growth">Growth</a> (Lv 12)
- <a href="move-lookup.html?q=slam">Slam</a> (Lv 18)
- <a href="move-lookup.html?q=low-sweep">Low Sweep</a> (Lv 24)
- <a href="move-lookup.html?q=ivy-cudgel">Ivy Cudgel</a> (Lv 30)
- <a href="move-lookup.html?q=throat-chop">Throat Chop</a> (Lv 36)
- <a href="move-lookup.html?q=synthesis">Synthesis</a> (Lv 42)
- <a href="move-lookup.html?q=spiky-shield">Spiky Shield</a> (Lv 48)
- <a href="move-lookup.html?q=power-whip">Power Whip</a> (Lv 54)
- <a href="move-lookup.html?q=superpower">Superpower</a> (Lv 60)
- <a href="move-lookup.html?q=wood-hammer">Wood Hammer</a> (Lv 66)

**Egg Moves**
- <a href="move-lookup.html?q=incompatible">Incompatible</a>

**Tutor Moves**
- <a href="move-lookup.html?q=counter">Counter</a>
- <a href="move-lookup.html?q=endure">Endure</a>
- <a href="move-lookup.html?q=sleep-talk">Sleep Talk</a>
- <a href="move-lookup.html?q=swords-dance">Swords Dance</a>
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
<div class="pokemon-tab-panel" id="pokemon-tabs-ogerpon-panel-3">
Types: Grass / Water • Egg Groups: -

<div class="pokemon-card">
<div class="card-column">
**Abilities**
- Water Absorb
- Embody Aspect (SpD) *(Hidden)*

**Type Matchups**

*Resists / Immune to*
- Water (0.25×)
- Ground (0.5×)
- Steel (0.5×)

*Weak to*
- Poison (2×)
- Flying (2×)
- Bug (2×)

**TM/HM Moves**
- <a href="move-lookup.html?q=bullet-seed">TM09 - Bullet Seed</a>
- <a href="move-lookup.html?q=sunny-day">TM11 - Sunny Day</a>
- <a href="move-lookup.html?q=taunt">TM12 - Taunt</a>
- <a href="move-lookup.html?q=protect">TM17 - Protect</a>
- <a href="move-lookup.html?q=rain-dance">TM18 - Rain Dance</a>
- <a href="move-lookup.html?q=giga-drain">TM19 - Giga Drain</a>
- <a href="move-lookup.html?q=solar-beam">TM22 - Solar Beam</a>
- <a href="move-lookup.html?q=brick-break">TM31 - Brick Break</a>
- <a href="move-lookup.html?q=sandstorm">TM37 - Sandstorm</a>
- <a href="move-lookup.html?q=rock-tomb">TM39 - Rock Tomb</a>
- <a href="move-lookup.html?q=facade">TM42 - Facade</a>
- <a href="move-lookup.html?q=rest">TM44 - Rest</a>
</div>
<div class="card-column">
<label><input type="checkbox" class="caught-check" data-species="ogerpon-wellspring" /> Caught</label>

**Base Stats**

| Stat | Value |
| --- | --- |
| HP | <span class="stat-value stat-mid">80</span> |
| Attack | <span class="stat-value stat-high">120</span> |
| Defense | <span class="stat-value stat-mid">84</span> |
| Sp. Atk | <span class="stat-value stat-mid">60</span> |
| Sp. Def | <span class="stat-value stat-high">96</span> |
| Speed | <span class="stat-value stat-high">110</span> |
| Total | <span class="stat-value stat-high">550</span> |

**Level-Up Moves**
- <a href="move-lookup.html?q=double-kick">Double Kick</a> (Lv 1)
- <a href="move-lookup.html?q=counter">Counter</a> (Lv 1)
- <a href="move-lookup.html?q=retaliate">Retaliate</a> (Lv 1)
- <a href="move-lookup.html?q=horn-leech">Horn Leech</a> (Lv 1)
- <a href="move-lookup.html?q=vine-whip">Vine Whip</a> (Lv 1)
- <a href="move-lookup.html?q=leech-seed">Leech Seed</a> (Lv 1)
- <a href="move-lookup.html?q=quick-attack">Quick Attack</a> (Lv 1)
- <a href="move-lookup.html?q=follow-me">Follow Me</a> (Lv 1)
- <a href="move-lookup.html?q=focus-energy">Focus Energy</a> (Lv 6)
- <a href="move-lookup.html?q=growth">Growth</a> (Lv 12)
- <a href="move-lookup.html?q=slam">Slam</a> (Lv 18)
- <a href="move-lookup.html?q=low-sweep">Low Sweep</a> (Lv 24)
- <a href="move-lookup.html?q=ivy-cudgel">Ivy Cudgel</a> (Lv 30)
- <a href="move-lookup.html?q=throat-chop">Throat Chop</a> (Lv 36)
- <a href="move-lookup.html?q=synthesis">Synthesis</a> (Lv 42)
- <a href="move-lookup.html?q=spiky-shield">Spiky Shield</a> (Lv 48)
- <a href="move-lookup.html?q=power-whip">Power Whip</a> (Lv 54)
- <a href="move-lookup.html?q=superpower">Superpower</a> (Lv 60)
- <a href="move-lookup.html?q=wood-hammer">Wood Hammer</a> (Lv 66)

**Egg Moves**
- <a href="move-lookup.html?q=incompatible">Incompatible</a>

**Tutor Moves**
- <a href="move-lookup.html?q=counter">Counter</a>
- <a href="move-lookup.html?q=endure">Endure</a>
- <a href="move-lookup.html?q=sleep-talk">Sleep Talk</a>
- <a href="move-lookup.html?q=swords-dance">Swords Dance</a>
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
#pokemon-tabs-ogerpon-tab-0:checked ~ .pokemon-tab-panels #pokemon-tabs-ogerpon-panel-0 { display: block; }
#pokemon-tabs-ogerpon-tab-1:checked ~ .pokemon-tab-panels #pokemon-tabs-ogerpon-panel-1 { display: block; }
#pokemon-tabs-ogerpon-tab-2:checked ~ .pokemon-tab-panels #pokemon-tabs-ogerpon-panel-2 { display: block; }
#pokemon-tabs-ogerpon-tab-3:checked ~ .pokemon-tab-panels #pokemon-tabs-ogerpon-panel-3 { display: block; }
</style>
</details>
