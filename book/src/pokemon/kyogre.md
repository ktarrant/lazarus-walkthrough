<details class="pokemon-card-container">
<summary>Kyogre (#427)</summary>
<div class="pokemon-tabs" id="pokemon-tabs-kyogre">
<input type="radio" name="pokemon-tabs-kyogre-group" id="pokemon-tabs-kyogre-tab-0" checked>
<label for="pokemon-tabs-kyogre-tab-0">Kyogre</label>
<input type="radio" name="pokemon-tabs-kyogre-group" id="pokemon-tabs-kyogre-tab-1">
<label for="pokemon-tabs-kyogre-tab-1">Primal Kyogre</label>
<div class="pokemon-tab-panels">
<div class="pokemon-tab-panel" id="pokemon-tabs-kyogre-panel-0">
Types: Water • Egg Groups: -

<div class="pokemon-card">
<div class="card-column">
**Abilities**
- Drizzle

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
- TM04 - Calm Mind
- TM06 - Toxic
- TM07 - Whirlpool
- TM13 - Ice Beam
- TM14 - Blizzard
- TM17 - Protect
- TM18 - Rain Dance
- TM24 - Thunderbolt
- TM25 - Thunder
- TM26 - Earthquake
- TM31 - Brick Break
- TM32 - Double Team
- TM34 - Shock Wave
- TM39 - Rock Tomb
- TM42 - Facade
- TM44 - Rest
- TM49 - Bulldoze
- TM56 - Scald
- TM58 - Thunder Wave
- HM03 - Surf
- HM04 - Strength
- HM06 - Rock Smash
- HM07 - Waterfall
- HM08 - Dive
</div>
<div class="card-column">
<label><input type="checkbox" class="caught-check" data-species="kyogre" /> Caught</label>

**Base Stats**

| Stat | Value |
| --- | --- |
| HP | <span class="stat-value stat-high">100</span> |
| Attack | <span class="stat-value stat-high">100</span> |
| Defense | <span class="stat-value stat-mid">90</span> |
| Sp. Atk | <span class="stat-value stat-high">150</span> |
| Sp. Def | <span class="stat-value stat-high">140</span> |
| Speed | <span class="stat-value stat-mid">90</span> |
| Total | <span class="stat-value stat-high">670</span> |

**Level-Up Moves**
- Ancient Power (Lv 1)
- Water Pulse (Lv 1)
- Scary Face (Lv 5)
- Aqua Tail (Lv 15)
- Body Slam (Lv 20)
- Aqua Ring (Lv 30)
- Ice Beam (Lv 35)
- Origin Pulse (Lv 45)
- Calm Mind (Lv 50)
- Muddy Water (Lv 60)
- Sheer Cold (Lv 65)
- Hydro Pump (Lv 75)
- Double-Edge (Lv 80)
- Water Spout (Lv 90)

**Egg Moves**
- Incompatible

**Tutor Moves**
- Body Slam
- Defense Curl
- Double-Edge
- Endure
- Icy Wind
- Mud-Slap
- Psych Up
- Rock Slide
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
<div class="pokemon-tab-panel" id="pokemon-tabs-kyogre-panel-1">
Types: Water • Egg Groups: -

<div class="pokemon-card">
<div class="card-column">
**Abilities**
- Primordial Sea

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
- TM04 - Calm Mind
- TM06 - Toxic
- TM07 - Whirlpool
- TM13 - Ice Beam
- TM14 - Blizzard
- TM17 - Protect
- TM18 - Rain Dance
- TM24 - Thunderbolt
- TM25 - Thunder
- TM26 - Earthquake
- TM31 - Brick Break
- TM32 - Double Team
- TM34 - Shock Wave
- TM39 - Rock Tomb
- TM42 - Facade
- TM44 - Rest
- TM49 - Bulldoze
- TM56 - Scald
- TM58 - Thunder Wave
- HM03 - Surf
- HM04 - Strength
- HM06 - Rock Smash
- HM07 - Waterfall
- HM08 - Dive

**Evolution Info**
Blue Orb?
</div>
<div class="card-column">
<label><input type="checkbox" class="caught-check" data-species="primal-kyogre" /> Caught</label>

**Base Stats**

| Stat | Value |
| --- | --- |
| HP | <span class="stat-value stat-high">100</span> |
| Attack | <span class="stat-value stat-high">150</span> |
| Defense | <span class="stat-value stat-mid">90</span> |
| Sp. Atk | <span class="stat-value stat-high">180</span> |
| Sp. Def | <span class="stat-value stat-high">160</span> |
| Speed | <span class="stat-value stat-mid">90</span> |
| Total | <span class="stat-value stat-high">770</span> |

**Level-Up Moves**
- Ancient Power (Lv 1)
- Water Pulse (Lv 1)
- Scary Face (Lv 5)
- Aqua Tail (Lv 15)
- Body Slam (Lv 20)
- Aqua Ring (Lv 30)
- Ice Beam (Lv 35)
- Origin Pulse (Lv 45)
- Calm Mind (Lv 50)
- Muddy Water (Lv 60)
- Sheer Cold (Lv 65)
- Hydro Pump (Lv 75)
- Double-Edge (Lv 80)
- Water Spout (Lv 90)

**Egg Moves**
- Incompatible

**Tutor Moves**
- Body Slam
- Defense Curl
- Double-Edge
- Endure
- Icy Wind
- Mud-Slap
- Psych Up
- Rock Slide
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
#pokemon-tabs-kyogre-tab-0:checked ~ .pokemon-tab-panels #pokemon-tabs-kyogre-panel-0 { display: block; }
#pokemon-tabs-kyogre-tab-1:checked ~ .pokemon-tab-panels #pokemon-tabs-kyogre-panel-1 { display: block; }
</style>
</details>
