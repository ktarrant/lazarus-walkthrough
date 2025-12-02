<details class="pokemon-card-container">
<summary>Gurdurr (#115)</summary>
<div class="pokemon-tabs" id="pokemon-tabs-gurdurr">
<input type="radio" name="pokemon-tabs-gurdurr-group" id="pokemon-tabs-gurdurr-tab-0">
<label for="pokemon-tabs-gurdurr-tab-0">Timburr</label>
<input type="radio" name="pokemon-tabs-gurdurr-group" id="pokemon-tabs-gurdurr-tab-1" checked>
<label for="pokemon-tabs-gurdurr-tab-1">Gurdurr</label>
<input type="radio" name="pokemon-tabs-gurdurr-group" id="pokemon-tabs-gurdurr-tab-2">
<label for="pokemon-tabs-gurdurr-tab-2">Conkeldurr</label>
<div class="pokemon-tab-panels">
<div class="pokemon-tab-panel" id="pokemon-tabs-gurdurr-panel-0">
Types: Fighting • Egg Groups: Human-Like

<div class="pokemon-card">
<div class="card-column">
**Abilities**
- Guts
- Sheer Force
- Iron Fist *(Hidden)*

**Type Matchups**

*Resists / Immune to*
- Bug (0.5×)
- Rock (0.5×)
- Dark (0.5×)

*Weak to*
- Flying (2×)
- Psychic (2×)
- Fairy (2×)

**TM/HM Moves**
- TM06 - Toxic
- TM08 - Bulk Up
- TM11 - Sunny Day
- TM12 - Taunt
- TM17 - Protect
- TM18 - Rain Dance
- TM20 - Poison Jab
- TM28 - Dig
- TM31 - Brick Break
- TM32 - Double Team
- TM39 - Rock Tomb
- TM42 - Facade
- TM44 - Rest
- TM45 - Attract
- TM46 - Thief
- TM53 - Power-Up Punch
- HM04 - Strength
- HM06 - Rock Smash

**Encounter Locations**
- Acrisia Mountains — Grass (Day) (10%)
- Acrisia Mountains — Grass (Night) (10%)
- Kipos Town — Grass (Day) (10%)
- Kipos Town — Grass (Night) (10%)
- Palati City — Grass (Day) (4%)
</div>
<div class="card-column">
<label><input type="checkbox" class="caught-check" data-species="timburr" /> Caught</label>

**Base Stats**

| Stat | Value |
| --- | --- |
| HP | <span class="stat-value stat-mid">75</span> |
| Attack | <span class="stat-value stat-mid">80</span> |
| Defense | <span class="stat-value stat-mid">55</span> |
| Sp. Atk | <span class="stat-value stat-low">25</span> |
| Sp. Def | <span class="stat-value stat-low">35</span> |
| Speed | <span class="stat-value stat-low">35</span> |
| Total | <span class="stat-value stat-low">305</span> |

**Level-Up Moves**
- Pound (Lv 1)
- Leer (Lv 1)
- Focus Energy (Lv 4)
- Bide (Lv 8)
- Low Kick (Lv 12)
- Rock Throw (Lv 16)
- Wake-Up Slap (Lv 20)
- Chip Away (Lv 24)
- Bulk Up (Lv 28)
- Slam (Lv 30)
- Rock Slide (Lv 31)
- Dynamic Punch (Lv 34)
- Scary Face (Lv 37)
- Hammer Arm (Lv 40)
- Stomping Tantrum (Lv 42)
- Stone Edge (Lv 44)
- Focus Punch (Lv 47)
- Superpower (Lv 50)

**Egg Moves**
- Drain Punch
- Endure
- Counter
- Comet Punch
- Foresight
- Smelling Salts
- Detect
- Wide Guard
- Force Palm
- Reversal
- Mach Punch
- Power-Up Punch

**Tutor Moves**
- Counter
- Double-Edge
- Dynamic Punch
- Endure
- Fire Punch
- Ice Punch
- Mega Kick
- Mega Punch
- Rock Slide
- Sleep Talk
- Snore
- Swagger
- Thunder Punch
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
<div class="pokemon-tab-panel" id="pokemon-tabs-gurdurr-panel-1">
Types: Fighting • Egg Groups: Human-Like

<div class="pokemon-card">
<div class="card-column">
**Abilities**
- Guts
- Sheer Force
- Iron Fist *(Hidden)*

**Type Matchups**

*Resists / Immune to*
- Bug (0.5×)
- Rock (0.5×)
- Dark (0.5×)

*Weak to*
- Flying (2×)
- Psychic (2×)
- Fairy (2×)

**TM/HM Moves**
- TM06 - Toxic
- TM08 - Bulk Up
- TM11 - Sunny Day
- TM12 - Taunt
- TM17 - Protect
- TM18 - Rain Dance
- TM20 - Poison Jab
- TM28 - Dig
- TM31 - Brick Break
- TM32 - Double Team
- TM39 - Rock Tomb
- TM42 - Facade
- TM44 - Rest
- TM45 - Attract
- TM46 - Thief
- TM53 - Power-Up Punch
- HM04 - Strength
- HM06 - Rock Smash

**Evolution Info**
Lv. 25

**Encounter Locations**
- Palati City — Grass (Day) (4%)
</div>
<div class="card-column">
<label><input type="checkbox" class="caught-check" data-species="gurdurr" /> Caught</label>

**Base Stats**

| Stat | Value |
| --- | --- |
| HP | <span class="stat-value stat-mid">85</span> |
| Attack | <span class="stat-value stat-high">105</span> |
| Defense | <span class="stat-value stat-mid">85</span> |
| Sp. Atk | <span class="stat-value stat-low">40</span> |
| Sp. Def | <span class="stat-value stat-low">50</span> |
| Speed | <span class="stat-value stat-low">40</span> |
| Total | <span class="stat-value stat-mid">405</span> |

**Level-Up Moves**
- Pound (Lv 1)
- Leer (Lv 1)
- Focus Energy (Lv 4)
- Bide (Lv 8)
- Low Kick (Lv 12)
- Rock Throw (Lv 16)
- Wake-Up Slap (Lv 20)
- Chip Away (Lv 24)
- Bulk Up (Lv 28)
- Slam (Lv 30)
- Rock Slide (Lv 31)
- Dynamic Punch (Lv 34)
- Scary Face (Lv 37)
- Hammer Arm (Lv 40)
- Stomping Tantrum (Lv 42)
- Stone Edge (Lv 44)
- Focus Punch (Lv 47)
- Superpower (Lv 50)

**Egg Moves**
- Drain Punch
- Endure
- Counter
- Comet Punch
- Foresight
- Smelling Salts
- Detect
- Wide Guard
- Force Palm
- Reversal
- Mach Punch
- Power-Up Punch

**Tutor Moves**
- Counter
- Double-Edge
- Dynamic Punch
- Endure
- Fire Punch
- Ice Punch
- Mega Kick
- Mega Punch
- Rock Slide
- Sleep Talk
- Snore
- Swagger
- Thunder Punch
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
<div class="pokemon-tab-panel" id="pokemon-tabs-gurdurr-panel-2">
Types: Fighting • Egg Groups: Human-Like

<div class="pokemon-card">
<div class="card-column">
**Abilities**
- Guts
- Sheer Force
- Iron Fist *(Hidden)*

**Type Matchups**

*Resists / Immune to*
- Bug (0.5×)
- Rock (0.5×)
- Dark (0.5×)

*Weak to*
- Flying (2×)
- Psychic (2×)
- Fairy (2×)

**TM/HM Moves**
- TM06 - Toxic
- TM08 - Bulk Up
- TM11 - Sunny Day
- TM12 - Taunt
- TM17 - Protect
- TM18 - Rain Dance
- TM20 - Poison Jab
- TM26 - Earthquake
- TM28 - Dig
- TM31 - Brick Break
- TM32 - Double Team
- TM39 - Rock Tomb
- TM42 - Facade
- TM44 - Rest
- TM45 - Attract
- TM46 - Thief
- TM49 - Bulldoze
- TM53 - Power-Up Punch
- HM04 - Strength
- HM06 - Rock Smash

**Evolution Info**
Linking Cord
</div>
<div class="card-column">
<label><input type="checkbox" class="caught-check" data-species="conkeldurr" /> Caught</label>

**Base Stats**

| Stat | Value |
| --- | --- |
| HP | <span class="stat-value stat-high">105</span> |
| Attack | <span class="stat-value stat-high">140</span> |
| Defense | <span class="stat-value stat-high">95</span> |
| Sp. Atk | <span class="stat-value stat-mid">55</span> |
| Sp. Def | <span class="stat-value stat-mid">65</span> |
| Speed | <span class="stat-value stat-low">45</span> |
| Total | <span class="stat-value stat-mid">505</span> |

**Level-Up Moves**
- Body Slam (Lv Evo)
- Pound (Lv 1)
- Leer (Lv 1)
- Focus Energy (Lv 4)
- Bide (Lv 8)
- Low Kick (Lv 12)
- Rock Throw (Lv 16)
- Wake-Up Slap (Lv 20)
- Chip Away (Lv 24)
- Bulk Up (Lv 28)
- Slam (Lv 30)
- Rock Slide (Lv 31)
- Dynamic Punch (Lv 34)
- Scary Face (Lv 37)
- Hammer Arm (Lv 40)
- Stomping Tantrum (Lv 42)
- Stone Edge (Lv 44)
- Focus Punch (Lv 47)
- Superpower (Lv 50)

**Egg Moves**
- Drain Punch
- Endure
- Counter
- Comet Punch
- Foresight
- Smelling Salts
- Detect
- Wide Guard
- Force Palm
- Reversal
- Mach Punch
- Power-Up Punch

**Tutor Moves**
- Body Slam
- Counter
- Double-Edge
- Dynamic Punch
- Endure
- Fire Punch
- Ice Punch
- Mega Kick
- Mega Punch
- Rock Slide
- Sleep Talk
- Snore
- Swagger
- Thunder Punch
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
#pokemon-tabs-gurdurr-tab-0:checked ~ .pokemon-tab-panels #pokemon-tabs-gurdurr-panel-0 { display: block; }
#pokemon-tabs-gurdurr-tab-1:checked ~ .pokemon-tab-panels #pokemon-tabs-gurdurr-panel-1 { display: block; }
#pokemon-tabs-gurdurr-tab-2:checked ~ .pokemon-tab-panels #pokemon-tabs-gurdurr-panel-2 { display: block; }
</style>
</details>
