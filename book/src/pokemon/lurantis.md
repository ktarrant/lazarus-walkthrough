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
- TM06 - Toxic
- TM09 - Bullet Seed
- TM11 - Sunny Day
- TM17 - Protect
- TM19 - Giga Drain
- TM20 - Poison Jab
- TM22 - Solar Beam
- TM32 - Double Team
- TM42 - Facade
- TM44 - Rest
- TM45 - Attract

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
- Fury Cutter (Lv 1)
- Leafage (Lv 5)
- Razor Leaf (Lv 10)
- Growth (Lv 14)
- Ingrain (Lv 19)
- Leaf Blade (Lv 23)
- Bug Bite (Lv 25)
- Synthesis (Lv 28)
- Slash (Lv 32)
- Cross Poison (Lv 35)
- Sweet Scent (Lv 37)
- Solar Beam (Lv 41)
- Sunny Day (Lv 46)

**Egg Moves**
- Weather Ball
- Giga Drain
- Aromatherapy
- Defog
- Leaf Storm

**Tutor Moves**
- Endure
- Fury Cutter
- Sleep Talk
- Snore
- Swagger
- Swords Dance
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
- TM06 - Toxic
- TM09 - Bullet Seed
- TM11 - Sunny Day
- TM17 - Protect
- TM18 - Rain Dance
- TM19 - Giga Drain
- TM20 - Poison Jab
- TM22 - Solar Beam
- TM31 - Brick Break
- TM32 - Double Team
- TM40 - Aerial Ace
- TM42 - Facade
- TM44 - Rest
- TM45 - Attract

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
- Petal Blizzard (Lv Evo)
- X-Scissor (Lv 1)
- Night Slash (Lv 1)
- Fury Cutter (Lv 1)
- Leafage (Lv 5)
- Razor Leaf (Lv 10)
- Growth (Lv 14)
- Ingrain (Lv 19)
- Leaf Blade (Lv 23)
- Bug Bite (Lv 25)
- Synthesis (Lv 28)
- Slash (Lv 32)
- Cross Poison (Lv 35)
- Sweet Scent (Lv 37)
- Leech Life (Lv 42)
- Solar Blade (Lv 45)
- Sunny Day (Lv 47)

**Egg Moves**
- Weather Ball
- Giga Drain
- Aromatherapy
- Defog
- Leaf Storm

**Tutor Moves**
- Endure
- Fury Cutter
- Sleep Talk
- Snore
- Swagger
- Swords Dance
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
