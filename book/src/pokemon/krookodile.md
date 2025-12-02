<details class="pokemon-card-container">
<summary>Krookodile (#101)</summary>
<div class="pokemon-tabs" id="pokemon-tabs-krookodile">
<input type="radio" name="pokemon-tabs-krookodile-group" id="pokemon-tabs-krookodile-tab-0">
<label for="pokemon-tabs-krookodile-tab-0">Sandile</label>
<input type="radio" name="pokemon-tabs-krookodile-group" id="pokemon-tabs-krookodile-tab-1">
<label for="pokemon-tabs-krookodile-tab-1">Krokorok</label>
<input type="radio" name="pokemon-tabs-krookodile-group" id="pokemon-tabs-krookodile-tab-2" checked>
<label for="pokemon-tabs-krookodile-tab-2">Krookodile</label>
<div class="pokemon-tab-panels">
<div class="pokemon-tab-panel" id="pokemon-tabs-krookodile-panel-0">
Types: Ground / Dark • Egg Groups: Field

<div class="pokemon-card">
<div class="card-column">
**Abilities**
- Intimidate
- Moxie
- Anger Point *(Hidden)*

**Type Matchups**

*Resists / Immune to*
- Electric (0×)
- Poison (0.5×)
- Psychic (0×)
- Rock (0.5×)
- Ghost (0.5×)
- Dark (0.5×)

*Weak to*
- Water (2×)
- Grass (2×)
- Ice (2×)
- Fighting (2×)
- Bug (2×)
- Fairy (2×)

**TM/HM Moves**
- TM06 - Toxic
- TM12 - Taunt
- TM17 - Protect
- TM26 - Earthquake
- TM28 - Dig
- TM31 - Brick Break
- TM32 - Double Team
- TM36 - Sludge Bomb
- TM37 - Sandstorm
- TM39 - Rock Tomb
- TM41 - Torment
- TM42 - Facade
- TM44 - Rest
- TM45 - Attract
- TM46 - Thief
- TM49 - Bulldoze
- TM55 - Snarl
- TM59 - Dark Pulse
- HM01 - Cut

**Held Item**
Black Glasses

**Encounter Locations**
- Erinys Path (West) — Grass (Day) (20%)
- Riverwalk Trail (West) — Grass (Day) (5%)
- Riverwalk Trail (West) — Grass (Night) (5%)
</div>
<div class="card-column">
<label><input type="checkbox" class="caught-check" data-species="sandile" /> Caught</label>

**Base Stats**

| Stat | Value |
| --- | --- |
| HP | <span class="stat-value stat-low">50</span> |
| Attack | <span class="stat-value stat-mid">72</span> |
| Defense | <span class="stat-value stat-low">35</span> |
| Sp. Atk | <span class="stat-value stat-low">35</span> |
| Sp. Def | <span class="stat-value stat-low">35</span> |
| Speed | <span class="stat-value stat-mid">65</span> |
| Total | <span class="stat-value stat-low">292</span> |

**Level-Up Moves**
- Leer (Lv 1)
- Rage (Lv 1)
- Bite (Lv 4)
- Sand Attack (Lv 7)
- Torment (Lv 10)
- Sand Tomb (Lv 13)
- Assurance (Lv 16)
- Bulldoze (Lv 19)
- Embargo (Lv 22)
- Swagger (Lv 24)
- Skitter Smack (Lv 27)
- Crunch (Lv 30)
- Dig (Lv 31)
- Scary Face (Lv 34)
- Foul Play (Lv 37)
- Sandstorm (Lv 40)
- Earthquake (Lv 43)
- Thrash (Lv 46)

**Egg Moves**
- Double-Edge
- Rock Climb
- Pursuit
- Uproar
- Fire Fang
- Thunder Fang
- Beat Up
- Focus Energy
- Counter
- Mean Look
- Me First
- Power Trip

**Tutor Moves**
- Body Slam
- Counter
- Double-Edge
- Endure
- Mud-Slap
- Rock Slide
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
<div class="pokemon-tab-panel" id="pokemon-tabs-krookodile-panel-1">
Types: Ground / Dark • Egg Groups: Field

<div class="pokemon-card">
<div class="card-column">
**Abilities**
- Intimidate
- Moxie
- Anger Point *(Hidden)*

**Type Matchups**

*Resists / Immune to*
- Electric (0×)
- Poison (0.5×)
- Psychic (0×)
- Rock (0.5×)
- Ghost (0.5×)
- Dark (0.5×)

*Weak to*
- Water (2×)
- Grass (2×)
- Ice (2×)
- Fighting (2×)
- Bug (2×)
- Fairy (2×)

**TM/HM Moves**
- TM02 - Dragon Claw
- TM06 - Toxic
- TM12 - Taunt
- TM17 - Protect
- TM26 - Earthquake
- TM28 - Dig
- TM31 - Brick Break
- TM32 - Double Team
- TM36 - Sludge Bomb
- TM37 - Sandstorm
- TM39 - Rock Tomb
- TM40 - Aerial Ace
- TM41 - Torment
- TM42 - Facade
- TM44 - Rest
- TM45 - Attract
- TM46 - Thief
- TM49 - Bulldoze
- TM53 - Power-Up Punch
- TM55 - Snarl
- TM59 - Dark Pulse
- HM01 - Cut
- HM04 - Strength
- HM06 - Rock Smash

**Held Item**
Black Glasses

**Evolution Info**
Lv. 29

**Encounter Locations**
- Fresco Isles — Grass (Day) (5%)
</div>
<div class="card-column">
<label><input type="checkbox" class="caught-check" data-species="krokorok" /> Caught</label>

**Base Stats**

| Stat | Value |
| --- | --- |
| HP | <span class="stat-value stat-mid">60</span> |
| Attack | <span class="stat-value stat-mid">82</span> |
| Defense | <span class="stat-value stat-low">45</span> |
| Sp. Atk | <span class="stat-value stat-low">45</span> |
| Sp. Def | <span class="stat-value stat-low">45</span> |
| Speed | <span class="stat-value stat-mid">74</span> |
| Total | <span class="stat-value stat-mid">351</span> |

**Level-Up Moves**
- Leer (Lv 1)
- Rage (Lv 1)
- Bite (Lv 4)
- Sand Attack (Lv 7)
- Torment (Lv 10)
- Sand Tomb (Lv 13)
- Assurance (Lv 16)
- Bulldoze (Lv 19)
- Embargo (Lv 22)
- Swagger (Lv 24)
- Skitter Smack (Lv 27)
- Crunch (Lv 30)
- Dig (Lv 31)
- Scary Face (Lv 34)
- Foul Play (Lv 37)
- Sandstorm (Lv 40)
- Earthquake (Lv 43)
- Thrash (Lv 46)

**Egg Moves**
- Double-Edge
- Rock Climb
- Pursuit
- Uproar
- Fire Fang
- Thunder Fang
- Beat Up
- Focus Energy
- Counter
- Mean Look
- Me First
- Power Trip

**Tutor Moves**
- Body Slam
- Counter
- Double-Edge
- Endure
- Mega Kick
- Mega Punch
- Mud-Slap
- Rock Slide
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
<div class="pokemon-tab-panel" id="pokemon-tabs-krookodile-panel-2">
Types: Ground / Dark • Egg Groups: Field

<div class="pokemon-card">
<div class="card-column">
**Abilities**
- Intimidate
- Moxie
- Anger Point *(Hidden)*

**Type Matchups**

*Resists / Immune to*
- Electric (0×)
- Poison (0.5×)
- Psychic (0×)
- Rock (0.5×)
- Ghost (0.5×)
- Dark (0.5×)

*Weak to*
- Water (2×)
- Grass (2×)
- Ice (2×)
- Fighting (2×)
- Bug (2×)
- Fairy (2×)

**TM/HM Moves**
- TM02 - Dragon Claw
- TM06 - Toxic
- TM08 - Bulk Up
- TM12 - Taunt
- TM17 - Protect
- TM26 - Earthquake
- TM28 - Dig
- TM31 - Brick Break
- TM32 - Double Team
- TM36 - Sludge Bomb
- TM37 - Sandstorm
- TM39 - Rock Tomb
- TM40 - Aerial Ace
- TM41 - Torment
- TM42 - Facade
- TM44 - Rest
- TM45 - Attract
- TM46 - Thief
- TM49 - Bulldoze
- TM53 - Power-Up Punch
- TM55 - Snarl
- TM59 - Dark Pulse
- HM01 - Cut
- HM04 - Strength
- HM06 - Rock Smash

**Held Item**
Black Glasses

**Evolution Info**
Lv. 40
</div>
<div class="card-column">
<label><input type="checkbox" class="caught-check" data-species="krookodile" /> Caught</label>

**Base Stats**

| Stat | Value |
| --- | --- |
| HP | <span class="stat-value stat-high">100</span> |
| Attack | <span class="stat-value stat-high">122</span> |
| Defense | <span class="stat-value stat-mid">80</span> |
| Sp. Atk | <span class="stat-value stat-mid">65</span> |
| Sp. Def | <span class="stat-value stat-mid">70</span> |
| Speed | <span class="stat-value stat-high">92</span> |
| Total | <span class="stat-value stat-mid">529</span> |

**Level-Up Moves**
- Power Trip (Lv 1)
- Leer (Lv 1)
- Rage (Lv 1)
- Bite (Lv 4)
- Sand Attack (Lv 7)
- Torment (Lv 10)
- Sand Tomb (Lv 13)
- Assurance (Lv 16)
- Bulldoze (Lv 19)
- Embargo (Lv 22)
- Swagger (Lv 24)
- Skitter Smack (Lv 27)
- Crunch (Lv 30)
- Dig (Lv 31)
- Scary Face (Lv 34)
- Foul Play (Lv 37)
- Sandstorm (Lv 40)
- Earthquake (Lv 43)
- Thrash (Lv 46)
- Jaw Lock (Lv 50)
- Outrage (Lv 54)

**Egg Moves**
- Double-Edge
- Rock Climb
- Pursuit
- Uproar
- Fire Fang
- Thunder Fang
- Beat Up
- Focus Energy
- Counter
- Mean Look
- Me First
- Power Trip

**Tutor Moves**
- Body Slam
- Counter
- Double-Edge
- Endure
- Mega Kick
- Mega Punch
- Mud-Slap
- Rock Slide
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
#pokemon-tabs-krookodile-tab-0:checked ~ .pokemon-tab-panels #pokemon-tabs-krookodile-panel-0 { display: block; }
#pokemon-tabs-krookodile-tab-1:checked ~ .pokemon-tab-panels #pokemon-tabs-krookodile-panel-1 { display: block; }
#pokemon-tabs-krookodile-tab-2:checked ~ .pokemon-tab-panels #pokemon-tabs-krookodile-panel-2 { display: block; }
</style>
</details>
