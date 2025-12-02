<details class="pokemon-card-container">
<summary>Armaldo (#278)</summary>
<div class="pokemon-tabs" id="pokemon-tabs-armaldo">
<input type="radio" name="pokemon-tabs-armaldo-group" id="pokemon-tabs-armaldo-tab-0">
<label for="pokemon-tabs-armaldo-tab-0">Anorith</label>
<input type="radio" name="pokemon-tabs-armaldo-group" id="pokemon-tabs-armaldo-tab-1" checked>
<label for="pokemon-tabs-armaldo-tab-1">Armaldo</label>
<div class="pokemon-tab-panels">
<div class="pokemon-tab-panel" id="pokemon-tabs-armaldo-panel-0">
Types: Rock / Bug • Egg Groups: Water 3

<div class="pokemon-card">
<div class="card-column">
**Abilities**
- Battle Armor
- Water Bubble *(Hidden)*

**Type Matchups**

*Resists / Immune to*
- Normal (0.5×)
- Poison (0.5×)

*Weak to*
- Water (2×)
- Rock (2×)
- Steel (2×)

**TM/HM Moves**
- TM03 - Water Pulse
- TM06 - Toxic
- TM11 - Sunny Day
- TM17 - Protect
- TM28 - Dig
- TM31 - Brick Break
- TM32 - Double Team
- TM37 - Sandstorm
- TM39 - Rock Tomb
- TM40 - Aerial Ace
- TM42 - Facade
- TM44 - Rest
- TM45 - Attract
- HM01 - Cut
- HM06 - Rock Smash

**Encounter Locations**
- Marmaro Island — Grass (Day) (6%)
- Marmaro Island — Grass (Night) (6%)
- Wanderer's Woods (South) — Grass (Night) (5%)
</div>
<div class="card-column">
<label><input type="checkbox" class="caught-check" data-species="anorith" /> Caught</label>

**Base Stats**

| Stat | Value |
| --- | --- |
| HP | <span class="stat-value stat-low">50</span> |
| Attack | <span class="stat-value stat-high">95</span> |
| Defense | <span class="stat-value stat-low">50</span> |
| Sp. Atk | <span class="stat-value stat-low">40</span> |
| Sp. Def | <span class="stat-value stat-low">50</span> |
| Speed | <span class="stat-value stat-mid">75</span> |
| Total | <span class="stat-value stat-mid">360</span> |

**Level-Up Moves**
- Scratch (Lv 1)
- Harden (Lv 1)
- Mud Sport (Lv 4)
- Bubble (Lv 7)
- Fury Cutter (Lv 10)
- Smack Down (Lv 13)
- Water Gun (Lv 15)
- Metal Claw (Lv 17)
- Ancient Power (Lv 21)
- Bug Bite (Lv 25)
- Razor Shell (Lv 29)
- Slash (Lv 34)
- Crush Claw (Lv 37)
- X-Scissor (Lv 40)
- Surging Strikes (Lv 44)
- Protect (Lv 49)
- Rock Blast (Lv 53)
- Stone Axe (Lv 55)

**Egg Moves**
- Rapid Spin
- Knock Off
- Screech
- Sand Attack
- Cross Poison
- Curse
- Iron Defense
- Water Pulse
- Aqua Jet

**Tutor Moves**
- Body Slam
- Double-Edge
- Endure
- Fury Cutter
- Mud-Slap
- Rock Slide
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
<div class="pokemon-tab-panel" id="pokemon-tabs-armaldo-panel-1">
Types: Rock / Bug • Egg Groups: Water 3

<div class="pokemon-card">
<div class="card-column">
**Abilities**
- Battle Armor
- Water Bubble *(Hidden)*

**Type Matchups**

*Resists / Immune to*
- Normal (0.5×)
- Poison (0.5×)

*Weak to*
- Water (2×)
- Rock (2×)
- Steel (2×)

**TM/HM Moves**
- TM03 - Water Pulse
- TM06 - Toxic
- TM11 - Sunny Day
- TM17 - Protect
- TM26 - Earthquake
- TM28 - Dig
- TM31 - Brick Break
- TM32 - Double Team
- TM37 - Sandstorm
- TM39 - Rock Tomb
- TM40 - Aerial Ace
- TM42 - Facade
- TM44 - Rest
- TM45 - Attract
- TM49 - Bulldoze
- HM01 - Cut
- HM04 - Strength
- HM06 - Rock Smash

**Evolution Info**
Lv. 40
</div>
<div class="card-column">
<label><input type="checkbox" class="caught-check" data-species="armaldo" /> Caught</label>

**Base Stats**

| Stat | Value |
| --- | --- |
| HP | <span class="stat-value stat-mid">85</span> |
| Attack | <span class="stat-value stat-high">125</span> |
| Defense | <span class="stat-value stat-high">100</span> |
| Sp. Atk | <span class="stat-value stat-mid">70</span> |
| Sp. Def | <span class="stat-value stat-mid">80</span> |
| Speed | <span class="stat-value stat-low">45</span> |
| Total | <span class="stat-value stat-mid">505</span> |

**Level-Up Moves**
- Scratch (Lv 1)
- Harden (Lv 1)
- Mud Sport (Lv 4)
- Water Gun (Lv 7)
- Fury Cutter (Lv 10)
- Smack Down (Lv 13)
- Metal Claw (Lv 17)
- Ancient Power (Lv 21)
- Bug Bite (Lv 25)
- Razor Shell (Lv 29)
- Slash (Lv 34)
- Crush Claw (Lv 37)
- X-Scissor (Lv 40)
- Surging Strikes (Lv 44)
- Protect (Lv 49)
- Rock Blast (Lv 53)
- Stone Axe (Lv 55)

**Egg Moves**
- Rapid Spin
- Knock Off
- Screech
- Sand Attack
- Cross Poison
- Curse
- Iron Defense
- Water Pulse
- Aqua Jet

**Tutor Moves**
- Body Slam
- Double-Edge
- Endure
- Fury Cutter
- Mud-Slap
- Rock Slide
- Seismic Toss
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
#pokemon-tabs-armaldo-tab-0:checked ~ .pokemon-tab-panels #pokemon-tabs-armaldo-panel-0 { display: block; }
#pokemon-tabs-armaldo-tab-1:checked ~ .pokemon-tab-panels #pokemon-tabs-armaldo-panel-1 { display: block; }
</style>
</details>
