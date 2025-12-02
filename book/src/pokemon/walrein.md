<details class="pokemon-card-container">
<summary>Walrein (#191)</summary>
<div class="pokemon-tabs" id="pokemon-tabs-walrein">
<input type="radio" name="pokemon-tabs-walrein-group" id="pokemon-tabs-walrein-tab-0">
<label for="pokemon-tabs-walrein-tab-0">Spheal</label>
<input type="radio" name="pokemon-tabs-walrein-group" id="pokemon-tabs-walrein-tab-1">
<label for="pokemon-tabs-walrein-tab-1">Sealeo</label>
<input type="radio" name="pokemon-tabs-walrein-group" id="pokemon-tabs-walrein-tab-2" checked>
<label for="pokemon-tabs-walrein-tab-2">Walrein</label>
<div class="pokemon-tab-panels">
<div class="pokemon-tab-panel" id="pokemon-tabs-walrein-panel-0">
Types: Ice / Water • Egg Groups: Water 1 / Field

<div class="pokemon-card">
<div class="card-column">
**Abilities**
- Thick Layers
- Fur Coat
- Ice Scales *(Hidden)*

**Type Matchups**

*Resists / Immune to*
- Water (0.5×)
- Ice (0.25×)

*Weak to*
- Electric (2×)
- Grass (2×)
- Fighting (2×)
- Rock (2×)

**TM/HM Moves**
- TM03 - Water Pulse
- TM06 - Toxic
- TM07 - Whirlpool
- TM13 - Ice Beam
- TM14 - Blizzard
- TM17 - Protect
- TM18 - Rain Dance
- TM26 - Earthquake
- TM32 - Double Team
- TM39 - Rock Tomb
- TM42 - Facade
- TM44 - Rest
- TM45 - Attract
- TM49 - Bulldoze
- TM50 - Deepwater Curse
- TM52 - Frost Breath
- HM03 - Surf
- HM04 - Strength
- HM06 - Rock Smash
- HM07 - Waterfall
- HM08 - Dive

**Encounter Locations**
- Erinys Path (East) — Fishing (20%)
- Erinys Path (East) — Surfing (60%)
- Froslass Cavern BF1 — Grass (Day) (10%)
- Froslass Cavern BF2 — Grass (Day) (10%)
- Froslass Cavern BF2 — Surfing (30%)
- Froslass Cavern F1 — Grass (Day) (10%)
- Pythios Town — Fishing (20%)
- Pythios Town — Surfing (60%)
</div>
<div class="card-column">
<label><input type="checkbox" class="caught-check" data-species="spheal" /> Caught</label>

**Base Stats**

| Stat | Value |
| --- | --- |
| HP | <span class="stat-value stat-mid">75</span> |
| Attack | <span class="stat-value stat-low">45</span> |
| Defense | <span class="stat-value stat-low">50</span> |
| Sp. Atk | <span class="stat-value stat-mid">55</span> |
| Sp. Def | <span class="stat-value stat-low">50</span> |
| Speed | <span class="stat-value stat-low">25</span> |
| Total | <span class="stat-value stat-low">300</span> |

**Level-Up Moves**
- Defense Curl (Lv 1)
- Powder Snow (Lv 1)
- Growl (Lv 1)
- Water Gun (Lv 1)
- Rollout (Lv 5)
- Encore (Lv 9)
- Ice Ball (Lv 13)
- Brine (Lv 17)
- Aurora Beam (Lv 21)
- Flip Turn (Lv 24)
- Body Slam (Lv 26)
- Rest (Lv 31)
- Snore (Lv 31)
- Hail (Lv 36)
- Ice Beam (Lv 39)
- Blizzard (Lv 41)
- Sheer Cold (Lv 46)

**Egg Moves**
- Water Sport
- Stockpile
- Swallow
- Spit Up
- Yawn
- Curse
- Fissure
- Signal Beam
- Aqua Ring
- Rollout
- Sleep Talk
- Water Pulse
- Belly Drum

**Tutor Moves**
- Body Slam
- Defense Curl
- Double-Edge
- Endure
- Icy Wind
- Mud-Slap
- Rock Slide
- Rollout
- Sleep Talk
- Snore
- Swagger
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
<div class="pokemon-tab-panel" id="pokemon-tabs-walrein-panel-1">
Types: Ice / Water • Egg Groups: Water 1 / Field

<div class="pokemon-card">
<div class="card-column">
**Abilities**
- Thick Layers
- Fur Coat
- Ice Scales *(Hidden)*

**Type Matchups**

*Resists / Immune to*
- Water (0.5×)
- Ice (0.25×)

*Weak to*
- Electric (2×)
- Grass (2×)
- Fighting (2×)
- Rock (2×)

**TM/HM Moves**
- TM03 - Water Pulse
- TM06 - Toxic
- TM07 - Whirlpool
- TM13 - Ice Beam
- TM14 - Blizzard
- TM17 - Protect
- TM18 - Rain Dance
- TM26 - Earthquake
- TM32 - Double Team
- TM39 - Rock Tomb
- TM42 - Facade
- TM44 - Rest
- TM45 - Attract
- TM49 - Bulldoze
- TM50 - Deepwater Curse
- TM52 - Frost Breath
- HM03 - Surf
- HM04 - Strength
- HM06 - Rock Smash
- HM07 - Waterfall
- HM08 - Dive

**Evolution Info**
Lv. 24

**Encounter Locations**
- Froslass Cavern BF2 — Surfing (9%)
- Péntepetal City — Fishing (20%)
- Péntepetal City — Surfing (60%)
</div>
<div class="card-column">
<label><input type="checkbox" class="caught-check" data-species="sealeo" /> Caught</label>

**Base Stats**

| Stat | Value |
| --- | --- |
| HP | <span class="stat-value stat-high">95</span> |
| Attack | <span class="stat-value stat-mid">60</span> |
| Defense | <span class="stat-value stat-mid">70</span> |
| Sp. Atk | <span class="stat-value stat-mid">80</span> |
| Sp. Def | <span class="stat-value stat-mid">70</span> |
| Speed | <span class="stat-value stat-low">45</span> |
| Total | <span class="stat-value stat-mid">420</span> |

**Level-Up Moves**
- Ice Fang (Lv Evo)
- Defense Curl (Lv 1)
- Powder Snow (Lv 1)
- Growl (Lv 1)
- Water Gun (Lv 1)
- Rollout (Lv 5)
- Encore (Lv 9)
- Ice Ball (Lv 13)
- Brine (Lv 17)
- Aurora Beam (Lv 21)
- Flip Turn (Lv 24)
- Body Slam (Lv 26)
- Anchor Shot (Lv 29)
- Rest (Lv 31)
- Snore (Lv 31)
- Calm Mind (Lv 35)
- Hail (Lv 38)
- Crunch (Lv 40)
- Deepwater Curse (Lv 43)
- Ice Beam (Lv 45)
- Blizzard (Lv 50)
- Sheer Cold (Lv 52)

**Egg Moves**
- Water Sport
- Stockpile
- Swallow
- Spit Up
- Yawn
- Curse
- Fissure
- Signal Beam
- Aqua Ring
- Rollout
- Sleep Talk
- Water Pulse
- Belly Drum

**Tutor Moves**
- Body Slam
- Defense Curl
- Double-Edge
- Endure
- Icy Wind
- Mud-Slap
- Rock Slide
- Rollout
- Sleep Talk
- Snore
- Swagger
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
<div class="pokemon-tab-panel" id="pokemon-tabs-walrein-panel-2">
Types: Ice / Water • Egg Groups: Water 1 / Field

<div class="pokemon-card">
<div class="card-column">
**Abilities**
- Thick Layers
- Fur Coat
- Ice Scales *(Hidden)*

**Type Matchups**

*Resists / Immune to*
- Water (0.5×)
- Ice (0.25×)

*Weak to*
- Electric (2×)
- Grass (2×)
- Fighting (2×)
- Rock (2×)

**TM/HM Moves**
- TM03 - Water Pulse
- TM06 - Toxic
- TM07 - Whirlpool
- TM13 - Ice Beam
- TM14 - Blizzard
- TM17 - Protect
- TM18 - Rain Dance
- TM26 - Earthquake
- TM32 - Double Team
- TM39 - Rock Tomb
- TM42 - Facade
- TM44 - Rest
- TM45 - Attract
- TM49 - Bulldoze
- TM50 - Deepwater Curse
- TM52 - Frost Breath
- HM03 - Surf
- HM04 - Strength
- HM06 - Rock Smash
- HM07 - Waterfall
- HM08 - Dive

**Evolution Info**
Lv. 44
</div>
<div class="card-column">
<label><input type="checkbox" class="caught-check" data-species="walrein" /> Caught</label>

**Base Stats**

| Stat | Value |
| --- | --- |
| HP | <span class="stat-value stat-high">120</span> |
| Attack | <span class="stat-value stat-mid">85</span> |
| Defense | <span class="stat-value stat-mid">90</span> |
| Sp. Atk | <span class="stat-value stat-high">100</span> |
| Sp. Def | <span class="stat-value stat-mid">90</span> |
| Speed | <span class="stat-value stat-mid">65</span> |
| Total | <span class="stat-value stat-high">550</span> |

**Level-Up Moves**
- Slack Off (Lv Evo)
- Swagger (Lv 1)
- Crunch (Lv 1)
- Ice Fang (Lv 1)
- Defense Curl (Lv 1)
- Powder Snow (Lv 1)
- Growl (Lv 1)
- Water Gun (Lv 1)
- Rollout (Lv 5)
- Encore (Lv 9)
- Ice Ball (Lv 13)
- Brine (Lv 17)
- Aurora Beam (Lv 21)
- Flip Turn (Lv 24)
- Body Slam (Lv 26)
- Anchor Shot (Lv 29)
- Rest (Lv 31)
- Snore (Lv 31)
- Calm Mind (Lv 35)
- Hail (Lv 38)
- Crunch (Lv 40)
- Deepwater Curse (Lv 43)
- Ice Beam (Lv 45)
- Blizzard (Lv 50)
- Sheer Cold (Lv 52)

**Egg Moves**
- Water Sport
- Stockpile
- Swallow
- Spit Up
- Yawn
- Curse
- Fissure
- Signal Beam
- Aqua Ring
- Rollout
- Sleep Talk
- Water Pulse
- Belly Drum

**Tutor Moves**
- Body Slam
- Defense Curl
- Double-Edge
- Endure
- Fury Cutter
- Icy Wind
- Mud-Slap
- Rock Slide
- Rollout
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
#pokemon-tabs-walrein-tab-0:checked ~ .pokemon-tab-panels #pokemon-tabs-walrein-panel-0 { display: block; }
#pokemon-tabs-walrein-tab-1:checked ~ .pokemon-tab-panels #pokemon-tabs-walrein-panel-1 { display: block; }
#pokemon-tabs-walrein-tab-2:checked ~ .pokemon-tab-panels #pokemon-tabs-walrein-panel-2 { display: block; }
</style>
</details>
