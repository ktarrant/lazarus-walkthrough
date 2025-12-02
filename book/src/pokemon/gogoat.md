<details class="pokemon-card-container">
<summary>Gogoat (#093)</summary>
<div class="pokemon-tabs" id="pokemon-tabs-gogoat">
<input type="radio" name="pokemon-tabs-gogoat-group" id="pokemon-tabs-gogoat-tab-0">
<label for="pokemon-tabs-gogoat-tab-0">Skiddo</label>
<input type="radio" name="pokemon-tabs-gogoat-group" id="pokemon-tabs-gogoat-tab-1" checked>
<label for="pokemon-tabs-gogoat-tab-1">Gogoat</label>
<input type="radio" name="pokemon-tabs-gogoat-group" id="pokemon-tabs-gogoat-tab-2">
<label for="pokemon-tabs-gogoat-tab-2">Mega Gogoat</label>
<div class="pokemon-tab-panels">
<div class="pokemon-tab-panel" id="pokemon-tabs-gogoat-panel-0">
Types: Grass • Egg Groups: Field

<div class="pokemon-card">
<div class="card-column">
**Abilities**
- Sap Sipper
- Grass Pelt
- Seed Sower *(Hidden)*

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
- TM08 - Bulk Up
- TM09 - Bullet Seed
- TM11 - Sunny Day
- TM17 - Protect
- TM18 - Rain Dance
- TM19 - Giga Drain
- TM22 - Solar Beam
- TM28 - Dig
- TM31 - Brick Break
- TM32 - Double Team
- TM42 - Facade
- TM44 - Rest
- TM45 - Attract
- TM49 - Bulldoze
- HM03 - Surf
- HM04 - Strength
- HM06 - Rock Smash

**Encounter Locations**
- Acrisia Mountains (Peak) — Grass (Night) (10%)
- Jusmail Town — Grass (Day) (10%)
- Jusmail Town — Grass (Night) (10%)
</div>
<div class="card-column">
<label><input type="checkbox" class="caught-check" data-species="skiddo" /> Caught</label>

**Base Stats**

| Stat | Value |
| --- | --- |
| HP | <span class="stat-value stat-mid">76</span> |
| Attack | <span class="stat-value stat-mid">65</span> |
| Defense | <span class="stat-value stat-mid">53</span> |
| Sp. Atk | <span class="stat-value stat-mid">62</span> |
| Sp. Def | <span class="stat-value stat-mid">57</span> |
| Speed | <span class="stat-value stat-mid">52</span> |
| Total | <span class="stat-value stat-mid">365</span> |

**Level-Up Moves**
- Tackle (Lv 1)
- Growth (Lv 1)
- Vine Whip (Lv 7)
- Tail Whip (Lv 9)
- Leech Seed (Lv 12)
- Razor Leaf (Lv 13)
- Worry Seed (Lv 16)
- Synthesis (Lv 20)
- Stomp (Lv 22)
- Bulldoze (Lv 26)
- Seed Bomb (Lv 30)
- Bulk Up (Lv 34)
- Double-Edge (Lv 38)
- Horn Leech (Lv 42)
- Leaf Blade (Lv 45)
- Milk Drink (Lv 50)
- Rock Slide (Lv 53)
- Megahorn (Lv 57)

**Egg Moves**
- Defense Curl
- Rollout
- Milk Drink
- Grassy Terrain

**Tutor Moves**
- Body Slam
- Defense Curl
- Double-Edge
- Endure
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
<div class="pokemon-tab-panel" id="pokemon-tabs-gogoat-panel-1">
Types: Grass / Ground • Egg Groups: Field

<div class="pokemon-card">
<div class="card-column">
**Abilities**
- Sap Sipper
- Grass Pelt
- Seed Sower *(Hidden)*

**Type Matchups**

*Resists / Immune to*
- Electric (0×)
- Ground (0.5×)
- Rock (0.5×)

*Weak to*
- Fire (2×)
- Ice (4×)
- Flying (2×)
- Bug (2×)

**TM/HM Moves**
- TM06 - Toxic
- TM08 - Bulk Up
- TM09 - Bullet Seed
- TM11 - Sunny Day
- TM17 - Protect
- TM18 - Rain Dance
- TM19 - Giga Drain
- TM22 - Solar Beam
- TM26 - Earthquake
- TM28 - Dig
- TM31 - Brick Break
- TM32 - Double Team
- TM40 - Aerial Ace
- TM42 - Facade
- TM44 - Rest
- TM45 - Attract
- TM49 - Bulldoze
- HM03 - Surf
- HM04 - Strength
- HM06 - Rock Smash

**Evolution Info**
Lv. 30

**Encounter Locations**
- Nyx Trails — Grass (Day) (10%)
- Tower of Dioxippus — Grass (Day) (5%)
- Tower of Dioxippus — Grass (Night) (5%)
- Wakewater Isle — Grass (Day) (20%)
</div>
<div class="card-column">
<label><input type="checkbox" class="caught-check" data-species="gogoat" /> Caught</label>

**Base Stats**

| Stat | Value |
| --- | --- |
| HP | <span class="stat-value stat-high">133</span> |
| Attack | <span class="stat-value stat-high">104</span> |
| Defense | <span class="stat-value stat-mid">67</span> |
| Sp. Atk | <span class="stat-value stat-high">97</span> |
| Sp. Def | <span class="stat-value stat-mid">81</span> |
| Speed | <span class="stat-value stat-mid">64</span> |
| Total | <span class="stat-value stat-high">546</span> |

**Level-Up Moves**
- Aerial Ace (Lv Evo)
- Earthquake (Lv 1)
- Tackle (Lv 1)
- Growth (Lv 1)
- Vine Whip (Lv 7)
- Tail Whip (Lv 9)
- Leech Seed (Lv 12)
- Razor Leaf (Lv 13)
- Worry Seed (Lv 16)
- Synthesis (Lv 20)
- Stomp (Lv 22)
- Bulldoze (Lv 26)
- Seed Bomb (Lv 30)
- Bulk Up (Lv 34)
- Double-Edge (Lv 38)
- Horn Leech (Lv 42)
- Leaf Blade (Lv 45)
- Milk Drink (Lv 50)
- Rock Slide (Lv 53)
- Megahorn (Lv 57)

**Egg Moves**
- Defense Curl
- Rollout
- Milk Drink
- Grassy Terrain

**Tutor Moves**
- Body Slam
- Defense Curl
- Double-Edge
- Endure
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
<div class="pokemon-tab-panel" id="pokemon-tabs-gogoat-panel-2">
Types: Grass / Ground • Egg Groups: Field

<div class="pokemon-card">
<div class="card-column">
**Abilities**
- Technician

**Type Matchups**

*Resists / Immune to*
- Electric (0×)
- Ground (0.5×)
- Rock (0.5×)

*Weak to*
- Fire (2×)
- Ice (4×)
- Flying (2×)
- Bug (2×)

**TM/HM Moves**
- TM06 - Toxic
- TM08 - Bulk Up
- TM09 - Bullet Seed
- TM11 - Sunny Day
- TM17 - Protect
- TM18 - Rain Dance
- TM19 - Giga Drain
- TM22 - Solar Beam
- TM26 - Earthquake
- TM28 - Dig
- TM31 - Brick Break
- TM32 - Double Team
- TM40 - Aerial Ace
- TM42 - Facade
- TM44 - Rest
- TM45 - Attract
- TM49 - Bulldoze
- HM03 - Surf
- HM04 - Strength
- HM06 - Rock Smash

**Evolution Info**
Gogoatite
</div>
<div class="card-column">
<label><input type="checkbox" class="caught-check" data-species="mega-gogoat" /> Caught</label>

**Base Stats**

| Stat | Value |
| --- | --- |
| HP | <span class="stat-value stat-high">133</span> |
| Attack | <span class="stat-value stat-high">134</span> |
| Defense | <span class="stat-value stat-high">97</span> |
| Sp. Atk | <span class="stat-value stat-high">127</span> |
| Sp. Def | <span class="stat-value stat-high">91</span> |
| Speed | <span class="stat-value stat-mid">64</span> |
| Total | <span class="stat-value stat-high">646</span> |

**Level-Up Moves**
- Aerial Ace (Lv Evo)
- Earthquake (Lv 1)
- Tackle (Lv 1)
- Growth (Lv 1)
- Vine Whip (Lv 7)
- Tail Whip (Lv 9)
- Leech Seed (Lv 12)
- Razor Leaf (Lv 13)
- Worry Seed (Lv 16)
- Synthesis (Lv 20)
- Stomp (Lv 22)
- Bulldoze (Lv 26)
- Seed Bomb (Lv 30)
- Bulk Up (Lv 34)
- Double-Edge (Lv 38)
- Horn Leech (Lv 42)
- Leaf Blade (Lv 45)
- Milk Drink (Lv 50)
- Rock Slide (Lv 53)
- Megahorn (Lv 57)

**Egg Moves**
- Defense Curl
- Rollout
- Milk Drink
- Grassy Terrain

**Tutor Moves**
- Body Slam
- Defense Curl
- Double-Edge
- Endure
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
</div>
</div>
<style>
#pokemon-tabs-gogoat-tab-0:checked ~ .pokemon-tab-panels #pokemon-tabs-gogoat-panel-0 { display: block; }
#pokemon-tabs-gogoat-tab-1:checked ~ .pokemon-tab-panels #pokemon-tabs-gogoat-panel-1 { display: block; }
#pokemon-tabs-gogoat-tab-2:checked ~ .pokemon-tab-panels #pokemon-tabs-gogoat-panel-2 { display: block; }
</style>
</details>
