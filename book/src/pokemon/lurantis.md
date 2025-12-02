<details class="pokemon-card-container">
<summary>Lurantis (#345)</summary>
<div class="pokemon-tabs" id="pokemon-tabs-lurantis">
<input type="radio" name="pokemon-tabs-lurantis-group" id="pokemon-tabs-lurantis-tab-0">
<label for="pokemon-tabs-lurantis-tab-0">Fomantis</label>
<input type="radio" name="pokemon-tabs-lurantis-group" id="pokemon-tabs-lurantis-tab-1" checked>
<label for="pokemon-tabs-lurantis-tab-1">Lurantis</label>
<div class="pokemon-tab-panels">
<div class="pokemon-tab-panel" id="pokemon-tabs-lurantis-panel-0">
Types: Grass • Egg Groups: Grass

<div class="pokemon-card">
<div class="card-column">
**Abilities**
- Leaf Guard
- Contrary
- Sharpness *(Hidden)*

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
- <a href="move-lookup.html?q=tm06-toxic">TM06 - Toxic</a>
- <a href="move-lookup.html?q=tm09-bullet-seed">TM09 - Bullet Seed</a>
- <a href="move-lookup.html?q=tm11-sunny-day">TM11 - Sunny Day</a>
- <a href="move-lookup.html?q=tm17-protect">TM17 - Protect</a>
- <a href="move-lookup.html?q=tm19-giga-drain">TM19 - Giga Drain</a>
- <a href="move-lookup.html?q=tm20-poison-jab">TM20 - Poison Jab</a>
- <a href="move-lookup.html?q=tm22-solar-beam">TM22 - Solar Beam</a>
- <a href="move-lookup.html?q=tm32-double-team">TM32 - Double Team</a>
- <a href="move-lookup.html?q=tm42-facade">TM42 - Facade</a>
- <a href="move-lookup.html?q=tm44-rest">TM44 - Rest</a>
- <a href="move-lookup.html?q=tm45-attract">TM45 - Attract</a>

**Held Item**
Miracle Seed

**Encounter Locations**
- Bronze Fields (South) — Grass (Day) (10%)
- Erinys Path (East) — Grass (Night) (20%)
- Kaptara Island (West) — Grass (Day) (20%)
- Kaptara Island (West) — Grass (Night) (20%)
- Sea of Asteri (West) — Grass (Day) (20%)
- Sea of Asteri (West) — Grass (Night) (20%)
</div>
<div class="card-column">
<label><input type="checkbox" class="caught-check" data-species="fomantis" /> Caught</label>

**Base Stats**

| Stat | Value |
| --- | --- |
| HP | <span class="stat-value stat-low">50</span> |
| Attack | <span class="stat-value stat-mid">55</span> |
| Defense | <span class="stat-value stat-low">40</span> |
| Sp. Atk | <span class="stat-value stat-low">50</span> |
| Sp. Def | <span class="stat-value stat-low">40</span> |
| Speed | <span class="stat-value stat-low">35</span> |
| Total | <span class="stat-value stat-low">270</span> |

**Level-Up Moves**
- <a href="move-lookup.html?q=fury-cutter">Fury Cutter</a> (Lv 1)
- <a href="move-lookup.html?q=leafage">Leafage</a> (Lv 5)
- <a href="move-lookup.html?q=razor-leaf">Razor Leaf</a> (Lv 10)
- <a href="move-lookup.html?q=growth">Growth</a> (Lv 14)
- <a href="move-lookup.html?q=ingrain">Ingrain</a> (Lv 19)
- <a href="move-lookup.html?q=leaf-blade">Leaf Blade</a> (Lv 23)
- <a href="move-lookup.html?q=bug-bite">Bug Bite</a> (Lv 25)
- <a href="move-lookup.html?q=synthesis">Synthesis</a> (Lv 28)
- <a href="move-lookup.html?q=slash">Slash</a> (Lv 32)
- <a href="move-lookup.html?q=cross-poison">Cross Poison</a> (Lv 35)
- <a href="move-lookup.html?q=sweet-scent">Sweet Scent</a> (Lv 37)
- <a href="move-lookup.html?q=solar-beam">Solar Beam</a> (Lv 41)
- <a href="move-lookup.html?q=sunny-day">Sunny Day</a> (Lv 46)

**Egg Moves**
- <a href="move-lookup.html?q=weather-ball">Weather Ball</a>
- <a href="move-lookup.html?q=giga-drain">Giga Drain</a>
- <a href="move-lookup.html?q=aromatherapy">Aromatherapy</a>
- <a href="move-lookup.html?q=defog">Defog</a>
- <a href="move-lookup.html?q=leaf-storm">Leaf Storm</a>

**Tutor Moves**
- <a href="move-lookup.html?q=endure">Endure</a>
- <a href="move-lookup.html?q=fury-cutter">Fury Cutter</a>
- <a href="move-lookup.html?q=sleep-talk">Sleep Talk</a>
- <a href="move-lookup.html?q=snore">Snore</a>
- <a href="move-lookup.html?q=swagger">Swagger</a>
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
<div class="pokemon-tab-panel" id="pokemon-tabs-lurantis-panel-1">
Types: Grass • Egg Groups: Grass

<div class="pokemon-card">
<div class="card-column">
**Abilities**
- Leaf Guard
- Contrary
- Sharpness *(Hidden)*

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
- <a href="move-lookup.html?q=tm06-toxic">TM06 - Toxic</a>
- <a href="move-lookup.html?q=tm09-bullet-seed">TM09 - Bullet Seed</a>
- <a href="move-lookup.html?q=tm11-sunny-day">TM11 - Sunny Day</a>
- <a href="move-lookup.html?q=tm17-protect">TM17 - Protect</a>
- <a href="move-lookup.html?q=tm18-rain-dance">TM18 - Rain Dance</a>
- <a href="move-lookup.html?q=tm19-giga-drain">TM19 - Giga Drain</a>
- <a href="move-lookup.html?q=tm20-poison-jab">TM20 - Poison Jab</a>
- <a href="move-lookup.html?q=tm22-solar-beam">TM22 - Solar Beam</a>
- <a href="move-lookup.html?q=tm31-brick-break">TM31 - Brick Break</a>
- <a href="move-lookup.html?q=tm32-double-team">TM32 - Double Team</a>
- <a href="move-lookup.html?q=tm40-aerial-ace">TM40 - Aerial Ace</a>
- <a href="move-lookup.html?q=tm42-facade">TM42 - Facade</a>
- <a href="move-lookup.html?q=tm44-rest">TM44 - Rest</a>
- <a href="move-lookup.html?q=tm45-attract">TM45 - Attract</a>

**Held Item**
Miracle Seed

**Evolution Info**
Lv. 29, Day

**Encounter Locations**
- Areios Hideout — Grass (Day) (8%)
- Kaptara Island (West) — Grass (Day) (5%)
- Kaptara Island (West) — Grass (Night) (5%)
- Nyx Trails — Grass (Day) (5%)
</div>
<div class="card-column">
<label><input type="checkbox" class="caught-check" data-species="lurantis" /> Caught</label>

**Base Stats**

| Stat | Value |
| --- | --- |
| HP | <span class="stat-value stat-mid">80</span> |
| Attack | <span class="stat-value stat-high">105</span> |
| Defense | <span class="stat-value stat-high">95</span> |
| Sp. Atk | <span class="stat-value stat-mid">80</span> |
| Sp. Def | <span class="stat-value stat-high">95</span> |
| Speed | <span class="stat-value stat-low">45</span> |
| Total | <span class="stat-value stat-mid">500</span> |

**Level-Up Moves**
- <a href="move-lookup.html?q=petal-blizzard">Petal Blizzard</a> (Lv Evo)
- <a href="move-lookup.html?q=x-scissor">X-Scissor</a> (Lv 1)
- <a href="move-lookup.html?q=night-slash">Night Slash</a> (Lv 1)
- <a href="move-lookup.html?q=fury-cutter">Fury Cutter</a> (Lv 1)
- <a href="move-lookup.html?q=leafage">Leafage</a> (Lv 5)
- <a href="move-lookup.html?q=razor-leaf">Razor Leaf</a> (Lv 10)
- <a href="move-lookup.html?q=growth">Growth</a> (Lv 14)
- <a href="move-lookup.html?q=ingrain">Ingrain</a> (Lv 19)
- <a href="move-lookup.html?q=leaf-blade">Leaf Blade</a> (Lv 23)
- <a href="move-lookup.html?q=bug-bite">Bug Bite</a> (Lv 25)
- <a href="move-lookup.html?q=synthesis">Synthesis</a> (Lv 28)
- <a href="move-lookup.html?q=slash">Slash</a> (Lv 32)
- <a href="move-lookup.html?q=cross-poison">Cross Poison</a> (Lv 35)
- <a href="move-lookup.html?q=sweet-scent">Sweet Scent</a> (Lv 37)
- <a href="move-lookup.html?q=leech-life">Leech Life</a> (Lv 42)
- <a href="move-lookup.html?q=solar-blade">Solar Blade</a> (Lv 45)
- <a href="move-lookup.html?q=sunny-day">Sunny Day</a> (Lv 47)

**Egg Moves**
- <a href="move-lookup.html?q=weather-ball">Weather Ball</a>
- <a href="move-lookup.html?q=giga-drain">Giga Drain</a>
- <a href="move-lookup.html?q=aromatherapy">Aromatherapy</a>
- <a href="move-lookup.html?q=defog">Defog</a>
- <a href="move-lookup.html?q=leaf-storm">Leaf Storm</a>

**Tutor Moves**
- <a href="move-lookup.html?q=endure">Endure</a>
- <a href="move-lookup.html?q=fury-cutter">Fury Cutter</a>
- <a href="move-lookup.html?q=sleep-talk">Sleep Talk</a>
- <a href="move-lookup.html?q=snore">Snore</a>
- <a href="move-lookup.html?q=swagger">Swagger</a>
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
#pokemon-tabs-lurantis-tab-0:checked ~ .pokemon-tab-panels #pokemon-tabs-lurantis-panel-0 { display: block; }
#pokemon-tabs-lurantis-tab-1:checked ~ .pokemon-tab-panels #pokemon-tabs-lurantis-panel-1 { display: block; }
</style>
</details>
